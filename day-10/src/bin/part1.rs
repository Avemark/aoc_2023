use day_10::part1::process;
use day_10::custom_error::AocError;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}