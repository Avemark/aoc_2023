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
    let mut positions = map.keys().filter(|key| key.ends_with('A')).collect::<Vec<&String>>();
    println!("simultaneous starts: {}", positions.len());

    let mut plan = choices.iter().cycle();
    loop {
        steps += 1;
        let next_step: bool = plan.next().unwrap_or_else(||
            plan.next().unwrap_or_else(|| panic!("Iter cycle not cycling"))
        ).clone();
        positions = positions.iter().map(
            |position| {
                match map.get(&position[..]) {
                    None => { panic!("illegal choice made {} is not in the map", position) }
                    Some(choice) => {
                        if next_step { &choice.0 } else { &choice.1 }
                    }
                }
            }).collect();
        // println!("positions: {:?}", positions );
        if positions.iter().all(|position| { position.ends_with('Z') }) {
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
        let input = include_str!("../example3.txt");
        assert_eq!("6", process(input)?);
        Ok(())
    }
}