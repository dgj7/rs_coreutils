use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;
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
    } else if config.should_count_bytes() || config.should_count_contents() {
        for file_path in config.file_paths.iter() {
            /* open the file; fail if unable to open */
            let file = match File::open(file_path) {
                Err(reason) => panic!("can't open {}: {}", path_to_string(file_path), reason),
                Ok(file) => file,
            };

            /* create counts; initialize to zero */
            let mut counts = Counts::new(0, 0, 0, 0, 0, file_path.to_path_buf());

            /* count bytes, if configured */
            if config.show_bytes {
                /* byte counting can be less manual */
                counts.bytes = match file.metadata() {
                    Err(reason) => panic!("failed to count bytes of {}: {}", path_to_string(file_path), reason),
                    Ok(metadata) => metadata.len()
                };
            }

            /* count contents, if configured */
            if config.should_count_contents() {
                perform_count_in_file(&file, &mut counts);
            }

            result.push(counts);
        }
    } else {
        // actual wc program waits for input; ours will just throw an error and exit
        panic!("no valid command line arguments provided");
    }

    return result
}

fn perform_count_in_file(file: &File, counts: &mut Counts) {
    /* loop over the lines in the file */
    for (line_index, line_result) in io::BufReader::new(file).lines().enumerate() {
        /* attempt to get the next line */
        let line = match line_result {
            Err(reason) => panic!("failed to read line {}: {}", line_index, reason),
            Ok(value) => value
        };

        /* increment line count */
        counts.lines = counts.lines + 1;

        /* increment max line */
        let line_len = line.len();
        if line_len > counts.max_line {
            counts.max_line = line_len;
        }

        /* loop over the characters in the line */
        let mut previous_was_whitespace = false;
        for (_char_index, character) in line.chars().enumerate() {
            /* increment char count */
            counts.chars = counts.chars + 1;

            /* increment word count */
            if char::is_whitespace(character) {
                if !previous_was_whitespace {
                    counts.words = counts.words + 1;
                }
                previous_was_whitespace = true;
            } else {
                previous_was_whitespace = false;
            }
        }
    }
}

fn print(config: &Config, counts : Vec<Counts>) {
    for count in counts.iter() {
        let bytes = if config.show_bytes { format!(" {}", count.bytes).trim_end().to_owned() } else { "".to_owned() };
        let chars = if config.show_chars { format!(" {}", count.chars).trim_end().to_owned() } else { "".to_owned() };
        let lines = if config.show_lines { format!(" {}", count.lines).trim_end().to_owned() } else { "".to_owned() };
        let words = if config.show_words { format!(" {}", count.words).trim_end().to_owned() } else { "".to_owned() };
        let max_len = if config.show_max_line { format!(" {}", count.max_line).trim_end().to_owned() } else { "".to_owned() };
        let file_name = if config.show_file_name { format!(" {}", path_to_string(&count.file_path)).trim_end().to_owned() } else { "".to_owned() };
        println!("{}{}{}{}{}{}", lines, words, bytes, chars, max_len, file_name);
    }
}

fn path_to_string(path_buf: &PathBuf) -> String {
    path_buf.clone().into_os_string().into_string().unwrap()
}
