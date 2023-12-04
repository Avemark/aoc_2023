use day_3::custom_error::AocError;
use day_3::part1::process;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
