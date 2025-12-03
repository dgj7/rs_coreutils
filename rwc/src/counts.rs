use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::PathBuf;
use crate::config::Config;
use crate::printer::{path_to_string, print_help, print_version};

pub struct Counts {
    pub bytes: u64,
    pub chars: usize,
    pub lines: usize,
    pub words: usize,
    pub max_line: usize,
    pub file_path: PathBuf
}

impl Counts {
    pub(crate) fn new(input_bytes: u64, input_chars: usize, input_lines: usize, input_words: usize, input_max_line: usize, input_file_path: PathBuf) -> Counts {
        Counts { bytes: input_bytes, chars: input_chars, lines: input_lines, words: input_words, max_line: input_max_line, file_path: input_file_path }
    }
}

pub(crate) fn count(config: &Config) -> Vec<Counts> {
    let mut result: Vec<Counts> = vec![];

    if config.show_version_exit {
        print_version();
    } else if config.show_help_exit {
        print_help();
    } else if config.should_count_bytes() || config.should_count_contents() {
        perform_count_in_files(&config, &mut result);
    } else {
        // actual wc program waits for input; ours will just throw an error and exit
        panic!("no valid command line arguments provided");
    }

    result
}

fn perform_count_in_files(config: &Config, result: &mut Vec<Counts>) {
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
}

fn perform_count_in_file(file: &File, counts: &mut Counts) {
    /* loop over the lines in the file */
    for (line_index, line_result) in io::BufReader::new(file).lines().enumerate() {
        /* attempt to get the next line */
        let line = match line_result {
            Err(reason) => panic!("failed to read line {}: {}", line_index, reason),
            Ok(value) => value
        };

        /* increment max line */
        let line_len = line.len();
        if line_len > counts.max_line {
            counts.max_line = line_len;
        }

        /* increment line count */
        counts.lines = counts.lines + 1;

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

    // todo: if the last character is a newline, add 1 to lines

    /* the last line of the file, whatever it is, doesn't end in a newline */
    if counts.lines > 0 {
        counts.lines = counts.lines - 1;
    }
}
