use std::path::PathBuf;

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
