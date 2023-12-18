use clap::{Command, Arg, ArgAction};
use std::error::Error;


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .author("MaxAlex")
        .version("1.0.1")
        .about("Rust version of head")
        .arg(Arg::new("files")
            .value_name("FILES")
            .help("Input files to program")
            .default_value("-")
            .num_args(1..))
        .arg(Arg::new("lines")
            .short('n')
            .long("lines")
            .value_names("LINES")
            .help("number_of_lines")
            .value_parser(clap::value_parser!(u64).range(1..))
            .default_value("10")
        )
        .arg(Arg::new("bytes")
            .short('c')
            .long("bytes")
            .value_names("BYTES")
            .conflicts_with("lines")
            .value_parser(clap::value_parser!(u64).range(1..))
            .help("Number of bytes"))
        .get_matches();

    Ok(Config {
        files: matches.get_many("files")
            .expect("need files to provide")
            .cloned()
            .collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned().unwrap(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}