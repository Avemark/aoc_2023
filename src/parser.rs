use std::fs;
use bevy::color::Color;
use bevy::prelude::Resource;
use bevy::utils::default;

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct GameBoard {
    rows: Vec<Vec<Position>>,
    pub guard: Guard,
    pub height: usize,
    pub width: usize,
}

impl GameBoard {
    pub fn get(&self, x: usize, y: usize) -> Option<Position> {
        if let Some(row) = self.rows.get(y) {
            row.get(x).cloned()
        } else {
            None
        }
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        GameBoard {
            guard: Guard { position: (0,0), direction: GuardDirection::Up},
            height: 0,
            width: 0,
            rows: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardDirection {
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guard {
    pub direction: GuardDirection,
    pub position: (usize, usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub visited: bool,
    pub stuff: bool,
}

impl Position {
    pub fn color(&self) -> Color {
        match self {
            &Position { stuff: true, ..} => { Color::srgb(5.0, 0.0, 0.0) },
            &Position { visited: true, ..} => { Color::srgb(1.0, 4.0, 1.5) },
            &_ => Color::srgb(1.5, 1.0, 4.0),
        }
    }
}

pub fn parse_game_board(filename: &str) -> GameBoard {
    let input = fs::read_to_string(filename).expect("could not read file");
    let mut guard: Guard = Guard { direction : GuardDirection::Up, position: (0, 0) };
    let mut rows = Vec::new();
    for (l_index, line) in input.lines().enumerate() {
        let mut positions = Vec::new();

        for (r_index, char) in line.chars().enumerate() {
            match char {
                '#' => positions.push(Position { visited: false, stuff: true }),
                '.' => positions.push(Position { visited: false, stuff: false }),
                '^' => {
                    guard = Guard { direction: GuardDirection::Up, position: (r_index, l_index) };
                    positions.push(Position { visited: true, stuff: false});
                    println!("up")
                },

                '>' => {
                    guard = Guard { direction: GuardDirection::Right, position: (r_index, l_index) };
                    positions.push(Position { visited: true, stuff: false});
                    println!("Right")
                },

                '<' => {
                    guard = Guard { direction: GuardDirection::Left, position: (r_index, l_index) };
                    positions.push(Position { visited: true, stuff: false});
                    println!("Left")
                },

                'v' => {
                    guard = Guard { direction: GuardDirection::Down, position: (r_index, l_index) };
                    positions.push(Position { visited: true, stuff: false});
                    println!("Down")
                },

                _ => {
                    println!("invalid char: {}", char);
                },
            }
        }
        rows.push(positions);
    }
    let height = rows.len();
    let width = rows[0].len();

    GameBoard { rows, guard, height, width, ..default() }
}