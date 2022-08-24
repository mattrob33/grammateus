use crate::chars::*;

const GREEK_UPPERS: [GreekChar; 24] = [UPPER_ALPHA, UPPER_BETA, UPPER_GAMMA, UPPER_DELTA, UPPER_EPSILON, UPPER_ZETA, UPPER_ETA, UPPER_THETA, UPPER_IOTA, UPPER_KAPPA, UPPER_LAMBDA, UPPER_MU, UPPER_NU, UPPER_XI, UPPER_OMICRON, UPPER_PI, UPPER_RHO, UPPER_SIGMA, UPPER_TAU, UPPER_UPSILON, UPPER_PHI, UPPER_CHI, UPPER_PSI, UPPER_OMEGA ];
const GREEK_LOWERS: [GreekChar; 25] = [LOWER_ALPHA, LOWER_BETA, LOWER_GAMMA, LOWER_DELTA, LOWER_EPSILON, LOWER_ZETA, LOWER_ETA, LOWER_THETA, LOWER_IOTA, LOWER_KAPPA, LOWER_LAMBDA, LOWER_MU, LOWER_NU, LOWER_XI, LOWER_OMICRON, LOWER_PI, LOWER_RHO, LOWER_SIGMA_FINAL, LOWER_SIGMA, LOWER_TAU, LOWER_UPSILON, LOWER_PHI, LOWER_CHI, LOWER_PSI, LOWER_OMEGA ];

#[test]
fn uppercase_letters_are_uppercase() {
    for char in GREEK_UPPERS {
        match char {
            GreekChar::TwoByte(first_byte, second_byte) => {
                assert!(is_uppercase_greek(&char), "[{}, {}]", first_byte, second_byte)
            },
            _ => panic!("Invalid char")
        }
    }
}

#[test]
fn reserved_block_char_is_not_uppercase() {
    let reserved_char = GreekChar::TwoByte(0xCE, 0xA2); // unicode \03A2
    assert!(!is_uppercase_greek(&reserved_char), "failed for reserved char");
}

#[test]
fn lowercase_letters_are_not_uppercase() {
    for char in GREEK_LOWERS {
        match char {
            GreekChar::TwoByte(first_byte, second_byte) => {
                assert!(!is_uppercase_greek(&char), "[{}, {}]", first_byte, second_byte)
            },
            _ => panic!("Invalid char")
        }
    }
}

#[test]
fn lowercase_letters_are_lowercase() {
    for char in GREEK_LOWERS {
        match char {
            GreekChar::TwoByte(first_byte, second_byte) => {
                assert!(is_lowercase_greek(&char), "[{}, {}]", first_byte, second_byte)
            },
            _ => panic!("Invalid char")
        }
    }
}

#[test]
fn reserved_block_char_is_not_lowercase() {
    let reserved_char = GreekChar::TwoByte(0xCE, 0xA2); // unicode \03A2
    assert!(!is_lowercase_greek(&reserved_char), "failed for reserved char");
}

#[test]
fn uppercase_letters_are_not_lowercase() {
    for char in GREEK_UPPERS {
        match char {
            GreekChar::TwoByte(first_byte, second_byte) => {
                assert!(!is_lowercase_greek(&char), "[{}, {}]", first_byte, second_byte)
            },
            _ => panic!("Invalid char")
        }
    }
}