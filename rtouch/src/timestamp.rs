use std::time::SystemTime;
use crate::config::Config;

pub(crate) fn timestamp(config: &Config) -> SystemTime {
    if config.use_specified_time_stamp {
        specified_time_stamp(config)
    } else if config.date_string.is_some() {
        date_string(config)
    } else {
        SystemTime::now()
    }
}

fn specified_time_stamp(_config: &Config) -> SystemTime {
    panic!("not implemented");// todo
}

fn date_string(_config: &Config) -> SystemTime {
    panic!("not implemented");// todo
}
