#![allow(dead_code)]

use ::std::str;

const ALPHABET: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
const CHUNK_SIZE: usize = 6;
const B64_MULT: usize = 3;

pub fn encode(value: &str) -> String {
    let mut data = value
        .as_bytes()
        .iter()
        .map(|v| format!("{:0>8b}", v))
        .collect::<String>();
    let padding = (CHUNK_SIZE - (data.len() % CHUNK_SIZE)) % CHUNK_SIZE;
    let mut result = vec![];

    data += &"0".repeat(padding);

    while !data.is_empty() {
        let i = usize::from_str_radix(data.drain(0..CHUNK_SIZE).as_str(), 2).unwrap();
        result.push(char::from_u32(ALPHABET[i] as _).unwrap());
    }

    "=".repeat(padding / 2).chars().for_each(|c| result.push(c));

    result.iter().collect::<String>()
}

pub fn decode(value: &str) -> String {
    let mut data: String = value
        .as_bytes()
        .iter()
        .map(|c| {
            format!(
                "{:0>6b}",
                ALPHABET.iter().position(|v| v == c).unwrap_or_default()
            )
        })
        .collect::<String>();
    let mut result = vec![];

    while !data.is_empty() {
        let byte = u8::from_str_radix(data.drain(0..8).as_str(), 2);
        match byte {
            Ok(0) => (),
            Ok(v) => result.push(v),
            Err(_) => (),
        }
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod test {
    use super::{decode, encode};

    struct B64Pair(pub &'static str, pub &'static str);

    fn test_data() -> Vec<B64Pair> {
        vec![
            B64Pair("Test String", "VGVzdCBTdHJpbmc="),
            B64Pair("bea is cool", "YmVhIGlzIGNvb2w="),
            B64Pair("🥺", "8J+lug=="),
            B64Pair("おはいよう！私の名前はBea!元気ですか？", "44GK44Gv44GE44KI44GG77yB56eB44Gu5ZCN5YmN44GvQmVhIeWFg+awl+OBp+OBmeOBi++8nw=="),
            B64Pair("These sentences feel random but they're not, I promise. I am just making sure to test as many different silly things I can!", "VGhlc2Ugc2VudGVuY2VzIGZlZWwgcmFuZG9tIGJ1dCB0aGV5J3JlIG5vdCwgSSBwcm9taXNlLiBJIGFtIGp1c3QgbWFraW5nIHN1cmUgdG8gdGVzdCBhcyBtYW55IGRpZmZlcmVudCBzaWxseSB0aGluZ3MgSSBjYW4h"),
            B64Pair("I need to test padding of each length, you see!!", "SSBuZWVkIHRvIHRlc3QgcGFkZGluZyBvZiBlYWNoIGxlbmd0aCwgeW91IHNlZSEh"),
        ]
    }

    #[test]
    fn encode_string() {
        test_data()
            .iter()
            .for_each(|B64Pair(i, o)| assert_eq!(encode(i), *o));
    }

    #[test]
    fn decode_string() {
        test_data()
            .iter()
            .for_each(|B64Pair(i, o)| assert_eq!(decode(o), *i));
    }
}
