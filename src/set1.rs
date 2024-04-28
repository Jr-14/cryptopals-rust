pub fn hex_to_base64(s: &str) -> String {
    let ss = s.as_bytes();
    println!("{:?}", ss);
    s.to_string()
}

fn get_str_len(s: &str) {
    println!("String length is {}", s.chars().count());
}

fn print_str_to_bits(s: &str){
    let bit_str: Vec<_> = s
        .as_bytes()
        .iter()
        .map(|&c| {
            let f = format!("{c:b}");
            let padding = 8 - f.chars().count();
            assert!(padding >= 0);
            "0".repeat(padding).to_string() + &f
        })
        .collect();
    println!("{:?}", bit_str);
}

fn decode_hex_to_base64(v: Vec<u8>) -> Vec<u8> {
    let c: Vec<u8> = v
        .iter()
        .zip([3, 2, 1, 0])
        .map(|(vc, num)| {
            println!("Val: {}, iter {}", vc, num);
            vc.clone()
        })
        .collect();
    println!("{:?}", c);
    c
}

fn print_vec_u8(v: Vec<u8>) {
    
}

/// Calculate the corresponding hex character into a number
fn hex_value(c: u8) -> Result<u8, &'static str> {
    match c {
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'0'..=b'9' => Ok(c -b'0'),
        _ => Err("Invalid hex character")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_hex_to_base64() {
        let s = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!(s, hex_to_base64(s));
    }

    #[test]
    #[ignore]
    fn test_str_length() {
        let r = "abcd";
        print_str_to_bits(&r);
    }

    #[test]
    #[ignore]
    fn print_another_string() {
        let s = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        print_str_to_bits(&s);
    }

    #[test]
    #[ignore]
    fn test_print_char() {
        let c = "c";
        print_str_to_bits(&c);
    }

    #[test]
    fn test_hex_value_lowercase() {
        let hex_chars = "0123456789abcdef";
        let vals = Vec::from([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        let hex_vals: Vec<u8> = hex_chars
            .as_bytes()
            .iter()
            .map(|b| hex_value(*b).unwrap())
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
            .map(|b| hex_value(*b).unwrap())
            .collect();
        assert_eq!(vals, hex_vals);
    }

    #[test]
    fn test_calc_hex() {
        let hex_string = "fa7b";
        let hex_iter = hex_string
            .as_bytes()
            .iter()
            .map(|b| hex_value(*b).unwrap());
        // let sum: u32 = hex_iter.fold(0u32, |sum ,i| sum + (i.clone() as u32));
        hex_iter.for_each(|b| {
            println!("{:08b}", b);
        });
    }

    // #[test]
    // fn test_decode_hex() {
    //     // let v = Vec::from([10, 20, 30, 40]);
    //     let v = Vec::from("0123456789abcdef".as_bytes());
    //     v.chunks(4).for_each(|c| decode_hex_to_base64(c.to_vec()));
    //     decode_hex_to_base64(v);
    // }
}
