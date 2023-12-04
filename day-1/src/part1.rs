pub fn process(input: &str) -> Result<usize, String> {
    input.split("\n").map(|row| evaluate_row(row)).sum()
}

fn evaluate_row(row: &str) -> Result<usize, String> {
    let digits = row
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<Vec<char>>();

    assert!(digits.len() >= 1, "Not enough numbers in {r}", r = row);

    let number: String = vec![digits.first(), digits.last()]
        .iter()
        .map(|n| n.expect("not enough numbers in the row"))
        .collect();
    Ok(number
        .parse::<usize>()
        .expect("Could not parse resulting integer"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_row() -> Result<(), String> {
        assert_eq!(evaluate_row("1abc2")?, 12);
        assert_eq!(evaluate_row("pqr3stu8vwx")?, 38);
        assert_eq!(evaluate_row("a1b2c3d4e5f")?, 15);
        assert_eq!(evaluate_row("treb7uchet")?, 77);

        Ok(())
    }

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example1.txt");
        let answer = 142;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
