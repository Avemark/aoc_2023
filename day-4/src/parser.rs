use crate::Card;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, Tuple};
use nom::IResult;

fn card(input: &str) -> IResult<&str, Card> {
    let (input, (winning, actual)) = preceded(card_name, number_sets)(input)?;

    Ok((input, Card { winning, actual }))
}

pub fn parse_card(input: &str) -> Result<Card, String> {
    let (_, card) = card(input).expect("Invalid card string");

    Ok(card)
}

fn number_sets(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    (number_set, preceded(tag(" |"), number_set)).parse(input)
}

fn number_set(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, numbers) = many1(number)(input)?;

    Ok((
        input,
        numbers
            .iter()
            .map(|n| n.parse::<usize>().expect("could not parse"))
            .collect(),
    ))
}

fn number(input: &str) -> IResult<&str, &str> {
    preceded(alt((tag("   "), tag("  "), tag(" "))), alphanumeric1)(input)
}

fn card_name(input: &str) -> IResult<&str, &str> {
    delimited(tag("Card"), number, tag(":"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        assert_eq!(
            card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Ok((
                "",
                Card {
                    winning: vec![41, 48, 83, 86, 17],
                    actual: vec![83, 86, 6, 31, 17, 9, 48, 53]
                }
            ))
        );
    }
}
