use crate::data::{Series, FloatSeries};

pub enum AdfConfidence {
    _90,
    _95,
    _99,
}

pub trait ADF<T, RHS = Series<T>> {
    fn perform_adf(
        &self,
        lag: usize,
        confidence: AdfConfidence,
    ) -> Result<(f64, f64, usize), String>;
}

impl ADF<f64> for Series<f64> {
    fn perform_adf(
        &self,
        lag: usize,
        confidence: AdfConfidence,
    ) -> Result<(f64, f64, usize), String> {
        let data = Series::from(&self.iter().rev().collect());
        if &lag >= &(data.len() / 2 - 2) {
            return Err("ADF: Maximum lag must be less than (Length/2 - 2)".to_string());
        } else {
            let observations: usize = data.len() - lag - 1;
            let current_difference =
                FloatSeries::from(&(0..observations).map(|x| data[x] - data[x + 1]).collect());
            let fst_difference =
                FloatSeries::from(&(0..observations).map(|x| *data[x + 1]).collect());
            let constant = FloatSeries::of_length(1.0, observations);

            let mut base_vector: Series<f64> = fst_difference.clone();
            let mut columns: usize = 2;
            base_vector.append(&constant);

            //Introduce Lags
            if lag > 0 {
                for n in 1..lag + 1 {
                    base_vector.append_vec(
                        &(0..observations)
                            .into_iter()
                            .map(|i| data[i + n] - data[i + n + 1])
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
            let mse_1: f64 = (0..observations)
                .map(|i| (current_difference[i] - y_hat[i]).powf(2.0) / (observations - columns) as f64)
                .sum();
            let mse_2: f64 = (0..observations)
                .map(|i| (fst_difference[i] - mean_x).powf(2.0))
                .sum();
            let sqr_err = (mse_1 / mse_2).sqrt();
            let test_statistic = coefficient[0] / sqr_err;

            let nobsf = observations as f64;
            let crit = match confidence {
                AdfConfidence::_90 => -2.56677 - 1.5384 / nobsf - 2.809 / nobsf / nobsf,
                AdfConfidence::_95 => {
                    -2.86154
                        - 2.8903 / nobsf
                        - 4.234 / nobsf / nobsf
                        - 40.040 / nobsf / nobsf / nobsf
                }
                AdfConfidence::_99 => {
                    -3.43035
                        - 6.5393 / nobsf
                        - 16.786 / nobsf / nobsf
                        - 79.433 / nobsf / nobsf / nobsf
                }
            };

            return Ok((test_statistic, crit, observations));
        }
    }
}

fn calc_covariance(matrix_a: &FloatSeries, rows: usize, columns: usize) -> FloatSeries {
    // First find the QR factorization of A: A = QR, where R is upper triangular matrix. Then do Ainv = R^-1*Q^T.
    let (qr_x, qr_y) = qr_diagonal(matrix_a, rows, columns);
    let qr_x_transposed = qr_x.transpose(rows, columns);
    let mut qr_y_inversed = FloatSeries::of_length(0.0, columns * columns);
    qr_y_inversed.matrix_set(1.0 / qr_y.matrix_get(0, 0, columns), 0, 0, columns);

    if columns != 1 {
        for y in 1..columns {
            for x in 0..y {
                qr_y_inversed.matrix_set(
                    (x..y)
                        .map(|k| {
                            qr_y_inversed.matrix_get(x, k, columns) * qr_y.matrix_get(k, y, columns)
                        })
                        .sum(),
                    x,
                    y,
                    columns,
                );
            }

            for x in 0..y {
                qr_y_inversed.matrix_set(
                    -qr_y_inversed.matrix_get(x, y, columns) / qr_y.matrix_get(y, y, columns),
                    x,
                    y,
                    columns,
                );
            }
            qr_y_inversed.matrix_set(1.0 / qr_y.matrix_get(y, y, columns), y, y, columns);
        }
    }

    return qr_y_inversed.multiply(&qr_x_transposed, columns, columns, rows);
}

fn qr_diagonal(matrix: &FloatSeries, rows: usize, columns: usize) -> (FloatSeries, FloatSeries) {
    let mut q = FloatSeries::of_length(0.0, rows * columns);
    let mut r = FloatSeries::of_length(0.0, columns * columns);
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
                let row_total: f64 = (0..rows).map(|i| q.matrix_get(i, j, rows) * a[i]).sum();
                r.matrix_set(row_total, j, k, columns);

                for i in 0..rows {
                    a[i] -= row_total * q.matrix_get(i, j, rows);
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

