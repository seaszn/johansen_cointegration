use chrono::NaiveDateTime;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct OHLC {
    time: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

impl OHLC {
    pub fn time(&self) -> f64{
        return NaiveDateTime::parse_from_str(&format!("{} 00:00:00", &self.time), "%Y-%m-%d %H:%M:%S").unwrap().timestamp() as f64;
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

    pub fn as_tupple(&self) -> (f64, f64){
        return (self.time(), self.close);
    } 

    // pub fn from(vector: [f64; 4]) -> OHLC {
    //     return OHLC {
    //         open: vector[0],
    //         high: vector[1],
    //         low: vector[2],
    //         close: vector[3],
    //         time: todo!(),
    //     }
    // }
}
