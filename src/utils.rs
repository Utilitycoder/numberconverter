use num_base::Based;

fn get_base_value(base: &str) -> usize {
    match base {
        "binary" => 2,
        "hex" => 16,
        "octal" => 8,
        "decimal" => 10,
        _ => {
            panic!("Unsupported base: {}", base);
        }
    }
}

pub fn convert(
    base_convert_to: &str,
    number_str: &str,
    base_convert_from: &str,
) -> Result<(), std::num::ParseIntError> {
    let convert_from = get_base_value(base_convert_from);
    let convert_to = get_base_value(base_convert_to);

    let number = Based::new(number_str, convert_from).to(convert_to).unwrap();

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
    use super::*;
    
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