use std::{collections::HashMap, ops::Range};

const BASE62_CHARS: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Key(String);

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Key::from(value.to_owned())
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Key {
    fn new(value: &str) -> Self {
        Self(value.to_string())
    }

    pub fn generate(num: u64, size: u8) -> Key {
        Self(Self::encode_base62(num, size))
    }

    pub fn generate_multiple(range: Range<u64>, size: u8) -> HashMap<u64, Key> {
        range.map(|n| (n, Self::generate(n, size))).collect()
    }

    fn encode_base62(mut num: u64, size: u8) -> String {
        let mut result = Vec::with_capacity(size as usize);

        for _ in 0..size as usize {
            let remainder = (num % 62) as usize;
            result.push(BASE62_CHARS[remainder]);
            num /= 62;
        }

        result.reverse();
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::key::Key;

    #[test]
    fn test_encode_base62() {
        assert_eq!(Key::new("0000000"), Key::generate(0, 7));
        assert_eq!(Key::new("0000001"), Key::generate(1, 7));
        assert_eq!(Key::new("000000z"), Key::generate(61, 7));
        assert_eq!(Key::new("0000010"), Key::generate(62, 7));
    }
}
