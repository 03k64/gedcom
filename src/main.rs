use clap::{App, Arg};
use gedcom::gedcom_to_relation_json;
use std::{
    error::Error,
    fs,
    fs::File,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let input_directory_arg = Arg::with_name("directory")
        .help("Specify a directory containing one or more GEDCOM files to convert")
        .long("directory")
        .required(true)
        .short("d")
        .takes_value(true)
        .value_name("directory");

    let arguments = App::new("gedcom")
        .version("0.1")
        .arg(input_directory_arg)
        .get_matches();

    let directory = arguments
        .value_of("directory")
        .ok_or("No input directory specified")?;

    if let Ok(dir_entries) = fs::read_dir(directory) {
        for entry in dir_entries.into_iter() {
            match entry {
                Err(_) => {}
                Ok(entry) => {
                    let input_path = entry.path();
                    let output_path = input_path.with_extension("json");

                    let mut input = File::open(input_path)?;
                    let mut gedcom = String::new();
                    input.read_to_string(&mut gedcom)?;

                    let json = gedcom_to_relation_json(gedcom.as_str())?;

                    let mut output = File::create(output_path)?;
                    output.write_all(json.as_bytes())?;
                }
            }
        }
    }

    Ok(())
}
