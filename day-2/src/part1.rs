use crate::{Grab};
use crate::parser::parse_game;

pub fn process(input: &str) -> Result<usize, String> {
    let reference = Grab { green: 13, red: 12, blue: 14 };
    let result = input
        .lines()
        .map( |line| { parse_game(line).expect("Game parse failure") } )
        .filter( |game| { game.possible_within(&reference) })
        .map( |game| { game.number })
        .collect::<Vec<usize>>();

    Ok(result.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example1.txt");
        let answer = 8;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
