// day1/src/lib.rs

// dependencies
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// function which solves the Day 1 challenge
pub fn day1_challenge(input: PathBuf) {
    println!("Day 1 Challenge");
    let mut buf = String::new();
    File::open(input).unwrap().read_to_string(&mut buf).unwrap();
    println!("{}", buf);

    println!("Answer:");
}
