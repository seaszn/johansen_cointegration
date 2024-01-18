use crate::data::{self, TimeSeries};

use super::types::OHLC;

pub fn from_file(path: &str) -> Result<impl TimeSeries, Box<dyn std::error::Error>> {
    let mut result = data::read_from_csv::<OHLC>(path)?;
    result.sort_by(|a, b| a.time().partial_cmp(&b.time()).unwrap());

    return Ok(result);
}
