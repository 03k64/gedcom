use super::{primitive::parse_digit, util::vec_to_string};
use nom::{
    error::{make_error, ErrorKind},
    multi::many_m_n,
    Err, IResult,
};

pub fn parse_level(input: &str) -> IResult<&str, u8> {
    match many_m_n(1, 2, parse_digit)(input).map(vec_to_string) {
        Err(e) => Err(e),
        Ok((input, output)) => match (output.len(), &output[0..1]) {
            (2, "0") => Err(Err::Failure(make_error(input, ErrorKind::ManyMN))),
            _ => match output.parse::<u8>() {
                Ok(level) => Ok((input, level)),
                Err(_) => Err(Err::Failure(make_error(input, ErrorKind::ManyMN))),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::parse_level;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn test_parse_level_single_digit_lo() {
        let input = "0";
        let expected = Ok(("", 0));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_single_digit_hi() {
        let input = "9";
        let expected = Ok(("", 9));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_double_digit_valid_lo() {
        let input = "10";
        let expected = Ok(("", 10));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_double_digit_valid_hi() {
        let input = "99";
        let expected = Ok(("", 99));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_double_digit_invalid_lo() {
        let input = "00";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_double_digit_invalid_hi() {
        let input = "09";
        let expected = Err(Err::Failure(("", ErrorKind::ManyMN)));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_triple_digit_lo() {
        let input = "100";
        let expected = Ok(("0", 10));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_level_triple_digit_hi() {
        let input = "999";
        let expected = Ok(("9", 99));
        let actual = parse_level(input);
        assert_eq!(actual, expected);
    }
}
