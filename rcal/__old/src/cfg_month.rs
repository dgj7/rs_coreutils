use std::fmt;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct MonthConfig {
    pub year: i16,
    pub month: i16,
}

impl MonthConfig {
    pub fn new(month_input: i16, year_input: i16) -> MonthConfig {
        if !(1..=12).contains(&month_input) {
            panic!("bad month input: {}", month_input);
        }

        if year_input < 1 {
            panic!("bad year input: {}", year_input);
        }

        MonthConfig { month: month_input, year: year_input }
    }

    pub fn prev(&self) -> MonthConfig {
        if self.month == 1 {
            MonthConfig { month: 12, year: self.year - 1 }
        } else {
            MonthConfig { month: self.month - 1, year: self.year }
        }
    }

    pub fn next(&self) -> MonthConfig {
        if self.month == 12 {
            MonthConfig { month: 1, year: self.year + 1 }
        } else {
            MonthConfig { month: self.month + 1, year: self.year }
        }
    }
}

impl fmt::Display for MonthConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.month, self.year)
    }
}

#[cfg(test)]
mod tests_sort {
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

#[cfg(test)]
mod tests_next {
    use crate::cfg_month::MonthConfig;

    #[test]
    fn test_next_month1() {
        let input = MonthConfig::new(1, 2020);
        let output = input.next();

        assert_eq!(2, output.month);
        assert_eq!(2020, output.year);
    }

    #[test]
    fn test_next_month6() {
        let input = MonthConfig::new(6, 2020);
        let output = input.next();

        assert_eq!(7, output.month);
        assert_eq!(2020, output.year);
    }

    #[test]
    fn test_next_month12() {
        let input = MonthConfig::new(12, 2020);
        let output = input.next();

        assert_eq!(1, output.month);
        assert_eq!(2021, output.year);
    }
}

#[cfg(test)]
mod tests_prev {
    use crate::cfg_month::MonthConfig;

    #[test]
    fn test_prev_month1() {
        let input = MonthConfig::new(1, 2020);
        let output = input.prev();

        assert_eq!(12, output.month);
        assert_eq!(2019, output.year);
    }

    #[test]
    fn test_prev_month6() {
        let input = MonthConfig::new(6, 2020);
        let output = input.prev();

        assert_eq!(5, output.month);
        assert_eq!(2020, output.year);
    }

    #[test]
    fn test_prev_month12() {
        let input = MonthConfig::new(12, 2020);
        let output = input.prev();

        assert_eq!(11, output.month);
        assert_eq!(2020, output.year);
    }
}
