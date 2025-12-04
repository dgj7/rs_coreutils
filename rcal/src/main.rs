extern crate alloc;

use crate::cal::cal;
use std::process;
use crate::time::today::TodayFactory;

mod time;
mod state;
mod formatter;
mod cal;

fn main() {
    let args = std::env::args();
    let result = cal(args.collect(), TodayFactory::Actual);
    match result {
        Ok(lines) => lines.iter().for_each(|line| println!("{}", line)),
        Err(errors) => {
            for error in errors {
                println!("{}: {}", error.code, error.message.unwrap_or_else(|| String::from("unknown")));
            }
            process::exit(999);
        },
    };
}
