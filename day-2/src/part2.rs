use crate::parser::parse_game;

pub fn process(input: &str) -> Result<usize, String> {
    let result = input
        .lines()
        .map(|line| parse_game(line).expect("Game parse failure"))
        .map(|game| game.power())
        .collect::<Vec<usize>>();

    Ok(result.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example2.txt");
        let answer = 2286;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
