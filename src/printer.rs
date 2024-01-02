use std::path::PathBuf;
use crate::config::Config;
use crate::counts::Counts;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BASE_WIDTH: usize = 2;

pub fn print_results(config: &Config, counts : Vec<Counts>) {
    for count in counts.iter() {
        let lines     = if config.show_lines     { format!("{:>width$}", count.lines,    width = calc_width(count.lines)) }          else { "".to_owned() };
        let words     = if config.show_words     { format!("{:>width$}", count.words,    width = calc_width(count.words)) }          else { "".to_owned() };
        let bytes     = if config.show_bytes     { format!("{:>width$}", count.bytes,    width = calc_width(count.bytes as usize)) } else { "".to_owned() };
        let chars     = if config.show_chars     { format!("{:>width$}", count.chars,    width = calc_width(count.chars)) }          else { "".to_owned() };
        let max_len   = if config.show_max_line  { format!("{:>width$}", count.max_line, width = calc_width(count.max_line)) }       else { "".to_owned() };

        let file_name = if config.show_file_name { format!(" {}", path_to_string(&count.file_path)).trim_end().to_owned() } else { "".to_owned() };

        println!("{}{}{}{}{}{}", lines, words, bytes, chars, max_len, file_name);
    }
}

fn calc_width(count: usize) -> usize {
    if count == 0 {
        return BASE_WIDTH
    }

    return count.ilog10() as usize + BASE_WIDTH
}

pub fn path_to_string(path_buf: &PathBuf) -> String {
    path_buf.clone().into_os_string().into_string().unwrap()
}

pub fn print_version() {
    println!("rwc {}", VERSION);
}

pub fn print_help() {
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
}
