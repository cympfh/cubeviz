use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, take_while, take_while1},
    combinator::{eof, map, opt, value},
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::entity::*;

pub fn parse(input: &str) -> Result<CubeViz, nom::Err<nom::error::Error<&str>>> {
    let (_, res) = alt((
        map(parse_face, |f| CubeViz::Face(f)),
        map(parse_cube, |c| CubeViz::Cube(c)),
    ))(input)?;
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

    let parse_attrs = map(many0(parse_attr), |v| v.into_iter().collect());
    let parse_colors = many1(parse_color);

    let (rest, (attrs, colors)) = delimited(head, tuple((parse_attrs, parse_colors)), tail)(input)?;
    if colors.len() == 9 {
        let mut data = [[Color::Mask; 3]; 3];
        data[0].copy_from_slice(&colors[0..3]);
        data[1].copy_from_slice(&colors[3..6]);
        data[2].copy_from_slice(&colors[6..9]);
        return Ok((rest, Face::new(data, None, attrs)));
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
        return Ok((rest, Face::new(data, Some(side), attrs)));
    }
    panic!("A Face must have 9 or 9+12 colors.")
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let head = tuple((
        commentable_spaces,
        tag("Cube"),
        commentable_spaces,
        tag("{"),
        commentable_spaces,
    ));
    let tail = tuple((tag("}"), commentable_spaces));

    let parse_attrs = map(many0(parse_attr), |v| v.into_iter().collect());
    let parse_colors = many1(parse_color);

    let (rest, (attrs, colors)) = delimited(head, tuple((parse_attrs, parse_colors)), tail)(input)?;
    if colors.len() != 9 * 6 {
        panic!("A Face must have 54 (=9*6) colors.")
    }
    let c = Cube::from(
        vec![
            colors[..3].to_vec(),
            colors[3..6].to_vec(),
            colors[6..9].to_vec(),
            colors[9..21].to_vec(),
            colors[21..33].to_vec(),
            colors[33..45].to_vec(),
            colors[45..48].to_vec(),
            colors[48..51].to_vec(),
            colors[51..54].to_vec(),
        ],
        attrs,
    );
    Ok((rest, c))
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

fn parse_attr(input: &str) -> IResult<&str, (String, AttributeValue)> {
    let str_value = alt((
        value(AttributeValue::Str(String::new()), tag("\"\"")),
        map(
            delimited(
                tag("\""),
                escaped_transform(
                    is_not("\"\\"),
                    '\\',
                    alt((
                        value("\\", tag("\\")),
                        value("\"", tag("\"")),
                        value("\'", tag("\'")),
                        value("\n", tag("n")),
                        value("\r", tag("r")),
                        value("\t", tag("t")),
                    )),
                ),
                tag("\""),
            ),
            AttributeValue::Str,
        ),
    ));
    map(
        tuple((
            terminated(identifier, commentable_spaces),
            terminated(tag("="), commentable_spaces),
            terminated(str_value, commentable_spaces),
            terminated(opt(alt((tag(","), tag(";")))), commentable_spaces),
        )),
        |(name, _, value, _)| (name, value),
    )(input)
}

pub fn identifier(input: &str) -> IResult<&str, String> {
    fn head(c: char) -> bool {
        c.is_alphabetic() || c == '_' || c == '#' || c == '@'
    }
    fn tail(c: char) -> bool {
        c.is_alphanumeric() || head(c)
    }
    let (input, s) = take_while1(head)(input)?;
    let (input, t) = take_while(tail)(input)?;
    let mut name = String::new();
    name.push_str(s);
    name.push_str(t);
    Ok((input, name))
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
