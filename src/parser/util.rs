use crate::models::gedcom::{GedcomLine, GedcomLineTag};
use std::iter::FromIterator;

pub fn five_tuple_to_gedcom_line(
    (input, (level, _delim, optional_xref_id, tag, optional_line_value, _terminator)): (
        &str,
        (
            u8,
            String,
            Option<String>,
            GedcomLineTag,
            Option<String>,
            String,
        ),
    ),
) -> (&str, GedcomLine) {
    let line = GedcomLine::builder()
        .with_level(level)
        .with_optional_line_value(optional_line_value)
        .with_tag(tag)
        .with_optional_xref_id(optional_xref_id)
        .build()
        .unwrap();

    (input, line)
}

#[inline]
pub fn char_to_string((input, output): (&str, char)) -> (&str, String) {
    (input, output.to_string())
}

#[inline]
pub fn four_tuple_to_string(
    (input, (a, b, c, d)): (&str, (String, String, String, String)),
) -> (&str, String) {
    vec_to_string((input, vec![a, b, c, d]))
}

pub fn five_tuple_to_string(
    (input, (a, b, c, d, e)): (&str, (String, String, String, String, String)),
) -> (&str, String) {
    vec_to_string((input, vec![a, b, c, d, e]))
}

pub fn tuple_to_string((input, (a, b)): (&str, (String, String))) -> (&str, String) {
    vec_to_string((input, vec![a, b]))
}

pub fn vec_to_string((input, output): (&str, Vec<String>)) -> (&str, String) {
    (input, String::from_iter(output))
}
