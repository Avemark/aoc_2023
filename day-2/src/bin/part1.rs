use day_2::part1::process;

fn main() -> Result<(), String> {
    let file = include_str!("../../input.txt");
    let result = process(file).expect("Could not run process!");
    println!("result: {}", result);

    Ok(())
}
