use day_9::part2::process;
use day_9::custom_error::AocError;

fn main() -> Result<(), AocError> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
