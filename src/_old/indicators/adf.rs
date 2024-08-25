use std::iter::Sum;

use crate::data::Series;

use num_traits::Float;

pub trait ADF<T, RHS = Series<T>> {
    fn perform_adf(&self, lag: usize, confidence: AdfConfidence) -> AdfResult<T>;
}

impl<T: Float + Copy + Clone + Sum> ADF<T> for Series<T> {
    fn perform_adf(&self, lag: usize, confidence: AdfConfidence) -> AdfResult<T> {
        let data = Series::from(&self.iter().rev().collect());
        if &lag >= &(data.len() / 2 - 2) {
            panic!("ADF: Maximum lag must be less than (Length / 2 - 2)");
        } else {
            let observations: usize = data.len() - lag - 1;
            let current_difference =
                Series::from(&(0..observations).map(|x| *data[x] - *data[x + 1]).collect());
            let fst_difference = Series::from(&(0..observations).map(|x| *data[x + 1]).collect());
            let constant = Series::of_length(T::from(1.0).unwrap(), observations);

            let mut base_vector: Series<T> = fst_difference.clone();
            let mut columns: usize = 2;
            base_vector.append(&constant);

            //Introduce Lags
            if lag > 0 {
                for n in 1..lag + 1 {
                    base_vector.append_vec(
                        &(0..observations)
                            .into_iter()
                            .map(|i| *data[i + n] - *data[i + n + 1])
                            .collect(),
                    );
                    columns += 1;
                }
            }

            //Regression
            let covariance = calc_covariance(&base_vector, observations, columns);
            let coefficient = covariance.multiply(&current_difference, columns, observations, 1);

            // // Standard Error
            let y_hat = base_vector.multiply(&coefficient, observations, columns, 1);
            let mean_x = fst_difference.average();
            let mse_1: T = (0..observations)
                .map(|i| {
                    (current_difference[i] - y_hat[i]).powi(2)
                        / T::from(observations - columns).unwrap()
                })
                .sum();
            let mse_2 = (0..observations)
                .map(|i| (fst_difference[i] - mean_x).powi(2))
                .sum();
            let sqr_err = (mse_1 / mse_2).sqrt();
            let test_statistic = coefficient[0] / sqr_err;

            let observations_f = observations as f64;
            let critical_value = T::from(match confidence {
                AdfConfidence::_90 => -2.56677 - 1.5384 / observations_f - 2.809 / observations_f / observations_f,
                AdfConfidence::_95 => {
                    -2.86154
                        - 2.8903 / observations_f
                        - 4.234 / observations_f / observations_f
                        - 40.040 / observations_f / observations_f / observations_f
                }
                AdfConfidence::_99 => {
                    -3.43035
                        - 6.5393 / observations_f
                        - 16.786 / observations_f / observations_f
                        - 79.433 / observations_f / observations_f / observations_f
                }
            })
            .unwrap();

            return AdfResult::from(
                test_statistic,
                critical_value,
                observations,
            );
        }
    }
}

fn calc_covariance<T: Copy + Clone + Float + Sum>(
    matrix_a: &Series<T>,
    rows: usize,
    columns: usize,
) -> Series<T> {
    // First find the QR factorization of A: A = QR, where R is upper triangular matrix. Then do Ainv = R^-1*Q^T.
    let (qr_x, qr_y) = qr_diagonal(matrix_a, rows, columns);
    let qr_x_transposed = qr_x.transpose(rows, columns);
    let mut qr_y_inversed = Series::of_length(T::from(0.0).unwrap(), columns * columns);
    qr_y_inversed.matrix_set(
        T::from(1.0).unwrap() / *qr_y.matrix_get(0, 0, columns),
        0,
        0,
        columns,
    );

    if columns != 1 {
        for y in 1..columns {
            for x in 0..y {
                qr_y_inversed.matrix_set(
                    (x..y)
                        .map(|k| {
                            *qr_y_inversed.matrix_get(x, k, columns)
                                * *qr_y.matrix_get(k, y, columns)
                        })
                        .sum(),
                    x,
                    y,
                    columns,
                );
            }

            for x in 0..y {
                qr_y_inversed.matrix_set(
                    -*qr_y_inversed.matrix_get(x, y, columns) / *qr_y.matrix_get(y, y, columns),
                    x,
                    y,
                    columns,
                );
            }
            qr_y_inversed.matrix_set(
                T::from(1.0).unwrap() / *qr_y.matrix_get(y, y, columns),
                y,
                y,
                columns,
            );
        }
    }

    return qr_y_inversed.multiply(&qr_x_transposed, columns, columns, rows);
}

fn qr_diagonal<T: Copy + Clone + Float + Sum>(
    matrix: &Series<T>,
    rows: usize,
    columns: usize,
) -> (Series<T>, Series<T>) {
    let mut q = Series::of_length(T::from(0.0).unwrap(), rows * columns);
    let mut r = Series::of_length(T::from(0.0).unwrap(), columns * columns);
    let mut a = Series::from(&(0..rows).map(|i| *matrix.matrix_get(i, 0, rows)).collect());

    let sqrt = a.normalize_sqrt();
    r.matrix_set(sqrt, 0, 0, rows);

    for i in 0..rows {
        q.matrix_set(a[i] / sqrt, i, 0, rows);
    }

    if columns != 1 {
        for k in 1..columns {
            a = Series::from(&(0..rows).map(|i| *matrix.matrix_get(i, k, rows)).collect());

            for j in 0..k {
                let row_total = (0..rows).map(|i| *q.matrix_get(i, j, rows) * a[i]).sum();
                r.matrix_set(row_total, j, k, columns);

                for i in 0..rows {
                    a[i] = a[i] - row_total * *q.matrix_get(i, j, rows);
                }
            }

            let sqrt = a.normalize_sqrt();
            r.matrix_set(sqrt, k, k, columns);

            for i in 0..rows {
                q.matrix_set(a[i] / sqrt, i, k, rows);
            }
        }
    }

    return (q, r);
}

pub enum AdfConfidence {
    _90,
    _95,
    _99,
}

pub struct AdfResult<T> {
    test_statistic: T,
    critical_value: T,
    _observations: usize,
}

impl<T: std::cmp::PartialOrd> AdfResult<T> {
    pub fn from(test_statistic: T, critical_value: T, observations: usize) -> AdfResult<T> {
        return AdfResult {
            test_statistic,
            critical_value,
            _observations: observations,
        };
    }

    pub fn test_statistic(&self) -> &T {
        return &self.test_statistic;
    }

    pub fn critical_value(&self) -> &T {
        return &self.critical_value;
    }
    // pub fn observations(&self) -> &usize {
    //     return &self.observations;
    // }

    pub fn stationary(&self) -> bool {
        return self.test_statistic < self.critical_value;
    }
}
