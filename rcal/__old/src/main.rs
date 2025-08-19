use crate::cal::generate_cal;
use crate::today::{DefaultToday};

mod cfg_app;
mod days;
mod formatter;
mod months;
mod cfg_month;
mod cfg_chunk;
mod cal;
mod cfg_args;
mod today;

fn main() {
    let args = std::env::args();
    let lines = generate_cal(args, &DefaultToday::new());
    lines.iter().for_each(|line| println!("{}", line));
}
