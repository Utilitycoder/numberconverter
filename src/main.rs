use clap::{App, Arg, SubCommand};

use numberconverter::utils::convert;

/// This is the main function for the number converter program.
/// It uses the clap library to handle command line arguments and subcommands.
/// The program supports a "convert" subcommand that takes three arguments:
/// - base_convert_to: The base to convert the number to (e.g., 'binary', 'hex', 'octal')
/// - base_to_convert_from: The base of the number to be converted
/// - number_to_convert: The number to be converted
///
/// The convert function from the utils module is used to perform the conversion.
/// If the conversion is successful, the result is printed to the console.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Number Converter")
        .version("1.0")
        .author("Utilitycoder")
        .about("Converts numbers between different bases")
        .subcommand(
            SubCommand::with_name("convert")
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
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("convert") {
        let base_convert_from = matches.value_of("base_to_convert_from").unwrap();
        let base_convert_to = matches.value_of("base_convert_to").unwrap();
        let number_str = matches.value_of("number_to_convert").unwrap();

        convert(base_convert_to, number_str, base_convert_from)?;
    }

    Ok(())
}
