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
        if bytes.len() % 2 != 0 { return Err(InvalidCharError) }
        let mut chars: Vec<GreekChar> = Vec::new();

        let mut byte_stack: [u8; 2] = [0, 0];
        let mut first_byte = true;

        for byte in bytes {
            if first_byte {
                byte_stack[0] = *byte;
            }
            else {
                byte_stack[1] = *byte;
                match GreekChar::try_new(byte_stack) {
                    Ok(greek_char) => chars.push(greek_char),
                    Err(e) => return Err(e)
                }
            }
            first_byte = !first_byte;
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

    pub fn to_upper(&self) -> Result<GreekWord, InvalidCharError> {
        let mut new_chars: Vec<GreekChar> = Vec::new();
        for char in &self.vec {
            match char.to_upper() {
                Ok(char) => new_chars.push(char),
                Err(e) => return Err(e)
            }
        }
        Ok(GreekWord::from_greek_chars(&new_chars))
    }

    pub fn to_lower(&self) -> Result<GreekWord, InvalidCharError> {
        let mut new_chars: Vec<GreekChar> = Vec::new();
        for (index, char) in self.vec.iter().enumerate() {
            match char.to_lower() {
                Ok(char) => {
                    if (char == LOWER_SIGMA) && (index == &self.vec.len() - 1) {
                        new_chars.push(LOWER_SIGMA_FINAL)
                    }
                    else {
                        new_chars.push(char)
                    }
                },
                Err(e) => return Err(e)
            }
        }
        Ok(GreekWord::from_greek_chars(&new_chars))
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
        assert_eq!(lower_logos.to_upper().unwrap(), upper_logos);
    }

    #[test]
    fn greek_word_to_lower() {
        let upper_logos = GreekWord::from_str("ΛΟΓΟΣ").expect("Unable to parse ΛΟΓΟΣ");
        let lower_logos = GreekWord::from_str("λογος").expect("Unable to parse λογος");
        assert_eq!(upper_logos.to_lower().unwrap(), lower_logos);
    }
}
