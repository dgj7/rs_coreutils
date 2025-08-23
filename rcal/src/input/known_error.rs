///
/// Storage for expected errors.
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct KnownError {
    pub(crate) code: i32,
    pub(crate) message: Option<String>,
}
