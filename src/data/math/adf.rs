use super::matrix::{invert, multiply, transpose};

pub fn calc(data: &Vec<f64>) -> f64 {
    let differences: Vec<f64> = (1..(data.len() - 1))
        .into_iter()
        .map(|x| data.get(x).unwrap() - data.get(x - 1).unwrap())
        .collect();

    let lagged_level = data.split_last().unwrap().1.to_vec();
    let matrix: Matrix = lagged_level
        .iter()
        .zip(&differences)
        .map(|(&a, &b)| [1.0, a, b].to_vec())
        .collect();

    let regression: Matrix = linear_regression(&matrix, &differences);
    return *regression.get(1).unwrap().get(0).unwrap();
}

pub type Matrix = Vec<Vec<f64>>;

fn linear_regression(matrix: &Matrix, dependent: &Vec<f64>) -> Matrix {
    let transposed = transpose(matrix);
    let multiplied = multiply(&transposed, matrix);
    let inverted = invert(&multiplied);
    let product = multiply(&inverted, &transposed);
    return multiply(
        &product,
        &dependent.iter().map(|&x| vec![x]).collect::<Matrix>(),
    );
}

// fn transpose(matrix: &Matrix) -> Matrix {
//     return matrix[0]
//         .iter()
//         .enumerate()
//         .map(|(i, _)| matrix.iter().map(|row| row[i]).collect::<Vec<f64>>())
//         .collect::<Matrix>();
// }

// fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
//     return a
//         .iter()
//         .map(|row| {
//             transpose(b)
//                 .iter()
//                 .map(|col| {
//                     row.iter()
//                         .enumerate()
//                         .map(|(i, elm)| elm * col[i])
//                         .sum::<f64>()
//                 })
//                 .collect::<Vec<f64>>()
//         })
//         .collect::<Vec<Vec<f64>>>();
// }

// fn invert(matrix: &Matrix) -> Matrix {
//     return matrix
//         .iter()
//         .map(|row| row.iter().map(|elm| 1.0 / elm).collect::<Vec<f64>>())
//         .collect();
// }
