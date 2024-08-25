use num_traits::Float;
use std::iter::Sum;

use crate::data::Series;

pub struct LinRegResult<T: Float + Copy + Clone + Sum> {
    series: Series<T>,
    slope: T,
    intercept: T,
}

impl<T: Float + Copy + Clone + Sum> LinRegResult<T> {
    pub fn slope(&self) -> T {
        return self.slope;
    }

    // pub fn intercept(&self) -> T {
    //     return self.intercept;
    // }

    // pub fn series(&self) -> &Series<T> {
    //     return &self.series;
    // }

    pub fn estimate(&self, x: T) -> T {
        return self.intercept + self.slope * x;
    }

    pub fn risiduals(&self) -> Series<T> {
        return Series::from(
            &self
                .series
                .iter()
                .enumerate()
                .map(|(i, &val)| val - self.estimate(T::from(i).unwrap()))
                .collect(),
        );
    }
}

pub trait LinReg<T: Float + Copy + Clone + Sum> {
    fn lin_reg(&self) -> LinRegResult<T>;
    fn reg_over(&self, other: &Self) -> LinRegResult<T>;
}

impl<T: Float + Copy + Clone + Sum> LinReg<T> for Series<T> {
    fn lin_reg(&self) -> LinRegResult<T> {
        return self.reg_over(&Series::from(
            &(0..self.len())
                .into_iter()
                .map(|x| T::from(x).unwrap())
                .collect(),
        ));
    }

    fn reg_over(&self, other: &Self) -> LinRegResult<T> {
        if self.len() != other.len() {
            panic!("Cannot regress over series with different lengths");
        } else {
            let n = T::from(self.len()).unwrap();

            let mean_x: T = other.iter().map(|x| *x).sum::<T>() / n;
            let mean_y: T = self.iter().map(|x| *x).sum::<T>() / n;

            let numerator: T = other
                .iter()
                .zip(self.iter())
                .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
                .sum::<T>();

            let denominator = other.iter().map(|&xi| (xi - mean_x).powi(2)).sum::<T>();

            let slope = numerator / denominator;
            let intercept = mean_y - slope * mean_x;

            return LinRegResult {
                series: self.clone(),
                slope,
                intercept,
            };
        }
    }
}
