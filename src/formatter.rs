use std::string::ToString;
use chrono::{Datelike, NaiveDate};
use crate::days::{calc_days_in_month};
use crate::cfg_app::AppConfig;
use crate::cfg_month::MonthConfig;
use string_builder::Builder;
use crate::months::month_name;

const BLANK_ROW: &str = "                     ";

pub fn format_months(config: AppConfig) -> Vec<String> {
    let mut lines = vec![];

    config.months
        .chunks(3)
        .for_each(|chunk| format_chunk(chunk)
            .iter()
            .for_each(|line| lines.push(line.to_owned())));

    return lines;
}

fn format_chunk(slice: &[MonthConfig]) -> Vec<String> {
    if slice.len() == 1 {
        return format_month(&slice[0]);
    } else if slice.len() > 3 {
        panic!("don't know what to do with more than 3 slice len");
    } else if slice.is_empty() {
        panic!("don't know what to do with empty slice");
    } else {
        /* prepare output */
        let mut results = vec![];

        /* get initial variables; for this case, there is at least 2 months */
        let mut first = format_month(&slice[0]);
        let mut second = format_month(&slice[1]);
        let mut largest = vec!(first.len(), second.len()).iter().max().unwrap().to_owned();

        /* the 3rd of 3 months is either a month, or just blank spaces */
        let mut third = if slice.len() < 3 {
            let mut output = vec![];
            extend(&mut output, largest);
            output
        } else {
            let output = format_month(&slice[2]);
            largest = vec!(first.len(), second.len(), output.len()).iter().max().unwrap().to_owned();
            output
        };

        /* expand each to be the maximum length */
        extend(&mut first, largest);
        extend(&mut second, largest);
        extend(&mut third, largest);

        /* combine each line of each of the 3 results into a single line, added to a single result */
        for idx in 0..largest {
            let x = first[idx].to_owned();
            let y = second[idx].to_owned();
            let z = third[idx].to_owned();
            results.push(format!("{} {} {}", x, y, z));
        }

        return results;
    }
}

fn format_month(config: &MonthConfig) -> Vec<String> {
    /* create initial variables */
    let first_day: i32 = (NaiveDate::from_ymd_opt(config.year as i32, config.month as u32, 1).unwrap().weekday().num_days_from_sunday() + 1) as i32;
    let mut next_index: i32 = 2 - first_day;
    let max: i32 = calc_days_in_month(config.month as u32, config.year as i32) as i32;
    let month_name = month_name(&config, false);

    /* create output lines var, and add static lines to it */
    let mut lines = vec![];
    lines.push(BLANK_ROW.to_string());
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
    while vector.len() != target_len - 1 {
        vector.push(BLANK_ROW.to_string());
    }
}

#[cfg(test)]
mod test {
    use crate::cfg_app::AppConfig;
    use crate::cfg_month::MonthConfig;
    use crate::formatter::format_months;

    #[test]
    fn test_jan2023_only() {
        let the_months = vec!(MonthConfig { month: 1, year: 2023 });
        let config = AppConfig { months: the_months };
        let lines = format_months(config);

        assert_eq!(8, lines.len());
        assert_eq!("                     ", lines.get(0).unwrap());
        assert_eq!("       January       ", lines.get(1).unwrap());
        assert_eq!(" Su Mo Tu We Th Fr Sa", lines.get(2).unwrap());
        assert_eq!("  1  2  3  4  5  6  7", lines.get(3).unwrap());
        assert_eq!("  8  9 10 11 12 13 14", lines.get(4).unwrap());
        assert_eq!(" 15 16 17 18 19 20 21", lines.get(5).unwrap());
        assert_eq!(" 22 23 24 25 26 27 28", lines.get(6).unwrap());
        assert_eq!(" 29 30 31            ", lines.get(7).unwrap());
    }

    #[test]
    #[ignore]
    fn test_5months_over2years() {
        let the_months = vec!(
            MonthConfig { month: 11, year: 2023 },
            MonthConfig { month: 12, year: 2023 },
            MonthConfig { month: 1, year: 2024 },
            MonthConfig { month: 2, year: 2024 },
            MonthConfig { month: 3, year: 2024 },
        );
        let config = AppConfig { months: the_months };
        let lines = format_months(config);

        assert_eq!(0, lines.len());
    }
}
