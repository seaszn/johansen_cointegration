// use crate::market::types::OHLC;


// pub fn transform_ohlc(data: &Vec<OHLC>) -> Vec<OHLC> {
//     return sanitize_ohlc(data)
//         .iter()
//         .map(|x| {
//             let y_1 = x.low().log2();
//             let y_2 = (x.high() - x.low()).log2();

//             let open_proxy = (x.open() - x.low()) / (x.high() - x.low());
//             let close_proxy = (x.close() - x.low()) / (x.high() - x.low());

//             let y_3 = (open_proxy / (1.0 - open_proxy)).log2();
//             let y_4 = (close_proxy / (1.0 - close_proxy)).log2();
//             return OHLC::new(y_1, y_2, y_3, y_4, x.time_string());
//         })
//         .collect();
// }

// pub fn reverse_transform_ohlc(data: &Vec<OHLC>) -> Vec<OHLC> {
//     return data
//         .iter()
//         .map(|x| {
//             let low = x.open().exp2();
//             let high = low + x.high().exp2();

//             let open_proxy = x.low().exp2() / (1.0 + x.low().exp2());
//             let close_proxy = x.close().exp2() / (1.0 + x.close().exp2());

//             let open = open_proxy * high + (1.0 - open_proxy) * low;
//             let close = close_proxy * high + (1.0 - close_proxy) * low;
//             return OHLC::new(open, high, low, close, x.time_string());
//         })
//         .collect();
// }

// fn sanitize_ohlc(data: &Vec<OHLC>) -> Vec<OHLC> {
//     return data
//         .iter()
//         .filter(|x| {
//             if x.open().eq(&0.0) && x.high().eq(&0.0) && x.low().eq(&0.0) && x.close().eq(&0.0) {
//                 return false;
//             }
//             return true;
//         })
//         .map(|x| x.clone())
//         .collect();
// }
