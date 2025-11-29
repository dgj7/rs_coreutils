///
/// Storage for expected errors.
///
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct KnownError {
    pub code: i32,
    pub message: Option<String>,
}
