pub struct Config {
    pub show_bytes: bool,
    pub show_chars: bool,
    pub show_lines: bool,
    pub show_words: bool,
    pub show_max_line: bool,
    pub show_file_name: bool,
    // todo: this might need to be a vector of paths
    pub file_path : String,
}

impl Config {
    // todo: from() method that takes parsed command line params as input
    pub fn new_test() -> Config {
        Config {
            show_bytes: true,
            show_chars: true,
            show_lines: true,
            show_words: true,
            show_max_line: true,
            show_file_name: true,
            file_path: "data/file1.txt".to_string()
        }
    }
}
