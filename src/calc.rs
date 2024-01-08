use chrono::NaiveDate;
use crate::config::MonthConfig;

const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

pub fn calc_days_in_month(month: u32, year: i32) -> i64 {
    return if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .unwrap()
    .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
    .num_days();
}

pub fn month_name(config: &MonthConfig, include_year: bool) -> String {
    let index: usize = (config.month - 1) as usize;
    let month = MONTHS[index];

    return if include_year {
        format!("{} {}", month, config.year)
    } else {
        month.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
}
