use std::ops::{Add, Mul, Sub, Div};

use serde::Serialize;

pub trait Numeric:
    Copy
    + Default
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::iter::Sum
{
}

#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    T: Numeric,
{
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Numeric,
{
    fn new(rows: usize, columns: usize, data: Vec<Vec<T>>) -> Self {
        assert_eq!(data.len(), rows);

        for row in &data {
            assert_eq!(row.len(), columns);
        }

        return Matrix {
            rows,
            columns,
            data,
        };
    }
}

impl<T> Add for Matrix<T>
where
    T: Numeric,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.columns, rhs.columns);

        return Matrix::new(
            self.rows,
            self.columns,
            (0..self.rows)
                .map(|i| {
                    (0..self.columns)
                        .map(|j| self.data[i][j] + rhs.data[i][j])
                        .collect::<Vec<T>>()
                })
                .collect(),
        );
    }
}

impl<T> Sub for Matrix<T>
where
    T: Numeric,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.columns, rhs.columns);

        return Matrix::new(
            self.rows,
            self.columns,
            (0..self.rows)
                .map(|i| {
                    (0..self.columns)
                        .map(|j| self.data[i][j] - rhs.data[i][j])
                        .collect::<Vec<T>>()
                })
                .collect(),
        );
    }
}


impl<T> Div for Matrix<T>
where
    T: Numeric,
{
    type Output = Matrix<T>;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.columns, rhs.columns);

        return Matrix::new(
            self.rows,
            self.columns,
            (0..self.rows)
                .map(|i| {
                    (0..self.columns)
                        .map(|j| self.data[i][j] / rhs.data[i][j])
                        .collect::<Vec<T>>()
                })
                .collect(),
        );
    }
}

impl<T> Mul for Matrix<T>
where
    T: Numeric,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.columns, rhs.rows);

        return Matrix::new(
            self.rows,
            self.columns,
            (0..self.rows)
                .map(|i| {
                    (0..rhs.columns)
                        .map(|j| {
                            (0..self.columns)
                                .map(|k| self.data[i][k] * rhs.data[k][j])
                                .sum()
                        })
                        .collect::<Vec<T>>()
                })
                .collect(),
        );
    }
}
