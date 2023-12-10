use day_8::part2::process;
use day_8::custom_error::AocError;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
