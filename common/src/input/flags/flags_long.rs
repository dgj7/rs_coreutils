use crate::input::flags::flag_data::{Flag, FlagValidator};
use crate::input::flags::flags_common::{read_dashes_and_name};
use crate::input::unrecognized::UnrecognizedArgument;

const DASH_COUNT: usize = 2;

pub struct LongFlags {
    pub flag_definitions: Vec<Flag>,
    enforce_dash_count: bool,
}

impl LongFlags {
    pub fn new_from_strings(potentials: &Vec<String>, enforce_dash_count: bool) -> LongFlags {
        let mut fd: Vec<Flag> = vec!();

        for potential in potentials {
            /* strip/split dashes and name */
            let (dashes, name) = read_dashes_and_name(&potential);

            /* do some validation, if configured */
            if enforce_dash_count {
                if dashes.len() == 0 {
                    panic!("{}: configuration error: this mode expects that supplied flags will have dashes", std::any::type_name::<Flag>());
                }
                if dashes.len() != DASH_COUNT {
                    panic!("{}: configuration error: this mode expects {} dashes", std::any::type_name::<Flag>(), DASH_COUNT);
                }
            }

            /* create the flag object itself */
            let result = Flag { expected_dash_count: DASH_COUNT, name };

            /* update the outgoing vector */
            if !fd.contains(&result) {
                fd.push(result);
            }
        }

        LongFlags { flag_definitions: fd, enforce_dash_count }
    }
}

impl FlagValidator for LongFlags {
    fn is_valid_flag(&self, flag: &str) -> bool {
        let (matches,unrecognized) = self.find_matching_flags(flag);
        !matches.is_empty() && unrecognized.is_empty()
    }

    fn find_matching_flags(&self, flag: &str) -> (Vec<Flag>, Vec<UnrecognizedArgument>) {
        let (dashes, name) = read_dashes_and_name(flag);
        let mut flags: Vec<Flag> = vec!();
        let mut unrecognized: Vec<UnrecognizedArgument> = vec!();
        for fd in self.flag_definitions.iter() {
            let dashes_ok = if self.enforce_dash_count {
                fd.expected_dash_count == dashes.len()
            } else {
                true
            };
            let name_ok = fd.name == name;
            if dashes_ok && name_ok {
                flags.push(fd.clone());
            }
        }

        if flags.is_empty() {
            unrecognized.push(UnrecognizedArgument { index: 0, argument: Some(flag.to_string())});
        }

        (flags,unrecognized)
    }
}

#[cfg(test)]
mod tests {
    use crate::input::flags::flag_data::FlagValidator;
    use crate::input::flags::flags_long::LongFlags;

    #[test]
    #[should_panic]
    fn test_invalid_initialization() {
        LongFlags::new_from_strings(&vec!["-v".to_owned()],true);
    }

    #[test]
    fn test_with_enforcement() {
        /* create object under test */
        let fv = LongFlags::new_from_strings(&vec![
            "--verbose".to_owned(),
            "--quiet".to_owned(),
        ], true);

        /* verify our creation */
        assert_eq!(2, fv.flag_definitions.len());

        /* success when exact match */
        assert_eq!(false, fv.is_valid_flag("-v"));
        assert_eq!(true, fv.is_valid_flag("--verbose"));

        /* fail when no dashes */
        assert_eq!(false, fv.is_valid_flag("v"));
        assert_eq!(false, fv.is_valid_flag("verbose"));

        /* fail when dash count doesn't match */
        assert_eq!(false, fv.is_valid_flag("--v"));
        assert_eq!(false, fv.is_valid_flag("-valid"));

        /* fail when names don't match */
        assert_eq!(false, fv.is_valid_flag("h"));
        assert_eq!(false, fv.is_valid_flag("help"));
        assert_eq!(false, fv.is_valid_flag("-h"));
        assert_eq!(false, fv.is_valid_flag("-help"));
    }

    #[test]
    fn test_without_enforcement() {
        /* create object under test */
        let fv = LongFlags::new_from_strings(&vec![
            "-v".to_owned(), "--verbose".to_owned(),
            "-q".to_owned(), "--quiet".to_owned(),
        ], false);

        /* verify our creation */
        assert_eq!(4, fv.flag_definitions.len());

        /* success when exact match */
        assert_eq!(true, fv.is_valid_flag("-v"));
        assert_eq!(true, fv.is_valid_flag("--verbose"));

        /* success, as dashes are not enforced */
        assert_eq!(true, fv.is_valid_flag("v"));
        assert_eq!(true, fv.is_valid_flag("verbose"));

        /* success, as dashes are not enforced */
        assert_eq!(true, fv.is_valid_flag("--v"));
        assert_eq!(true, fv.is_valid_flag("-verbose"));

        /* fail when names don't match */
        assert_eq!(false, fv.is_valid_flag("h"));
        assert_eq!(false, fv.is_valid_flag("help"));
        assert_eq!(false, fv.is_valid_flag("-h"));
        assert_eq!(false, fv.is_valid_flag("-help"));
    }
}
