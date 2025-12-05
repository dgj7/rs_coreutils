use common::input::known_error::KnownError;
use crate::state::config::{Config};
use crate::output::formatter;
use crate::state::app_state::ApplicationState;
use crate::time::today::TodayFactory;

pub fn cal(args: Vec<String>, today_factory: TodayFactory) -> Result<Vec<String>, Vec<KnownError>> {
    let config = Config::new(&args);
    let today = today_factory.create();
    let state = ApplicationState::new(&config, today.as_ref());
    let lines = formatter::format_calendar(&config.errors, state)
        .iter()
        .skip(1)
        .map(|s| s.to_owned())
        .collect();

    if config.errors.is_empty() {
        Ok(lines)
    } else {
        Err(config.errors)
    }
}

#[cfg(test)]
mod no_flag_happy_path_tests {
    use crate::output::cal::cal;
    use crate::time::today::TodayFactory;

    #[test]
    fn no_args() {
        let args = vec!(String::from(""));
        let result = cal(args, TodayFactory::Other { y: 0, m: 0 });

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(true, lines.len() > 0);

        assert_eq!("                                                                 ", lines[0]);
        assert_eq!(" Su Mo Tu We Th Fr Sa                                            ", lines[2]);
    }

    #[test]
    fn test_no_inds_year_only() {
        let args = vec!(
            String::from(""),
            String::from("2012"),
        );

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(37, lines.len());

        assert_eq!("                              2012                               ", lines[0]);
        assert_eq!("       January              February                March        ", lines[1]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[2]);
        assert_eq!("  1  2  3  4  5  6  7            1  2  3  4               1  2  3", lines[3]);
        assert_eq!("  8  9 10 11 12 13 14   5  6  7  8  9 10 11   4  5  6  7  8  9 10", lines[4]);
        assert_eq!(" 15 16 17 18 19 20 21  12 13 14 15 16 17 18  11 12 13 14 15 16 17", lines[5]);
        assert_eq!(" 22 23 24 25 26 27 28  19 20 21 22 23 24 25  18 19 20 21 22 23 24", lines[6]);
        assert_eq!(" 29 30 31              26 27 28 29           25 26 27 28 29 30 31", lines[7]);
        assert_eq!("                                                                 ", lines[8]);
        assert_eq!("                              2012                               ", lines[9]);// todo: don't think this belongs here
        assert_eq!("        April                  May                  June         ", lines[10]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[11]);
        assert_eq!("  1  2  3  4  5  6  7         1  2  3  4  5                  1  2", lines[12]);
        assert_eq!("  8  9 10 11 12 13 14   6  7  8  9 10 11 12   3  4  5  6  7  8  9", lines[13]);
        assert_eq!(" 15 16 17 18 19 20 21  13 14 15 16 17 18 19  10 11 12 13 14 15 16", lines[14]);
        assert_eq!(" 22 23 24 25 26 27 28  20 21 22 23 24 25 26  17 18 19 20 21 22 23", lines[15]);
        assert_eq!(" 29 30                 27 28 29 30 31        24 25 26 27 28 29 30", lines[16]);
        assert_eq!("                                                                 ", lines[17]);
        assert_eq!("                              2012                               ", lines[18]);// todo: doesnt belong here
        assert_eq!("        July                 August               September      ", lines[19]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[20]);
        assert_eq!("  1  2  3  4  5  6  7            1  2  3  4                     1", lines[21]);
        assert_eq!("  8  9 10 11 12 13 14   5  6  7  8  9 10 11   2  3  4  5  6  7  8", lines[22]);
        assert_eq!(" 15 16 17 18 19 20 21  12 13 14 15 16 17 18   9 10 11 12 13 14 15", lines[23]);
        assert_eq!(" 22 23 24 25 26 27 28  19 20 21 22 23 24 25  16 17 18 19 20 21 22", lines[24]);
        assert_eq!(" 29 30 31              26 27 28 29 30 31     23 24 25 26 27 28 29", lines[25]);
        assert_eq!("                                             30                  ", lines[26]);
        assert_eq!("                                                                 ", lines[27]);
        assert_eq!("                              2012                               ", lines[28]);// todo: doesn't belong here
        assert_eq!("       October              November              December       ", lines[29]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[30]);
        assert_eq!("     1  2  3  4  5  6               1  2  3                     1", lines[31]);
        assert_eq!("  7  8  9 10 11 12 13   4  5  6  7  8  9 10   2  3  4  5  6  7  8", lines[32]);
        assert_eq!(" 14 15 16 17 18 19 20  11 12 13 14 15 16 17   9 10 11 12 13 14 15", lines[33]);
        assert_eq!(" 21 22 23 24 25 26 27  18 19 20 21 22 23 24  16 17 18 19 20 21 22", lines[34]);
        assert_eq!(" 28 29 30 31           25 26 27 28 29 30     23 24 25 26 27 28 29", lines[35]);
        assert_eq!("                                             30 31               ", lines[36]);
    }

    #[test]
    fn no_flags_month_and_year() {
        let args = vec!(
            String::from(""),
            String::from("mar"),
            String::from("1985"),
        );

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(9, lines.len());

        assert_eq!("                                                                 ", lines[0]);
        assert_eq!("        March                                                    ", lines[1]);
        assert_eq!(" Su Mo Tu We Th Fr Sa                                            ", lines[2]);
        assert_eq!("                 1  2                                            ", lines[3]);
        assert_eq!("  3  4  5  6  7  8  9                                            ", lines[4]);
        assert_eq!(" 10 11 12 13 14 15 16                                            ", lines[5]);
        assert_eq!(" 17 18 19 20 21 22 23                                            ", lines[6]);
        assert_eq!(" 24 25 26 27 28 29 30                                            ", lines[7]);
        assert_eq!(" 31                                                              ", lines[8]);
    }
}

#[cfg(test)]
mod flag_tests {
    use crate::output::cal::cal;
    use crate::time::today::TodayFactory;

    #[test]
    fn test_year_and_before() {
        let args = "exe 1986 -B 3".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(47, lines.len());

        assert_eq!("                              1985                               ", lines[0]);
        assert_eq!("       October              November              December       ", lines[1]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[2]);
        assert_eq!("        1  2  3  4  5                  1  2   1  2  3  4  5  6  7", lines[3]);
        assert_eq!("  6  7  8  9 10 11 12   3  4  5  6  7  8  9   8  9 10 11 12 13 14", lines[4]);
        assert_eq!(" 13 14 15 16 17 18 19  10 11 12 13 14 15 16  15 16 17 18 19 20 21", lines[5]);
        assert_eq!(" 20 21 22 23 24 25 26  17 18 19 20 21 22 23  22 23 24 25 26 27 28", lines[6]);
        assert_eq!(" 27 28 29 30 31        24 25 26 27 28 29 30  29 30 31            ", lines[7]);
        assert_eq!("                                                                 ", lines[8]);
        assert_eq!("                              1986                               ", lines[9]);
        assert_eq!("       January              February                March        ", lines[10]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[11]);
        assert_eq!("           1  2  3  4                     1                     1", lines[12]);
        assert_eq!("  5  6  7  8  9 10 11   2  3  4  5  6  7  8   2  3  4  5  6  7  8", lines[13]);
        assert_eq!(" 12 13 14 15 16 17 18   9 10 11 12 13 14 15   9 10 11 12 13 14 15", lines[14]);
        assert_eq!(" 19 20 21 22 23 24 25  16 17 18 19 20 21 22  16 17 18 19 20 21 22", lines[15]);
        assert_eq!(" 26 27 28 29 30 31     23 24 25 26 27 28     23 24 25 26 27 28 29", lines[16]);
        assert_eq!("                                             30 31               ", lines[17]);
        assert_eq!("                                                                 ", lines[18]);
        assert_eq!("                              1986                               ", lines[19]);// todo: doesn't belong here?
        assert_eq!("        April                  May                  June         ", lines[20]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[21]);
        assert_eq!("        1  2  3  4  5               1  2  3   1  2  3  4  5  6  7", lines[22]);
        assert_eq!("  6  7  8  9 10 11 12   4  5  6  7  8  9 10   8  9 10 11 12 13 14", lines[23]);
        assert_eq!(" 13 14 15 16 17 18 19  11 12 13 14 15 16 17  15 16 17 18 19 20 21", lines[24]);
        assert_eq!(" 20 21 22 23 24 25 26  18 19 20 21 22 23 24  22 23 24 25 26 27 28", lines[25]);
        assert_eq!(" 27 28 29 30           25 26 27 28 29 30 31  29 30               ", lines[26]);
        assert_eq!("                                                                 ", lines[27]);
        assert_eq!("                              1986                               ", lines[28]);// todo: doesn't belong here?
        assert_eq!("        July                 August               September      ", lines[29]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[30]);
        assert_eq!("        1  2  3  4  5                  1  2      1  2  3  4  5  6", lines[31]);
        assert_eq!("  6  7  8  9 10 11 12   3  4  5  6  7  8  9   7  8  9 10 11 12 13", lines[32]);
        assert_eq!(" 13 14 15 16 17 18 19  10 11 12 13 14 15 16  14 15 16 17 18 19 20", lines[33]);
        assert_eq!(" 20 21 22 23 24 25 26  17 18 19 20 21 22 23  21 22 23 24 25 26 27", lines[34]);
        assert_eq!(" 27 28 29 30 31        24 25 26 27 28 29 30  28 29 30            ", lines[35]);
        assert_eq!("                       31                                        ", lines[36]);
        assert_eq!("                                                                 ", lines[37]);
        assert_eq!("                              1986                               ", lines[38]);// todo: doesn't belong here?
        assert_eq!("       October              November              December       ", lines[39]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[40]);
        assert_eq!("           1  2  3  4                     1      1  2  3  4  5  6", lines[41]);
        assert_eq!("  5  6  7  8  9 10 11   2  3  4  5  6  7  8   7  8  9 10 11 12 13", lines[42]);
        assert_eq!(" 12 13 14 15 16 17 18   9 10 11 12 13 14 15  14 15 16 17 18 19 20", lines[43]);
        assert_eq!(" 19 20 21 22 23 24 25  16 17 18 19 20 21 22  21 22 23 24 25 26 27", lines[44]);
        assert_eq!(" 26 27 28 29 30 31     23 24 25 26 27 28 29  28 29 30 31         ", lines[45]);
        assert_eq!("                       30                                        ", lines[46]);
    }

    #[test]
    fn test_month_year_and_before() {
        let args = "exe feb 1987 -B 3".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(16, lines.len());

        assert_eq!("    November 1986         December 1986         January 1987     ", lines[0]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[1]);
        assert_eq!("                    1      1  2  3  4  5  6               1  2  3", lines[2]);
        assert_eq!("  2  3  4  5  6  7  8   7  8  9 10 11 12 13   4  5  6  7  8  9 10", lines[3]);
        assert_eq!("  9 10 11 12 13 14 15  14 15 16 17 18 19 20  11 12 13 14 15 16 17", lines[4]);
        assert_eq!(" 16 17 18 19 20 21 22  21 22 23 24 25 26 27  18 19 20 21 22 23 24", lines[5]);
        assert_eq!(" 23 24 25 26 27 28 29  28 29 30 31           25 26 27 28 29 30 31", lines[6]);
        assert_eq!(" 30                                                              ", lines[7]);
        assert_eq!("                                                                 ", lines[8]);
        assert_eq!("                                                                 ", lines[9]);// todo: year missing?
        assert_eq!("      February                                                   ", lines[10]);
        assert_eq!(" Su Mo Tu We Th Fr Sa                                            ", lines[11]);
        assert_eq!("  1  2  3  4  5  6  7                                            ", lines[12]);
        assert_eq!("  8  9 10 11 12 13 14                                            ", lines[13]);
        assert_eq!(" 15 16 17 18 19 20 21                                            ", lines[14]);
        assert_eq!(" 22 23 24 25 26 27 28                                            ", lines[15]);
    }

    #[test]
    fn test_year_month_and_after() {
        let args = "exe nov 1987 -A 4".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(17, lines.len());

        assert_eq!("    November 1987         December 1987         January 1988     ", lines[0]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[1]);
        assert_eq!("  1  2  3  4  5  6  7         1  2  3  4  5                  1  2", lines[2]);
        assert_eq!("  8  9 10 11 12 13 14   6  7  8  9 10 11 12   3  4  5  6  7  8  9", lines[3]);
        assert_eq!(" 15 16 17 18 19 20 21  13 14 15 16 17 18 19  10 11 12 13 14 15 16", lines[4]);
        assert_eq!(" 22 23 24 25 26 27 28  20 21 22 23 24 25 26  17 18 19 20 21 22 23", lines[5]);
        assert_eq!(" 29 30                 27 28 29 30 31        24 25 26 27 28 29 30", lines[6]);
        assert_eq!("                                             31                  ", lines[7]);
        assert_eq!("                                                                 ", lines[8]);
        assert_eq!("                              1988                               ", lines[9]);// todo: this line shouldn't be here
        assert_eq!("      February                March                              ", lines[10]);// todo: these should each have the year next to them
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa                      ", lines[11]);
        assert_eq!("     1  2  3  4  5  6         1  2  3  4  5                      ", lines[12]);
        assert_eq!("  7  8  9 10 11 12 13   6  7  8  9 10 11 12                      ", lines[13]);
        assert_eq!(" 14 15 16 17 18 19 20  13 14 15 16 17 18 19                      ", lines[14]);
        assert_eq!(" 21 22 23 24 25 26 27  20 21 22 23 24 25 26                      ", lines[15]);
        assert_eq!(" 28 29                 27 28 29 30 31                            ", lines[16]);
    }

    #[test]
    fn test_year_month_before_and_after() {
        let args = "exe jun 2002 -B 3 -A 4".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_ok());
        let lines = result.unwrap();

        assert_eq!(28, lines.len());

        assert_eq!("                              2002                               ", lines[0]);
        assert_eq!("        March                 April                  May         ", lines[1]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[2]);
        assert_eq!("                 1  2      1  2  3  4  5  6            1  2  3  4", lines[3]);
        assert_eq!("  3  4  5  6  7  8  9   7  8  9 10 11 12 13   5  6  7  8  9 10 11", lines[4]);
        assert_eq!(" 10 11 12 13 14 15 16  14 15 16 17 18 19 20  12 13 14 15 16 17 18", lines[5]);
        assert_eq!(" 17 18 19 20 21 22 23  21 22 23 24 25 26 27  19 20 21 22 23 24 25", lines[6]);
        assert_eq!(" 24 25 26 27 28 29 30  28 29 30              26 27 28 29 30 31   ", lines[7]);
        assert_eq!(" 31                                                              ", lines[8]);
        assert_eq!("                                                                 ", lines[9]);
        assert_eq!("                              2002                               ", lines[10]);
        assert_eq!("        June                  July                 August        ", lines[11]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa", lines[12]);
        assert_eq!("                    1      1  2  3  4  5  6               1  2  3", lines[13]);
        assert_eq!("  2  3  4  5  6  7  8   7  8  9 10 11 12 13   4  5  6  7  8  9 10", lines[14]);
        assert_eq!("  9 10 11 12 13 14 15  14 15 16 17 18 19 20  11 12 13 14 15 16 17", lines[15]);
        assert_eq!(" 16 17 18 19 20 21 22  21 22 23 24 25 26 27  18 19 20 21 22 23 24", lines[16]);
        assert_eq!(" 23 24 25 26 27 28 29  28 29 30 31           25 26 27 28 29 30 31", lines[17]);
        assert_eq!(" 30                                                              ", lines[18]);
        assert_eq!("                                                                 ", lines[19]);
        assert_eq!("                              2002                               ", lines[20]);
        assert_eq!("      September              October                             ", lines[21]);
        assert_eq!(" Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa                      ", lines[22]);
        assert_eq!("  1  2  3  4  5  6  7         1  2  3  4  5                      ", lines[23]);
        assert_eq!("  8  9 10 11 12 13 14   6  7  8  9 10 11 12                      ", lines[24]);
        assert_eq!(" 15 16 17 18 19 20 21  13 14 15 16 17 18 19                      ", lines[25]);
        assert_eq!(" 22 23 24 25 26 27 28  20 21 22 23 24 25 26                      ", lines[26]);
        assert_eq!(" 29 30                 27 28 29 30 31                            ", lines[27]);
    }
}

#[cfg(test)]
mod error_tests {
    use crate::output::cal::cal;
    use crate::time::today::TodayFactory;

    #[test]
    fn test_no_flags_month_only() {
        let args = "exe jan".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(false, result.is_ok());
        let lines = result.unwrap_err();

        assert_eq!(1, lines.len());
        assert_eq!("rcal: not a valid year jan", lines[0].message.clone().unwrap());
    }

    #[test]
    fn no_flags_year_and_month() {
        let args = "exe 1985 mar".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_err());
        let lines: Vec<_> = result.unwrap_err().iter().map(|x| x.message.clone().unwrap()).collect();

        assert_eq!(1, lines.len());

        assert_eq!("rcal: not a valid year mar", lines[0]);
    }

    #[test]
    fn test_year_month_and_before() {
        let args = "exe 1987 feb -B 3".split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

        let result = cal(args, TodayFactory::Actual);

        assert_eq!(true, result.is_err());
        let lines: Vec<_> = result.unwrap_err().iter().map(|x| x.message.clone().unwrap()).collect();

        assert_eq!(1, lines.len());

        assert_eq!("rcal: not a valid year feb", lines[0]);
    }
}
