use super::Series;

pub struct RollingWindow<'a> {
    series: &'a Series<f64>,
    window_size: usize,
    position: usize,
}

impl<'a> RollingWindow<'a> {
    pub fn from(series: &Series<f64>, window_size: usize) -> RollingWindow {
        if series.len() < window_size {}

        return RollingWindow {
            series,
            window_size,
            position: 0,
        };
    }


}

impl<'a> Iterator for RollingWindow<'a> {
    type Item = Series<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        self.position += 1;
        return self.series.take(self.position - 1, self.window_size);
    }
}
