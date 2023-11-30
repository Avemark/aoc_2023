pub fn process(_input: &str) -> Result<String, String> {
    Ok("TODO".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example.txt");
        let answer = String::from("TODO");
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}