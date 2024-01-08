use std::env::Args;
use chrono::Datelike;

pub struct AppConfig {
    pub months: Vec<MonthConfig>
}

pub struct MonthConfig {
    pub month: i16,
    pub year: i16,
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

impl MonthConfig {
    pub fn new(month_input: i16, year_input: i16) -> MonthConfig {
        if month_input < 1 || month_input > 12 {
            panic!("bad month input: {}", month_input);
        }

        if year_input < 1 {
            panic!("bad year input: {}", year_input);
        }

        return MonthConfig { month: month_input, year: year_input }
    }
}
