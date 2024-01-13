use chrono::NaiveDate;

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

#[cfg(test)]
mod tests {
    use super::*;

    //
}
