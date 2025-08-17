mod config;

use crate::config::Config;

fn main() {
    let _config = Config::new(std::env::args());
    // todo: process to application state
    // todo: print application state
}
