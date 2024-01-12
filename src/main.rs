extern crate string_builder;

use std::string::ToString;
use chrono::{Datelike, NaiveDate};
use crate::calc::{calc_days_in_month, month_name};
use crate::config::{AppConfig, MonthConfig};
use string_builder::Builder;

mod config;
mod calc;

const BLANK_ROW: &str = "                     ";

fn main() {
    let args = std::env::args();
    let config = AppConfig::new(args);
    let lines = format_months(config);

    lines.iter().for_each(|line| println!("{}", line));
}

fn format_months(config: AppConfig) -> Vec<String> {
    let mut results = vec![];

    config.months
        .chunks(3)
        .for_each(|three| {
            format_chunk(three)
                .iter()
                .for_each(|result| {
                    let formatted = format!("{}", result);
                    results.push(formatted)
                });
        });
    return results;
}

fn format_chunk(slice: &[MonthConfig]) -> Vec<String> {
    if slice.is_empty() {
        panic!("don't know what to do with empty slice")
    } else if slice.len() > 1 {
        let mut results = vec![];
        let mut first = format_month(&slice[0]);
        let mut second = format_month(&slice[1]);

        if slice.len() < 3 {
            let largest = vec!(first.len(), second.len()).iter().max().unwrap().to_owned();
            extend(&mut first, largest);
            extend(&mut second, largest);

            for idx in 0..largest {
                let x = first[idx].to_owned();
                let y = second[idx].to_owned();
                results.push(format!("{} {}", x, y));
            }
        } else {
            let mut third = format_month(&slice[2]);
            let largest = vec!(first.len(), second.len(), third.len()).iter().max().unwrap().to_owned();
            extend(&mut first, largest);
            extend(&mut second, largest);
            extend(&mut third, largest);

            for idx in 0..largest {
                let x = first[idx].to_owned();
                let y = second[idx].to_owned();
                let z = third[idx].to_owned();
                results.push(format!("{} {} {}", x, y, z));
            }
        }

        return results;
    } else {
        return format_month(&slice[0]);
    }
}

fn format_month(config: &MonthConfig) -> Vec<String> {
    let first_day: i32 = (NaiveDate::from_ymd_opt(config.year as i32, config.month as u32, 1).unwrap().weekday().num_days_from_sunday() + 1) as i32;
    let mut next_index: i32 = 2 - first_day;
    let max: i32 = calc_days_in_month(config.month as u32, config.year as i32) as i32;
    let month_name = month_name(&config, false);

    let mut lines = vec![];
    lines.push(BLANK_ROW.to_string());
    lines.push(format!("{:^21}", month_name));
    lines.push(" Su Mo Tu We Th Fr Sa".to_string());

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
    while vector.len() != target_len - 1 {
        vector.push(BLANK_ROW.to_string());
    }
}
