use crate::coordinate::Coordinate;

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
