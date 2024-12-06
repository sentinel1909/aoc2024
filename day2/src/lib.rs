// day2/src/lib/lib.rs

// dependencies
use parser::parse_report;

// function which checks if the levels in a report are all increasing
fn levels_increasing(input: Vec<u32>) -> bool {
    
    for i in 1..input.len() {
        if input[i] <= input[i - 1] {
            return false
        }
    }
    true 
}

// function which checks if the levels in a report are all increasing
fn levels_decreasing(input: Vec<u32>) -> bool {
    
    for i in 1..input.len() {
        if input[i] >= input[i - 1] {
            return false
        }
    }
    true 
}

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

// tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_levels_increasing() {
        let report: Vec<u32> = vec![1, 3, 6, 7, 9];
        let result = levels_increasing(report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_levels_decreasing() {
        let report: Vec<u32> = vec![7, 6, 4, 2, 1];
        let result = levels_decreasing(report);
        let expected = true;
        assert_eq!(result, expected);
    }
}