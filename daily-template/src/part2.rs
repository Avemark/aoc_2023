use crate::custom_error::AocError;

pub fn process(
    _input: &str,
) -> Result<String, AocError> {
    todo!("part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        todo!("haven't built test yet");
        let input = include_str!("../example.txt");
        assert_eq!("", process(input)?);
        Ok(())
    }
}