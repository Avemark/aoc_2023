use crate::parser::parse_card;
use crate::Card;

pub fn process(input: &str) -> Result<usize, String> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).expect("Failed to parse a line"))
        .collect::<Vec<Card>>();

    let score = cards.iter().map(|card| card.points()).sum::<usize>();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example1.txt");
        let answer = 13;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
