pub fn process(input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example2.txt");
        let answer = 281;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
