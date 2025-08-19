use std::string::ToString;
use crate::cfg_month::MonthConfig;

const MONTH_DISPLAY_NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const MONTH_FULL_ARGS: [&str; 12] = [    "january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];
const MONTH_ABBR_ARGS: [&str; 12] = [    "jan",     "feb",      "mar",   "apr",   "may", "jun",  "jul",  "aug",    "sep",       "oct",     "nov",      "dec"];

pub fn month_display_name(config: &MonthConfig, include_year: bool) -> String {
    let index: usize = (config.month - 1) as usize;
    let month = MONTH_DISPLAY_NAMES[index];

    if include_year {
        format!("{} {}", month, config.year)
    } else {
        month.to_string()
    }
}

pub fn month_arg_match(month_arg: &str) -> Option<i16> {
    /* look for full month names */
    for (index, value) in MONTH_FULL_ARGS.iter().enumerate() {
        if month_arg.to_lowercase() == *value {
            return Some((index+1) as i16);
        }
    }

    /* look for partial month names */
    for (index, value) in MONTH_ABBR_ARGS.iter().enumerate() {
        if month_arg.to_lowercase() == *value {
            return Some((index+1) as i16);
        }
    }

    /* none found */
    None
}

#[cfg(test)]
mod tests {
    use crate::cfg_month::MonthConfig;
    use crate::months::month_display_name;

    #[test]
    #[should_panic]
    fn test_month_0() {
        month_display_name(&MonthConfig{month: 0, year: 2024}, true);
    }

    #[test]
    fn test_month_1() {
        assert_eq!("January 2024", month_display_name(&MonthConfig{month: 1, year: 2024}, true));
    }

    #[test]
    fn test_month_12() {
        assert_eq!("December", month_display_name(&MonthConfig{month: 12, year: 2024}, false));
    }

    #[test]
    #[should_panic]
    fn test_month_13() {
        assert_eq!("", month_display_name(&MonthConfig{month: 13, year: 2024}, false));
    }
}
