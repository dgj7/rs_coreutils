use chrono::Datelike;
use crate::time::month::Month;

pub trait Today {
    fn make_today(&self) -> Month;
}

pub struct DefaultToday {}

impl Today for DefaultToday {
    fn make_today(&self) -> Month {
        let current_date = chrono::Utc::now();
        let the_year = current_date.year() as i16;
        let the_month = current_date.month() as i16;
        Month { year: the_year, month: the_month }
    }
}

impl DefaultToday {
    pub(crate) fn new() -> Self {
        DefaultToday{}
    }
}
