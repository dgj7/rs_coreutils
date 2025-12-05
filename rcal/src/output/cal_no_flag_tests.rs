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
