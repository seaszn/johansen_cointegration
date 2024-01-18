use std::{
    iter::Sum,
    ops::{Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug)]
pub struct Series<T: Copy + Clone>(Vec<T>);

pub type FloatSeries = Series<f64>;

impl<T: Clone + Copy> Series<T> {
    pub fn from(data: &Vec<T>) -> Self {
        return Series(data.to_vec());
    }

    pub fn empty() -> Series<T> {
        return Series::from(&vec![]);
    }

    pub fn clone(&self) -> Series<T> {
        return Series(self.0.clone());
    }

    pub fn of_length(value: T, length: usize) -> Series<T> {
        return Series(vec![value; length]);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn append(&mut self, data: &Series<T>) {
        for i in 0..data.len() {
            self.0.push(data[i])
        }
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn append_vec(&mut self, data: &Vec<T>) {
        for i in 0..data.len() {
            self.0.push(data[i])
        }
    }

    pub fn take(&self, start: usize, length: usize) -> Option<Series<T>> {
        let mut result = Self::empty();

        if start + length > self.len() {
            return None;
        }

        for i in start..length + start {
            result.push(self.0[i]);
        }

        return Some(result);
    }

    pub fn _to_vec(&self) -> &Vec<T> {
        return &self.0;
    }
}

impl Series<f64> {
    pub fn normalize_sqrt(&self) -> f64 {
        return (0..self.len())
            .map(|x| self.0[x].powf(2.0))
            .sum::<f64>()
            .sqrt();
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
}

impl Series<i64> {
    pub fn _transpose(&self, rows: usize, columns: usize) -> Series<i64> {
        let mut result = Self::of_length(0, rows * columns);
        for x in 0..rows {
            for y in 0..columns {
                result.matrix_set(*self.matrix_get(x, y, rows), y, x, columns);
            }
        }

        return result;
    }

    pub fn _multiply(
        &self,
        other: &Self,
        rows_a: usize,
        cols_a: usize,
        cols_b: usize,
    ) -> Series<i64> {
        let mut result = Self::of_length(0, rows_a * cols_b);
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
}

impl<T: Sum + Div<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone + From<i32>>
    Series<T>
{
    pub fn sum(&self) -> T {
        return self.0.iter().map(|x| *x).sum();
    }

    pub fn average(&self) -> T {
        let len = self.len() as i32;
        return self.sum() / T::from(len);
    }

    pub fn matrix_get(&self, x: usize, y: usize, rows: usize) -> &T {
        return &self.0[x + rows * y];
    }

    pub fn matrix_set(&mut self, value: T, x: usize, y: usize, rows: usize) -> &Series<T> {
        self.0[x + rows * y] = value;

        return self;
    }
}

impl<T: Copy + Clone> Index<usize> for Series<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<T: Copy + Clone> IndexMut<usize> for Series<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<T: Copy + Clone> IntoIterator for Series<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        return self.0.into_iter();
    }
}

pub trait TimeSeries {
    fn time(&self) -> Vec<i64>;
    fn open(&self) -> Series<f64>;
    fn high(&self) -> Series<f64>;
    fn low(&self) -> Series<f64>;
    fn close(&self) -> Series<f64>;
}
