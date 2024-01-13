mod cfg_app;
mod days;
mod formatter;
mod months;
mod cfg_month;

fn main() {
    let args = std::env::args();
    let config = cfg_app::AppConfig::new(args);
    let lines = formatter::format_months(config);
    lines.iter().for_each(|line| println!("{}", line));
}
