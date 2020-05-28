use super::{pointer::parse_pointer, primitive::parse_delim};
use nom::{sequence::terminated, IResult};

pub fn parse_optional_xref_id(input: &str) -> IResult<&str, String> {
    terminated(parse_xref_id, parse_delim)(input)
}

fn parse_xref_id(input: &str) -> IResult<&str, String> {
    parse_pointer(input)
}

#[cfg(test)]
mod tests {
    use super::parse_optional_xref_id;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn test_parse_optional_xref_id_valid() {
        let input = "@APS1@ ";
        let expected = Ok(("", String::from("@APS1@")));
        let actual = parse_optional_xref_id(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_optional_xref_id_invalid_no_trailing_space() {
        let input = "@APS1@";
        let expected = Err(Err::Error(("", ErrorKind::Char)));
        let actual = parse_optional_xref_id(input);
        assert_eq!(actual, expected);
    }
}
