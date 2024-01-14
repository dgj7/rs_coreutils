use crate::{cfg_app, formatter};

pub fn generate_cal(args: std::env::Args) -> Vec<String> {
    let config = cfg_app::AppConfig::new(args);
    return formatter::format_from_app_config(config);
}
