use std::env::Args;

///
/// Storage for the application configuration.
///
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[allow(dead_code)] // todo: remove this after the fields are used
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
    ///
    /// Public constructor, accepting command-line [Args].
    ///
    pub(crate) fn new(args: Args) -> Config {
        Self::from_vector(&args.collect::<Vec<String>>())
    }

    ///
    /// Helper function to initialize from [Vector] of [String].
    ///
    fn from_vector(args: &Vec<String>) -> Config {
        let mut config = Self::default();

        let mut prev_arg_month = false;
        let mut prev_arg_country_code = false;
        let mut prev_arg_months_add_after = false;
        let mut prev_arg_months_add_before = false;
        let mut prev_arg_debug_current_date = false;
        let mut prev_arg_debug_highlighting = false;
        let mut prev_arg_first_week_has_at_least_days = false;

        for (index, argument) in args.into_iter().skip(1).enumerate() {
            if prev_arg_month {
                prev_arg_month = false;
                config.month = Some(argument.to_owned());
            } else if prev_arg_country_code {
                prev_arg_country_code = false;
                config.country_code = Some(argument.to_owned());
            } else if prev_arg_months_add_after {
                prev_arg_months_add_after = false;
                config.months_add_after = Some(argument.to_owned());
            } else if prev_arg_months_add_before {
                prev_arg_months_add_before = false;
                config.months_add_before = Some(argument.to_owned());
            } else if prev_arg_debug_current_date {
                prev_arg_debug_current_date = false;
                config.debug_current_date = Some(argument.to_owned());
            } else if prev_arg_debug_highlighting {
                prev_arg_debug_highlighting = false;
                config.debug_highlighting = Some(argument.to_owned());
            } else if prev_arg_first_week_has_at_least_days {
                prev_arg_first_week_has_at_least_days = false;
                config.first_week_has_at_least_days = Some(argument.to_owned());
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
                    "-W" => prev_arg_first_week_has_at_least_days = true,

                    _ => config
                        .unrecognized
                        .push(Unrecognized::new(index, argument.to_owned())),
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

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    fn test_all_args() {
        let args = vec![
            String::from("-h"),
            String::from("-J"),
            String::from("-e"),
            String::from("-j"),
            String::from("-m"), String::from("january"),
            String::from("-o"),
            String::from("-p"),
            String::from("-s"), String::from("uk"),
            String::from("-w"),
            String::from("-y"),
            String::from("-3"),
            String::from("-1"),
            String::from("-A"), String::from("5"),
            String::from("-B"), String::from("6"),
            String::from("-C"),
            String::from("-d"), String::from("2012-11"),
            String::from("-H"), String::from("2002-06-08"),
            String::from("-M"),
            String::from("-S"),
            String::from("-W"), String::from("4"),
            String::from("-b"),
        ];

        let config: &Config = &Config::from_vector(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(true, config.display_julian_calendar);
        assert_eq!(true, config.display_date_of_easter);
        assert_eq!(true, config.display_julian_days);
        assert_eq!(true, config.display_date_orthodox_easter);
        assert_eq!(true, config.print_country_codes);
        assert_eq!(true, config.print_number_of_week);
        assert_eq!(true, config.display_for_current_year);
        assert_eq!(true, config.previous_current_next_month);
        assert_eq!(true, config.only_current_month);
        assert_eq!(true, config.cal_mode);
        assert_eq!(true, config.weeks_start_monday);
        assert_eq!(true, config.weeks_start_sunday);
        assert_eq!(true, config.use_old_style_format);

        assert_eq!("january", config.month.clone().unwrap());
        assert_eq!("uk", config.country_code.clone().unwrap());
        assert_eq!("5", config.months_add_after.clone().unwrap());
        assert_eq!("6", config.months_add_before.clone().unwrap());
        assert_eq!("2012-11", config.debug_current_date.clone().unwrap());
        assert_eq!("2002-06-08", config.debug_highlighting.clone().unwrap());
        assert_eq!("4", config.first_week_has_at_least_days.clone().unwrap());

        assert_eq!(0, config.unrecognized.len());

        assert_eq!(
            "Config { turn_off_highlight_today: false, display_julian_calendar: true, display_date_of_easter: true, display_julian_days: true, display_date_orthodox_easter: true, print_country_codes: true, print_number_of_week: true, display_for_current_year: true, previous_current_next_month: true, only_current_month: true, cal_mode: true, weeks_start_monday: true, weeks_start_sunday: true, use_old_style_format: true, month: Some(\"january\"), country_code: Some(\"uk\"), months_add_after: Some(\"5\"), months_add_before: Some(\"6\"), debug_current_date: Some(\"2012-11\"), debug_highlighting: Some(\"2002-06-08\"), first_week_has_at_least_days: Some(\"4\"), unrecognized: [] }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_no_args() {
        let args: Vec<String> = Vec::new();

        let config = Config::from_vector(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(false, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(false, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(false, config.print_country_codes);
        assert_eq!(false, config.print_number_of_week);
        assert_eq!(false, config.display_for_current_year);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(false, config.weeks_start_monday);
        assert_eq!(false, config.weeks_start_sunday);
        assert_eq!(false, config.use_old_style_format);

        assert_eq!(None, config.month);
        assert_eq!(None, config.country_code);
        assert_eq!(None, config.months_add_after);
        assert_eq!(None, config.months_add_before);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());

        assert_eq!(
            "Config { turn_off_highlight_today: false, display_julian_calendar: false, display_date_of_easter: false, display_julian_days: false, display_date_orthodox_easter: false, print_country_codes: false, print_number_of_week: false, display_for_current_year: false, previous_current_next_month: false, only_current_month: false, cal_mode: false, weeks_start_monday: false, weeks_start_sunday: false, use_old_style_format: false, month: None, country_code: None, months_add_after: None, months_add_before: None, debug_current_date: None, debug_highlighting: None, first_week_has_at_least_days: None, unrecognized: [] }",
            format!("{:?}", config)
        );
    }
}
