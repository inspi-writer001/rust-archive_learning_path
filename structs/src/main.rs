use std::fmt::format;

struct Candle {
    ticker: String,
    open: i64,
    high: i64,
    low: i64,
    close: i64,
    decimals: u32,
}

trait ICandle {
    fn convert_value_to_f64(&self, value: i64) -> f64;
    fn get_delta(&self) -> f64;
    fn get_info(&self) -> String;
}

impl ICandle for Candle {
    fn convert_value_to_f64(&self, value: i64) -> f64 {
        value as f64 / (10_i64.pow(self.decimals)) as f64
    }
    fn get_delta(&self) -> f64 {
        self.convert_value_to_f64(self.close - self.open)
    }
    fn get_info(&self) -> String {
        format!(
            "{} ({} {} {} {})",
            self.ticker,
            self.convert_value_to_f64(self.open),
            self.convert_value_to_f64(self.high),
            self.convert_value_to_f64(self.low),
            self.convert_value_to_f64(self.close)
        )
    }
}

fn main() {
    println!("Hello, world!");

    let today_price: Candle = Candle {
        ticker: "SOLUSDC".to_string(),
        open: 9522,
        high: 10083,
        low: 9517,
        close: 9981,
        decimals: 9,
    };
    println!("{}", today_price.ticker);
    println!("{}", today_price.get_info());
    println!("{}", today_price.get_delta());
}
