use std::error::Error;
use super::types::OHLC;

pub fn read_from_file(path: &str) -> Result<Vec<OHLC>, Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;
    let mut parse_result: Vec<OHLC> = vec![];

    for result in reader.records(){
        let record = result?;
        parse_result.push(record.deserialize::<OHLC>(None)?);
    }

    return Ok(parse_result);
}