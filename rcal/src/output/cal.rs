use common::input::known_error::KnownError;
use crate::state::config::{Config};
use crate::output::formatter;
use crate::state::app_state::ApplicationState;
use crate::time::today::TodayFactory;

pub fn cal(args: Vec<String>, today_factory: TodayFactory) -> Result<Vec<String>, Vec<KnownError>> {
    let config = Config::new(&args);
    let today = today_factory.create();
    let state = ApplicationState::new(&config, today.as_ref());
    let lines = formatter::format_calendar(&config.errors, state)
        .iter()
        .skip(1)
        .map(|s| s.to_owned())
        .collect();

    if config.errors.is_empty() {
        Ok(lines)
    } else {
        Err(config.errors)
    }
}
