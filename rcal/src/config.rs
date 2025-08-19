use std::env::Args;

///
/// Storage for the application configuration.
///
pub(crate) struct Config {
    pub(crate) turn_off_highlight_today: bool,
    pub(crate) display_julian_calendar: bool,
    pub(crate) display_date_of_easter: bool,
    pub(crate) display_julian_days: bool,
    pub(crate) display_date_orthodox_easter: bool,
    pub(crate) print_country_codes: bool,
    pub(crate) print_number_of_week: bool,
    pub(crate) display_for_current_year: bool,
    pub(crate) previous_current_next_month: bool,
    pub(crate) only_current_month: bool,
    pub(crate) cal_mode: bool,
    pub(crate) weeks_start_monday: bool,
    pub(crate) weeks_start_sunday: bool,
    pub(crate) use_old_style_format: bool,

    pub(crate) month: Option<String>,
    pub(crate) country_code: Option<String>,
    pub(crate) months_add_after: Option<String>,
    pub(crate) months_add_before: Option<String>,
    pub(crate) debug_current_date: Option<String>,
    pub(crate) debug_highlighting: Option<String>,
    pub(crate) first_week_has_at_least_days: Option<String>,

    pub(crate) unrecognized: Vec<Unrecognized>,
}

///
/// Storage for arguments that aren't immediately recognized.
/// 
pub(crate) struct Unrecognized {
    pub(crate) index: usize,
    pub(crate) argument: Option<String>,
}

impl Default for Config {
    ///
    /// Generate [Default] configuration.
    ///
    fn default() -> Config {
        Config {
            turn_off_highlight_today: false,
            display_julian_calendar: false,
            display_date_of_easter: false,
            display_julian_days: false,
            display_date_orthodox_easter: false,
            print_country_codes: false,
            print_number_of_week: false,
            display_for_current_year: false,
            previous_current_next_month: false,
            only_current_month: false,
            cal_mode: false,
            weeks_start_monday: false,
            weeks_start_sunday: false,
            use_old_style_format: false,

            month: None,
            country_code: None,
            months_add_after: None,
            months_add_before: None,
            debug_current_date: None,
            debug_highlighting: None,
            first_week_has_at_least_days: None,

            unrecognized: vec![],
        }
    }
}

impl Config {
    pub(crate) fn new(args: Args) -> Config {
        let mut config = Self::default();

        let mut prev_arg_month = false;
        let mut prev_arg_country_code = false;
        let mut prev_arg_months_add_after = false;
        let mut prev_arg_months_add_before = false;
        let mut prev_arg_debug_current_date = false;
        let mut prev_arg_debug_highlighting = false;
        let mut prev_arg_first_week_has_at_least_days = false;

        for (index, argument) in args.skip(1).enumerate() {
            if prev_arg_month {
                prev_arg_month = false;
                config.month = Some(argument);
            } else if prev_arg_country_code {
                prev_arg_country_code = false;
                config.country_code = Some(argument);
            } else if prev_arg_months_add_after {
                prev_arg_months_add_after = false;
                config.months_add_after = Some(argument);
            } else if prev_arg_months_add_before {
                prev_arg_months_add_before = false;
                config.months_add_before = Some(argument);
            } else if prev_arg_debug_current_date {
                prev_arg_debug_current_date = false;
                config.debug_current_date = Some(argument);
            } else if prev_arg_debug_highlighting {
                prev_arg_debug_highlighting = false;
                config.debug_highlighting = Some(argument);
            } else if prev_arg_first_week_has_at_least_days {
                prev_arg_first_week_has_at_least_days = false;
                config.first_week_has_at_least_days = Some(argument);
            } else {
                match argument.as_str() {
                    "-h" => config.turn_off_highlight_today = true,
                    "-J" => config.display_julian_calendar = true,
                    "-e" => config.display_date_of_easter = true,
                    "-j" => config.display_julian_days = true,
                    "-o" => config.display_date_orthodox_easter = true,
                    "-p" => config.print_country_codes = true,
                    "-w" => config.print_number_of_week = true,
                    "-y" => config.display_for_current_year = true,
                    "-3" => config.previous_current_next_month = true,
                    "-1" => config.only_current_month = true,
                    "-C" => config.cal_mode = true,
                    "-M" => config.weeks_start_monday = true,
                    "-S" => config.weeks_start_sunday = true,
                    "-b" => config.use_old_style_format = true,

                    "-m" => prev_arg_month = true,
                    "-s" => prev_arg_country_code = true,
                    "-A" => prev_arg_months_add_after = true,
                    "-B" => prev_arg_months_add_before = true,
                    "-d" => prev_arg_debug_current_date = true,
                    "-H" => prev_arg_debug_highlighting = true,

                    _ => config.unrecognized.push(Unrecognized::new(index, argument)),
                }
            }
        }

        config
    }
}

impl Unrecognized {
    fn new(idx: usize, arg: String) -> Unrecognized {
        Unrecognized {
            index: idx,
            argument: Some(arg),
        }
    }
}
