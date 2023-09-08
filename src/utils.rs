use num_base::Based;

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
            println!("Binary: {}", number.val);
        }
        "hex" => {
            println!("Hexadecimal: 0x{}", number.val);
        }
        "octal" => {
            println!("Octal: {}", number.val);
        }
        "decimal" => {
            println!("Decimal: {}", number.val);
        }
        _ => {
            println!("Unsupported base: {}", base_convert_to);
        }
    }

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
