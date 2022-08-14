use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    combinator::{eof, map, opt, value},
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
        commentable_spaces,
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
            value(Color::White, tag("W")),
            value(Color::Yellow, tag("Y")),
            value(Color::Red, tag("R")),
            value(Color::Orange, tag("O")),
            value(Color::Blue, tag("B")),
            value(Color::Green, tag("G")),
            value(Color::Mask, tag(".")),
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
