extern crate string_builder;

use std::string::ToString;
use chrono::{Datelike, NaiveDate};
use crate::calc::{calc_days_in_month, month_name};
use crate::config::{AppConfig, MonthConfig};
use string_builder::Builder;

mod config;
mod calc;

fn main() {
    let args = std::env::args();
    let config = AppConfig::new(args);
    let mut lines = format_months(config);

    // todo: remove; also, `lines` doesnt need to be `mut` after these lines are removed
    format_month(&MonthConfig::new(2, 2024)).iter().for_each(|x| lines.push((**x).parse().unwrap()));
    format_month(&MonthConfig::new(6, 2024)).iter().for_each(|x| lines.push((**x).parse().unwrap()));

    lines.iter().for_each(|line| println!("{}", line));
}

fn format_months(config: AppConfig) -> Vec<String> {
    // todo: there will eventually be multiple months; merge them here
    return format_month(&config.months[0]);
}

fn format_month(config: &MonthConfig) -> Vec<String> {
    //println!("-----");
    //println!("month={}, year={}", config.month, config.year);

    let first_day: i32 = (NaiveDate::from_ymd_opt(config.year as i32, config.month as u32, 1).unwrap().weekday().num_days_from_sunday() + 1) as i32;
    let mut next_index: i32 = 2 - first_day;
    let max: i32 = calc_days_in_month(config.month as u32, config.year as i32) as i32;
    let month_name = month_name(&config, false);
    //println!("first_day={}, next_index={}, max={}", first_day, next_index, max);

    let mut lines = vec![];
    lines.push("                     ".to_string());
    lines.push(format!("{:^21}", month_name));
    lines.push(" Su Mo Tu We Th Fr Sa".to_string());

    let mut prev_row_max: i32 = 0;
    for _line_number in 1..=7 {
        if prev_row_max <= max {
            let mut line_builder = Builder::default();
            for _column_number in 0..7 {
                line_builder.append(format!(" {:>2}", format_day(next_index, max)));
                //print!(" {:>2}", next);
                next_index = next_index + 1;
            }
            prev_row_max = next_index;
            //println!("{}", line_builder.string().unwrap());
            lines.push(line_builder.string().unwrap().clone());
        }
    }
    //println!("-----");

    return lines;
}

fn format_day(day: i32, max_days: i32) -> String {
    return if day > max_days {
        "  ".to_string()
    } else if day <= 0 {
        "  ".to_string()
    } else {
        format!("{:>2}", day)
    }
}
