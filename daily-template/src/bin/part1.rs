use {{crate_name}}::part1::process;
use {{crate_name}}::custom_error::AocError;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}