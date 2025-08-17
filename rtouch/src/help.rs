use crate::config::Config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn version(_config: &Config) {
    println!("rtouch {}", VERSION);
}

pub(crate) fn help(config: &Config) {
    version(config);
}
