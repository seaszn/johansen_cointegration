use data::TimeSeries;

use crate::adf::ADF;
mod adf;
mod data;
mod market;

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
    // let window_size = 100;
    let solusd = market::stream::from_file("./_temp/SOLUSD.csv")?
        .close()
        .take_last(100);
    // let soleth = market::stream::from_file("./_temp/SOLETH.csv")?.close();
    // let coefficient: Vec<f64> = solusd.iter().zip(&soleth).map(|(&a, &b)| a / b).collect();

    if let Ok((test_statistic, critical_value, nobs)) = solusd.perform_adf(0, adf::AdfConfidence::_90) {
        println!("{}", test_statistic);
        println!("{}", critical_value);
        println!("{}", nobs);
        println!("{}", test_statistic < critical_value);
    }

    // let mut lag = 0;
    // let mut beta = 0.0;
    // let mut price_data = take_last(&original_data, window_size);
    // while beta > significance_level || lag == 0 {
    //     (_, beta) = adf::calc(&price_data);

    //     if beta > significance_level {
    //         lag += 1;
    //         price_data = difference(&take_last(&original_data, window_size + lag), lag);
    //     } else {
    //         break;
    //     }
    // }

    // println!("time series stationary with differencing of {}", lag);

    return Ok(());
}
