use crate::time::name::month_num_to_name;
use crate::time::today::TodayFactory;
use common::input::known_error::KnownError;
use common::input::flags::flags_unrecognized::UnrecognizedFlag;

const VALID_FLAGS: &str = "hJejopw31CMSb";
const VALID_COMBINED_FLAGS: &str = "bhJjpwSM";

///
/// Storage for the application configuration.
///
#[derive(Debug, Clone)]
pub(crate) struct Config {
    /* input indicators */
    pub(crate) turn_off_highlight_today: bool,
    pub(crate) display_julian_calendar: bool,
    pub(crate) display_date_of_easter: bool,
    pub(crate) display_julian_days: bool,
    pub(crate) display_date_orthodox_easter: bool,
    pub(crate) print_country_codes: bool,
    pub(crate) print_number_of_week: bool,
    pub(crate) previous_current_next_month: bool,
    pub(crate) only_current_month: bool,
    pub(crate) cal_mode: bool,
    pub(crate) weeks_start_monday: bool,
    pub(crate) weeks_start_sunday: bool,
    pub(crate) use_old_style_format: bool,

    /* string fields read from command line */
    pub(crate) month: Option<String>,
    pub(crate) country_code: Option<String>,
    pub(crate) year: Option<u16>,
    pub(crate) after: Option<usize>,
    pub(crate) before: Option<usize>,
    pub(crate) debug_current_date: Option<String>,
    pub(crate) debug_highlighting: Option<String>,
    pub(crate) first_week_has_at_least_days: Option<String>,

    /* unrecognized arguments */
    pub(crate) unrecognized: Vec<UnrecognizedFlag>,
    pub(crate) errors: Vec<KnownError>,
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
            previous_current_next_month: false,
            only_current_month: false,
            cal_mode: false,
            weeks_start_monday: false,
            weeks_start_sunday: false,
            use_old_style_format: false,

            month: None,
            country_code: None,
            debug_current_date: None,
            debug_highlighting: None,
            first_week_has_at_least_days: None,

            year: None,
            after: None,
            before: None,

            unrecognized: vec![],
            errors: vec![],
        }
    }
}

impl Config {
    ///
    /// Public constructor to initialize from [Vector] of [String].
    ///
    pub(crate) fn new(args: &[String]) -> Config {
        let mut config = Self::default();

        /* initial handling of arguments */
        if args.len() == 1 && !args.first().unwrap().starts_with("-") {
            let today = TodayFactory::Actual.create().make_today();
            config.month = Option::from(month_num_to_name(today.month));
            config.year = Option::from(today.year);
        } else {
            let mut prev_arg_month = false;
            let mut prev_arg_country_code = false;
            let mut prev_arg_year = false;
            let mut prev_arg_months_add_after = false;
            let mut prev_arg_months_add_before = false;
            let mut prev_arg_debug_current_date = false;
            let mut prev_arg_debug_highlighting = false;
            let mut prev_arg_first_week_has_at_least_days = false;

            for (index, argument) in args.iter().enumerate() {
                if index == 0 {
                    continue;
                }

                if prev_arg_month {
                    prev_arg_month = false;
                    config.month = Some(argument.to_owned());
                } else if prev_arg_country_code {
                    prev_arg_country_code = false;
                    config.country_code = Some(argument.to_owned());
                } else if prev_arg_year {
                    prev_arg_year = false;
                    config.year = Some(argument.parse::<u16>().unwrap());
                } else if prev_arg_months_add_after {
                    prev_arg_months_add_after = false;
                    config.after = Some(argument.parse::<usize>().unwrap());
                } else if prev_arg_months_add_before {
                    prev_arg_months_add_before = false;
                    config.before = Some(argument.parse::<usize>().unwrap());
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
                        "-m" => prev_arg_month = true,
                        "-s" => prev_arg_country_code = true,
                        "-y" => prev_arg_year = true,
                        "-A" => prev_arg_months_add_after = true,
                        "-B" => prev_arg_months_add_before = true,
                        "-d" => prev_arg_debug_current_date = true,
                        "-H" => prev_arg_debug_highlighting = true,
                        "-W" => prev_arg_first_week_has_at_least_days = true,

                        _ => {
                            if argument.starts_with('-') {
                                /* check for combined arguments */
                                if Self::is_valid_flag(VALID_COMBINED_FLAGS, argument) {
                                    for ch in argument.chars() {
                                        Self::set_config_for_flag(&mut config, ch)
                                    }
                                } else if argument.len() == 2 && Self::is_valid_flag(VALID_FLAGS, argument) {
                                    Self::set_config_for_flag(&mut config, argument.chars().nth(1).unwrap(), )
                                } else {
                                    config
                                        .unrecognized
                                        .push(UnrecognizedFlag::new(index, argument.to_owned()))
                                }
                            } else {
                                config
                                    .unrecognized
                                    .push(UnrecognizedFlag::new(index, argument.to_owned()))
                            }
                        }
                    }
                }
            }
        }

        /* sort unrecognized, just in case */
        config.unrecognized.sort();

        /* print contents */
        //config.unrecognized.iter().for_each(|u| println!("{:?}", u));

        /* deal with unrecognized args */
        if config.unrecognized.len() == 1 {
            /* 1 arg: year */
            let ua = config.unrecognized.remove(0);
            let temp_year = ua.argument.expect("argument missing");

            if let Ok(year) = temp_year.parse::<u16>() {
                config.year = Some(year);
            } else {
                config.errors.push(KnownError {
                    code: 1,
                    message: Some(format!("rcal: not a valid year {}", temp_year)),
                })
            }
        } else if config.unrecognized.len() == 2 {
            /* 2 args: month year */
            if let Some(pos) = config.unrecognized.iter().position(|u| u.index == 1) {
                let ua = config.unrecognized.remove(pos);
                config.month = ua.argument;
            }

            if let Some(pos) = config.unrecognized.iter().position(|u| u.index == 2) {
                let ua = config.unrecognized.remove(pos);
                let temp_year = ua.argument.expect("argument missing");

                if let Ok(year) = temp_year.parse::<u16>() {
                    config.year = Some(year);
                } else {
                    config.errors.push(KnownError {
                        code: 1,
                        message: Some(format!("rcal: not a valid year {}", temp_year)),
                    })
                }
            }
        }

        /* done */
        config
    }

    fn is_valid_flag(flags: &str, argument: &str) -> bool {
        for ch in argument.chars() {
            if ch == '-' {
                continue;
            }
            if !flags.contains(ch) {
                return false;
            }
        }
        true
    }

    fn set_config_for_flag(config: &mut Config, flag: char) {
        match flag {
            '-' => {}
            'h' => config.turn_off_highlight_today = true,
            'J' => config.display_julian_calendar = true,
            'e' => config.display_date_of_easter = true,
            'j' => config.display_julian_days = true,
            'o' => config.display_date_orthodox_easter = true,
            'p' => config.print_country_codes = true,
            'w' => config.print_number_of_week = true,
            '3' => config.previous_current_next_month = true,
            '1' => config.only_current_month = true,
            'C' => config.cal_mode = true,
            'M' => config.weeks_start_monday = true,
            'S' => config.weeks_start_sunday = true,
            'b' => config.use_old_style_format = true,
            _ => panic!("unrecognized flag: {}", flag),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    fn test_all_args() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("-h"),
            String::from("-J"),
            String::from("-e"),
            String::from("-j"),
            String::from("-m"),
            String::from("january"),
            String::from("-o"),
            String::from("-p"),
            String::from("-s"),
            String::from("uk"),
            String::from("-w"),
            String::from("-y"),
            String::from("2012"),
            String::from("-3"),
            String::from("-1"),
            String::from("-A"),
            String::from("5"),
            String::from("-B"),
            String::from("6"),
            String::from("-C"),
            String::from("-d"),
            String::from("2012-11"),
            String::from("-H"),
            String::from("2002-06-08"),
            String::from("-M"),
            String::from("-S"),
            String::from("-W"),
            String::from("4"),
            String::from("-b"),
        ];

        let config: &Config = &Config::new(&args);

        assert_eq!(true, config.turn_off_highlight_today);
        assert_eq!(true, config.display_julian_calendar);
        assert_eq!(true, config.display_date_of_easter);
        assert_eq!(true, config.display_julian_days);
        assert_eq!(true, config.display_date_orthodox_easter);
        assert_eq!(true, config.print_country_codes);
        assert_eq!(true, config.print_number_of_week);
        assert_eq!(true, config.previous_current_next_month);
        assert_eq!(true, config.only_current_month);
        assert_eq!(true, config.cal_mode);
        assert_eq!(true, config.weeks_start_monday);
        assert_eq!(true, config.weeks_start_sunday);
        assert_eq!(true, config.use_old_style_format);

        assert_eq!("january", config.month.clone().unwrap());
        assert_eq!("uk", config.country_code.clone().unwrap());
        assert_eq!(2012, config.year.clone().unwrap());
        assert_eq!(6, config.before.clone().unwrap());
        assert_eq!(5, config.after.clone().unwrap());
        assert_eq!("2012-11", config.debug_current_date.clone().unwrap());
        assert_eq!("2002-06-08", config.debug_highlighting.clone().unwrap());
        assert_eq!("4", config.first_week_has_at_least_days.clone().unwrap());

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: true, \
            display_julian_calendar: true, \
            display_date_of_easter: true, \
            display_julian_days: true, \
            display_date_orthodox_easter: true, \
            print_country_codes: true, \
            print_number_of_week: true, \
            previous_current_next_month: true, \
            only_current_month: true, \
            cal_mode: true, \
            weeks_start_monday: true, \
            weeks_start_sunday: true, \
            use_old_style_format: true, \
            month: Some(\"january\"), \
            country_code: Some(\"uk\"), \
            year: Some(2012), \
            after: Some(5), \
            before: Some(6), \
            debug_current_date: Some(\"2012-11\"), \
            debug_highlighting: Some(\"2002-06-08\"), \
            first_week_has_at_least_days: Some(\"4\"), \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_combined_args_no_year() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("-bhJjpwSM"),
        ];

        let config = Config::new(&args);

        assert_eq!(true, config.turn_off_highlight_today);
        assert_eq!(true, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(true, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(true, config.print_country_codes);
        assert_eq!(true, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(true, config.weeks_start_monday);
        assert_eq!(true, config.weeks_start_sunday);
        assert_eq!(true, config.use_old_style_format);

        assert_eq!(None, config.month);
        assert_eq!(None, config.country_code);
        assert_eq!(None, config.year);
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: true, \
            display_julian_calendar: true, \
            display_date_of_easter: false, \
            display_julian_days: true, \
            display_date_orthodox_easter: false, \
            print_country_codes: true, \
            print_number_of_week: true, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: true, \
            weeks_start_sunday: true, \
            use_old_style_format: true, \
            month: None, \
            country_code: None, \
            year: None, \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_combined_args() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("-bhJjpwSM"),
            String::from("2021"),
        ];

        let config = Config::new(&args);

        assert_eq!(true, config.turn_off_highlight_today);
        assert_eq!(true, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(true, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(true, config.print_country_codes);
        assert_eq!(true, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(true, config.weeks_start_monday);
        assert_eq!(true, config.weeks_start_sunday);
        assert_eq!(true, config.use_old_style_format);

        assert_eq!(None, config.month);
        assert_eq!(None, config.country_code);
        assert_eq!(2021, config.year.unwrap());
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: true, \
            display_julian_calendar: true, \
            display_date_of_easter: false, \
            display_julian_days: true, \
            display_date_orthodox_easter: false, \
            print_country_codes: true, \
            print_number_of_week: true, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: true, \
            weeks_start_sunday: true, \
            use_old_style_format: true, \
            month: None, \
            country_code: None, \
            year: Some(2021), \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_year_only() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("2022"),
        ];

        let config = Config::new(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(false, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(false, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(false, config.print_country_codes);
        assert_eq!(false, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(false, config.weeks_start_monday);
        assert_eq!(false, config.weeks_start_sunday);
        assert_eq!(false, config.use_old_style_format);

        assert_eq!(None, config.month);
        assert_eq!(None, config.country_code);
        assert_eq!(2022, config.year.clone().unwrap());
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: false, \
            display_julian_calendar: false, \
            display_date_of_easter: false, \
            display_julian_days: false, \
            display_date_orthodox_easter: false, \
            print_country_codes: false, \
            print_number_of_week: false, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: false, \
            weeks_start_sunday: false, \
            use_old_style_format: false, \
            month: None, \
            country_code: None, \
            year: Some(2022), \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_month_year() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("jan"),
            String::from("2019"),
        ];

        let config = Config::new(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(false, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(false, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(false, config.print_country_codes);
        assert_eq!(false, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(false, config.weeks_start_monday);
        assert_eq!(false, config.weeks_start_sunday);
        assert_eq!(false, config.use_old_style_format);

        assert_eq!("jan", config.month.clone().unwrap());
        assert_eq!(None, config.country_code);
        assert_eq!(2019, config.year.clone().unwrap());
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: false, \
            display_julian_calendar: false, \
            display_date_of_easter: false, \
            display_julian_days: false, \
            display_date_orthodox_easter: false, \
            print_country_codes: false, \
            print_number_of_week: false, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: false, \
            weeks_start_sunday: false, \
            use_old_style_format: false, \
            month: Some(\"jan\"), \
            country_code: None, \
            year: Some(2019), \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_year_month() {
        let args = vec![
            String::from("rcal.exe"),
            String::from("2018"),
            String::from("feb"),
        ];

        let config = Config::new(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(false, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(false, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(false, config.print_country_codes);
        assert_eq!(false, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(false, config.weeks_start_monday);
        assert_eq!(false, config.weeks_start_sunday);
        assert_eq!(false, config.use_old_style_format);

        assert_eq!("2018", config.month.clone().unwrap());
        assert_eq!(None, config.country_code);
        assert_eq!(None, config.year);
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(1, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: false, \
            display_julian_calendar: false, \
            display_date_of_easter: false, \
            display_julian_days: false, \
            display_date_orthodox_easter: false, \
            print_country_codes: false, \
            print_number_of_week: false, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: false, \
            weeks_start_sunday: false, \
            use_old_style_format: false, \
            month: Some(\"2018\"), \
            country_code: None, \
            year: None, \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [KnownError { code: 1, message: Some(\"rcal: not a valid year feb\") }] \
            }",
            format!("{:?}", config)
        );
    }

    #[test]
    fn test_no_args() {
        let args: Vec<String> = Vec::new();

        let config = Config::new(&args);

        assert_eq!(false, config.turn_off_highlight_today);
        assert_eq!(false, config.display_julian_calendar);
        assert_eq!(false, config.display_date_of_easter);
        assert_eq!(false, config.display_julian_days);
        assert_eq!(false, config.display_date_orthodox_easter);
        assert_eq!(false, config.print_country_codes);
        assert_eq!(false, config.print_number_of_week);
        assert_eq!(false, config.previous_current_next_month);
        assert_eq!(false, config.only_current_month);
        assert_eq!(false, config.cal_mode);
        assert_eq!(false, config.weeks_start_monday);
        assert_eq!(false, config.weeks_start_sunday);
        assert_eq!(false, config.use_old_style_format);

        assert_eq!(None, config.month);
        assert_eq!(None, config.country_code);
        assert_eq!(None, config.year);
        assert_eq!(None, config.before);
        assert_eq!(None, config.after);
        assert_eq!(None, config.debug_current_date);
        assert_eq!(None, config.debug_highlighting);
        assert_eq!(None, config.first_week_has_at_least_days);

        assert_eq!(0, config.unrecognized.len());
        assert_eq!(0, config.errors.len());

        assert_eq!(
            "Config { \
            turn_off_highlight_today: false, \
            display_julian_calendar: false, \
            display_date_of_easter: false, \
            display_julian_days: false, \
            display_date_orthodox_easter: false, \
            print_country_codes: false, \
            print_number_of_week: false, \
            previous_current_next_month: false, \
            only_current_month: false, \
            cal_mode: false, \
            weeks_start_monday: false, \
            weeks_start_sunday: false, \
            use_old_style_format: false, \
            month: None, \
            country_code: None, \
            year: None, \
            after: None, \
            before: None, \
            debug_current_date: None, \
            debug_highlighting: None, \
            first_week_has_at_least_days: None, \
            unrecognized: [], \
            errors: [] \
            }",
            format!("{:?}", config)
        );
    }
}
