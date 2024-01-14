use std::string::ToString;
use chrono::{Datelike, NaiveDate};
use crate::days::{calc_days_in_month};
use crate::cfg_app::AppConfig;
use crate::cfg_month::MonthConfig;
use string_builder::Builder;
use crate::cfg_chunk::{ChunkConfig, YearMode};
use crate::cfg_chunk::YearMode::WithMonth;
use crate::formatter::Position::{Center, Left, Right};
use crate::months::month_display_name;

const BLANK_ROW: &str = "                     ";

pub fn format_from_app_config(config: AppConfig) -> Vec<String> {
    let mut lines = vec![];

    config.chunks
        .iter()
        .for_each(|chunk| format_chunk(chunk)
            .iter()
            .for_each(|line| lines.push(line.to_owned())));

    return lines;
}

enum Position {
    Left,
    Center,
    Right,
}

fn format_chunk(chunk: &ChunkConfig) -> Vec<String> {
    /* prepare output */
    let mut output = vec![];

    /* grab each of the 3 months, which may be an empty vector */
    let mut left = format_month(&chunk.left, chunk, Left);
    let mut center = match &chunk.center {
        Some(x) => format_month(x, &chunk, Center),
        None => vec![]
    };
    let mut right = match &chunk.right {
        Some(x) => format_month(x, &chunk, Right),
        None => vec![]
    };

    /* figure out which one has the most lines */
    let largest = find_largest(&left, &center, &right);

    /* potentially expand each to match the month with the most lines */
    extend(&mut left, largest);
    extend(&mut center, largest);
    extend(&mut right, largest);

    /* combine each line of each of the 3 results into a single line, added to a single result */
    for idx in 0..largest {
        let x = left[idx].to_owned();
        let y = center[idx].to_owned();
        let z = right[idx].to_owned();
        output.push(format!("{} {} {}", x, y, z));
    }

    /* done */
    return output;
}

fn format_month(month_config: &MonthConfig, chunk_config: &ChunkConfig, position: Position) -> Vec<String> {
    /* create initial variables */
    let first_day: i32 = (NaiveDate::from_ymd_opt(month_config.year as i32, month_config.month as u32, 1).unwrap().weekday().num_days_from_sunday() + 1) as i32;
    let mut next_index: i32 = 2 - first_day;
    let max: i32 = calc_days_in_month(month_config.month as u32, month_config.year as i32) as i32;
    let month_name = month_display_name(&month_config, matches!(chunk_config.year_mode, WithMonth));

    /* create output lines var, and add static lines to it */
    let mut lines = vec![];
    lines.push(BLANK_ROW.to_string());

    /* if we're on the center node and year config is own-line, add the year, centered */
    if matches!(&chunk_config.year_mode, YearMode::OwnLine) {
        if matches!(position, Center) {
            lines.push(format!("{:^21}", month_config.year))
        } else {
            lines.push(BLANK_ROW.to_string());
        }
    }

    /* add month name and days of week */
    lines.push(format!("{:^21}", month_name));
    lines.push(" Su Mo Tu We Th Fr Sa".to_string());

    /* iterate over configs and format */
    let mut prev_row_max: i32 = 0;
    for _line_number in 1..=7 {
        if prev_row_max <= max {
            let mut line_builder = Builder::default();
            for _column_number in 0..7 {
                line_builder.append(format!(" {:>2}", format_day(next_index, max)));
                next_index = next_index + 1;
            }
            prev_row_max = next_index;
            lines.push(line_builder.string().unwrap().clone());
        }
    }

    /* done */
    return lines;
}

fn format_day(day: i32, max_days: i32) -> String {
    return if day > max_days {
        "  ".to_string()
    } else if day <= 0 {
        "  ".to_string()
    } else {
        format!("{:>2}", day)
    };
}

fn extend(vector: &mut Vec<String>, target_len: usize) {
    while vector.len() != target_len {
        vector.push(BLANK_ROW.to_string());
    }
}

fn find_largest(left: &Vec<String>, center: &Vec<String>, right: &Vec<String>) -> usize {
    return vec!(left.len(), center.len(), right.len()).iter().max().unwrap().to_owned();
}

#[cfg(test)]
mod test {
    use crate::cfg_app::AppConfig;
    use crate::cfg_chunk::ChunkConfig;
    use crate::cfg_chunk::YearMode::{OwnLine, WithMonth};
    use crate::cfg_month::MonthConfig;
    use crate::formatter::format_from_app_config;

    #[test]
    fn test_one_month() {
        let app_config = AppConfig {
            chunks: vec!(
                ChunkConfig::one(MonthConfig::new(1, 2024), WithMonth)
            )
        };
        let result = format_from_app_config(app_config);

        assert_eq!(8, result.len());
        assert_eq!("                                                                 ", result.get(0).unwrap());
        assert_eq!("    January 2024                                                 ", result.get(1).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa                                            ", result.get(2).unwrap());
        assert_eq!("     1  2  3  4  5  6                                            ", result.get(3).unwrap());
        assert_eq!("  7  8  9 10 11 12 13                                            ", result.get(4).unwrap());
        assert_eq!(" 14 15 16 17 18 19 20                                            ", result.get(5).unwrap());
        assert_eq!(" 21 22 23 24 25 26 27                                            ", result.get(6).unwrap());
        assert_eq!(" 28 29 30 31                                                     ", result.get(7).unwrap());
    }

    #[test]
    fn test_two_months() {
        let app_config = AppConfig {
            chunks: vec!(
                ChunkConfig::two(
                    MonthConfig::new(2, 2024),
                    MonthConfig::new(3, 2024),
                    WithMonth)
            )
        };
        let result = format_from_app_config(app_config);

        assert_eq!(9, result.len());
        assert_eq!("                                                                 ", result.get(0).unwrap());
        assert_eq!("    February 2024          March 2024                            ", result.get(1).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa                      ", result.get(2).unwrap());
        assert_eq!("              1  2  3                  1  2                      ", result.get(3).unwrap());
        assert_eq!("  4  5  6  7  8  9 10   3  4  5  6  7  8  9                      ", result.get(4).unwrap());
        assert_eq!(" 11 12 13 14 15 16 17  10 11 12 13 14 15 16                      ", result.get(5).unwrap());
        assert_eq!(" 18 19 20 21 22 23 24  17 18 19 20 21 22 23                      ", result.get(6).unwrap());
        assert_eq!(" 25 26 27 28 29        24 25 26 27 28 29 30                      ", result.get(7).unwrap());
        assert_eq!("                       31                                        ", result.get(8).unwrap());
    }

    #[test]
    fn test_three_months() {
        let app_config = AppConfig {
            chunks: vec!(
                ChunkConfig::three(
                    MonthConfig::new(4, 2024),
                    MonthConfig::new(5, 2024),
                    MonthConfig::new(6, 2024),
                    OwnLine)
            )
        };
        let result = format_from_app_config(app_config);

        assert_eq!(10, result.len());
        assert_eq!("                                                                 ", result.get(0).unwrap());
        assert_eq!("                              2024                               ", result.get(1).unwrap());
        assert_eq!("        April                  May                  June         ", result.get(2).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", result.get(3).unwrap());
        assert_eq!("     1  2  3  4  5  6            1  2  3  4                     1", result.get(4).unwrap());
        assert_eq!("  7  8  9 10 11 12 13   5  6  7  8  9 10 11   2  3  4  5  6  7  8", result.get(5).unwrap());
        assert_eq!(" 14 15 16 17 18 19 20  12 13 14 15 16 17 18   9 10 11 12 13 14 15", result.get(6).unwrap());
        assert_eq!(" 21 22 23 24 25 26 27  19 20 21 22 23 24 25  16 17 18 19 20 21 22", result.get(7).unwrap());
        assert_eq!(" 28 29 30              26 27 28 29 30 31     23 24 25 26 27 28 29", result.get(8).unwrap());
        assert_eq!("                                             30                  ", result.get(9).unwrap());
    }

    #[test]
    fn test_four_months() {
        let app_config = AppConfig {
            chunks: vec!(
                ChunkConfig::three(
                    MonthConfig::new(7, 2024),
                    MonthConfig::new(8, 2024),
                    MonthConfig::new(9, 2024),
                    OwnLine),
                ChunkConfig::one(MonthConfig::new(10, 2024), WithMonth)
            )
        };
        let result = format_from_app_config(app_config);

        assert_eq!(17, result.len());
        assert_eq!("                                                                 ", result.get(0).unwrap());
        assert_eq!("                              2024                               ", result.get(1).unwrap());
        assert_eq!("        July                 August               September      ", result.get(2).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", result.get(3).unwrap());
        assert_eq!("     1  2  3  4  5  6               1  2  3   1  2  3  4  5  6  7", result.get(4).unwrap());
        assert_eq!("  7  8  9 10 11 12 13   4  5  6  7  8  9 10   8  9 10 11 12 13 14", result.get(5).unwrap());
        assert_eq!(" 14 15 16 17 18 19 20  11 12 13 14 15 16 17  15 16 17 18 19 20 21", result.get(6).unwrap());
        assert_eq!(" 21 22 23 24 25 26 27  18 19 20 21 22 23 24  22 23 24 25 26 27 28", result.get(7).unwrap());
        assert_eq!(" 28 29 30 31           25 26 27 28 29 30 31  29 30               ", result.get(8).unwrap());
        assert_eq!("                                                                 ", result.get(9).unwrap());
        assert_eq!("    October 2024                                                 ", result.get(10).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa                                            ", result.get(11).unwrap());
        assert_eq!("        1  2  3  4  5                                            ", result.get(12).unwrap());
        assert_eq!("  6  7  8  9 10 11 12                                            ", result.get(13).unwrap());
        assert_eq!(" 13 14 15 16 17 18 19                                            ", result.get(14).unwrap());
        assert_eq!(" 20 21 22 23 24 25 26                                            ", result.get(15).unwrap());
        assert_eq!(" 27 28 29 30 31                                                  ", result.get(16).unwrap());
    }
}
