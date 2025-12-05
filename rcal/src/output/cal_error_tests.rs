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
