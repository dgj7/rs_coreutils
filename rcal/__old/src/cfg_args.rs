use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="dgj7", version, about="rewrite of `cal` in Rust")]
pub struct CalArguments {
    pub year: Option<i16>,
    pub month: Option<String>,

    #[clap(short='B')]
    pub before: Option<usize>,
    #[clap(short='A')]
    pub after: Option<usize>,
}
