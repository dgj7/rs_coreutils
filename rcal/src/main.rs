use crate::config::Config;

mod config;

fn main() {
    let _config = Config::new(std::env::args());

    println!("hello, world!");
}
