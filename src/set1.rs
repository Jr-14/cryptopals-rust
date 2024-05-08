pub fn hex_to_base64(hex_string: &str) -> String {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789/+";
    hex_string
        .as_bytes()
        .iter()
        .map(|byte| {
            match hex_value(byte) {
                Ok(val) => val,
                _ => panic!("Invalid hex character"),
            }
        })
        .collect::<Vec<u8>>()
        .chunks(3)
        .fold(String::from(""), |mut acc, chunk| {
            if chunk.len() == 3 {
                let first_index: usize = ((chunk[0] << 2) + (chunk[1] >> 2)).into();
                let second_index: usize = (((chunk[1] & 3) << 4) + (chunk[2])).into();
                let first_char = match base64_chars.chars().nth(first_index) {
                    Some(c) => c,
                    None => panic!("No base64 char found"),
                };
                let second_char = match base64_chars.chars().nth(second_index) {
                    Some(c) => c,
                    None => panic!("No base64 char found"),
                };
                acc.push(first_char);
                acc.push(second_char);
            } else if chunk.len() == 2 {
                let first_index: usize = ((chunk[0] << 2) + (chunk[1] >> 2)).into();
                let second_index: usize = (((chunk[1] & 3) << 4)).into();
                let first_char = match base64_chars.chars().nth(first_index) {
                    Some(c) => c,
                    None => panic!("No base64 char found"),
                };
                let second_char = match base64_chars.chars().nth(second_index) {
                    Some(c) => c,
                    None => panic!("No base64 char found"),
                };
                acc.push(first_char);
                acc.push(second_char);
                acc.push('=');
            } else {
                let first_index: usize = ((chunk[0] << 2)).into();
                let first_char = match base64_chars.chars().nth(first_index) {
                    Some(c) => c,
                    None => panic!("No base64 char found"),
                };
                acc.push(first_char);
                acc.push('=');
                acc.push('=');
            }
            acc
        })
}

/// Calculate the corresponding hex character into a number
fn hex_value(c: &u8) -> Result<u8, &'static str> {
    match c {
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err("Invalid hex character"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_value_lowercase() {
        let hex_chars = "0123456789abcdef";
        let vals = Vec::from([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        let hex_vals: Vec<u8> = hex_chars
            .as_bytes()
            .iter()
            .map(|b| hex_value(b).unwrap())
            .collect();
        assert_eq!(vals, hex_vals);
    }

    #[test]
    fn test_hex_value_uppercase() {
        let hex_chars = "0123456789ABCDEF";
        let vals = Vec::from([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        let hex_vals: Vec<u8> = hex_chars
            .as_bytes()
            .iter()
            .map(|b| hex_value(b).unwrap())
            .collect();
        assert_eq!(vals, hex_vals);
    }

    #[test]
    fn test_hex_to_base64() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_string = hex_to_base64(hex_string);
        assert_eq!(base64_string, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    #[test]
    fn test_hex_to_base64_2() {
        let hex_string = "ffff";
        let base64_string = hex_to_base64(hex_string);
        assert_eq!(base64_string, "++8==");
    }
}
