// mod indicators;

// use data::TimeSeries;
// use indicators::adf::{self, ADF};

// use crate::indicators::lin_reg::LinReg;

mod data;
mod market;

fn main(){

}

// fn main() {
//     if let Err(e) = test_cointegration() {
//         eprintln!("{:#?}", e);
//     }

//     // let test: OHLC = OHLC::from([10.0, 50.0, 5.0, 25.0]);
//     // let series: Vec<OHLC> = vec![test];
//     // let _transformed = transform_ohlc(&series);
//     // let _reverse_transformed = reverse_transform_ohlc(&_transformed);

//     // println!("{:#?}", series);
//     // println!("{:#?}", _transformed);
//     // println!("{:#?}", _reverse_transformed);
// }

// fn test_cointegration() -> Result<(), Box<dyn std::error::Error>> {
//     let window_size = 500;
//     let max_lag = 30;

//     let solusd = market::stream::from_file("./_temp/SOLUSD.csv")?.close();
//     let soleth = market::stream::from_file("./_temp/SOLETH.csv")?.close();
//     let mut lag = 0;

//     // Run cointegration test with a lag from 0, to the set max lag to itterate over multiple dimensions of statinarity
//     // If stationarity is found and all conditions are met, perform the Engle-Granger cointegration test and break the loop
//     while lag < max_lag {
//         // The the difference, or RoC, from both the time series with the specific length to ensure stationarity
//         let transformed_solusd = solusd.lag_differenced(lag).take_last(window_size);
//         let transformed_soleth = soleth.lag_differenced(lag).take_last(window_size);

//         // Perform ADF test on both the nominal and relative time series
//         let nominal_adf_result = transformed_solusd.perform_adf(0, adf::AdfConfidence::_90);
//         let relative_adf_result = transformed_soleth.perform_adf(0, adf::AdfConfidence::_90);

//         if !nominal_adf_result.stationary() && !relative_adf_result.stationary() {
//             // Check if the nominal residuals are stationary
//             let nominal_risduals_stationary = transformed_solusd
//                 .lin_reg()
//                 .risiduals()
//                 .perform_adf(0, adf::AdfConfidence::_90)
//                 .stationary();

//             // Check if the relative residuals are stationary
//             let relative_risduals_stationary = transformed_soleth
//                 .lin_reg()
//                 .risiduals()
//                 .perform_adf(0, adf::AdfConfidence::_90)
//                 .stationary();
            
//             // If both the nominal and relative risiduals are stationary, perform Engle-Granger test
//             if nominal_risduals_stationary && relative_risduals_stationary {
//                 let s = transformed_soleth.reg_over(&transformed_solusd);
//                 let adf= s.risiduals().perform_adf(0, adf::AdfConfidence::_90);
//                 let adf_osc = adf.test_statistic() - adf.critical_value();

//                 println!("both risiduals stationary on lag order {}, ({})", lag, adf_osc);

//                 // //Check if the regression is stationary, if so, we test for the Null Hypothisis
//                 // if s.risiduals()
//                 //     .perform_adf(0, adf::AdfConfidence::_90)
//                 //     .stationary()
//                 // {
//                 //     let b_1 = s.slope();
//                     // break;
//                 // }
//             }
//         }

//         lag += 1;
//     }

//     // let usd_nominated = RollingWindow::from(&solusd, window_size);
//     // let eth_nominated = RollingWindow::from(&soleth, window_size);

//     // // usd_nominated.last_window();

//     // for (nominal, relative) in usd_nominated.zip(eth_nominated) {

//     // let mut lag = 0;
//     // let mut beta = 0.0;
//     // let mut price_data = take_last(&original_data, window_size);
//     // while beta > significance_level || lag == 0 {
//     //     (_, beta) = adf::calc(&price_data);

//     //     if beta > significance_level {
//     //         lag += 1;
//     //         price_data = difference(&take_last(&original_data, window_size + lag), lag);
//     //     } else {
//     //         break;
//     //     }
//     // }

//     // println!("time series stationary with differencing of {}", lag);

//     return Ok(());
// }
