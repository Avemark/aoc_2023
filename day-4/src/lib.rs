mod parser;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    winning: Vec<usize>,
    actual: Vec<usize>,
}

impl Card {
    pub fn points(&self) -> usize {
        let winners = self.winning_numbers();

        if 0usize == winners {
            return 0usize;
        }

        2usize.pow(winners as u32 - 1)
    }

    pub fn winning_numbers(&self) -> usize {
        self.actual
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points() {
        let card = Card {
            winning: vec![1, 2, 3],
            actual: vec![4, 5, 6],
        };
        assert_eq!(card.points(), 0);
        let card = Card {
            winning: vec![1, 2, 3],
            actual: vec![3, 4, 5, 6],
        };
        assert_eq!(card.points(), 1);
        let card = Card {
            winning: vec![1, 2, 3],
            actual: vec![1, 2, 3, 4, 5, 6],
        };
        assert_eq!(card.points(), 4);
    }
}
