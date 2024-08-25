// use super::Series;

// pub struct RollingWindow<'a> {
//     series: &'a Series<f64>,
//     window_size: usize,
//     position: usize,
//     padding: usize,
// }

// impl<'a> RollingWindow<'a> {
//     pub fn from(series: &Series<f64>, window_size: usize) -> RollingWindow {
//         if series.len() < window_size {}

//         return RollingWindow {
//             series,
//             window_size,
//             position: 0,
//             padding: 0,
//         };
//     }

//     // pub fn last_window(&self){
//     //     // let risidual 
//     //     println!("{}", self.series.len());
//     //     println!("{}", self.window_size);
//     //     println!("{}", self.series.len() % self.window_size);
//     // }
// }

// impl<'a> Iterator for RollingWindow<'a> {
//     type Item = Series<f64>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.position += 1;
//         return self.series.take(self.position - 1, self.window_size);
//     }
// }
