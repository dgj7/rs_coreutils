use crate::input::flags::flag_data::Flag;

const DASH : &str = "-";

pub fn validate_dashes(enforce_dash_count: bool, expected_dash_count: usize, dashes: &String) {
    /* exit if enforcement is disabled */
    if !enforce_dash_count {
        return;
    }

    /* panic if dashes has non-dashes */
    if dashes.chars().any(|c| !DASH.contains(c)) {
        panic!("dashes string contains non-dash characters: [{}]", dashes);
    }

    /* handle if dash count zero */
    if dashes.len() == 0 && expected_dash_count != 0 {
        panic!("{}: configuration error: this mode expects that supplied flags will have dashes", std::any::type_name::<Flag>());
    }

    /* handle if dash count doesn't match expected */
    if dashes.len() != expected_dash_count {
        panic!("{}: configuration error: this mode expects {} dashes", std::any::type_name::<Flag>(), expected_dash_count);
    }
}
