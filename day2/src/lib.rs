// day2/src/lib/lib.rs

// dependencies
use parser::parse_report;

// function which solves the Day 2, Puzzle 1 challenge
pub fn day2_puzzle1_challenge(buffer: String) {
    
    println!("Day 2, Puzzle 1 Challenge");

    // parse the input buffer
    println!("Parsing raw reports...");
    for item in buffer.lines() {
        let (_, raw_report) = parse_report(item).unwrap();
        let mut numeric_report: Vec<u32> = Vec::new();
        for level in raw_report {
            let numeric_level = level.parse::<u32>().unwrap();
            numeric_report.push(numeric_level);
        }
        println!("{:?}", numeric_report);
    }
}
