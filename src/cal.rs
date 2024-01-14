use crate::{cfg_app, formatter};

pub fn print_cal(args: std::env::Args) {
    let config = cfg_app::AppConfig::new(args);
    let lines = formatter::format_months(config);
    lines.iter().for_each(|line| println!("{}", line));
}
