#![allow(dead_code)]

use crate::greek_char::{GreekChar, InvalidCharError};
use crate::greek_char::{LOWER_SIGMA, LOWER_SIGMA_FINAL};

#[derive(Debug, PartialEq, Eq)]
pub struct GreekWord {
    vec: Vec<GreekChar>
}

impl GreekWord {
    pub fn new() -> Self {
        GreekWord { vec: Vec::new() }
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Self, InvalidCharError> {
        let mut chars: Vec<GreekChar> = Vec::new();

        let mut byte_stack: [u8; 3] = [0, 0, 0];
        let mut test_char_len = 0;

        for byte in bytes {
            match test_char_len {
                0 => {
                    byte_stack[0] = *byte;
                },
                1 => {
                    byte_stack[1] = *byte;
                    if let Ok(test_char) = GreekChar::try_new(&byte_stack[0..2]) {
                        chars.push(test_char);
                        test_char_len = 0;
                        byte_stack = [0, 0, 0];
                        continue;
                    }
                },
                2 => {
                    byte_stack[2] = *byte;
                    match GreekChar::try_new(&byte_stack) {
                        Ok(greek_char) => chars.push(greek_char),
                        Err(e) => return Err(e)
                    }
                    test_char_len = 0;
                    byte_stack = [0, 0, 0];
                    continue;
                },
                _ => return Err(InvalidCharError)
            };
            test_char_len += 1;
        }
        Ok(GreekWord { vec: chars })
    }

    pub fn from_greek_chars(chars: &Vec<GreekChar>) -> Self {
        GreekWord { vec: chars.clone() }
    }

    pub fn from_string(str: &String) -> Result<Self, InvalidCharError> {
        GreekWord::from_bytes(&str.as_bytes().to_vec())
    }

    pub fn from_str(str: &str) -> Result<Self, InvalidCharError> {
        GreekWord::from_bytes(&str.as_bytes().to_vec())
    }

    pub fn to_upper(&self) -> GreekWord {
        let mut new_chars: Vec<GreekChar> = Vec::new();
        for char in &self.vec {
            new_chars.push(char.to_upper())
        }
        GreekWord::from_greek_chars(&new_chars)
    }

    pub fn to_lower(&self) -> GreekWord {
        let mut new_chars: Vec<GreekChar> = Vec::new();
        for (index, char) in self.vec.iter().enumerate() {
            let char = char.to_lower();
            if (char == LOWER_SIGMA) && (index == &self.vec.len() - 1) {
                new_chars.push(LOWER_SIGMA_FINAL)
            }
            else {
                new_chars.push(char)
            }
        }
        GreekWord::from_greek_chars(&new_chars)
    }

    pub fn strip_diacritics(&self) -> GreekWord {
        let mut new_chars: Vec<GreekChar> = Vec::new();
        for char in &self.vec {
            new_chars.push(char.stripped())
        }
        GreekWord::from_greek_chars(&new_chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_word() {
        let lower_logos = GreekWord::from_str("λογος").expect("Unable to parse λογος");
        assert_eq!(lower_logos, lower_logos);
    }

    #[test]
    fn greek_word_to_upper() {
        let lower_logos = GreekWord::from_str("λογος").expect("Unable to parse λογος");
        let upper_logos = GreekWord::from_str("ΛΟΓΟΣ").expect("Unable to parse ΛΟΓΟΣ");
        assert_eq!(lower_logos.to_upper(), upper_logos);
    }

    #[test]
    fn greek_word_with_diacritics_to_upper() {
        let lower_logos = GreekWord::from_str("λόγος").expect("Unable to parse λόγος");
        let upper_logos = GreekWord::from_str("ΛΌΓΟΣ").expect("Unable to parse ΛΟΓΟΣ");
        assert_eq!(lower_logos.to_upper(), upper_logos);
    }

    #[test]
    fn greek_word_to_lower() {
        let upper_logos = GreekWord::from_str("ΛΟΓΟΣ").expect("Unable to parse ΛΟΓΟΣ");
        let lower_logos = GreekWord::from_str("λογος").expect("Unable to parse λογος");
        assert_eq!(upper_logos.to_lower(), lower_logos);
    }

    #[test]
    fn strip_diacritics_succeeds() {
        let logos = GreekWord::from_str("λόγος").expect("Unable to parse λόγος");
        let stripped_logos = GreekWord::from_str("λογος").expect("Unable to parse λογος");
        assert_eq!(logos.strip_diacritics(), stripped_logos);
    }
}
