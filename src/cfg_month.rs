use std::fmt;
use chrono::Datelike;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MonthConfig {
    pub year: i16,
    pub month: i16,
}

impl MonthConfig {
    pub fn new(month_input: i16, year_input: i16) -> MonthConfig {
        if month_input < 1 || month_input > 12 {
            panic!("bad month input: {}", month_input);
        }

        if year_input < 1 {
            panic!("bad year input: {}", year_input);
        }

        return MonthConfig { month: month_input, year: year_input }
    }

    pub fn this_month() -> MonthConfig {
        let current_date = chrono::Utc::now();
        let the_year = current_date.year() as i16;
        let the_month = current_date.month() as i16;
        return MonthConfig { year: the_year, month: the_month };
    }
}

impl fmt::Display for MonthConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.month, self.year)
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg_month::MonthConfig;

    #[test]
    fn test_month_config_vector_sort() {
        let mut unsorted = vec!(
            MonthConfig::new(4, 2023),
            MonthConfig::new(3, 2021),
            MonthConfig::new(2, 2022),
            MonthConfig::new(1, 2024),
        );

        unsorted.sort();

        assert_eq!(4, unsorted.len());
        assert_eq!("3/2021", format!("{}", unsorted.get(0).unwrap()));
        assert_eq!("2/2022", format!("{}", unsorted.get(1).unwrap()));
        assert_eq!("4/2023", format!("{}", unsorted.get(2).unwrap()));
        assert_eq!("1/2024", format!("{}", unsorted.get(3).unwrap()));
    }
}
