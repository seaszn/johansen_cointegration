use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

use crate::data::{TimeSeries, Series};


#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
pub struct OHLC {
    #[serde(deserialize_with = "deserialize_time_string")]
    time: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

fn deserialize_time_string<'de, D: Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    let deserialized: Option<String> = Deserialize::deserialize(d)?;

    match deserialized {
        Some(time_string) => Ok(NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", time_string),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
        .timestamp()),
        None => return Ok(0),
    }
}

impl OHLC {
    pub fn time(&self) -> i64 {
        return self.time;
    }

    // pub fn open(&self) -> &f64 {
    //     return &self.open;
    // }

    // pub fn high(&self) -> &f64 {
    //     return &self.high;
    // }
    // pub fn low(&self) -> &f64 {
    //     return &self.low;
    // }
    // pub fn close(&self) -> &f64 {
    //     return &self.close;
    // }
}

impl Add for OHLC {
    type Output = OHLC;
    fn add(self, rhs: Self) -> Self::Output {
        return OHLC {
            time: self.time,
            open: self.open + rhs.open,
            high: self.high + rhs.high,
            low: self.low + rhs.low,
            close: self.close + rhs.close,
        };
    }
}

impl AddAssign for OHLC {
    fn add_assign(&mut self, rhs: Self) {
        self.open += rhs.open;
        self.high += rhs.high;
        self.low += rhs.low;
        self.close += rhs.close;
    }
}

impl Sub for OHLC {
    type Output = OHLC;

    fn sub(self, rhs: Self) -> Self::Output {
        return OHLC {
            time: self.time,
            open: self.open - rhs.open,
            high: self.high - rhs.high,
            low: self.low - rhs.low,
            close: self.close - rhs.close,
        };
    }
}

impl SubAssign for OHLC {
    fn sub_assign(&mut self, rhs: Self) {
        self.open -= rhs.open;
        self.high -= rhs.high;
        self.low -= rhs.low;
        self.close -= rhs.close;
    }
}

impl Mul for OHLC {
    type Output = OHLC;
    fn mul(self, rhs: Self) -> Self::Output {
        return OHLC {
            time: self.time,
            open: self.open * rhs.open,
            high: self.high * rhs.high,
            low: self.low * rhs.low,
            close: self.close * rhs.close,
        };
    }
}

impl MulAssign for OHLC {
    fn mul_assign(&mut self, rhs: Self) {
        self.open *= rhs.open;
        self.high *= rhs.high;
        self.low *= rhs.low;
        self.close *= rhs.close;
    }
}

impl Div for OHLC {
    type Output = OHLC;
    fn div(self, rhs: Self) -> Self::Output {
        return OHLC {
            time: self.time,
            open: self.open / rhs.open,
            high: self.high / rhs.high,
            low: self.low / rhs.low,
            close: self.close / rhs.close,
        };
    }
}

impl DivAssign for OHLC {
    fn div_assign(&mut self, rhs: Self) {
        self.open /= rhs.open;
        self.high /= rhs.high;
        self.low /= rhs.low;
        self.close /= rhs.close;
    }
}

impl TimeSeries for Vec<OHLC> {
    fn time(&self) -> Vec<i64> {
        return self.iter().map(|x| x.time).collect();
    }
    fn open(&self) -> Series<f64> {
        return Series::from(&self.iter().map(|x| x.open).collect())
    }
    fn high(&self) -> Series<f64> {
        return Series::from(&self.iter().map(|x| x.high).collect())
    }
    fn low(&self) -> Series<f64> {
        return Series::from(&self.iter().map(|x| x.low).collect())
    }
    fn close(&self) -> Series<f64>{
        return Series::from(&self.iter().map(|x| x.close).collect())
    }
}