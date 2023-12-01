use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::IResult;
use nom::multi::{many1, many_till};

pub fn process(input: &str) -> Result<usize, String> {
    input.split("\n").map(|row| evaluate_row(row) ).sum()
}

fn evaluate_row(row: &str) -> Result<usize, String> {
    let (_tail,digits) = digits_in_row(row).expect("Could not parse any digits in row");

    assert!(digits.len() >= 1, "Not enough numbers in {r}", r=row);

    let number: String = vec![digits.first(), digits.last()]
        .iter()
        .map(|n| n.expect("failed to get element from vector").to_string())
        .collect();
    assert_eq!(number.len(), 2);
    Ok(number.parse::<usize>().expect("Could not parse resulting integer"))
}


fn one(input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("one")(input)?;

    Ok((&input[1..], 1))
}

fn two(input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("two")(input)?;

    Ok((&input[1..], 2))
}

fn three(input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("three")(input)?;

    Ok((&input[1..], 3))
}

fn four (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("four")(input)?;

    Ok((&input[1..], 4))
}

fn five (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("five")(input)?;

    Ok((&input[1..], 5))
}
fn six (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("six")(input)?;

    Ok((&input[1..], 6))
}
fn seven (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("seven")(input)?;

    Ok((&input[1..], 7))
}
fn eight (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("eight")(input)?;

    Ok((&input[1..], 8))
}

fn nine (input: &str) -> IResult<&str, usize> {
    let (_, _one) = tag("nine")(input)?;

    Ok((&input[1..], 9))
}

fn zero (input: &str) -> IResult<&str, usize> {
    let (_, _) = tag("zero")(input)?;

    Ok((&input[1..], 0))
}

fn any_written_digit(input: &str) -> IResult<&str, usize> {
    let (input, digit) = alt(
        (one, two, three, four, five, six, seven, eight, nine, zero)
    )(input)?;

    Ok((input, digit))
}

fn single_digit(input: &str) -> IResult<&str, usize> {
    let (input, first) = take(1usize)(input)?;
    let (_, digit) = digit1(first)?;
    Ok((input, digit.parse::<usize>().expect("couldn't parse a digit")))
}

fn any_input_digit(input: &str) -> IResult<&str, usize> {
    alt((single_digit, any_written_digit))(input)
}


fn digit_with_garbage(input: &str) -> IResult<&str, usize> {
    let (input, (_, digit)) = many_till(take(1usize), any_input_digit)(input)?;
    Ok((input, digit))
}

fn digits_in_row(input: &str) -> IResult<&str, Vec<usize>> {
    many1(digit_with_garbage)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_in_row() -> Result<(), String> {
        assert_eq!(digits_in_row("1abc2"), Ok(("", vec![1,2])));
        assert_eq!(digits_in_row("pqr3stu8vwx"), Ok(("vwx", vec![3,8])));
        assert_eq!(digits_in_row("a1b2c3d4e5f"), Ok(("f", vec![1,2,3,4,5])));
        assert_eq!(digits_in_row("treb7uchet"), Ok(("uchet", vec![7])));
        assert_eq!(digits_in_row("oneight"), Ok(("ight", vec![1,8])));


        Ok(())
    }

    #[test]
    fn test_digit_with_garbage() {
        assert_eq!(digit_with_garbage("8"), Ok(("", 8)));
        assert_eq!(digit_with_garbage("foo8"), Ok(("", 8)));
        assert_eq!(digit_with_garbage("foo8bar"), Ok(("bar", 8)));
        assert_eq!(digit_with_garbage("fooeightbar"), Ok(("ightbar", 8)));
    }

    #[test]
    fn test_any_input_digit() {
        assert_eq!(any_input_digit("7"), Ok(("", 7)));
        assert_eq!(any_input_digit("seven"), Ok(("even", 7)));
        assert_eq!(any_input_digit("seven7"), Ok(("even7", 7)));
        assert_eq!(any_input_digit("7seven"), Ok(("seven", 7)));
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(single_digit("6c"), Ok(("c", 6)));
    }

    #[test]
    fn test_one() {
        assert!(one("foo").is_err());
        assert_eq!(one("one"), Ok(("ne", 1)));
        assert_eq!(one("onefoo"), Ok(("nefoo", 1)));
    }

    #[test]
    fn test_two() {
        assert!(two("one").is_err());
        assert_eq!(two("two"), Ok(("wo", 2)));
        assert_eq!(two("twofoo"), Ok(("wofoo", 2)));
    }

    #[test]
    fn test_any_written_digit() {
        assert!(any_written_digit("foo").is_err());
        assert_eq!(any_written_digit("one"), Ok(("ne", 1)));
        assert_eq!(any_written_digit("two"), Ok(("wo", 2)));
    }

    #[test]
    fn test_evaluate_row() -> Result<(), String> {
        assert_eq!(evaluate_row("two1nine")?, 29);
        assert_eq!(evaluate_row("eightwothree")?, 83);
        assert_eq!(evaluate_row("abcone2threexyz")?, 13);
        assert_eq!(evaluate_row("xtwone3four")?, 24);
        assert_eq!(evaluate_row("4nineeightseven2")?, 42);
        assert_eq!(evaluate_row("zoneight234")?, 14);
        assert_eq!(evaluate_row("7pqrstsixteen")?, 76);
        assert_eq!(evaluate_row("fouronevhnrz44")?, 44);
        assert_eq!(evaluate_row("oneight")?, 18);

        Ok(())
    }

    #[test]
    fn test_process() -> Result<(), String> {
        let input = include_str!("../example2.txt");
        let answer = 281;
        assert_eq!(process(input)?, answer);

        Ok(())
    }
}