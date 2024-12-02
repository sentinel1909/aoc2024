// parser/src/lib.rs

// dependencies
use nom::character::complete::alphanumeric0; 
use nom::IResult;

// parser to recognize a number
pub fn parse_numeric(input: &str) -> IResult<&str, &str> {
  alphanumeric0(input)
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
}