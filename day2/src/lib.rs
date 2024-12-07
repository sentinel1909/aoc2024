// day2/src/lib/lib.rs

// dependencies
use parser::parse_report;

// function which checks if the levels in a report are all increasing
fn levels_all_increasing(input: &[u32]) -> bool {
    for i in 1..input.len() {
        if input[i] < input[i - 1] {
            return false;
        }
    }
    true
}

// function which checks if the levels in a report are all increasing
fn levels_all_decreasing(input: &[u32]) -> bool {
    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            return false;
        }
    }
    true
}

// function to check whether two adjacent levels differ by at least one and at most three
fn adjacent_levels_differ_by_at_least_1_or_at_most_3(input: &[u32]) -> bool {
    input
        .windows(2)
        .all(|e| ((e[1].abs_diff(e[0])) >= 1 && (e[1].abs_diff(e[0])) <= 3))
}

// function which solves the Day 2, Puzzle 1 challenge
pub fn day2_puzzle1_challenge(buffer: String) {
    println!("Day 2, Puzzle 1 Challenge");

    // parse the input buffer
    println!("Parsing raw reports...");
    let mut safe_count = 0;
    let mut unsafe_count = 0;
    for item in buffer.lines() {
        let (_, raw_report) = parse_report(item).unwrap();
        let mut numeric_report: Vec<u32> = Vec::new();
        for level in raw_report {
            let numeric_level = level.parse::<u32>().unwrap();
            numeric_report.push(numeric_level);
        }

        if (levels_all_decreasing(&numeric_report)
            && adjacent_levels_differ_by_at_least_1_or_at_most_3(&numeric_report))
            || (levels_all_increasing(&numeric_report)
                && adjacent_levels_differ_by_at_least_1_or_at_most_3(&numeric_report))
        {
            safe_count += 1;
        } else {
            unsafe_count += 1;
        }
    }
    println!("There are: {} safe reports.", safe_count);
    println!("There are: {} unsafe reports.", unsafe_count);
}

// tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_levels_all_increasing_returns_true() {
        let report: Vec<u32> = vec![1, 2, 3, 4, 5];
        let result = levels_all_increasing(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_levels_all_decreasing_returns_true() {
        let report: Vec<u32> = vec![5, 4, 3, 2, 1];
        let result = levels_all_decreasing(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_two_adjacent_levels_differ_by_at_least_one_or_at_most_three() {
        let report: Vec<u32> = vec![1, 2, 5, 8, 9];
        let result = adjacent_levels_differ_by_at_least_1_or_at_most_3(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_levels_increasing_and_levels_differ_by_at_least_1_or_at_most_3() {
        let report: Vec<u32> = vec![1, 2, 5, 8, 9];
        let result = levels_all_increasing(&report) && adjacent_levels_differ_by_at_least_1_or_at_most_3(&report);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn check_levels_decreasing_and_levels_differ_by_at_least_1_or_at_most_3() {
        let report: Vec<u32> = vec![9, 8, 5, 2, 1];
        let result = levels_all_decreasing(&report) && adjacent_levels_differ_by_at_least_1_or_at_most_3(&report);
        let expected = true;
        assert_eq!(result, expected);
    }
}
