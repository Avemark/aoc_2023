use crate::coordinate::{Coordinate, decrement, increment};

pub mod coordinate;
pub mod custom_error;
pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy)]
pub struct PartNumber {
    top_left: Coordinate,
    bot_right: Coordinate,
    number: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
    symbol: char,
    position: Coordinate,
}

pub struct PartNumberBits<'a> {
    start: Coordinate,
    number: &'a str
}

impl PartNumber {
    pub fn new(start: Coordinate, number: &str, far_bot_right: Coordinate) -> Self {
        let top_left = Coordinate {
            x: decrement(start.x),
            y: decrement(start.y)
        };
        let bot_right = Coordinate {
            x: increment(start.x + number.len(), far_bot_right.x),
            y: increment(start.y, far_bot_right.y)
        };
        let number = number.parse::<usize>().expect("unparseable number string");
        Self {
            top_left,
            bot_right,
            number
        }
    }

    pub fn is_adjacent(&self, target: Coordinate) -> bool {
        self.is_level(target) && self.is_plumb(target)
    }

    fn is_level(&self, target: Coordinate) -> bool {
        target.x <= self.top_left.x && target.x >= self.bot_right.x
    }

    fn is_plumb(&self, target: Coordinate) -> bool {
        target.y >= self.top_left.y && target.y <= self.bot_right.y
    }
}
