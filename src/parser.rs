use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    combinator::{eof, map, opt},
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::entity::*;

pub fn parse(input: &str) -> Result<CubeViz, nom::Err<nom::error::Error<&str>>> {
    let (_, res) = map(parse_face, |f| CubeViz::Face(f))(input)?;
    Ok(res)
}

fn parse_face(input: &str) -> IResult<&str, Face> {
    let head = tuple((
        tag("Face"),
        commentable_spaces,
        tag("{"),
        commentable_spaces,
    ));
    let tail = tuple((tag("}"), commentable_spaces));
    let (rest, colors) = delimited(head, many1(parse_color), tail)(input)?;
    if colors.len() == 9 {
        let mut data = [[Color::Mask; 3]; 3];
        data[0].copy_from_slice(&colors[0..3]);
        data[1].copy_from_slice(&colors[3..6]);
        data[2].copy_from_slice(&colors[6..9]);
        return Ok((rest, Face::new(data, None)));
    }
    if colors.len() == 9 + 12 {
        let mut data = [[Color::Mask; 3]; 3];
        data[0].copy_from_slice(&colors[4..7]);
        data[1].copy_from_slice(&colors[9..12]);
        data[2].copy_from_slice(&colors[14..17]);
        let mut side = [[Color::Mask; 3]; 4];
        side[0].copy_from_slice(&colors[0..3]);
        side[1].copy_from_slice(&[colors[3], colors[8], colors[13]]);
        side[2].copy_from_slice(&[colors[7], colors[12], colors[17]]);
        side[3].copy_from_slice(&colors[18..21]);
        return Ok((rest, Face::new(data, Some(side))));
    }
    panic!("A Face must have 9 or 9+12 colors.")
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    terminated(
        alt((
            map(tag("W"), |_| Color::White),
            map(tag("Y"), |_| Color::Yellow),
            map(tag("R"), |_| Color::Red),
            map(tag("O"), |_| Color::Orange),
            map(tag("B"), |_| Color::Blue),
            map(tag("G"), |_| Color::Green),
            map(tag("."), |_| Color::Mask),
        )),
        commentable_spaces,
    )(input)
}

pub fn spaces(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("//")(input)?;
    let (input, _) = opt(is_not("\n\r"))(input)?;
    alt((eof, spaces))(input)
}

pub fn commentable_spaces(input: &str) -> IResult<&str, ()> {
    let (input, _) = spaces(input)?;
    let (input, _) = many0(tuple((comment, spaces)))(input)?;
    Ok((input, ()))
}
