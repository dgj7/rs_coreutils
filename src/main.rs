use crate::cal::print_cal;

mod cfg_app;
mod days;
mod formatter;
mod months;
mod cfg_month;
mod cfg_chunk;
mod cal;

fn main() {
    let args = std::env::args();
    print_cal(args);
}
