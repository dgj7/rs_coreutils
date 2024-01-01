pub struct Counts {
    pub bytes: i64,
    pub chars: i64,
    pub lines: i64,
    pub words: i64,
    pub max_line: i64,
}

impl Counts {
    pub(crate) fn new(input_bytes: i64, input_chars: i64, input_lines: i64, input_words: i64, input_max_line: i64) -> Counts {
        Counts { bytes: input_bytes, chars: input_chars, lines: input_lines, words: input_words, max_line: input_max_line }
    }
}
