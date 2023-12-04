use crate::parser::parse_card;
use crate::Card;

pub fn process(input: &str) -> Result<usize, String> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).expect("Failed to parse a line"))
        .collect::<Vec<Card>>();
    let mut copies = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let wins = card.winning_numbers();
        if wins > 0 {
            for n in 1..wins + 1 {
                assert!(index + n <= cards.len(), "Winnings exceed table");
                copies[index + n] += copies[index];
            }
        }
    }

    Ok(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example2.txt");
        let answer = 30;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
