use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::{IResult};
use nom::sequence::{Tuple, delimited, terminated};

pub mod custom_error;

pub mod part1;
pub mod part2;

pub fn parse_choices(input: &str) -> Vec<bool> {
    input.chars().map(|lr| lr == 'L').collect()
}

pub fn parse_map(input: Vec<&str>) -> HashMap<String, (String,String)> {
    let mut map = HashMap::new();
    for line in input {
        let (_rest, (name, (left, right))) = (
            terminated(alphanumeric1, tag(" = ")),
            delimited(
                tag("("),
                parse_choice,
                tag(")")
            )
        ).parse(line).unwrap_or_else(|_| panic!("failed to parse {}", line));
        map.insert(name.to_string(), (left.to_string(), right.to_string()));
    }
    map
}

fn parse_choice(input: &str) -> IResult<&str, (&str, &str)> {
    (terminated(alphanumeric1, tag(", ")), alphanumeric1).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_choices() {
        assert_eq!(parse_choices("LRLR"), vec![true, false, true, false]);
    }

    #[test]
    fn test_parse_map() {
        let input = vec!["AAA = (BBB, CCC)", "BBB = (AAA, CCC)"];
        let mut expectation = HashMap::new();
        expectation.insert(String::from("AAA"), (String::from("BBB"), String::from("CCC")));
        expectation.insert(String::from("BBB"), (String::from("AAA"), String::from("CCC")));

        assert_eq!(parse_map(input), expectation);
    }
}
