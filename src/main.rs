mod config;
mod days;
mod formatter;
mod months;

fn main() {
    let args = std::env::args();
    let config = config::AppConfig::new(args);
    let lines = formatter::format_months(config);
    lines.iter().for_each(|line| println!("{}", line));
}
