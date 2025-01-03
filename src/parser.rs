use nom::branch::alt;
use nom::combinator::value;
use nom::IResult;

pub struct Gameboard {
    rows: Vec<Vec<Position>>,
}

struct Stuff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardDirection {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    direction: GuardDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Things {
    Stuff,
    Guard,
    Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    visited: bool,
    contents: Things,
}

fn parse_guard(input: &str) -> IResult<&str, Guard> {
    alt(
        value(Guard {direction: GuardDirection::Up}, tag("^"))
    )
}

fn parse_thing(input: &str) -> IResult<&str, Things> {
    alt((parse_guard,parse_empty, parse_stuff))
}

pub fn parse_game_board(filename: &str) -> IResult<&str, Gameboard> {
    
}