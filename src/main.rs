extern crate string_builder;

use crate::config::AppConfig;

mod config;
mod calc;
mod formatter;

fn main() {
    let args = std::env::args();
    let config = AppConfig::new(args);
    let lines = formatter::format_months(config);

    lines.iter().for_each(|line| println!("{}", line));
}
