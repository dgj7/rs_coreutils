extern crate alloc;

use crate::config::Config;

mod config;
mod time;
mod state;
mod formatter;

fn main() {
    let _config = Config::new(std::env::args());

    println!("hello, world!");
}
