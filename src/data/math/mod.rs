// mod linear::linear;
pub mod adf;
pub mod matrix;

// #[derive(Debug, Clone)]
// pub struct RegressionResult {
//     r_squared: f64,
//     variance: f64,
//     covariance: f64,
//     ssr: f64,
//     sst: f64,
//     slope: f64,
//     intercept: f64,
//     dataset: Vec<(f64, f64)>,
// }

// impl RegressionResult {
//     pub fn new(
//         r_squared: f64,
//         variance: f64,
//         covariance: f64,
//         ssr: f64,
//         sst: f64,
//         slope: f64,
//         intercept: f64,
//         dataset: Vec<(f64, f64)>,
//     ) -> RegressionResult {
//         return RegressionResult {
//             r_squared,
//             variance,
//             covariance,
//             ssr,
//             sst,
//             slope,
//             intercept,
//             dataset,
//         };
//     }

//     pub fn r_squared(&self) -> &f64 {
//         &self.r_squared
//     }

//     pub fn variance(&self) -> &f64 {
//         &self.variance
//     }

//     pub fn covariance(&self) -> &f64 {
//         &self.covariance
//     }

//     pub fn ssr(&self) -> &f64{
//         &self.ssr
//     }

//     pub fn sst(&self) -> &f64{
//         &self.sst
//     }

//     pub fn slope(&self) -> &f64 {
//         &self.slope
//     }

//     pub fn intercept(&self) -> &f64 {
//         &self.intercept
//     }

//     pub fn data(&self) -> &Vec<(f64, f64)> {
//         &self.dataset
//     }
// }
