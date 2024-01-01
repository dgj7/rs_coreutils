#[allow(dead_code)]

use crate::config::Config;
use crate::counts::Counts;

mod counts;
mod config;

fn main() {
    let config = Config::new_test();
    let counts = count(&config);
    print(&config, counts);
}

fn count(_config: &Config) -> Vec<Counts> {
    let mut result: Vec<Counts> = vec![];

    for n in 1 ..=3 {
        let counts = Counts::new(10*n, 11*n, 12*n, 13*n, 14*n);
        result.push(counts);
    }

    return result
}

fn print(config: &Config, counts : Vec<Counts>) {
    for count in counts.iter() {
        let bytes = if config.show_bytes { format!(" {}", count.bytes).trim_end().to_owned() } else { "".to_owned() };
        let chars = if config.show_chars { format!(" {}", count.chars).trim_end().to_owned() } else { "".to_owned() };
        let lines = if config.show_lines { format!(" {}", count.lines).trim_end().to_owned() } else { "".to_owned() };
        let words = if config.show_words { format!(" {}", count.words).trim_end().to_owned() } else { "".to_owned() };
        let maxln = if config.show_max_line { format!(" {}", count.max_line).trim_end().to_owned() } else { "".to_owned() };
        let filen = if config.show_file_name { format!(" {}", config.file_path).trim_end().to_owned() } else { "".to_owned() };
        println!("{}{}{}{}{}{}", bytes, chars, lines, words, maxln, filen);
    }
}
