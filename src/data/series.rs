use std::{
    iter::Sum,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug)]
pub struct Series<T>(Vec<T>);

impl Series<f64> {
    pub fn from(data: &Vec<f64>) -> Self {
        return Series(data.to_vec());
    }

    pub fn clone(&self) -> Series<f64> {
        return Series(self.0.clone());
    }

    pub fn of_length(value: f64, length: usize) -> Series<f64> {
        return Series(vec![value; length]);
    }

    pub fn append(&mut self, data: &Series<f64>) {
        for i in 0..data.len() {
            self.0.push(data[i])
        }
    }

    pub fn append_vec(&mut self, data: &Vec<f64>) {
        for i in 0..data.len() {
            self.0.push(data[i])
        }
    }

    pub fn normalize_sqrt(&self) -> f64 {
        return (0..self.len())
            .map(|x| self.0[x].powf(2.0))
            .sum::<f64>()
            .sqrt();
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn matrix_set(&mut self, value: f64, x: usize, y: usize, rows: usize) -> &Series<f64> {
        self.0[x + rows * y] = value;

        return self;
    }

    pub fn matrix_get(&self, x: usize, y: usize, rows: usize) -> &f64 {
        return &self.0[x + rows * y];
    }

    pub fn transpose(&self, rows: usize, columns: usize) -> Series<f64> {
        let mut result = Self::of_length(0.0, rows * columns);
        for x in 0..rows {
            for y in 0..columns {
                result.matrix_set(*self.matrix_get(x, y, rows), y, x, columns);
            }
        }

        return result;
    }

    pub fn sum(&self) -> f64 {
        return self.0.iter().map(|x| *x).sum();
    }

    pub fn average(&self) -> f64 {
        return self.sum() / (self.len() as f64);
    }

    pub fn multiply(
        &self,
        other: &Self,
        rows_a: usize,
        cols_a: usize,
        cols_b: usize,
    ) -> Series<f64> {
        let mut result = Self::of_length(0.0, rows_a * cols_b);
        let rows_b = cols_a;

        for i in 0..rows_a {
            for j in 0..cols_b {
                result.matrix_set(
                    (0..cols_a)
                        .map(|k| self.matrix_get(i, k, rows_a) * other.matrix_get(k, j, rows_b))
                        .sum(),
                    i,
                    j,
                    rows_a,
                );
            }
        }

        return result;
    }

    pub fn take_last(&self, lenght: usize) -> Series<f64> {
        return Series::from(&self.0.iter().rev().take(lenght).rev().map(|&x| x).collect());
    }
}

impl<T> Index<usize> for Series<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T> IndexMut<usize> for Series<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T> IntoIterator for Series<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.0.into_iter();
    }
}

pub trait Numerical: Sub + Add + Mul + Div + Sized + Clone + Copy + Sum + From<i32> {}

impl Numerical for i128 {}
impl Numerical for i64 {}
impl Numerical for f64 {}
impl Numerical for i32 {}

pub trait TimeSeries {
    fn time(&self) -> Vec<i64>;
    fn open(&self) -> Series<f64>;
    fn high(&self) -> Series<f64>;
    fn low(&self) -> Series<f64>;
    fn close(&self) -> Series<f64>;
}
