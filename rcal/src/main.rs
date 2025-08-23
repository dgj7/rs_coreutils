extern crate alloc;

use std::process;
use crate::cal::cal;

mod config;
mod time;
mod state;
mod formatter;
mod cal;
mod input;

fn main() {
    let args = std::env::args();
    let result = cal(args.collect());
    match result {
        Ok(lines) => lines.iter().for_each(|line| println!("{}", line)),
        Err(errors) => {
            if let Some(error) = errors.first() {
                println!("{}", error.message.clone().unwrap());
                process::exit(error.code);
            } else {
                panic!("rcal: Result Err, with no errors");
            }
        },
    };
}
