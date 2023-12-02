use nom::branch::{alt, permutation};
use nom::combinator::{eof, peek};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use crate::Grab;
use nom::IResult;
use nom::sequence::{delimited, preceded, terminated, Tuple};

pub fn grab(input: &str) -> IResult<&str, Grab> {
    let (input, (red, blue, green)) =
        terminated(
            permutation((
                alt((color_not_present, red)),
                alt((color_not_present, blue)),
                alt((color_not_present, green))
                )
            ),
            tag(";")
        )(input)?;

    Ok((input, Grab { red, green, blue }))
}

fn color_not_present(input: &str) -> IResult<&str, usize> {
    let _ = eof(input)?;

    Ok((input, 0))
}

fn red(input: &str) -> IResult<&str, usize> {
    color(input, "red")
}

fn blue(input: &str) -> IResult<&str, usize> {
    color(input, "blue")
}

fn green(input: &str) -> IResult<&str, usize> {
    color(input, "green")
}


fn color<'a>(input: &'a str, color: &str)-> IResult<&'a str, usize> {
    let (input, (n, _color)) = (
        preceded(tag(" "), digit1),
        delimited(
            tag(" "),
            tag(color),
            alt(
                (
                    tag(","),
                    peek(tag(";"))
                )
            )
        )
    ).parse(input)?;

    Ok((input, n.parse::<usize>().expect("Could not parse number")))

}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_color_not_present()

    #[test]
    fn test_grab() -> Result<(), String> {

        assert_eq!(grab(" 3 green, 3 red, 3 blue;"), Ok(("", Grab::all(3))));
        assert_eq!(grab(" 3 green, 3 blue, 3 red;"), Ok(("", Grab::all(3))));

        assert_eq!(grab(" 3 green;"), Ok(("", Grab { green: 3, red: 0, blue: 0})));
        Ok(())
    }
}
