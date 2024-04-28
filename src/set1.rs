pub fn hex_to_base64(s: &str) -> String {
    let ss = s.as_bytes();
    println!("{:?}", ss);
    s.to_string()
}

fn get_str_len(s: &str) {
    println!("String length is {}", s.chars().count());
}

fn print_str_to_bits(s: &str) {
    let bit_str: Vec<_> = s
        .as_bytes()
        .iter()
        .map(|&c| {
            let f = format!("{c:b}");
            let padding = 8 - f.chars().count();
            assert!(padding >= 0);
            f
        })
        .collect();
    println!("{:?}", bit_str);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(s, hex_to_base64(s));
    }

    #[test]
    fn test_str_length() {
        let r = "abcd";
        print_str_to_bits(&r);
    }

    #[test]
    fn print_another_string() {
        let s = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        print_str_to_bits(&s);
    }
}
