const DASH: char = '-';

#[derive(PartialEq,Clone)]
pub struct Flag {
    dash_count: usize,
    name: String,
}

pub struct StringFlagValidator {
    flags : Vec<Flag>
}

impl StringFlagValidator {
    pub fn new_from_strings(flags: Vec<String>) -> StringFlagValidator {
        let mut vec : Vec<Flag> = Vec::new();
        for flag in flags {
            let dashes : String = flag.chars()
                .take_while(|&f| f == DASH)
                .collect();
            let name : String = flag.chars()
                .skip_while(|&f| f == DASH)
                .collect();

            if dashes.len() == 0 {
                panic!("configuration error: flag without dashes");
            }

            let result = Flag { dash_count: dashes.len(), name };

            if !vec.contains(&result) {
                vec.push(result);
            }
        }

        Self::new_from_flags(&vec)
    }

    pub fn new_from_flags(flags: &Vec<Flag>) -> StringFlagValidator {
        StringFlagValidator { flags : flags.to_owned() }
    }

    pub fn is_valid_flag(&self, flag: &str) -> bool {
        let dashes : String = flag.chars()
            .take_while(|&f| f == DASH)
            .collect();
        let name : String = flag.chars()
            .skip_while(|&f| f == DASH)
            .collect();

        self.flags
            .iter()
            .find(|&f| f.name == name && f.dash_count == dashes.len())
            .is_some()
    }

    #[allow(dead_code)]// test-only function
    fn read(&self) -> Vec<String> {
        self.flags
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::flags::StringFlagValidator;

    #[test]
    fn test() {
        let fv = StringFlagValidator::new_from_strings(vec![
            "-v".to_owned(), "--verbose".to_owned(),
            "-q".to_owned(), "--quiet".to_owned(),
        ]);

        assert_eq!(4, fv.read().len());

        /* success when exact match */
        assert_eq!(true, fv.is_valid_flag("-v"));
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
}
