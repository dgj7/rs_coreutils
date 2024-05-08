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

    #[test]
    fn february_non_leap() {
        assert_eq!(28, calc_days_in_month(2, 2023));
    }

    #[test]
    fn february_leap() {
        assert_eq!(29, calc_days_in_month(2, 2024));
    }

    #[test]
    fn april2023() {
        assert_eq!(30, calc_days_in_month(4, 2023));
    }

    #[test]
    fn oct2023() {
        assert_eq!(31, calc_days_in_month(10, 2023));
    }
}
