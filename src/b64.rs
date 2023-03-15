#![allow(dead_code)]

use::std::str;

const ALPHABET: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
const CHUNK_SIZE: usize = 6;
const B64_MULT: usize = 3;

pub fn encode(value: &str) -> String {
    let mut data = value.as_bytes().iter().map(|v| format!("{:0>8b}", v)).collect::<String>();
    let padding = CHUNK_SIZE - (data.len() % CHUNK_SIZE);
    data += &"0".repeat(padding);
    let mut result = vec![];
    while !data.is_empty() {
        let i = usize::from_str_radix(data.drain(0..CHUNK_SIZE).as_str(), 2).unwrap();
        result.push(char::from_u32(ALPHABET[i] as _).unwrap());
        dbg!(char::from_u32(ALPHABET[i] as _).unwrap());
    }

    result.iter().collect::<String>()
}

#[cfg(test)]
mod test {
    use super::encode;

    #[test]
    fn encode_string() {
        let output = encode("Test String");
        assert_eq!(output, "VGVzdCBTdHJpbmc");
    }
}