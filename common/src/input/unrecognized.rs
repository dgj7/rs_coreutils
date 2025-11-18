///
/// Storage for arguments that aren't immediately recognized.
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct UnrecognizedArgument {
    pub index: usize,
    pub argument: Option<String>,
}

impl UnrecognizedArgument {
    pub fn new(idx: usize, arg: String) -> UnrecognizedArgument {
        UnrecognizedArgument {
            index: idx,
            argument: Some(arg),
        }
    }
}
