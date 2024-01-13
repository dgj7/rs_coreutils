use std::env::Args;
use chrono::Datelike;
use crate::cfg_month::MonthConfig;

pub struct AppConfig {
    pub months: Vec<MonthConfig>
}

impl AppConfig {
    pub fn new(args: Args) -> AppConfig {
        if args.len() == 1 {
            let current_date = chrono::Utc::now();
            let year = current_date.year() as i16;
            let month = current_date.month() as i16;
            let mut months = vec![];
            months.push(MonthConfig::new(month, year));
            return AppConfig { months };
        } else {
            // todo: implement args
            panic!("additional args: not yet implemented")
        }
    }
}
