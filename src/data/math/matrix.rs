
pub type Matrix = Vec<Vec<f64>>;

pub fn transpose(matrix: &Matrix) -> Matrix {
    return matrix[0]
        .iter()
        .enumerate()
        .map(|(i, _)| matrix.iter().map(|row| row[i]).collect::<Vec<f64>>())
        .collect::<Matrix>();
}

pub fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
    return a
        .iter()
        .map(|row| {
            transpose(b)
                .iter()
                .map(|col| {
                    row.iter()
                        .enumerate()
                        .map(|(i, elm)| elm * col[i])
                        .sum::<f64>()
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
}

pub fn invert(matrix: &Matrix) -> Matrix {
    return matrix
        .iter()
        .map(|row| row.iter().map(|elm| 1.0 / elm).collect::<Vec<f64>>())
        .collect();
}
