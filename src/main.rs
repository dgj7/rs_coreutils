use crate::config::Config;
use crate::counts::Counts;

mod counts;
mod config;

fn main() {
    let config = Config::new(std::env::args());
    if help(&config) { return };
    let counts = count(&config);
    print(&config, counts);
}

fn help(config: &Config) -> bool {
    if config.show_version_exit {
        println!("version: todo");
        return config.show_version_exit
    }

    if config.show_help_exit {
        println!("help: todo");
        return config.show_help_exit
    }

    println!("help or version not specified");
    return false;
}

fn count(config: &Config) -> Vec<Counts> {
    let mut result: Vec<Counts> = vec![];

    // todo: stop using fake results here
    for (index, file_name) in config.file_paths.iter().enumerate() {
        let counts = Counts::new(10*(index+1), 11*(index+1), 13*(index+1), 14*(index+1), 15*(index+1), file_name.to_path_buf());
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
        let filen = if config.show_file_name { format!(" {}", count.file_path.clone().into_os_string().into_string().unwrap()).trim_end().to_owned() } else { "".to_owned() };
        println!("{}{}{}{}{}{}", bytes, chars, lines, words, maxln, filen);
    }
}
