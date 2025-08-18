use std::fs::File;
use crate::config::Config;
use std::time::SystemTime;


pub(crate) struct Timestamp {
    pub(crate) accessed: SystemTime,
    pub(crate) modified: SystemTime,
}

pub(crate) fn timestamp(config: &Config) -> Timestamp {
    if config.use_specified_time_stamp {
        specified_time_stamp(config)
    } else if config.date_string.is_some() {
        date_string(config)
    } else if config.reference_file.is_some() {
        reference_file(config)
    } else {
        Timestamp {
            accessed: SystemTime::now(),
            modified: SystemTime::now(),
        }
    }
}

fn specified_time_stamp(_config: &Config) -> Timestamp {
    //config.specified_time_stamp.unwrap();
    panic!("not implemented"); // todo
}

fn date_string(_config: &Config) -> Timestamp {
    panic!("not implemented"); // todo
}


fn reference_file(config: &Config) -> Timestamp {
    let path = config.reference_file.as_ref().unwrap();
    match File::open(path) {
        Ok(file) => {
            let modified = file.metadata().unwrap().modified().unwrap();
            let accessed = file.metadata().unwrap().accessed().unwrap();
            Timestamp { accessed, modified, }
        },
        Err(e) => panic!("can't open file [{}]: [{}]", path, e),
    }
}
