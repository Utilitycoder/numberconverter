use colored::*;
use num_base::Based;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

// This function returns the base value for a given string representation of the base.
// If the base is not supported, it returns an error.
fn get_base_value(base: &str) -> Result<usize, &'static str> {
    match base {
        "binary" => Ok(2),
        "hex" => Ok(16),
        "octal" => Ok(8),
        "decimal" => Ok(10),
        _ => Err("Unsupported base"),
    }
}

// Add a new struct to represent a conversion
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Conversion {
    from_base: String,
    to_base: String,
    original_number: String,
    converted_number: String,
}

// Add a new vector to store the conversion history
pub static mut CONVERSION_HISTORY: Vec<Conversion> = Vec::new();

pub fn print_conversion_history() {
    unsafe {
        for conversion in &CONVERSION_HISTORY {
            println!("{:?}", conversion);
        }
    }
}

// This function converts a number from one base to another.
// It takes three arguments:
// - base_convert_to: the base to convert the number to
// - number_str: the number to convert, as a string
// - base_convert_from: the base of the number to convert
// It returns a Result. If the conversion is successful, it returns Ok. If an error occurs, it returns Err.
pub fn convert(
    base_convert_to: &str,
    number_str: &str,
    base_convert_from: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the base values for the input and output bases
    let convert_from = get_base_value(base_convert_from)?;
    let convert_to = get_base_value(base_convert_to)?;

    // Convert the number from the input base to the output base
    let number = Based::new(number_str, convert_from)
        .to(convert_to)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // Print the converted number in the appropriate format
    match base_convert_to {
        "binary" => {
            println!("{}", format!("Binary: {}", number.val).to_string().green());
        }
        "hex" => {
            println!(
                "{}",
                format!("Hexadecimal: 0x{}", number.val).to_string().red()
            );
        }
        "octal" => {
            println!("{}", format!("Octal: {}", number.val).to_string().blue());
        }
        "decimal" => {
            println!(
                "{}",
                format!("Decimal: {}", number.val).to_string().yellow()
            );
        }
        _ => {
            println!("Unsupported base: {}", base_convert_to);
        }
    }

    let conversion = Conversion {
        from_base: base_convert_from.to_string(),
        to_base: base_convert_to.to_string(),
        original_number: number_str.to_string(),
        converted_number: number.val.to_string(),
    };

    unsafe {
        CONVERSION_HISTORY.push(conversion);
    }

    // Write the conversion history to a file
    // Write the conversion history to a file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("conversion_history.txt")?;

    unsafe {
        let last_conversion = CONVERSION_HISTORY.last().unwrap();
        let json = serde_json::to_string(last_conversion)?;
        writeln!(file, "{}", json)?;
    }
    load_conversion_history()?;

    Ok(())
}

pub fn load_conversion_history() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./conversion_history.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let conversion: Conversion = serde_json::from_str(&line)?;
        unsafe {
            CONVERSION_HISTORY.push(conversion);
        }
    }

    print_conversion_history();

    Ok(())
}

mod tests {
    #[allow(unused_imports)]
    use super::convert;

    #[test]
    fn test_convert_binary_to_decimal() {
        convert("decimal", "10", "binary").unwrap();
    }

    #[test]
    fn test_convert_binary_to_hex() {
        convert("hex", "1010", "binary").unwrap();
    }

    #[test]
    fn test_convert_binary_to_octal() {
        convert("octal", "1010", "binary").unwrap();
    }

    #[test]
    fn test_convert_hex_to_decimal() {
        convert("decimal", "10", "hex").unwrap();
    }

    #[test]
    fn test_convert_hex_to_binary() {
        convert("binary", "1a", "hex").unwrap();
    }

    #[test]
    fn test_convert_hex_to_octal() {
        convert("octal", "1a", "hex").unwrap();
    }

    #[test]
    fn test_convert_octal_to_decimal() {
        convert("decimal", "12", "octal").unwrap();
    }

    #[test]
    fn test_convert_octal_to_binary() {
        convert("binary", "12", "octal").unwrap();
    }

    #[test]
    fn test_convert_octal_to_hex() {
        convert("hex", "12", "octal").unwrap();
    }

    #[test]
    fn test_convert_decimal_to_binary() {
        convert("binary", "10", "decimal").unwrap();
    }

    #[test]
    fn test_convert_decimal_to_hex() {
        convert("hex", "10", "decimal").unwrap();
    }
}
