use crate::custom_error::AocError;

pub fn process(
    _input: &str,
) -> Result<String, AocError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<(), AocError> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
