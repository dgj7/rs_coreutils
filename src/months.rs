use crate::cfg_month::MonthConfig;

const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
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
    use crate::cfg_month::MonthConfig;
    use crate::months::month_name;

    #[test]
    #[should_panic]
    fn test_month_0() {
        month_name(&MonthConfig{month: 0, year: 2024}, true);
    }

    #[test]
    fn test_month_1() {
        assert_eq!("January 2024", month_name(&MonthConfig{month: 1, year: 2024}, true));
    }

    #[test]
    fn test_month_12() {
        assert_eq!("December", month_name(&MonthConfig{month: 12, year: 2024}, false));
    }

    #[test]
    #[should_panic]
    fn test_month_13() {
        assert_eq!("", month_name(&MonthConfig{month: 13, year: 2024}, false));
    }
}
