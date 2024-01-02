use crate::config::Config;
use crate::counts::Counts;

mod counts;
mod config;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let config = Config::new(std::env::args());
    let counts = count(&config);
    print(&config, counts);
}

fn count(config: &Config) -> Vec<Counts> {
    let mut result: Vec<Counts> = vec![];

    if config.show_version_exit {
        println!("rwc {}", VERSION);
    } else if config.show_help_exit {
        println!("Usage: wc [OPTION]... [FILE]...");
        println!("    or:  wc [OPTION]... --files0-from=F");
        println!("Print newline, word, and byte counts for each FILE, and a total line if");
        println!("more than one FILE is specified.  A word is a non-zero-length sequence of");
        println!("characters delimited by white space.");
        println!();
        println!("    With no FILE, or when FILE is -, read standard input.");
        println!();
        println!("    The options below may be used to select which counts are printed, always in");
        println!("the following order: newline, word, character, byte, maximum line length.");
        println!("    -c, --bytes            print the byte counts");
        println!("    -m, --chars            print the character counts");
        println!("    -l, --lines            print the newline counts");
        println!("    --files0-from=F    read input from the files specified by");
        println!("NUL-terminated names in file F;");
        println!("If F is - then read names from standard input");
        println!("    -L, --max-line-length  print the maximum display width");
        println!("    -w, --words            print the word counts");
        println!("    --help     display this help and exit");
        println!("    --version  output version information and exit");
    } else if config.show_bytes || config.show_chars || config.show_lines || config.show_words || config.show_max_line {
        // todo: stop using fake results here
        for (index, file_name) in config.file_paths.iter().enumerate() {
            let counts = Counts::new(10*(index+1), 11*(index+1), 13*(index+1), 14*(index+1), 15*(index+1), file_name.to_path_buf());
            result.push(counts);
        }
    } else {
        // actual wc program waits for input; ours will just throw an error and exit
        panic!("no valid command line arguments provided");
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
