use crate::months::month::Month;

pub struct Chunk {
    pub left: Month,
    pub center: Option<Month>,
    pub right: Option<Month>,
    pub year_mode: YearMode
}

pub enum YearMode {
    WithMonth,
    NoDisplay,
    OwnLine,
}

impl Chunk {
    pub fn one(p_left: Month, p_year_mode: YearMode) -> Chunk {
        Chunk { left: p_left, center: None, right: None, year_mode: p_year_mode }
    }

    pub fn two(p_left: Month, p_center: Month, p_year_mode: YearMode) -> Chunk {
        Chunk { left: p_left, center: Option::from(p_center), right: None, year_mode: p_year_mode }
    }

    pub fn three(p_left: Month, p_center: Month, p_right: Month, p_year_mode: YearMode) -> Chunk {
        Chunk { left: p_left, center: Option::from(p_center), right: Option::from(p_right), year_mode: p_year_mode }
    }
}
