// binary/src/main.rs

// dependencies
use clap::Parser;
use day1::{day1_puzzle1_challenge, day2_puzzle2_challenge};
use errors::AppError;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// struct type to represent the command line arguments
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: f32,               // values with decimals, i.e. 1.1, represent days and parts, i.e. 1.1 = day 1, part 1
    #[arg(short, long)]
    input_file: String,
}

// function to open and read the contents of the challenge input file
fn read_input_file(file_name: String) -> Result<String, AppError> {
    // create the file path using the incoming input file name
    let abs_path = format!("/Users/jeff/Development/aoc2024/input/{}", file_name);
    let path = PathBuf::from(abs_path);
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main() -> Result<(), AppError> {
    // output a general welcome message
    println!("Hello, Advent of Code 2024!");

    // get the command line arguments
    let args = Args::parse();

    // get the content of the input file
    let challenge_input = read_input_file(args.input_file)?;

    // select the appropriate solution function, depending on the challenge day, error out if the wrong day is entered
    match args.day {
        1.1 => day1_puzzle1_challenge(challenge_input),
        1.2 => day2_puzzle2_challenge(challenge_input),
        _ => eprintln!("Invalid challenge name, please try again..."),
    }

    Ok(())
}

// tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_input_file_returns_string_with_test_content() {
        let file_name = "test.txt".to_string();
        let result = read_input_file(file_name).unwrap();
        assert_eq!(result, "AOC 2024 test input file.".to_string());
    }
}
