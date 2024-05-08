use crate::cfg_month::MonthConfig;

pub struct ChunkConfig {
    pub left: MonthConfig,
    pub center: Option<MonthConfig>,
    pub right: Option<MonthConfig>,
    pub year_mode: YearMode
}

pub enum YearMode {
    WithMonth,
    NoDisplay,
    OwnLine,
}

impl ChunkConfig {
    pub fn one(p_left: MonthConfig, p_year_mode: YearMode) -> ChunkConfig {
        return ChunkConfig { left: p_left, center: None, right: None, year_mode: p_year_mode };
    }

    pub fn two(p_left: MonthConfig, p_center: MonthConfig, p_year_mode: YearMode) -> ChunkConfig {
        return ChunkConfig { left: p_left, center: Option::from(p_center), right: None, year_mode: p_year_mode };
    }

    pub fn three(p_left: MonthConfig, p_center: MonthConfig, p_right: MonthConfig, p_year_mode: YearMode) -> ChunkConfig {
        return ChunkConfig { left: p_left, center: Option::from(p_center), right: Option::from(p_right), year_mode: p_year_mode };
    }
}
