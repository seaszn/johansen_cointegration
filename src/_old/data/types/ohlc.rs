#[derive(Clone, Debug, serde::Deserialize)]
pub struct OHLC {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl OHLC {
    // pub fn time(&self) -> f64 {
    //     return NaiveDateTime::parse_from_str(
    //         &format!("{} 00:00:00", &self.time),
    //         "%Y-%m-%d %H:%M:%S",
    //     )
    //     .unwrap()
    //     .timestamp() as f64;
    // }

    pub fn time_string(&self) -> String {
        return self.time.clone();
    }

    pub fn open(&self) -> &f64 {
        return &self.open;
    }

    pub fn high(&self) -> &f64 {
        return &self.high;
    }
    pub fn low(&self) -> &f64 {
        return &self.low;
    }
    pub fn close(&self) -> &f64 {
        return &self.close;
    }

    pub fn new(open: f64, high: f64, low: f64, close: f64, time: String) -> OHLC {
        return OHLC {
            open,
            high,
            low,
            close,
            time,
        };
    }
}
