extern crate alloc;

use crate::cal::cal;
use std::process;

mod config;
mod time;
mod state;
mod formatter;
mod cal;

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
                println!("rcal: Result Err, with no errors");
                process::exit(999);
            }
        },
    };
}
