use day_12::part1::process;
use day_12::custom_error::AocError;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}