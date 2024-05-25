use super::challenge1;
use super::challenge2;

pub fn single_byte_xor_cipher(encoded_string: &str) -> String {
    let encoded_bits = hex_string_to_bytes(encoded_string);
    let mut vec_result: Vec<String> = Vec::new();

    for c in b'a'..=b'z' {
        let output: Vec<u8> = encoded_bits.iter().map(|b| b ^ c).collect();
        println!("{:?}", output);
        let output = std::str::from_utf8(&output).unwrap().to_string();
        vec_result.push(output);
        // println!("{:8b}", bits);
        // println!("{:?}", c);
    }

    // for res in vec_result.iter() {
    //     println!("{:?}", res);
    // }

    let mut max_score = 0;
    let mut sentence = String::from("");

    for res_string in vec_result.iter() {
        let score = score_string(res_string);
        if score > max_score {
            max_score = score;
            sentence = res_string.to_string();
        }
    }

    println!("max score is {max_score} with sentence: {sentence}");
    sentence
}

fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    s
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            (challenge1::hex_value(&chunk[0]).unwrap() << 4) + challenge1::hex_value(&chunk[1]).unwrap()
        })
        .collect()
}

// Give a score for each character
fn score_byte_char(c: u8) -> i32 {
    match c {
        b'e' | b'E' => 127,
        b't' | b'T' => 91,
        b'a' | b'A' => 82,
        b'o' | b'O' => 75,
        b'i' | b'I' => 70,
        b'n' | b'N' => 67,
        b's' | b'S' => 63,
        b'h' | b'H' => 61,
        b'r' | b'R' => 60,
        b'd' | b'D' => 43,
        b'l' | b'L' => 40,
        b'c' | b'C' => 28,
        b'u' | b'U' => 28,
        b'm' | b'M' => 24,
        b'f' | b'F' => 22,
        b'g' | b'G' => 20,
        b'y' | b'Y' => 20,
        b'p' | b'P' => 19,
        b'b' | b'B' => 15,
        b'v' | b'V' => 9,
        b'k' | b'K' => 7,
        b'j' | b'J' => 1,
        b'x' | b'X' => 1,
        b'q' | b'Q' => 1,
        b'z' | b'Z' => 1,
        b'\0' => 80,
        _ => 0,
    }
} 

fn score_string(s: &str) -> i32 {
    s.chars().into_iter().fold(0, |sum, val| sum + score_byte_char(val as u8))
} 

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score_byte_char() {
        let s = "e".chars().nth(0).unwrap() as u8;
        let char_score = score_byte_char(s);
        assert_eq!(char_score, 127);
    }

    #[test]
    fn test_score_string() {
        let s = String::from("cool");
        let total_score = score_string(&s);
        assert_eq!(total_score, 218);

        let s = "cheater";
        let total_score = score_string(s);
        assert_eq!(total_score, 576);

        let s = "asdf";
        let total_score = score_string(s);
        assert_eq!(total_score, 210);
    }

    #[test]
    fn test_hex_string_to_bytes() {
        let s = "ff";
        let vec = hex_string_to_bytes(s);
        assert_eq!(vec, vec![255]);

        let s = "ff5e";
        let vec = hex_string_to_bytes(s);
        assert_eq!(vec, vec![255, 94]);
    }

    #[test]
    fn test_single_byte_xor_cipher() {
        let plain_text = single_byte_xor_cipher("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        assert_eq!(plain_text, "cOOKING\0mc\u{7}S\0LIKE\0A\0POUND\0OF\0BACON");
    }
}
