use crate::{cfg_app, formatter};
use crate::today::Today;

pub fn generate_cal(args: std::env::Args, today: &impl Today) -> Vec<String> {
    let config = cfg_app::AppConfig::new(args, today);
    formatter::format_from_app_config(config)
        .iter()
        .skip(1)
        .map(|s| s.to_owned())
        .collect()
}
