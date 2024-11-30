// binary/src/main.rs

// dependencies
use clap::Parser;
use day1::day1_challenge;
use std::path::PathBuf;

// struct type to represent the command line arguments
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    println!("Hello, Advent of Code 2024!");

    let args = Args::parse();
    let raw_path = format!("../../input/{}", args.input_file);
    let path = PathBuf::from(raw_path);

    match args.day {
        1 => day1_challenge(path),
        _ => eprintln!("Invalid challenge name, please try again..."),
    }
}
