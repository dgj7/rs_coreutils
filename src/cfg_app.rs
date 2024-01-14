use std::env::Args;
use chrono::Datelike;
use crate::cfg_chunk::{ChunkConfig, YearMode};
use crate::cfg_month::MonthConfig;

pub struct AppConfig {
    pub chunks: Vec<ChunkConfig>
}

impl AppConfig {
    pub fn new(args: Args) -> AppConfig {
        if args.len() == 1 {
            /* get the current date info */
            let current_date = chrono::Utc::now();
            let year = current_date.year() as i16;
            let month = current_date.month() as i16;

            /* make vector of chunks */
            let mut chunks = vec![];
            chunks.push(ChunkConfig::one(MonthConfig::new(month, year), YearMode::WithMonth));

            /* done */
            return AppConfig { chunks };
        } else {
            let mut months: Vec<MonthConfig> = vec![];

            // todo: add to the months, by implementing args

            /* convert to chunks */
            let mut chunks = vec![];
            for chunk in months.chunks(3) {
                let mut chunk_config = if chunk.len() == 1 {
                    let left = chunk.get(0).unwrap().clone();
                    ChunkConfig::one(left, YearMode::NoDisplay)
                } else if chunk.len() == 2 {
                    let left = chunk.get(0).unwrap().clone();
                    let center = chunk.get(1).unwrap().clone();
                    ChunkConfig::two(left, center, YearMode::NoDisplay)
                } else {
                    let left = chunk.get(0).unwrap().clone();
                    let center = chunk.get(1).unwrap().clone();
                    let right = chunk.get(2).unwrap().clone();
                    ChunkConfig::three(left, center, right, YearMode::NoDisplay)
                };
                chunks.push(chunk_config);
            }

            /* done */
            return AppConfig { chunks };
        }
    }
}
