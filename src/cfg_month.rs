#[derive(Copy, Clone)]
pub struct MonthConfig {
    pub month: i16,
    pub year: i16,
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
}
