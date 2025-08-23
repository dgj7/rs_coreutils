///
/// Storage for arguments that aren't immediately recognized.
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UnrecognizedArgument {
    pub(crate) index: usize,
    pub(crate) argument: Option<String>,
}
