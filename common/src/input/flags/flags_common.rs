const DASH: char = '-';

pub(crate) fn read_dashes_and_name(flag: &str) -> (String, String) {
    let cleansed = flag.trim();
    let dashes: String = cleansed.chars()
        .take_while(|&f| f == DASH)
        .collect();
    let name: String = cleansed.chars()
        .skip_while(|&f| f == DASH)
        .collect();
    (dashes, name)
}
