extern crate alloc;

use crate::config::Config;
use crate::state::app_state::ApplicationState;
use crate::time::today::TodayFactory;

mod config;
mod time;
mod state;
mod formatter;

fn main() {
    let args = std::env::args();
    let lines = cal(args.collect());
    lines.iter().for_each(|line| println!("{}", line));
}

pub fn cal(args: Vec<String>) -> Vec<String> {
    let config = Config::new(&args);
    let today = TodayFactory::Actual.create();
    let state = ApplicationState::new(config, today.as_ref());
    formatter::format_from_app_config(state)
        .iter()
        .skip(1)
        .map(|s| s.to_owned())
        .collect()
}
