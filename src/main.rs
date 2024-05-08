use crate::config::Config;

mod counts;
mod config;
mod printer;

fn main() {
    let config = Config::new(std::env::args());
    let counts = counts::count(&config);
    printer::print_results(&config, counts);
}
