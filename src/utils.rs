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

    match base_convert_to {
        "binary" => {
            let number = Based::new(number_str, convert_from).to(convert_to).unwrap();
            println!("Binary: {}", number);
        }
        "hex" => {
            let number = Based::new(number_str, convert_from).to(convert_to).unwrap();
            println!("Hexadecimal: 0x{}", number);
        }
        "octal" => {
            let number = Based::new(number_str, convert_from).to(convert_to).unwrap();
            println!("Octal: {}", number);
        }
        "decimal" => {
            let number = Based::new(number_str, convert_from).to(convert_to).unwrap();
            println!("Decimal: {}", number);
        }
        _ => {
            println!("Unsupported base: {}", base_convert_to);
        }
    }

    Ok(())
}
