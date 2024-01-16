use data::{math::adf, reader};
use util::vec::take_last;
mod data;
mod util;

fn main() {
    if let Err(e) = test_cointegration() {
        eprintln!("{:#?}", e);
    }

    // let test: OHLC = OHLC::from([10.0, 50.0, 5.0, 25.0]);
    // let series: Vec<OHLC> = vec![test];
    // let _transformed = transform_ohlc(&series);
    // let _reverse_transformed = reverse_transform_ohlc(&_transformed);

    // println!("{:#?}", series);
    // println!("{:#?}", _transformed);
    // println!("{:#?}", _reverse_transformed);
}

fn test_cointegration() -> Result<(), Box<dyn std::error::Error>> {
    let window_size = 20;
    let _sol_usd = take_last(reader::read_from_file("./_temp/SOLUSD.csv")?, window_size);
    let _sol_eth = take_last(reader::read_from_file("./_temp/SOLETH.csv")?, window_size);

    let _sol_usd_close: Vec<f64> = _sol_eth.iter().map(|x| *x.close()).collect();
    let beta = adf::calc(&_sol_usd_close);
    let significance_level = 0.05f64;

    if beta < significance_level {
        // data is non-stationary
        println!("non-stationary, take 1 order difference")
    } else {
        println!("stationary, perform johansen cointegration")
        // data is stationary
    }

    return Ok(());
}
