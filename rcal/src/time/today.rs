use chrono::Datelike;
use crate::time::month::Month;

pub trait Today {
    fn make_today(&self) -> Month;
}

#[allow(dead_code)]
pub(crate) enum TodayFactory {
    Actual,
    Other { y: u16, m: u16 },
}

impl TodayFactory {
    pub(crate) fn create(&self) -> Box<dyn Today> {
        match self {
            TodayFactory::Actual => {
                Box::new(ActualToday {})
            },
            TodayFactory::Other { y, m } => {
                Box::new(OtherToday { year: *y, month: *m })
            }
        }
    }
}

pub struct ActualToday {}

impl Today for ActualToday {
    fn make_today(&self) -> Month {
        let current_date = chrono::Utc::now();
        let the_year = current_date.year() as u16;
        let the_month = current_date.month() as u16;
        Month { year: the_year, month: the_month }
    }
}

pub(crate) struct OtherToday {
    pub(crate) year: u16,
    pub(crate) month: u16,
}

impl Today for OtherToday {
    fn make_today(&self) -> Month {
        Month { year: self.year, month: self.month }
    }
}
