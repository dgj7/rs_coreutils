mod config;
mod timestamp;
mod help;

use std::fs::{File, FileTimes};
use crate::config::Config;
use crate::help::{help, version};
use crate::timestamp::timestamp;

///
/// Main program entrypoint.
///
fn main() {
    let config = Config::new(std::env::args());

    if config.print_help_and_exit {
        help(&config);
    } else if config.print_version_and_exit {
        version(&config);
    } else {
        config.file_paths
            .iter()
            .for_each(|path| {
                if let Some(file) = find_file(&config, path) {
                    let times = find_times(&config);
                    file.set_times(times).unwrap();
                }
            });
    }
}

///
/// Find the file at the given path.
///
fn find_file(config: &Config, path: &str) -> Option<File> {
    if config.do_not_create {
        File::open(path).ok()
    } else {
        File::create(path).ok()
    }
}

///
/// Find the accessed/modified times for the file.
///
fn find_times(config: &Config) -> FileTimes {
    let times = FileTimes::new();
    let ts = timestamp(config);

    if config.change_only_access_time {
        times.set_accessed(ts.accessed);
    } else if config.change_only_modification_time {
        times.set_modified(ts.modified);
    } else {
        times.set_accessed(ts.accessed);
        times.set_modified(ts.modified);
    }

    times
}
