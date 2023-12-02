mod parser;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grab {
    green: usize,
    red: usize,
    blue: usize,
}


impl Grab {
    fn can_fit_in(&self, other: &Self) -> bool {
        self.green <= other.green && self.red <= other.red && self.blue <= other.blue
    }

    fn all(num: usize) -> Self {
        Self { green: num, blue: num, red: num }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Game {
    grabs: Vec<Grab>
}

impl Game {
    fn possible_within(&self, reference: &Grab) -> bool {
        for grab in &self.grabs {
            if !grab.can_fit_in(reference) {
                return false
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_within() -> Result<(), String> {
        let game = Game {
            grabs: vec! [
                Grab::all(1),
                Grab::all(3)
            ]
        };

        assert!(game.possible_within(&Grab::all(5)));
        assert!(game.possible_within(&Grab::all(3)));
        assert!(!game.possible_within(&Grab::all(1)));

        Ok(())
    }

    #[test]
    fn test_can_fit_in() -> Result<(), String> {
        let twos = Grab {
            red: 2,
            blue: 2,
            green: 2,
        };

        assert!(Grab {
            red: 2,
            green: 2,
            blue: 2
        }
        .can_fit_in(&twos));
        assert!(!Grab {
            red: 3,
            green: 1,
            blue: 1
        }
        .can_fit_in(&twos));

        Ok(())
    }
}
