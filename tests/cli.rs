use headr;
use assert_cmd::{Command};
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::io::prelude::*;
use std::{
    error::Error,
    fs::{self, File},
};
use std::num::IntErrorKind::Empty;
use clap::builder::Str;
use clap::parser::ValueSource::CommandLine;

type TestResult = Result<(), Box<dyn Error>>;


const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TEN: &str = "./tests/inputs/ten.txt";

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn gen_bad_file() -> String {
    loop {
        let filename = random_string();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

// fn dies_bad_files() {
//     let bad_file = gen_bad_file();
//
//     let cmd = Command::cargo_bin(PRG)?
//         .arg(bad_file);
// }





#[test]
fn dies_bad_lines() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("error: invalid value '{bad}' for '--lines <LINES>': invalid digit found in string");
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("error: invalid value '{bad}' for '--bytes <BYTES>': invalid digit found in string");
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));

    Ok(())
}

#[test]
fn dies_both_bytes_and_lines() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("error: the argument '--bytes <BYTES>' cannot be used with '--lines <LINES>'");
    let mut cmd = Command::cargo_bin(PRG)?;
    cmd.args(["-c", "1", "-n", "2", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));
    //print!("{:#?}", cmd);
    Ok(())
}