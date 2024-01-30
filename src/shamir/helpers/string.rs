use crate::shamir::constants::FIELD_BITS;

fn binary_to_u8(data: &str) -> Result<u8, String> {
    match u8::from_str_radix(data, 2) {
        Ok(v) => Ok(v),
        _ => Err(format!("Invalid binary character: `{}`.", data)),
    }
}

pub(crate) fn pad_left(data: &str, width: usize) -> String {
    format!("{:0>width$}", data, width = width)
}

pub(crate) fn hex_to_binary(data: &str) -> Result<String, String> {
    let mut binary = String::from("");
    for char in data.chars().rev() {
        if let Some(num) = char.to_digit(16) {
            binary = pad_left(&format!("{:b}", num), 4) + &binary;
        } else {
            return Err(format!("Invalid hex character: `{}`.", char));
        }
    }
    Ok(binary)
}

pub(crate) fn binary_to_hex(data: &str) -> Result<String, String> {
    let mut hex = String::from("");
    let binary = pad_left(data, 4);
    let mut i = binary.len();
    while i >= 4 {
        let num = binary_to_u8(&binary[(i - 4)..i])?;
        i -= 4;
        hex = format!("{:x}", num) + &hex;
    }
    Ok(hex)
}

pub(crate) fn split_binary(data: &str, pad_length: Option<usize>) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = vec![];
    let binary = match pad_length {
        Some(v) => pad_left(data, v),
        None => data.to_owned(),
    };

    let mut i = binary.len();
    while i > FIELD_BITS {
        let num = binary_to_u8(&binary[(i - FIELD_BITS)..i])?;
        i -= FIELD_BITS;
        result.push(num);
    }
    result.push(binary_to_u8(&binary[..i])?);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::shamir::helpers::string::{binary_to_hex, hex_to_binary, split_binary};

    #[test]
    fn test_hex_to_binary() {
        assert_eq!("01110011011010000110000101101101011010010111001001010011011001010110001101110010011001010111010001010100011001010111001101110100", hex_to_binary("7368616d697253656372657454657374").unwrap());
    }

    #[test]
    fn test_binary_to_hex() {
        assert_eq!("7368616d697253656372657454657374", binary_to_hex("01110011011010000110000101101101011010010111001001010011011001010110001101110010011001010111010001010100011001010111001101110100").unwrap());
    }

    #[test]
    fn test_split_binary() {
        let expected_data = vec![
            116, 115, 101, 84, 116, 101, 114, 99, 101, 83, 114, 105, 109, 97, 104, 115,
        ];
        assert_eq!(expected_data, split_binary("01110011011010000110000101101101011010010111001001010011011001010110001101110010011001010111010001010100011001010111001101110100", None).unwrap());
    }
}
