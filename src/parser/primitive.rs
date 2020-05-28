use super::util::{char_to_string, tuple_to_string};
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    character::complete::{char, one_of},
    sequence::pair,
    IResult,
};

lazy_static! {
    static ref OTHERCHAR: Vec<u8> = vec![
        0x21, 0x22, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x3A,
        0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x5B, 0x5C, 0x5D, 0x5E, 0x60, 0x7B, 0x7C, 0x7D, 0x7E, 0x80,
        0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
        0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E,
        0x9F, 0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD,
        0xAE, 0xAF, 0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC,
        0xBD, 0xBE, 0xBF, 0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB,
        0xCC, 0xCD, 0xCE, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA,
        0xDB, 0xDC, 0xDD, 0xDE, 0xDF, 0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9,
        0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8,
        0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE,
    ];
}

const ALPHA: [u8; 53] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7A, 0x5F,
];
const AT: char = '@';
const CR: char = '\r';
const DELIM: char = ' ';
const DIGIT: [u8; 10] = [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39];
const HASH: char = '#';
const LF: char = '\n';

pub fn parse_alpha(input: &str) -> IResult<&str, String> {
    one_of(&ALPHA[..])(input).map(char_to_string)
}

pub fn parse_alphanum(input: &str) -> IResult<&str, String> {
    alt((parse_alpha, parse_digit))(input)
}

pub fn parse_anychar(input: &str) -> IResult<&str, String> {
    alt((
        parse_alpha,
        parse_delim,
        parse_digit,
        parse_double_at,
        parse_hash,
        parse_otherchar,
    ))(input)
}

pub fn parse_at(input: &str) -> IResult<&str, String> {
    char(AT)(input).map(char_to_string)
}

pub fn parse_carriage_return(input: &str) -> IResult<&str, String> {
    char(CR)(input).map(char_to_string)
}

pub fn parse_delim(input: &str) -> IResult<&str, String> {
    char(DELIM)(input).map(char_to_string)
}

pub fn parse_digit(input: &str) -> IResult<&str, String> {
    one_of(&DIGIT[..])(input).map(char_to_string)
}

fn parse_double_at(input: &str) -> IResult<&str, String> {
    pair(parse_at, parse_at)(input).map(tuple_to_string)
}

pub fn parse_hash(input: &str) -> IResult<&str, String> {
    char(HASH)(input).map(char_to_string)
}

pub fn parse_line_feed(input: &str) -> IResult<&str, String> {
    char(LF)(input).map(char_to_string)
}

pub fn parse_nonat(input: &str) -> IResult<&str, String> {
    alt((
        parse_alpha,
        parse_delim,
        parse_digit,
        parse_hash,
        parse_otherchar,
    ))(input)
}

pub fn parse_otherchar(input: &str) -> IResult<&str, String> {
    one_of(&OTHERCHAR[..])(input).map(char_to_string)
}
