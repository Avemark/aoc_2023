use std::collections::HashMap;
use crate::custom_error::AocError;
use crate::{parse_choices, parse_map};

pub fn process(input: &str) -> Result<String, AocError> {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    let choices = parse_choices(lines.remove(0));
    lines.remove(0);

    let map = parse_map(lines);

    let steps = find_destination(map, choices);

    Ok(steps.to_string())
}

fn find_destination(map: HashMap<String, (String, String)>, choices: Vec<bool>) -> usize {
    let mut steps = 0usize;
    let mut position = String::from("AAA");
    let mut plan = choices.iter().cycle();
    loop {
        steps += 1;
        position = match map.get(&*position) {
            None => { panic!("illegal choice made {} is not in the map", position) }
            Some(choice) => {
                let next_step: bool = plan.next().unwrap_or_else(||
                    plan.next().unwrap_or_else(|| panic!("Iter cycle not cycling"))
                ).clone();

                if next_step { &choice.0 } else { &choice.1 }
            }
        }.clone();
        if position == "ZZZ" {
            break;
        }
    }
    return steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        let input = include_str!("../example1.txt");
        assert_eq!("2", process(input)?);
        let input = include_str!("../example2.txt");
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
