use crate::{Game, Grab};
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1};
use nom::combinator::{eof, peek};
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated, Tuple};
use nom::IResult;

fn grab(input: &str) -> IResult<&str, Grab> {
    peek(alt((red,blue,green)))(input)?;
    let (input, (red, blue, green)) = terminated(
        permutation((
            alt((color_not_present, red)),
            alt((color_not_present, blue)),
            alt((color_not_present, green)),
        )),
        grab_end,
    )(input)?;

    Ok((input, Grab { red, green, blue }))
}

fn grab_end(input: &str) -> IResult<&str, &str> {
    let (input, _) = alt((tag(";"), eof))(input)?;

    Ok((input, ""))
}

pub fn parse_game(input: &str) -> Result<Game, String> {
    let (_, game)  = game(input).expect("Could not parse game line");

    Ok(game)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, (game_no, grabs)) = (
        delimited(tag("Game "), alphanumeric1, tag(":")),
        many1(grab),
    )
        .parse(input)?;

    Ok((
        input,
        Game {
            number: game_no.parse::<usize>().expect("Weird Game number could not be parsed"),
            grabs,
        },
    ))
}

fn grabs(input: &str) -> IResult<&str, Vec<Grab>> {
    many1(grab)(input)
}

fn color_not_present(input: &str) -> IResult<&str, usize> {
    peek(grab_end)(input)?;

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

fn color<'a>(input: &'a str, color: &str) -> IResult<&'a str, usize> {
    let (input, (n, _color)) = (
        preceded(tag(" "), digit1),
        delimited(tag(" "), tag(color), alt((tag(","), peek(grab_end)))),
    )
        .parse(input)?;

    Ok((input, n.parse::<usize>().expect("Could not parse number")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grab_end() -> Result<(), String> {
        assert_eq!(grab_end(""), Ok(("", "")));
        assert_eq!(grab_end(";"), Ok(("", "")));

        Ok(())
    }

    #[test]
    fn test_color_not_present() -> Result<(), String> {
        assert_eq!(color_not_present(";"), Ok((";", 0)));
        assert_eq!(color_not_present(""), Ok(("", 0)));

        Ok(())
    }

    #[test]
    fn test_grab() -> Result<(), String> {
        assert_eq!(grab(" 3 green, 3 red, 3 blue;"), Ok(("", Grab::all(3))));
        assert_eq!(grab(" 3 green, 3 blue, 3 red;"), Ok(("", Grab::all(3))));
        assert_eq!(
            grab(" 3 green;"),
            Ok((
                "",
                Grab {
                    green: 3,
                    red: 0,
                    blue: 0
                }
            ))
        );

        assert_eq!(grab(" 3 green, 3 blue, 3 red"), Ok(("", Grab::all(3))));
        assert_eq!(
            grab(" 3 green"),
            Ok((
                "",
                Grab {
                    green: 3,
                    red: 0,
                    blue: 0
                }
            ))
        );

        Ok(())
    }

    #[test]
    fn test_grabs() -> Result<(), String> {
        assert_eq!(
            grabs(" 3 green;"),
            Ok((
                "",
                vec![
                    Grab {
                        green: 3,
                        blue: 0,
                        red: 0
                    }
                ]
            ))
        );
        assert_eq!(
            grabs(" 3 green; 3 green"),
            Ok((
                "",
                vec![
                    Grab {
                        green: 3,
                        blue: 0,
                        red: 0
                    },
                    Grab {
                        green: 3,
                        blue: 0,
                        red: 0
                    }
                ]
            ))
        );

        Ok(())
    }

    #[test]
    fn test_game() -> Result<(), String> {
        assert_eq!(
            game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok((
                "",
                Game {
                    number: 1,
                    grabs: vec![
                        Grab {
                            green: 0,
                            blue: 3,
                            red: 4
                        },
                        Grab {
                            green: 2,
                            red: 1,
                            blue: 6
                        },
                        Grab {
                            green: 2,
                            red: 0,
                            blue: 0
                        }
                    ]
                }
            ))
        );

        Ok(())
    }
}
