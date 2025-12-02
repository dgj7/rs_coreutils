use crate::input::flags::flag_data::{Flag, FlagValidator};
use crate::input::flags::flags_common::read_dashes_and_name;
use crate::input::flags::flags_dash_enf::validate_dashes;
use crate::input::flags::flags_unrecognized::UnrecognizedFlag;

const DASH_COUNT : usize = 1;

pub struct ShortFlags {
    pub flag_definitions: Vec<Flag>,
    enforce_dash_count: bool,
}

impl ShortFlags {

    pub fn new_from_combined_string(flags: &str, enforce_dash_count: bool) -> ShortFlags {
        let mut fd: Vec<Flag> = vec!();

        /* strip/split dashes and name */
        let (dashes, name) = read_dashes_and_name(&flags);

        /* do some validation, if configured */
        validate_dashes(enforce_dash_count, DASH_COUNT, &dashes);

        /* loop over combined flags */
        for fp in name.chars().map(|c| c.to_string()) {
            fd.push(Flag { expected_dash_count: DASH_COUNT, name: fp });
        }

        /* done */
        ShortFlags { flag_definitions: fd, enforce_dash_count }
    }

    fn search_for_name_matches(&self, index : usize, name: &str) -> (Vec<Flag>, Vec<UnrecognizedFlag>) {
        let mut flags : Vec<Flag> = vec!();
        let mut unrecognized : Vec<UnrecognizedFlag> = vec!();

        for flag_def in self.flag_definitions.iter() {
            if name == flag_def.name {
                flags.push(flag_def.clone());
            }
        }

        if flags.is_empty() {
            unrecognized.push(UnrecognizedFlag { index, argument: Some(name.to_string()) });
        }

        (flags, unrecognized)
    }
}

impl FlagValidator for ShortFlags {
    fn is_valid_flag(&self, flag: &str) -> bool {
        let (matches,unrecognized) = self.find_matching_flags(flag);
        !matches.is_empty() && unrecognized.is_empty()
    }

    fn find_matching_flags(&self, flag: &str) -> (Vec<Flag>, Vec<UnrecognizedFlag>) {
        let (dashes, name) = read_dashes_and_name(flag);
        let mut flags: Vec<Flag> = vec!();
        let mut unrecognized: Vec<UnrecognizedFlag> = vec!();

        for (pf_idx, potential_flag) in name.chars().map(|c| c.to_string()).enumerate() {
            if self.enforce_dash_count && dashes.len() != DASH_COUNT {
                unrecognized.push(UnrecognizedFlag { index: pf_idx, argument: Some(potential_flag.to_string()) });
            } else {
                let (pf_flags, pf_unrecognized) = self.search_for_name_matches(pf_idx, &potential_flag);
                flags.extend(pf_flags);
                unrecognized.extend(pf_unrecognized);
            }
        }

        (flags, unrecognized)
    }
}

#[cfg(test)]
mod tests {
    use crate::input::flags::flag_data::FlagValidator;
    use crate::input::flags::flags_short::ShortFlags;

    #[test]
    #[should_panic]
    fn test_invalid_initialization() {
        ShortFlags::new_from_combined_string("--verbose", true);
    }

    #[test]
    fn test_with_enforcement() {
        /* create object under test */
        let fv = ShortFlags::new_from_combined_string("-vqabc", true);

        /* verify our creation */
        assert_eq!(5, fv.flag_definitions.len());

        /* success when exact match */
        assert_eq!(true, fv.is_valid_flag("-v"));

        /* fail when not exact match */
        assert_eq!(false, fv.is_valid_flag("v"));
        assert_eq!(false, fv.is_valid_flag("--v"));

        /* fail when names don't match */
        assert_eq!(false, fv.is_valid_flag("h"));
        assert_eq!(false, fv.is_valid_flag("help"));
        assert_eq!(false, fv.is_valid_flag("-h"));
        assert_eq!(false, fv.is_valid_flag("-help"));

        /* test with combined args successful */
        let (ca1f, ca1u) = fv.find_matching_flags("-vq");
        assert_eq!(2, ca1f.len());
        assert_eq!(0, ca1u.len());

        /* test with combined args failure */
        let (ca2f, ca2u) = fv.find_matching_flags("-xyz");
        assert_eq!(0, ca2f.len());
        assert_eq!(3, ca2u.len());
    }

    #[test]
    fn test_without_enforcement() {
        /* create object under test */
        let fv = ShortFlags::new_from_combined_string("-vqabc", false);

        /* verify our creation */
        assert_eq!(5, fv.flag_definitions.len());

        /* success if names match; because dashes not enforced */
        assert_eq!(true, fv.is_valid_flag("-v"));
        assert_eq!(true, fv.is_valid_flag("v"));
        assert_eq!(true, fv.is_valid_flag("--v"));

        /* fail when names dont match */
        assert_eq!(false, fv.is_valid_flag("h"));
        assert_eq!(false, fv.is_valid_flag("help"));
        assert_eq!(false, fv.is_valid_flag("-h"));
        assert_eq!(false, fv.is_valid_flag("-help"));

        /* test with combined args successful */
        let (ca1f, ca1u) = fv.find_matching_flags("vq");
        assert_eq!(2, ca1f.len());
        assert_eq!(0, ca1u.len());

        /* test with combined args failure */
        let (ca2f, ca2u) = fv.find_matching_flags("xyz");
        assert_eq!(0, ca2f.len());
        assert_eq!(3, ca2u.len());
    }
}
