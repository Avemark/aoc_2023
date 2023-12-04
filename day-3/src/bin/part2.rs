use day_3::custom_error::AocError;
use day_3::part2::process;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
