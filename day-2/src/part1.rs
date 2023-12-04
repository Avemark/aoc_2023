pub fn process(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example1.txt");
        let answer = 142;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}
