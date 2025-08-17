mod config;
mod timestamp;

//use std::fs::{File, FileTimes};
//use std::time::SystemTime;
use crate::config::Config;

fn main() {
    let _config = Config::new(std::env::args());
    // todo: process to application state
    // todo: print application state

    //let file = File::open("test.txt").unwrap();
    //file.metadata().unwrap();
    //let times = FileTimes::new()
    //    .set_accessed(SystemTime::now());
}
