pub mod custom_error;

pub mod part1;
pub mod part2;

#[derive(Eq, PartialEq, Debug)]
enum SpringStatus {
    Whole,
    Broken,
    Unknown
}