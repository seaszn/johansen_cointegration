use std::io::Error;

pub enum ConfidenceLevel {
    _90,
    _95,
    _99,
}

pub fn calc(a: &Vec<f64>, lag: usize, confidence: ConfidenceLevel) -> Result<(), Error> {
    if lag >= a.len() / 2 - 2 {
        return Err(Error::new(
            std::io::ErrorKind::Unsupported,
            "ADF maximum lag must be less than (length / 2 - 2)",
        ));
    } else {
        let nobs = a.len() - lag - 1;
        let mut y: Vec<f64> = vec![];
        let mut x: Vec<f64> = vec![];
        let mut x0: Vec<f64> = vec![];

        for i in 0..nobs {
            y.push(a[i] - a[i + 1]);
            x.push(a[i + 1]);
            x0.push(1.0)
        }

        let mut X = [x.clone(), x0].concat();
        let mut M = 2;

        // // Introduce Lags
        if lag > 0 {
            for n in 1..lag + 1 {
                for i in 0..nobs {
                    X.push(a[i + n] - a[i + n + 1]);
                }

                M += 2;
            }
        }


        // // Regression
        let c = pinv(&X, nobs, M);
        // let coeff = multiply(&c, &y, M, nobs, 1);

        // // Standard Error
        // let y_hat = multiply(&X, &coeff, nobs, M, 1);
        // let mean_x: f64 = x.iter().sum::<f64>() / (x.len() as f64);

        // let sum_1: f64 = (0..(nobs - 1))
        //     .map(|i| ((y[i] - y_hat[i]).powf(2.0)) / ((nobs - M) as f64))
        //     .sum();
        // let sum_2: f64 = (0..(nobs - 1)).map(|i| (x[i] - mean_x).powf(2.0)).sum();
        // let se = (sum_1 / sum_2).sqrt();

        // println!("{:#?}", se);
        // let adf = coeff[0] / se;
        // let nobsf = nobs as f64;

        // let crit = match confidence {
        //     ConfidenceLevel::_90 => -2.56677 - 1.5384 / nobsf - 2.809 / nobsf / nobsf,
        //     ConfidenceLevel::_95 => {
        //         -2.86154 - 2.8903 / nobsf - 4.234 / nobsf / nobsf - 40.040 / nobsf / nobsf / nobsf
        //     }
        //     ConfidenceLevel::_99 => {
        //         -3.43035 - 6.5393 / nobsf - 16.786 / nobsf / nobsf - 79.433 / nobsf / nobsf / nobsf
        //     }
        // };

        // // println!("{:#?}", adf);
        // println!("{:#?}", crit);

        println!("{}", c.len());

        Ok(())
    }
}

// confirmed
fn pinv(A: &Vec<f64>, rows: usize, columns: usize) -> Vec<f64> {
    let (Q, R) = qr_diag(&mut A.clone(), rows, columns);
    let QT = transpose(&Q, rows, columns);

    let mut rinv: Vec<f64> = vec![0.0; columns * columns];

    matrix_set(
        &mut rinv,
        1.0 / matrix_get(&R, 0, 0, columns),
        0,
        0,
        columns,
    );

    if columns != 1 {
        for j in 1..columns - 1 {
            for i in 0..j - 1 {
                let mut r = 0.0;
                for k in i..j - 1 {
                    r += matrix_get(&rinv, i, k, columns) * matrix_get(&R, k, j, columns)
                }
                matrix_set(&mut rinv, r, i, j, columns);
            }
            for k in 0..j - 1 {
                let val = rinv.clone();
                matrix_set(
                    &mut rinv,
                    -matrix_get(&val, k, j, columns) / matrix_get(&R, j, j, columns),
                    k,
                    j,
                    columns,
                );
            }
            matrix_set(
                &mut rinv,
                1.0 / matrix_get(&R, j, j, columns),
                j,
                j,
                columns,
            );
            // matrix_set(rinv, value, row, column, rows)
        }
    }

    return multiply(&rinv, &QT, columns, columns, rows);
}

// confirmed
fn qr_diag(data: &mut Vec<f64>, rows: usize, columns: usize) -> (Vec<f64>, Vec<f64>) {
    // let mut aux = 0.0;
    let mut Q: Vec<f64> = vec![0.0; rows * columns];
    let mut R: Vec<f64> = vec![0.0; columns * columns];
    let mut a: Vec<f64> = vec![0.0; rows];
    let mut q: Vec<f64> = vec![0.0; rows];
    let mut aux = 0.0;

    // Get first column and its normalized
    for i in 0..(rows - 1) {
        a[i] = *matrix_get(data, i, 0, rows)
    }

    let mut r = vnorm(&a);
    matrix_set(&mut R, r, 0, 0, columns);

    for i in 0..(rows - 1) {
        matrix_set(&mut Q, a[i] / r, i, 0, rows);
    }

    if columns != 1 {
        for k in 1..columns - 1 {
            for i in 0..rows - 1 {
                a[i] = *matrix_get(&data, i, k, rows);
            }
            for j in 0..k - 1 {
                r = 0.0;
                for i in 0..rows - 1 {
                    r += matrix_get(&Q, i, j, rows);
                }
                matrix_set(&mut R, r, j, k, columns);

                for i in 0..rows - 1 {
                    aux = a[i] - r * matrix_get(&Q, i, j, rows);
                    a[i] = aux;
                }
            }

            r = vnorm(&a);
            matrix_set(&mut R, r, k, k, columns);

            for i in 0..rows - 1 {
                matrix_set(&mut Q, a[i] / r, i, k, rows);
            }
        }
    }

    return (Q, R);
}

// confirmed
fn multiply(
    a: &Vec<f64>,
    b: &Vec<f64>,
    rows_a: usize,
    columns_a: usize,
    columns_b: usize,
) -> Vec<f64> {
    let mut c: Vec<f64> = vec![0.0; rows_a * columns_b];
    let rows_b = columns_a;
    let mut element_c = 0.0;

    for i in 0..rows_a - 1 {
        for j in 0..columns_b - 1 {
            element_c = 0.0;

            for k in 0..columns_a - 1 {
                element_c += matrix_get(a, i, k, rows_a) * matrix_get(b, k, j, rows_b)
            }

            matrix_set(&mut c, element_c, i, j, rows_a);
        }
    }

    return c;
}

// confirmed
fn transpose(data: &Vec<f64>, rows: usize, columns: usize) -> Vec<f64> {
    let mut at: Vec<f64> = vec![0.0; rows * columns];
    for i in 0..rows - 1 {
        for j in 0..columns - 1 {
            matrix_set(&mut at, *matrix_get(data, i, j, rows), j, i, columns);
        }
    }

    return at;
}

// confirmed
fn vnorm(X: &Vec<f64>) -> f64 {
    return (0..X.len() - 1).map(|i| X[i].powf(2.0)).sum::<f64>().sqrt();
}

// confirmed
fn matrix_get(data: &Vec<f64>, i: usize, j: usize, rows: usize) -> &f64 {
    return &data[i + rows * j];
}

// confirmed
fn matrix_set(data: &mut Vec<f64>, value: f64, i: usize, cojumn: usize, rows: usize) -> &Vec<f64> {
    data[i + rows * cojumn] = value;

    return data;
}
