///
/// Storage for arguments that aren't immediately recognized.
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnrecognizedFlag {
    pub index: usize,
    pub argument: Option<String>,
}

impl UnrecognizedFlag {
    pub fn new(idx: usize, arg: String) -> UnrecognizedFlag {
        UnrecognizedFlag {
            index: idx,
            argument: Some(arg),
        }
    }
}
