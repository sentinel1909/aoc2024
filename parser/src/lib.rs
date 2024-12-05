// parser/src/lib.rs

// dependencies
use nom::character::complete::{alphanumeric0, digit1, space1};
use nom::multi::separated_list1;
use nom::IResult;

// parser to recognize a number
pub fn parse_numeric(input: &str) -> IResult<&str, &str> {
    alphanumeric0(input)
}

// parser for the Day 2, Puzzle 1 challenge
pub fn parse_report(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, digit1)(input)
}

// tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_numeric_returns_a_number() {
        let input = "8675309   9035768";
        let result = parse_numeric(input);
        let expected = Ok(("   9035768", "8675309"));
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_report_returns_a_vector_of_levels() {
        let input = "10 20 30 40 50 60";
        let result = parse_report(input);
        let expected = Ok(("", vec!["10", "20", "30", "40", "50", "60"]));
        assert_eq!(result, expected);
    }
}
