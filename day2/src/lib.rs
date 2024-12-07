// day2/src/lib/lib.rs

// dependencies
use parser::parse_report;

// function which checks if the levels in a report are all increasing
fn levels_increasing(input: &[u32]) -> bool {
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            return false;
        }
    }
    true
}

// function which checks if the levels in a report are all increasing
fn levels_decreasing(input: &[u32]) -> bool {
    for i in 1..input.len() {
        if input[i] < input[i - 1] {
            return false;
        }
    }
    true
}

// function to check whether two adjacent levels differ by at least one and at most three
fn adjacent_levels_at_most_1_or_3(input: &[u32]) -> bool {
    input
        .windows(2)
        .all(|e| ((e[1].abs_diff(e[0])) >= 1 && (e[1].abs_diff(e[0])) <= 3))
}

// function which solves the Day 2, Puzzle 1 challenge
pub fn day2_puzzle1_challenge(buffer: String) {
    println!("Day 2, Puzzle 1 Challenge");

    // parse the input buffer
    println!("Parsing raw reports...");
    let mut count = 0;
    for item in buffer.lines() {
        let (_, raw_report) = parse_report(item).unwrap();
        let mut numeric_report: Vec<u32> = Vec::new();
        for level in raw_report {
            let numeric_level = level.parse::<u32>().unwrap();
            numeric_report.push(numeric_level);
        }

        if levels_decreasing(&numeric_report) && adjacent_levels_at_most_1_or_3(&numeric_report)
            || levels_increasing(&numeric_report) && adjacent_levels_at_most_1_or_3(&numeric_report)
        {
            count += 1;
        }
    }
    println!("There are: {} safe reports.", count);
}

// tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_levels_all_increasing() {
        let report: Vec<u32> = vec![1, 3, 2, 4, 5];
        let result = levels_increasing(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_levels_all_decreasing() {
        let report: Vec<u32> = vec![7, 6, 4, 2, 1];
        let result = levels_decreasing(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_two_adjacent_levels_differ_by_at_most_one_or_at_most_three() {
        let report: Vec<u32> = vec![1, 2, 7, 8, 9];
        let result = adjacent_levels_at_most_1_or_3(&report);
        let expected = false;
        assert_eq!(result, expected);
    }
}
