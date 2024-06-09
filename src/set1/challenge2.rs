use super::challenge1;
use std::iter::zip;

pub fn fixed_xor(hex_string_one: &str, hex_string_two: &str) -> String {
    let string_one_bits: Vec<u8> = hex_string_one
        .as_bytes()
        .iter()
        .map(|byte| challenge1::hex_value(byte).unwrap())
        .collect();
    let string_two_bits: Vec<u8> = hex_string_two
        .as_bytes()
        .iter()
        .map(|byte| challenge1::hex_value(byte).unwrap())
        .collect();

    let hex_strings = "0123456789abcdef";
    zip(string_one_bits, string_two_bits)
        .map(|(x, y)| hex_strings.chars().nth((x ^ y) as usize).unwrap())
        .fold(String::from(""), |acc, value| acc + &value.to_string())
}

#[cfg(test)]
mod tests {
    use super::fixed_xor;

    #[test]
    fn test_fixed_xor() {
        let xor_val = fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        );
        assert_eq!(xor_val, "746865206b696420646f6e277420706c6179");
    }
}
