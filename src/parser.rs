mod level;
mod line_value;
mod pointer;
mod primitive;
mod tag;
mod terminator;
mod util;
mod xref_id;

use self::level::parse_level;
use self::line_value::parse_optional_line_value;
use self::primitive::parse_delim;
use self::tag::parse_tag;
use self::terminator::parse_terminator;
use self::util::five_tuple_to_gedcom_line;
use self::xref_id::parse_optional_xref_id;
use crate::models::gedcom::GedcomLine;
use nom::{
    character::complete::char,
    combinator::opt,
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_gedcom(input: &str) -> IResult<&str, Vec<GedcomLine>> {
    preceded(parse_optional_bom, many1(parse_gedcom_line))(input)
}

fn parse_optional_bom(input: &str) -> IResult<&str, Option<char>> {
    opt(char('\u{feff}'))(input)
}

fn parse_gedcom_line(input: &str) -> IResult<&str, GedcomLine> {
    tuple((
        parse_level,
        parse_delim,
        opt(parse_optional_xref_id),
        parse_tag,
        opt(parse_optional_line_value),
        parse_terminator,
    ))(input)
    .map(five_tuple_to_gedcom_line)
}

#[cfg(test)]
mod tests {
    use super::parse_gedcom_line;
    use crate::models::gedcom::{GedcomLine, GedcomLineTag};

    #[test]
    fn test_parse_gedcom_line_valid_example_one() {
        let input = "0 HEAD\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Header)
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_two() {
        let input = "0 @F1@ FAM\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Family)
            .with_optional_xref_id(Some(String::from("@F1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_three() {
        let input = "0 @I1@ INDI\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .with_optional_xref_id(Some(String::from("@I1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_four() {
        let input = "0 @I2@ INDI\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .with_optional_xref_id(Some(String::from("@I2@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_five() {
        let input = "0 @I3@ INDI\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .with_optional_xref_id(Some(String::from("@I3@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_six() {
        let input = "0 @I4@ INDI\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Individual)
            .with_optional_xref_id(Some(String::from("@I4@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_seven() {
        let input = "0 @SUBM1@ SUBM\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Submitter)
            .with_optional_xref_id(Some(String::from("@SUBM1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_eight() {
        let input = "0 TRLR\r\n";
        let line = GedcomLine::builder()
            .with_level(0)
            .with_tag(GedcomLineTag::Trailer)
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_nine() {
        let input = "1 _ROOT @I1@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Custom(String::from("_ROOT")))
            .with_optional_line_value(Some(String::from("@I1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_ten() {
        let input = "1 _UID 1A063542-AC5D-4FEA-A91C-FCCFD1B8C9C1\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Custom(String::from("_UID")))
            .with_optional_line_value(Some(String::from("1A063542-AC5D-4FEA-A91C-FCCFD1B8C9C1")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_eleven() {
        let input = "1 _UID 9CE91117-3D67-4F34-9581-5DC2F8808674\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Custom(String::from("_UID")))
            .with_optional_line_value(Some(String::from("9CE91117-3D67-4F34-9581-5DC2F8808674")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twelve() {
        let input = "1 _UID B4C39338-64A4-4AC3-BFA6-E522B4059EC4\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Custom(String::from("_UID")))
            .with_optional_line_value(Some(String::from("B4C39338-64A4-4AC3-BFA6-E522B4059EC4")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirteen() {
        let input = "1 _UID DD0EC592-BAF7-4756-B706-094E2B8D436E\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Custom(String::from("_UID")))
            .with_optional_line_value(Some(String::from("DD0EC592-BAF7-4756-B706-094E2B8D436E")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fourteen() {
        let input = "1 BIRT\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Birth)
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifteen() {
        let input = "1 CHAN\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Change)
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_sixteen() {
        let input = "1 CHAR UTF-8\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Character)
            .with_optional_line_value(Some(String::from("UTF-8")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_seventeen() {
        let input = "1 CHIL @I1@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Child)
            .with_optional_line_value(Some(String::from("@I1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_eighteen() {
        let input = "1 CHIL @I4@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Child)
            .with_optional_line_value(Some(String::from("@I4@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_nineteen() {
        let input = "1 DATE 15 APR 2020\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Date)
            .with_optional_line_value(Some(String::from("15 APR 2020")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty() {
        let input = "1 DEST FINDMYPAST\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Destination)
            .with_optional_line_value(Some(String::from("FINDMYPAST")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_one() {
        let input = "1 FAMC @F1@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::FamilyChild)
            .with_optional_line_value(Some(String::from("@F1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_two() {
        let input = "1 FAMS @F1@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::FamilySpouse)
            .with_optional_line_value(Some(String::from("@F1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_three() {
        let input = "1 FILE Siblings.ged\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::File)
            .with_optional_line_value(Some(String::from("Siblings.ged")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_four() {
        let input = "1 GEDC\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Gedcom)
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_five() {
        let input = "1 HUSB @I3@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Husband)
            .with_optional_line_value(Some(String::from("@I3@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_six() {
        let input = "1 LANG English\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Language)
            .with_optional_line_value(Some(String::from("English")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_seven() {
        let input = "1 NAME Frank /Henderson/\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Frank /Henderson/")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_eight() {
        let input = "1 NAME Gavin /Henderson/\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Gavin /Henderson/")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_twenty_nine() {
        let input = "1 NAME Jane /Smith/\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Jane /Smith/")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty() {
        let input = "1 NAME Not known\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Not known")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_one() {
        let input = "1 NAME Rachel /Henderson/\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Rachel /Henderson/")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_two() {
        let input = "1 SEX F\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Sex)
            .with_optional_line_value(Some(String::from("F")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_three() {
        let input = "1 SEX M\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Sex)
            .with_optional_line_value(Some(String::from("M")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_four() {
        let input = "1 SOUR FINDMYPAST\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Source)
            .with_optional_line_value(Some(String::from("FINDMYPAST")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_five() {
        let input = "1 SUBM @SUBM1@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Submitter)
            .with_optional_line_value(Some(String::from("@SUBM1@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_siz() {
        let input = "1 WIFE @I2@\r\n";
        let line = GedcomLine::builder()
            .with_level(1)
            .with_tag(GedcomLineTag::Wife)
            .with_optional_line_value(Some(String::from("@I2@")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_seven() {
        let input = "2 _PRIM Y\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Custom(String::from("_PRIM")))
            .with_optional_line_value(Some(String::from("Y")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_eight() {
        let input = "2 CORP DC Thomson Family History\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Corporate)
            .with_optional_line_value(Some(String::from("DC Thomson Family History")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_thirty_nine() {
        let input = "2 DATE 1 Jan 1990\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Date)
            .with_optional_line_value(Some(String::from("1 Jan 1990")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty() {
        let input = "2 DATE 15 APR 2020\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Date)
            .with_optional_line_value(Some(String::from("15 APR 2020")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_one() {
        let input = "2 FORM LINEAGE-LINKED\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Format)
            .with_optional_line_value(Some(String::from("LINEAGE-LINKED")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_two() {
        let input = "2 GIVN Frank\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::GivenName)
            .with_optional_line_value(Some(String::from("Frank")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_three() {
        let input = "2 GIVN Gavin\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::GivenName)
            .with_optional_line_value(Some(String::from("Gavin")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_four() {
        let input = "2 GIVN Jane\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::GivenName)
            .with_optional_line_value(Some(String::from("Jane")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_five() {
        let input = "2 GIVN Rachel\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::GivenName)
            .with_optional_line_value(Some(String::from("Rachel")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_siz() {
        let input = "2 NAME Findmypast Family Tree\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Name)
            .with_optional_line_value(Some(String::from("Findmypast Family Tree")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_seven() {
        let input = "2 SURN Henderson\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Surname)
            .with_optional_line_value(Some(String::from("Henderson")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_eight() {
        let input = "2 SURN Smith\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Surname)
            .with_optional_line_value(Some(String::from("Smith")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_forty_nine() {
        let input = "2 TIME 15:44:29\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Time)
            .with_optional_line_value(Some(String::from("15:44:29")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty() {
        let input = "2 VERS 2.0\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Version)
            .with_optional_line_value(Some(String::from("2.0")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_one() {
        let input = "2 VERS 5.5.1\r\n";
        let line = GedcomLine::builder()
            .with_level(2)
            .with_tag(GedcomLineTag::Version)
            .with_optional_line_value(Some(String::from("5.5.1")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_two() {
        let input = "3 ADDR The Glebe, 6 Chapel Place, Rivington Street\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Address)
            .with_optional_line_value(Some(String::from(
                "The Glebe, 6 Chapel Place, Rivington Street",
            )))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_three() {
        let input = "3 TIME 16:42:39\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Time)
            .with_optional_line_value(Some(String::from("16:42:39")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_four() {
        let input = "3 TIME 16:43:01\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Time)
            .with_optional_line_value(Some(String::from("16:43:01")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_five() {
        let input = "3 TIME 16:43:06\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Time)
            .with_optional_line_value(Some(String::from("16:43:06")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_six() {
        let input = "3 TIME 16:44:00\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Time)
            .with_optional_line_value(Some(String::from("16:44:00")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_seven() {
        let input = "3 WWW www.findmypast.com\r\n";
        let line = GedcomLine::builder()
            .with_level(3)
            .with_tag(GedcomLineTag::Web)
            .with_optional_line_value(Some(String::from("www.findmypast.com")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_eight() {
        let input = "4 CITY London\r\n";
        let line = GedcomLine::builder()
            .with_level(4)
            .with_tag(GedcomLineTag::City)
            .with_optional_line_value(Some(String::from("London")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_fifty_nine() {
        let input = "4 POST EC2A 3DQ\r\n";
        let line = GedcomLine::builder()
            .with_level(4)
            .with_tag(GedcomLineTag::PostalCode)
            .with_optional_line_value(Some(String::from("EC2A 3DQ")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_gedcom_line_valid_example_sixty() {
        let input = "4 CTRY England\r\n";
        let line = GedcomLine::builder()
            .with_level(4)
            .with_tag(GedcomLineTag::Country)
            .with_optional_line_value(Some(String::from("England")))
            .build()
            .unwrap();
        let expected = Ok(("", line));
        let actual = parse_gedcom_line(input);
        assert_eq!(actual, expected);
    }
}
