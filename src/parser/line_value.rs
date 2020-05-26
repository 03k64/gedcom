use super::{
    pointer::parse_pointer,
    primitive::{parse_anychar, parse_at, parse_delim, parse_hash, parse_nonat},
    util::{five_tuple_to_string, vec_to_string},
};
use nom::{
    branch::alt,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_optional_line_value(input: &str) -> IResult<&str, String> {
    preceded(parse_delim, parse_line_value)(input)
}

fn parse_escape(input: &str) -> IResult<&str, String> {
    tuple((
        parse_at,
        parse_hash,
        parse_escape_text,
        parse_at,
        parse_nonat,
    ))(input)
    .map(five_tuple_to_string)
}

fn parse_escape_text(input: &str) -> IResult<&str, String> {
    many1(parse_anychar)(input).map(vec_to_string)
}

fn parse_line_item(input: &str) -> IResult<&str, String> {
    many1(parse_line_item_inner)(input).map(vec_to_string)
}

fn parse_line_item_inner(input: &str) -> IResult<&str, String> {
    alt((parse_anychar, parse_escape))(input)
}

fn parse_line_value(input: &str) -> IResult<&str, String> {
    alt((parse_line_item, parse_pointer))(input)
}

#[cfg(test)]
mod tests {
    use super::{parse_escape, parse_line_item, parse_optional_line_value};
    use nom::{error::ErrorKind, Err};

    #[test]
    fn test_parse_escape_valid() {
        let input = "@#ESCAPE@1";
        let expected = Ok(("", String::from("@#ESCAPE@1")));
        let actual = parse_escape(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_escape_invalid_no_leading_at() {
        let input = "#ESCAPE@1";
        let expected = Err(Err::Error(("#ESCAPE@1", ErrorKind::OneOf)));
        let actual = parse_escape(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_escape_invalid_no_leading_hash() {
        let input = "@ESCAPE@1";
        let expected = Err(Err::Error(("ESCAPE@1", ErrorKind::OneOf)));
        let actual = parse_escape(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_escape_invalid_no_trailing_at() {
        let input = "@#ESCAPE1";
        let expected = Err(Err::Error(("", ErrorKind::OneOf)));
        let actual = parse_escape(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_escape_invalid_no_trailing_non_at() {
        let input = "@#ESCAPE@";
        let expected = Err(Err::Error(("", ErrorKind::OneOf)));
        let actual = parse_escape(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_single_escape() {
        let input = "@#ESCAPE@1";
        let expected = Ok(("", String::from("@#ESCAPE@1")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_multiple_escapes() {
        let input = "@#ESCAPE@1@#ESCAPE@2";
        let expected = Ok(("", String::from("@#ESCAPE@1@#ESCAPE@2")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_single_non_escape() {
        let input = "A";
        let expected = Ok(("", String::from("A")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_multiple_non_escapes() {
        let input = "NON_ESCAPES_WITH_DOUBLE_AT_@@";
        let expected = Ok(("", String::from("NON_ESCAPES_WITH_DOUBLE_AT_@@")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_single_non_escape_with_single_escape() {
        let input = "A@#ESCAPE@1";
        let expected = Ok(("", String::from("A@#ESCAPE@1")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_single_escape_with_single_non_escape() {
        let input = "@#ESCAPE@1A";
        let expected = Ok(("", String::from("@#ESCAPE@1A")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_line_item_valid_multiple_escapes_with_multiple_non_escapes() {
        let input = "FOO@#ESCAPE@1BAR@#ESCAPE@2";
        let expected = Ok(("", String::from("FOO@#ESCAPE@1BAR@#ESCAPE@2")));
        let actual = parse_line_item(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_optional_line_value_valid() {
        let input = " FOO";
        let expected = Ok(("", String::from("FOO")));
        let actual = parse_optional_line_value(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_optional_line_value_invalid_no_leading_space() {
        let input = "FOO";
        let expected = Err(Err::Error(("FOO", ErrorKind::OneOf)));
        let actual = parse_optional_line_value(input);
        assert_eq!(actual, expected);
    }
}
