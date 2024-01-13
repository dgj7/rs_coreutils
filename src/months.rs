use crate::config::MonthConfig;

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
