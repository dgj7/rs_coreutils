use std::env::Args;

///
/// Storage for the application configuration.
///
pub(crate) struct Config {
    print_help_and_exit: bool,
    print_version_and_exit: bool,
    unrecognized: Vec<String>,

    file_paths: Vec<String>,

    change_only_access_time: bool,
    change_only_modification_time: bool,

    do_not_create: bool,
    affect_symlink_instead_of_file: bool,

    date_string: Option<String>,
    reference_file: Option<String>,
    time_word: Option<String>,

    use_specified_time_stamp: bool,
    specified_time_stamp: Option<String>,
}

impl Default for Config {
    ///
    /// Generate default configuration.
    ///
    fn default() -> Self {
        Config {
            print_help_and_exit: false,
            print_version_and_exit: false,
            unrecognized: vec!(),

            file_paths: vec!(),

            change_only_access_time: false,
            change_only_modification_time: false,

            affect_symlink_instead_of_file: false,
            do_not_create: false,

            date_string: None,
            reference_file: None,
            time_word: None,

            use_specified_time_stamp: false,
            specified_time_stamp: None,
        }
    }
}

impl Config {
    ///
    /// Initialize the application configuration based on provided application arguments.
    ///
    pub(crate) fn new(args: Args) -> Config {
        /* initialize with the default configuration */
        let mut config = Self::default();
        let mut prior_was_time_stamp = false;
        let mut file_paths_started = false;

        /* modify the default configuration based on input arguments */
        for argument in args.skip(1) {
            if file_paths_started {
                config.file_paths.push(argument);
            } else if prior_was_time_stamp {
                config.specified_time_stamp = Some(argument);
                prior_was_time_stamp = false;
            } else {
                match argument.as_str() {
                    /* print help and exit */
                    "--help" => config.print_help_and_exit = true,

                    /* print version and exit */
                    "--version" => config.print_version_and_exit = true,

                    /* change only access time */
                    "-a" => config.change_only_access_time = true,

                    /* change only modification time */
                    "-m" => config.change_only_modification_time = true,

                    /* don't create any files */
                    "-c" => config.do_not_create = true,
                    "--no-create" => config.do_not_create = true,

                    /* affect symlink instead of a file */
                    "-h" => config.affect_symlink_instead_of_file = true,
                    "--no-dereference" => config.affect_symlink_instead_of_file = true,

                    /* use a specified time stamp */
                    "-t" => {
                        config.use_specified_time_stamp = true;
                        prior_was_time_stamp = true;
                    },

                    /* special file ind */
                    "-" => {
                        config.file_paths.push(argument);
                        file_paths_started = true;
                    },

                    /* args that start with a value and use = */
                    _ => {
                        if argument.starts_with("-d=") || argument.starts_with("--date=") {
                            let actual = argument.split("=").collect::<Vec<&str>>()[1];
                            config.date_string = Some(actual.to_string());
                        } else if argument.starts_with("-r=") || argument.starts_with("--reference=") {
                            let actual = argument.split("=").collect::<Vec<&str>>()[1];
                            config.reference_file = Some(actual.to_string());
                        } else if argument.starts_with("--time=") {
                            let actual = argument.split("=").collect::<Vec<&str>>()[1];
                            config.time_word = Some(actual.to_string());
                        } else if argument.starts_with("-") && argument.len() > 1 {
                            config.unrecognized.push(argument);
                        } else {
                            config.file_paths.push(argument);
                            file_paths_started = true;
                        }
                    }
                }
            }
        }

        config
    }
}
