use clap::{App, Arg};
use std::num::ParseIntError;

use numberconverter::utils::convert;

fn main() -> Result<(), ParseIntError> {
    let matches = App::new("Number Converter")
        .version("1.0")
        .author("Utilitycoder")
        .about("Converts numbers between different bases")
        .arg(
            Arg::with_name("base_convert_to")
                .help("The type of base to convert to (e.g., 'binary', 'hex', 'octal')")
                .required(true),
        )
        .arg(
            Arg::with_name("base_to_convert_from")
                .help("Specifies the base of the number you want to convert")
                .required(true),
        )
        .arg(
            Arg::with_name("number_to_convert")
                .help("The number you want to convert")
                .required(true),
        )
        .get_matches();

    let base_convert_from = matches.value_of("base_to_convert_from").unwrap();
    let base_convert_to = matches.value_of("base_convert_to").unwrap();
    let number_str = matches.value_of("number_to_convert").unwrap();

    let _ = convert(base_convert_to, number_str, base_convert_from);

    Ok(())
}
