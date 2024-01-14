use crate::cal::generate_cal;

mod cfg_app;
mod days;
mod formatter;
mod months;
mod cfg_month;
mod cfg_chunk;
mod cal;

fn main() {
    let args = std::env::args();
    let lines = generate_cal(args);
    lines.iter().for_each(|line| println!("{}", line));
}
