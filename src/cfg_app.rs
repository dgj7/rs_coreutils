use std::collections::HashSet;
use std::env::Args;
use clap::Parser;
use crate::cfg_args::CalArguments;
use crate::cfg_chunk::{ChunkConfig, YearMode};
use crate::cfg_chunk::YearMode::{NoDisplay, OwnLine, WithMonth};
use crate::cfg_month::MonthConfig;
use crate::months::month_arg_match;
use crate::today::Today;

pub struct AppConfig {
    pub chunks: Vec<ChunkConfig>,
}

impl AppConfig {
    //today_factory: fn() -> MonthConfig
    pub fn new(args: Args, today: &impl Today) -> AppConfig {
        return if args.len() == 1 {
            AppConfig { chunks: vec!(ChunkConfig::one(today.make_today(), WithMonth)) }
        } else {
            AppConfig { chunks: month_configs_to_chunk_configs(args_to_month_configs(CalArguments::parse())) }
        };
    }
}

fn args_to_month_configs(arguments: CalArguments) -> Vec<MonthConfig> {
    /* create storage */
    let mut months = vec![];

    /* first, check out the year and month arguments, as those ones don't have flags */
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

    /* IMPORTANT last step: sort the returning months */
    months.sort();
    months.dedup();

    /* done */
    return months;
}

fn month_configs_to_chunk_configs(month_configs: Vec<MonthConfig>) -> Vec<ChunkConfig> {
    /* create storage */
    let mut chunks = vec![];
    let mut years_displayed_on_own_line = vec![];

    /* iterate over all months; break into chunks of 3, each of which becomes a chunk config */
    for chunk in month_configs.chunks(3) {
        let chunk_config = if chunk.len() == 1 {
            let left = chunk.get(0).unwrap().clone();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::one(left, year_mode)
        } else if chunk.len() == 2 {
            let left = chunk.get(0).unwrap().clone();
            let center = chunk.get(1).unwrap().clone();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::two(left, center, year_mode)
        } else {
            let left = chunk.get(0).unwrap().clone();
            let center = chunk.get(1).unwrap().clone();
            let right = chunk.get(2).unwrap().clone();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::three(left, center, right, year_mode)
        };
        chunks.push(chunk_config);
    }

    /* done */
    return chunks;
}

fn determine_year_mode(chunk: &[MonthConfig], years_displayed_on_own_line: &mut Vec<i16>) -> YearMode {
    let years_in_current_chunk: HashSet<i16> = chunk.iter()
        .map(|c| c.year)
        .collect();
    return if years_in_current_chunk.len() > 1 {
        WithMonth
    } else if years_in_current_chunk.len() == 1 {
        let this_chunks_year = years_in_current_chunk.iter()
            .next()
            .unwrap()
            .to_owned();
        if years_displayed_on_own_line.contains(&this_chunks_year) {
            NoDisplay
        } else {
            OwnLine
        }
    } else {
        NoDisplay
    }
}
