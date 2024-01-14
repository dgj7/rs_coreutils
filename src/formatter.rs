use std::string::ToString;
use chrono::{Datelike, NaiveDate};
use crate::days::{calc_days_in_month};
use crate::cfg_app::AppConfig;
use crate::cfg_month::MonthConfig;
use string_builder::Builder;
use crate::cfg_chunk::ChunkConfig;
use crate::cfg_chunk::YearMode::WithMonth;
use crate::months::month_name;

const BLANK_ROW: &str = "                     ";

pub fn format_months(config: AppConfig) -> Vec<String> {
    let mut lines = vec![];

    config.chunks
        .iter()
        .for_each(|chunk| format_chunk(chunk)
            .iter()
            .for_each(|line| lines.push(line.to_owned())));

    return lines;
}

fn format_chunk(chunk: &ChunkConfig) -> Vec<String> {
    /* prepare output */
    let mut output = vec![];

    /* grab each of the 3 months, which may be an empty vector */
    let mut left = format_month(&chunk.left, chunk);
    let mut center = match &chunk.center {
        Some(x) => format_month(x, &chunk),
        None => vec![]
    };
    let mut right = match &chunk.right {
        Some(x) => format_month(x, &chunk),
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

fn format_month(month_config: &MonthConfig, chunk_config: &ChunkConfig) -> Vec<String> {
    /* create initial variables */
    let first_day: i32 = (NaiveDate::from_ymd_opt(month_config.year as i32, month_config.month as u32, 1).unwrap().weekday().num_days_from_sunday() + 1) as i32;
    let mut next_index: i32 = 2 - first_day;
    let max: i32 = calc_days_in_month(month_config.month as u32, month_config.year as i32) as i32;
    let month_name = month_name(&month_config, matches!(chunk_config.year_mode, WithMonth));

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
    use crate::cfg_month::MonthConfig;
    use crate::formatter::format_months;

    // todo: re-add tests when formatter is reorganized to use chunk config
}
