use super::{
    primitive::{parse_alphanum, parse_at, parse_nonat},
    util::{four_tuple_to_string, vec_to_string},
};
use nom::{multi::many0, sequence::tuple, IResult};

pub fn parse_pointer(input: &str) -> IResult<&str, String> {
    tuple((parse_at, parse_alphanum, parse_pointer_string, parse_at))(input)
        .map(four_tuple_to_string)
}

fn parse_pointer_char(input: &str) -> IResult<&str, String> {
    parse_nonat(input)
}

fn parse_pointer_string(input: &str) -> IResult<&str, String> {
    many0(parse_pointer_char)(input).map(vec_to_string)
}

#[cfg(test)]
mod tests {
    use super::parse_pointer;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn test_parse_pointer_valid_alphanum_no_pointer_string() {
        let input = "@A@";
        let expected = Ok(("", String::from("@A@")));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_valid_alphanum_valid_pointer_string_short() {
        let input = "@APS1@";
        let expected = Ok(("", String::from("@APS1@")));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_valid_alphanum_valid_pointer_string_long() {
        let input = "@A_POINTER_STRING_1@";
        let expected = Ok(("", String::from("@A_POINTER_STRING_1@")));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_no_leading_at() {
        let input = "APS1@";
        let expected = Err(Err::Error(("APS1@", ErrorKind::OneOf)));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_no_trailing_at() {
        let input = "@APS1";
        let expected = Err(Err::Error(("", ErrorKind::OneOf)));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_invalid_alphanum_no_pointer_string() {
        let input = "@#@";
        let expected = Err(Err::Error(("#@", ErrorKind::OneOf)));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_pointer_no_alphanum_no_pointer_string() {
        let input = "@@";
        let expected = Err(Err::Error(("@", ErrorKind::OneOf)));
        let actual = parse_pointer(input);
        assert_eq!(actual, expected);
    }
}
