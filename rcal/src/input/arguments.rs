use common::input::known_error::KnownError;
use common::input::command::{BooleanCommand, CommandDefinition, YearCommand, StringCommand, CountCommand};
use common::input::unrecognized::UnrecognizedArgument;
use crate::time::name::month_num_to_name;
use crate::time::today::TodayFactory;

///
/// Storage for arguments parsed.
///
pub(crate) struct Arguments {
    pub(crate) turn_off_highlight_today: BooleanCommand,
    pub(crate) display_julian_calendar: BooleanCommand,
    pub(crate) display_date_of_easter: BooleanCommand,
    pub(crate) display_julian_days: BooleanCommand,
    pub(crate) month: StringCommand,
    pub(crate) display_date_orthodox_easter: BooleanCommand,
    pub(crate) print_country_codes: BooleanCommand,
    pub(crate) country_code: StringCommand,
    pub(crate) print_number_of_week: BooleanCommand,
    pub(crate) year: YearCommand,
    pub(crate) previous_current_next_month: BooleanCommand,
    pub(crate) only_current_month: BooleanCommand,
    pub(crate) after: CountCommand,
    pub(crate) before: CountCommand,
    pub(crate) cal_mode: BooleanCommand,
    pub(crate) debug_current_date: StringCommand,
    pub(crate) debug_highlighting: StringCommand,
    pub(crate) weeks_start_monday: BooleanCommand,
    pub(crate) weeks_start_sunday: BooleanCommand,
    pub(crate) first_week_has_at_least_days: StringCommand,
    pub(crate) use_old_style_format: BooleanCommand,

    /* problems */
    pub(crate) unrecognized: Vec<UnrecognizedArgument>,
    pub(crate) errors: Vec<KnownError>,
}

impl Arguments {
    fn initialize(&self) -> Self {
        Arguments {
            turn_off_highlight_today: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Turns off highlighting of today."),
                },
                value: false,
            },
            display_julian_calendar: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display Julian Calendar, if combined with the -o option, display date of Orthodox Easter according to the Julian Calendar."),
                },
                value: false,
            },
            display_date_of_easter: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display date of Easter (for western churches)."),
                },
                value: false,
            },
            display_julian_days: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display Julian days (days one-based, numbered from January 1)."),
                },
                value: false,
            },
            month: StringCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display the specified month.  If month is specified as a decimal number, appending ‘f’ or ‘p’ displays the same month of the following or previous year respectively."),
                },
                value: None,
            },
            display_date_orthodox_easter: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display date of Orthodox Easter (Greek and Russian Orthodox Churches)."),
                },
                value: false,
            },
            print_country_codes: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Print the country codes and switching days from Julian to Gregorian Calendar as they are assumed by ncal.  The country code as determined from the local environment is marked with an asterisk."),
                },
                value: false,
            },
            country_code: StringCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Assume the switch from Julian to Gregorian Calendar at the date associated with the country_code.  If not specified, ncal tries to guess the switch date from the local environment or falls back to September 2, 1752.  This was when Great Britain and her colonies switched to the Gregorian Calendar."),
                },
                value: None,
            },
            print_number_of_week: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Print the number of the week below each week column."),
                },
                value: false,
            },
            year: YearCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display a calendar for the specified year. This option is implied when a year but no month are specified on the input line."),
                },
                value: None,
            },
            previous_current_next_month: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display the previous, current and next month surrounding today."),
                },
                value: false,
            },
            only_current_month: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Display only the current month. This is the default."),
                },
                value: false,
            },
            after: CountCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Months to add after. The specified number of months is added to the end of the display. This is in addition to any date range selected by the -y, -3, or -1 options. For example, “cal -y -B2 -A2” shows everything from November of the previous year to February of the following year. Negative numbers are allowed, in which case the specified number of months is subtracted. For example, “cal -y -B-6” shows July to December. And “cal -A11” simply shows the next 12 months."),
                },
                value: None,
            },
            before: CountCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Months to add before. The specified number of months is added to the beginning of the display. See -A for examples."),
                },
                value: None,
            },
            cal_mode: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Completely switch to cal mode. For cal like output only, use -b instead."),
                },
                value: false,
            },
            debug_current_date: StringCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Use yyyy-mm as the current date (for debugging of date selection)."),
                },
                value: None,
            },
            debug_highlighting: StringCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Use yyyy-mm-dd as the current date (for debugging of highlighting)."),
                },
                value: None,
            },
            weeks_start_monday: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Weeks start on Monday."),
                },
                value: false,
            },
            weeks_start_sunday: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Weeks start on Sunday."),
                },
                value: false,
            },
            first_week_has_at_least_days: StringCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("First week of the year has at least number days."),
                },
                value: None,
            },
            use_old_style_format: BooleanCommand {
                def: CommandDefinition {
                    ind: String::from(""),
                    desc: String::from("Use oldstyle format for ncal output."),
                },
                value: false,
            },

            unrecognized: vec![],
            errors: vec![],
        }
    }
    pub(crate) fn parse_args(&self, args: std::env::Args) -> Self {
        self.parse_string_vec(args.collect())
    }

    fn parse_string_vec(&self, args: Vec<String>) -> Self {
        let mut output = self.initialize();

        /* initial handling of arguments */
        if args.len() == 1 {
            let today = TodayFactory::Actual.create().make_today();
            output.month.value = Option::from(month_num_to_name(today.month));
            output.year.value = Option::from(today.year);
        } else {
            let mut prev_arg_month = false;
            let mut prev_arg_country_code = false;
            let mut prev_arg_year = false;
            let mut prev_arg_months_add_after = false;
            let mut prev_arg_months_add_before = false;
            let mut prev_arg_debug_current_date = false;
            let mut prev_arg_debug_highlighting = false;
            let mut prev_arg_first_week_has_at_least_days = false;

            for (index, argument) in args.into_iter().enumerate() {
                if index == 0 {
                    continue;
                }

                if prev_arg_month {
                    prev_arg_month = false;
                    output.month.value = Some(argument.to_owned());
                } else if prev_arg_country_code {
                    prev_arg_country_code = false;
                    output.country_code.value = Some(argument.to_owned());
                } else if prev_arg_year {
                    prev_arg_year = false;
                    output.year.value = Some(argument.parse::<u16>().unwrap());
                } else if prev_arg_months_add_after {
                    prev_arg_months_add_after = false;
                    output.after.value = Some(argument.parse::<usize>().unwrap());
                } else if prev_arg_months_add_before {
                    prev_arg_months_add_before = false;
                    output.before.value = Some(argument.parse::<usize>().unwrap());
                } else if prev_arg_debug_current_date {
                    prev_arg_debug_current_date = false;
                    output.debug_current_date.value = Some(argument.to_owned());
                } else if prev_arg_debug_highlighting {
                    prev_arg_debug_highlighting = false;
                    output.debug_highlighting.value = Some(argument.to_owned());
                } else if prev_arg_first_week_has_at_least_days {
                    prev_arg_first_week_has_at_least_days = false;
                    output.first_week_has_at_least_days.value = Some(argument.to_owned());
                } else {
                    match argument.as_str() {
                        "-h" => output.turn_off_highlight_today.value = true,
                        "-J" => output.display_julian_calendar.value = true,
                        "-e" => output.display_date_of_easter.value = true,
                        "-j" => output.display_julian_days.value = true,
                        "-o" => output.display_date_orthodox_easter.value = true,
                        "-p" => output.print_country_codes.value = true,
                        "-w" => output.print_number_of_week.value = true,
                        "-3" => output.previous_current_next_month.value = true,
                        "-1" => output.only_current_month.value = true,
                        "-C" => output.cal_mode.value = true,
                        "-M" => output.weeks_start_monday.value = true,
                        "-S" => output.weeks_start_sunday.value = true,
                        "-b" => output.use_old_style_format.value = true,

                        "-m" => prev_arg_month = true,
                        "-s" => prev_arg_country_code = true,
                        "-y" => prev_arg_year = true,
                        "-A" => prev_arg_months_add_after = true,
                        "-B" => prev_arg_months_add_before = true,
                        "-d" => prev_arg_debug_current_date = true,
                        "-H" => prev_arg_debug_highlighting = true,
                        "-W" => prev_arg_first_week_has_at_least_days = true,

                        _ => output
                            .unrecognized
                            .push(UnrecognizedArgument{index, argument: Some(argument.to_owned())}),
                    }
                }
            }
        }

        /* sort unrecognized, just in case */
        output.unrecognized.sort();

        /* print contents */
        //output.unrecognized.iter().for_each(|u| println!("{:?}", u));

        /* deal with unrecognized args */
        if output.unrecognized.len() == 1 {/* 1 arg: year */
            if let Some(pos) = output.unrecognized.iter().position(|u| u.index == 1) {
                let ua = output.unrecognized.remove(pos);
                let temp_year = ua.argument.expect("argument missing");

                if let Ok(year) = temp_year.parse::<u16>() {
                    output.year.value = Some(year);
                } else {
                    output
                        .errors
                        .push(KnownError { code: 1, message: Some(format!("rcal: not a valid year {}", temp_year)) })
                }
            }
        } else if output.unrecognized.len() == 2 {/* 2 args: month year */
            if let Some(pos) = output.unrecognized.iter().position(|u| u.index == 1) {
                let ua = output.unrecognized.remove(pos);
                output.month.value = ua.argument;
            }

            if let Some(pos) = output.unrecognized.iter().position(|u| u.index == 2) {
                let ua = output.unrecognized.remove(pos);
                let temp_year = ua.argument.expect("argument missing");

                if let Ok(year) = temp_year.parse::<u16>() {
                    output.year.value = Some(year);
                } else {
                    output
                        .errors
                        .push(KnownError { code: 1, message: Some(format!("rcal: not a valid year {}", temp_year)) })
                }
            }
        }

        return output;
    }
}
