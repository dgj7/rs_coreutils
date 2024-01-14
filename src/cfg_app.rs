use std::env::Args;
use clap::Parser;
use crate::cfg_args::CalArguments;
use crate::cfg_chunk::{ChunkConfig, YearMode};
use crate::cfg_month::MonthConfig;
use crate::months::month_arg_match;

pub struct AppConfig {
    pub chunks: Vec<ChunkConfig>,
}

impl AppConfig {
    pub fn new(args: Args) -> AppConfig {
        return if args.len() == 1 {
            AppConfig { chunks: vec!(ChunkConfig::one(MonthConfig::this_month(), YearMode::WithMonth)) }
        } else {
            AppConfig { chunks: month_configs_to_chunk_configs(args_to_month_configs(CalArguments::parse())) }
        };
    }
}

fn args_to_month_configs(arguments: CalArguments) -> Vec<MonthConfig> {
    let mut months = vec![];

    if arguments.year.is_some() {
        let the_year = arguments.year.unwrap();

        let maybe_month_number = arguments.month.and_then(|m| month_arg_match(&m));
        if maybe_month_number.is_some() {
            let the_month = maybe_month_number.unwrap();
            months.push(MonthConfig::new(the_year, the_month));
        } else {
            for the_month in 1..=12 {
                months.push(MonthConfig::new(the_year, the_month));
            }
        }
    }

    return months;
}

fn month_configs_to_chunk_configs(month_configs: Vec<MonthConfig>) -> Vec<ChunkConfig> {
    // todo: each of these needs to compute the year mode more intelligently; ie, not NoDisplay
    let mut chunks = vec![];

    for chunk in month_configs.chunks(3) {
        let chunk_config = if chunk.len() == 1 {
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

    return chunks;
}
