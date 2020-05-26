use clap::{App, Arg};
use gedcom::gedcom_to_relation_json;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_arg = Arg::with_name("input")
        .help("Specify a GEDCOM file to use as input")
        .long("input")
        .required(true)
        .short("i")
        .takes_value(true)
        .value_name("input");

    let output_file_arg = Arg::with_name("output")
        .help("Specify a path to write the JSON output to")
        .long("output")
        .required(true)
        .short("o")
        .takes_value(true)
        .value_name("output");

    let arguments = App::new("gedcom")
        .version("0.1")
        .arg(input_file_arg)
        .arg(output_file_arg)
        .get_matches();

    let input_file = arguments
        .value_of("input")
        .ok_or("No input file specified")?;

    let output_file = arguments
        .value_of("output")
        .ok_or("No output file specified")?;

    let mut input_file = File::open(input_file)?;
    let mut gedcom_input = String::new();
    input_file.read_to_string(&mut gedcom_input)?;

    let json = gedcom_to_relation_json(gedcom_input.as_str())?;

    let mut output_file = File::create(output_file)?;
    output_file.write_all(json.as_bytes())?;

    Ok(())
}
