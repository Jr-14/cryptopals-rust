pub fn hex_to_base64(s: &str) -> String {
    let ss = s.as_bytes();
    println!("{:?}", ss);
    s.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(s, hex_to_base64(s));
    }
}
