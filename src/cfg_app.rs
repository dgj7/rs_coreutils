use std::env::Args;
use chrono::Datelike;
use crate::cfg_chunk::{ChunkConfig, YearMode};
use crate::cfg_month::MonthConfig;

pub struct AppConfig {
    pub months: Vec<MonthConfig>,
    pub chunks: Vec<ChunkConfig>
}

impl AppConfig {
    pub fn new(args: Args) -> AppConfig {
        if args.len() == 1 {
            /* get the current date info */
            let current_date = chrono::Utc::now();
            let year = current_date.year() as i16;
            let month = current_date.month() as i16;

            /* make vector of months */
            let mut months = vec![];
            months.push(MonthConfig::new(month, year));

            /* make vector of chunks */
            let mut chunks = vec![];
            chunks.push(ChunkConfig::one(MonthConfig::new(month, year), YearMode::WithMonth));

            /* done */
            return AppConfig { months, chunks };
        } else {
            // todo: implement args
            panic!("additional args: not yet implemented")
        }
    }
}
