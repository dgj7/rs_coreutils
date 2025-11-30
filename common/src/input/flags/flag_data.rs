use crate::input::unrecognized::UnrecognizedArgument;

#[derive(Debug,PartialEq,Clone)]
pub struct Flag {
    pub expected_dash_count: usize,
    pub name: String,
}

pub trait FlagValidator {
    fn is_valid_flag(&self, flag: &str) -> bool;
    fn find_matching_flags(&self, flag: &str) -> (Vec<Flag>, Vec<UnrecognizedArgument>);
}
