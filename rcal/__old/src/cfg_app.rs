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
    pub fn new(args: Args, today: &impl Today) -> AppConfig {
        if args.len() == 1 {
            AppConfig { chunks: vec!(ChunkConfig::one(today.make_today(), WithMonth)) }
        } else {
            AppConfig { chunks: month_configs_to_chunk_configs(args_to_month_configs(CalArguments::parse(), today)) }
        }
    }
}

fn args_to_month_configs(arguments: CalArguments, today: &impl Today) -> Vec<MonthConfig> {
    /* create storage */
    let mut months = vec![];

    /* first: check out the year and month arguments, as those ones don't have flags */
    if arguments.year.is_some() {
        let the_year = arguments.year.unwrap();

        if let Some(the_month) = arguments.month.and_then(|m| month_arg_match(&m)) {
            months.push(MonthConfig::new(the_month, the_year));
        } else {
            for the_month in 1..=12 {
                months.push(MonthConfig::new(the_month, the_year));
            }
        }
    } else if arguments.month.is_some() {
        let year = arguments.month.unwrap();
        panic!("not a valid year {}", year);
    } else {
        months.push(today.make_today());
    }

    /* next: check for months before the current month */
    if arguments.before.is_some() {
        let maybe_min = months.iter().min();
        if maybe_min.is_some() {
            let mut count = arguments.before.unwrap();
            let mut prev = *maybe_min.unwrap();
            while count > 0 {
                prev = prev.prev();
                months.push(prev);
                count -= 1;
            }
        }
    }

    /* next: months after */
    if arguments.after.is_some() {
        let maybe_max = months.iter().max();
        if maybe_max.is_some() {
            let mut count = arguments.after.unwrap();
            let mut next = *maybe_max.unwrap();
            while count > 0 {
                next = next.next();
                months.push(next);
                count -= 1;
            }
        }
    }

    /* IMPORTANT last step: sort the returning months */
    months.sort();
    months.dedup();

    /* done */
    months
}

fn month_configs_to_chunk_configs(month_configs: Vec<MonthConfig>) -> Vec<ChunkConfig> {
    /* create storage */
    let mut chunks = vec![];
    let mut years_displayed_on_own_line = vec![];

    /* iterate over all months; break into chunks of 3, each of which becomes a chunk config */
    for chunk in month_configs.chunks(3) {
        let chunk_config = if chunk.len() == 1 {
            let left = *chunk.first().unwrap();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::one(left, year_mode)
        } else if chunk.len() == 2 {
            let left = *chunk.first().unwrap();
            let center = *chunk.get(1).unwrap();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::two(left, center, year_mode)
        } else {
            let left = *chunk.first().unwrap();
            let center = *chunk.get(1).unwrap();
            let right = *chunk.get(2).unwrap();
            let year_mode = determine_year_mode(chunk, &mut years_displayed_on_own_line);
            ChunkConfig::three(left, center, right, year_mode)
        };
        chunks.push(chunk_config);
    }

    /* done */
    chunks
}

fn determine_year_mode(chunk: &[MonthConfig], years_displayed_on_own_line: &mut [i16]) -> YearMode {
    let years_in_current_chunk: HashSet<i16> = chunk.iter()
        .map(|c| c.year)
        .collect();
    if years_in_current_chunk.len() > 1 {
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

#[cfg(test)]
mod tests_new_app_cfg {
    // todo
}

#[cfg(test)]
mod tests_month_configs_vector {
    use crate::cfg_app::args_to_month_configs;
    use crate::cfg_args::CalArguments;
    use crate::cfg_month::MonthConfig;
    use crate::today::Today;
    struct TestOnlyToday {}

    impl Today for TestOnlyToday {
        fn make_today(&self) -> MonthConfig {
            return MonthConfig { month: 2, year: 2024 };
        }
    }

    #[test]
    fn test_no_args() {
        let input = CalArguments {
            year: None,
            month: None,
            before: None,
            after: None,
        };
        let output = args_to_month_configs(input, &TestOnlyToday{});

        assert_eq!(1, output.len());
        assert_eq!("2/2024", format!("{}", output.get(0).unwrap()));
    }

    #[test]
    #[should_panic]
    fn test_month_only() {
        let input = CalArguments {
            year: None,
            month: Some("January".to_string()),
            before: None,
            after: None,
        };
        args_to_month_configs(input, &TestOnlyToday{});
    }

    #[test]
    fn test_year_only() {
        let input = CalArguments {
            year: Some(2022),
            month: None,
            before: None,
            after: None,
        };
        let output = args_to_month_configs(input, &TestOnlyToday{});

        assert_eq!(12, output.len());
        assert_eq!("1/2022", format!("{}", output.get(0).unwrap()));
        assert_eq!("2/2022", format!("{}", output.get(1).unwrap()));
        assert_eq!("3/2022", format!("{}", output.get(2).unwrap()));
        assert_eq!("4/2022", format!("{}", output.get(3).unwrap()));
        assert_eq!("5/2022", format!("{}", output.get(4).unwrap()));
        assert_eq!("6/2022", format!("{}", output.get(5).unwrap()));
        assert_eq!("7/2022", format!("{}", output.get(6).unwrap()));
        assert_eq!("8/2022", format!("{}", output.get(7).unwrap()));
        assert_eq!("9/2022", format!("{}", output.get(8).unwrap()));
        assert_eq!("10/2022", format!("{}", output.get(9).unwrap()));
        assert_eq!("11/2022", format!("{}", output.get(10).unwrap()));
        assert_eq!("12/2022", format!("{}", output.get(11).unwrap()));
    }

    #[test]
    fn test_month_and_year() {
        let input = CalArguments {
            year: Some(2024),
            month: Some("may".to_string()),
            before: None,
            after: None,
        };
        let output = args_to_month_configs(input, &TestOnlyToday{});

        assert_eq!(1, output.len());
        assert_eq!("5/2024", format!("{}", output.get(0).unwrap()));
    }

    #[test]
    fn test_before_only() {
        let input = CalArguments {
            year: None,
            month: None,
            before: Some(3),
            after: None,
        };
        let output = args_to_month_configs(input, &TestOnlyToday{});

        assert_eq!(4, output.len());
        assert_eq!("11/2023", format!("{}", output.get(0).unwrap()));
        assert_eq!("12/2023", format!("{}", output.get(1).unwrap()));
        assert_eq!("1/2024", format!("{}", output.get(2).unwrap()));
        assert_eq!("2/2024", format!("{}", output.get(3).unwrap()));
    }

    #[test]
    fn test_after_only() {
        let input = CalArguments {
            year: None,
            month: None,
            before: None,
            after: Some(4),
        };
        let output = args_to_month_configs(input, &TestOnlyToday{});

        assert_eq!(5, output.len());
        assert_eq!("2/2024", format!("{}", output.get(0).unwrap()));
        assert_eq!("3/2024", format!("{}", output.get(1).unwrap()));
        assert_eq!("4/2024", format!("{}", output.get(2).unwrap()));
        assert_eq!("5/2024", format!("{}", output.get(3).unwrap()));
        assert_eq!("6/2024", format!("{}", output.get(4).unwrap()));
    }
}

#[cfg(test)]
mod tests_chunk_configs_vector {
    // todo
}
