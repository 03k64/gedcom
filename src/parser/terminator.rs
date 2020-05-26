use super::{
    primitive::{parse_carriage_return, parse_line_feed},
    util::tuple_to_string,
};
use nom::{branch::alt, sequence::pair, IResult};

pub fn parse_terminator(input: &str) -> IResult<&str, String> {
    alt((
        parse_crlf,
        parse_lfcr,
        parse_carriage_return,
        parse_line_feed,
    ))(input)
}

fn parse_crlf(input: &str) -> IResult<&str, String> {
    pair(parse_carriage_return, parse_line_feed)(input).map(tuple_to_string)
}

fn parse_lfcr(input: &str) -> IResult<&str, String> {
    pair(parse_line_feed, parse_carriage_return)(input).map(tuple_to_string)
}
