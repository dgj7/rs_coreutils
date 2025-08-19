use std::env::Args;
use std::path::PathBuf;

pub struct Config {
    pub show_bytes: bool,
    pub show_chars: bool,
    pub show_lines: bool,
    pub show_words: bool,
    pub show_max_line: bool,
    pub show_file_name: bool,

    pub show_help_exit: bool,
    pub show_version_exit: bool,

    pub file_paths: Vec<PathBuf>,
}

impl Config {
    pub fn should_count_bytes(&self) -> bool {
        return self.show_bytes;
    }

    pub fn should_count_contents(&self) -> bool {
        return self.show_chars || self.show_lines || self.show_words || self.show_max_line;
    }

    pub fn new(args: Args) -> Config {
        let mut config = Config {
            show_bytes: false,
            show_chars: false,
            show_lines: false,
            show_words: false,
            show_max_line: false,

            show_file_name: true,// always true

            show_help_exit: false,
            show_version_exit: false,

            file_paths: vec![],
        };

        // loop over args, skipping the 1st; that is the program name (`rwc`)
        for argument in args.skip(1) {
            match argument.as_str() {
                "-c" => config.show_bytes = true,
                "--bytes" => config.show_bytes = true,

                "-m" => config.show_chars = true,
                "--chars" => config.show_chars = true,

                "-l" => config.show_lines = true,
                "--lines" => config.show_lines = true,

                "-L" => config.show_max_line = true,
                "--max-line-length" => config.show_max_line = true,

                "-w" => config.show_words = true,
                "--words" => config.show_words = true,

                "--help" => config.show_help_exit = true,
                "--version" => config.show_version_exit = true,

                // todo: --files0-from=F

                _ => {
                    if argument.starts_with("-") {
                        // todo: this should print help
                        panic!("unknown arg: {}", argument);
                    }

                    let buf = PathBuf::from(argument);
                    config.file_paths.push(buf);
                }
            }
        }

        return config;
    }
}
