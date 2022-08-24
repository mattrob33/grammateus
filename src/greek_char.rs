#![allow(dead_code)]

#[derive(Debug)]
pub struct InvalidCharError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreekChar([u8; 2]);

impl GreekChar {
    pub fn try_new(bytes: [u8; 2]) -> Result<Self, InvalidCharError> {
        return match bytes[0] {
            0xCE => {
                if bytes[1] < 0x91 {
                    Err(InvalidCharError) // below minimum range
                } else if bytes[1] > 0xBF {
                    Err(InvalidCharError) // above maximum range
                } else if bytes[1] == 0xA2 {
                    Err(InvalidCharError) // this is a reserved char
                } else {
                    Ok(GreekChar(bytes)) // valid Greek char
                }
            }

            0xCF => {
                if bytes[1] < 0x80 {
                    Err(InvalidCharError) // below minimum range
                }
                else if bytes[1] > 0x89 {
                    Err(InvalidCharError) // above maximum range
                }
                else {
                    Ok(GreekChar(bytes)) // valid Greek char
                }
            }

            _ => Err(InvalidCharError)
        }
    }

    pub fn is_uppercase(&self) -> bool {
        is_uppercase_greek(self)
    }

    pub fn is_lowercase(&self) -> bool {
        is_lowercase_greek(self)
    }

    pub fn to_upper(&self) -> Result<GreekChar, InvalidCharError> {
        let upper: GreekChar = match self {
            &LOWER_ALPHA => UPPER_ALPHA,
            &LOWER_BETA => UPPER_BETA,
            &LOWER_GAMMA => UPPER_GAMMA,
            &LOWER_DELTA => UPPER_DELTA,
            &LOWER_EPSILON => UPPER_EPSILON,
            &LOWER_ZETA => UPPER_ZETA,
            &LOWER_ETA => UPPER_ETA,
            &LOWER_THETA => UPPER_THETA,
            &LOWER_IOTA => UPPER_IOTA,
            &LOWER_KAPPA => UPPER_KAPPA,
            &LOWER_LAMBDA => UPPER_LAMBDA,
            &LOWER_MU => UPPER_MU,
            &LOWER_NU => UPPER_NU,
            &LOWER_XI => UPPER_XI,
            &LOWER_OMICRON => UPPER_OMICRON,
            &LOWER_PI => UPPER_PI,
            &LOWER_RHO => UPPER_RHO,
            &LOWER_SIGMA_FINAL => UPPER_SIGMA,
            &LOWER_SIGMA => UPPER_SIGMA,
            &LOWER_TAU => UPPER_TAU,
            &LOWER_UPSILON => UPPER_UPSILON,
            &LOWER_PHI => UPPER_PHI,
            &LOWER_CHI => UPPER_CHI,
            &LOWER_PSI => UPPER_PSI,
            &LOWER_OMEGA => UPPER_OMEGA,

            &UPPER_ALPHA => UPPER_ALPHA,
            &UPPER_BETA => UPPER_BETA,
            &UPPER_GAMMA => UPPER_GAMMA,
            &UPPER_DELTA => UPPER_DELTA,
            &UPPER_EPSILON => UPPER_EPSILON,
            &UPPER_ZETA => UPPER_ZETA,
            &UPPER_ETA => UPPER_ETA,
            &UPPER_THETA => UPPER_THETA,
            &UPPER_IOTA => UPPER_IOTA,
            &UPPER_KAPPA => UPPER_KAPPA,
            &UPPER_LAMBDA => UPPER_LAMBDA,
            &UPPER_MU => UPPER_MU,
            &UPPER_NU => UPPER_NU,
            &UPPER_XI => UPPER_XI,
            &UPPER_OMICRON => UPPER_OMICRON,
            &UPPER_PI => UPPER_PI,
            &UPPER_RHO => UPPER_RHO,
            &UPPER_SIGMA => UPPER_SIGMA,
            &UPPER_TAU => UPPER_TAU,
            &UPPER_UPSILON => UPPER_UPSILON,
            &UPPER_PHI => UPPER_PHI,
            &UPPER_CHI => UPPER_CHI,
            &UPPER_PSI => UPPER_PSI,
            &UPPER_OMEGA => UPPER_OMEGA,

            _ => return Err(InvalidCharError)
        };
        Ok(upper)
    }

    pub fn to_lower(&self) -> Result<GreekChar, InvalidCharError> {
        let lower: GreekChar = match self {
            &UPPER_ALPHA => LOWER_ALPHA,
            &UPPER_BETA => LOWER_BETA,
            &UPPER_GAMMA => LOWER_GAMMA,
            &UPPER_DELTA => LOWER_DELTA,
            &UPPER_EPSILON => LOWER_EPSILON,
            &UPPER_ZETA => LOWER_ZETA,
            &UPPER_ETA => LOWER_ETA,
            &UPPER_THETA => LOWER_THETA,
            &UPPER_IOTA => LOWER_IOTA,
            &UPPER_KAPPA => LOWER_KAPPA,
            &UPPER_LAMBDA => LOWER_LAMBDA,
            &UPPER_MU => LOWER_MU,
            &UPPER_NU => LOWER_NU,
            &UPPER_XI => LOWER_XI,
            &UPPER_OMICRON => LOWER_OMICRON,
            &UPPER_PI => LOWER_PI,
            &UPPER_RHO => LOWER_RHO,
            &UPPER_SIGMA => LOWER_SIGMA,
            &UPPER_TAU => LOWER_TAU,
            &UPPER_UPSILON => LOWER_UPSILON,
            &UPPER_PHI => LOWER_PHI,
            &UPPER_CHI => LOWER_CHI,
            &UPPER_PSI => LOWER_PSI,
            &UPPER_OMEGA => LOWER_OMEGA,

            &LOWER_ALPHA => LOWER_ALPHA,
            &LOWER_BETA => LOWER_BETA,
            &LOWER_GAMMA => LOWER_GAMMA,
            &LOWER_DELTA => LOWER_DELTA,
            &LOWER_EPSILON => LOWER_EPSILON,
            &LOWER_ZETA => LOWER_ZETA,
            &LOWER_ETA => LOWER_ETA,
            &LOWER_THETA => LOWER_THETA,
            &LOWER_IOTA => LOWER_IOTA,
            &LOWER_KAPPA => LOWER_KAPPA,
            &LOWER_LAMBDA => LOWER_LAMBDA,
            &LOWER_MU => LOWER_MU,
            &LOWER_NU => LOWER_NU,
            &LOWER_XI => LOWER_XI,
            &LOWER_OMICRON => LOWER_OMICRON,
            &LOWER_PI => LOWER_PI,
            &LOWER_RHO => LOWER_RHO,
            &LOWER_SIGMA_FINAL => LOWER_SIGMA_FINAL,
            &LOWER_SIGMA => LOWER_SIGMA,
            &LOWER_TAU => LOWER_TAU,
            &LOWER_UPSILON => LOWER_UPSILON,
            &LOWER_PHI => LOWER_PHI,
            &LOWER_CHI => LOWER_CHI,
            &LOWER_PSI => LOWER_PSI,
            &LOWER_OMEGA => LOWER_OMEGA,

            _ => return Err(InvalidCharError)
        };
        Ok(lower)
    }
}

pub fn is_lowercase_greek(char: &GreekChar) -> bool {
    return match char.0.len() {
        2 => {
            if char.0[0] == 0xCE {
                return (char.0[1] >= 0xB1) && (char.0[1] <= 0xBF)
            }
            else if char.0[0] == 0xCF {
                return (char.0[1] >= 0x80) && (char.0[1] <= 0x89)
            }
            false
        }
        _ => false
    }
}

pub fn is_uppercase_greek(char: &GreekChar) -> bool {
    return match char.0.len() {
        2 => {
            if char.0[0] == 0xCE {
                if (char.0[1] >= 0x91) && (char.0[1] <= 0xA9) {
                    return char.0[1] != 0xA2; // unicode \03A2 is reserved
                }
            }
            false
        }
        _ => false
    };
}

pub const UPPER_ALPHA: GreekChar = GreekChar([0xCE, 0x91]);
pub const UPPER_BETA: GreekChar = GreekChar([0xCE, 0x92]);
pub const UPPER_GAMMA: GreekChar = GreekChar([0xCE, 0x93]);
pub const UPPER_DELTA: GreekChar = GreekChar([0xCE, 0x94]);
pub const UPPER_EPSILON: GreekChar = GreekChar([0xCE, 0x95]);
pub const UPPER_ZETA: GreekChar = GreekChar([0xCE, 0x96]);
pub const UPPER_ETA: GreekChar = GreekChar([0xCE, 0x97]);
pub const UPPER_THETA: GreekChar = GreekChar([0xCE, 0x98]);
pub const UPPER_IOTA: GreekChar = GreekChar([0xCE, 0x99]);
pub const UPPER_KAPPA: GreekChar = GreekChar([0xCE, 0x9A]);
pub const UPPER_LAMBDA: GreekChar = GreekChar([0xCE, 0x9B]);
pub const UPPER_MU: GreekChar = GreekChar([0xCE, 0x9C]);
pub const UPPER_NU: GreekChar = GreekChar([0xCE, 0x9D]);
pub const UPPER_XI: GreekChar = GreekChar([0xCE, 0x9E]);
pub const UPPER_OMICRON: GreekChar = GreekChar([0xCE, 0x9F]);
pub const UPPER_PI: GreekChar = GreekChar([0xCE, 0xA0]);
pub const UPPER_RHO: GreekChar = GreekChar([0xCE, 0xA1]);
pub const UPPER_SIGMA: GreekChar = GreekChar([0xCE, 0xA3]);
pub const UPPER_TAU: GreekChar = GreekChar([0xCE, 0xA4]);
pub const UPPER_UPSILON: GreekChar = GreekChar([0xCE, 0xA5]);
pub const UPPER_PHI: GreekChar = GreekChar([0xCE, 0xA6]);
pub const UPPER_CHI: GreekChar = GreekChar([0xCE, 0xA7]);
pub const UPPER_PSI: GreekChar = GreekChar([0xCE, 0xA8]);
pub const UPPER_OMEGA: GreekChar = GreekChar([0xCE, 0xA9]);

pub const LOWER_ALPHA: GreekChar = GreekChar([0xCE, 0xB1]);
pub const LOWER_BETA: GreekChar = GreekChar([0xCE, 0xB2]);
pub const LOWER_GAMMA: GreekChar = GreekChar([0xCE, 0xB3]);
pub const LOWER_DELTA: GreekChar = GreekChar([0xCE, 0xB4]);
pub const LOWER_EPSILON: GreekChar = GreekChar([0xCE, 0xB5]);
pub const LOWER_ZETA: GreekChar = GreekChar([0xCE, 0xB6]);
pub const LOWER_ETA: GreekChar = GreekChar([0xCE, 0xB7]);
pub const LOWER_THETA: GreekChar = GreekChar([0xCE, 0xB8]);
pub const LOWER_IOTA: GreekChar = GreekChar([0xCE, 0xB9]);
pub const LOWER_KAPPA: GreekChar = GreekChar([0xCE, 0xBA]);
pub const LOWER_LAMBDA: GreekChar = GreekChar([0xCE, 0xBB]);
pub const LOWER_MU: GreekChar = GreekChar([0xCE, 0xBC]);
pub const LOWER_NU: GreekChar = GreekChar([0xCE, 0xBD]);
pub const LOWER_XI: GreekChar = GreekChar([0xCE, 0xBE]);
pub const LOWER_OMICRON: GreekChar = GreekChar([0xCE, 0xBF]);
pub const LOWER_PI: GreekChar = GreekChar([0xCF, 0x80]);
pub const LOWER_RHO: GreekChar = GreekChar([0xCF, 0x81]);
pub const LOWER_SIGMA_FINAL: GreekChar = GreekChar([0xCF, 0x82]);
pub const LOWER_SIGMA: GreekChar = GreekChar([0xCF, 0x83]);
pub const LOWER_TAU: GreekChar = GreekChar([0xCF, 0x84]);
pub const LOWER_UPSILON: GreekChar = GreekChar([0xCF, 0x85]);
pub const LOWER_PHI: GreekChar = GreekChar([0xCF, 0x86]);
pub const LOWER_CHI: GreekChar = GreekChar([0xCF, 0x87]);
pub const LOWER_PSI: GreekChar = GreekChar([0xCF, 0x88]);
pub const LOWER_OMEGA: GreekChar = GreekChar([0xCF, 0x89]);

#[cfg(test)]
mod tests {
    use super::*;

    const GREEK_UPPERS: [GreekChar; 24] = [UPPER_ALPHA, UPPER_BETA, UPPER_GAMMA, UPPER_DELTA, UPPER_EPSILON, UPPER_ZETA, UPPER_ETA, UPPER_THETA, UPPER_IOTA, UPPER_KAPPA, UPPER_LAMBDA, UPPER_MU, UPPER_NU, UPPER_XI, UPPER_OMICRON, UPPER_PI, UPPER_RHO, UPPER_SIGMA, UPPER_TAU, UPPER_UPSILON, UPPER_PHI, UPPER_CHI, UPPER_PSI, UPPER_OMEGA ];
    const GREEK_LOWERS: [GreekChar; 25] = [LOWER_ALPHA, LOWER_BETA, LOWER_GAMMA, LOWER_DELTA, LOWER_EPSILON, LOWER_ZETA, LOWER_ETA, LOWER_THETA, LOWER_IOTA, LOWER_KAPPA, LOWER_LAMBDA, LOWER_MU, LOWER_NU, LOWER_XI, LOWER_OMICRON, LOWER_PI, LOWER_RHO, LOWER_SIGMA_FINAL, LOWER_SIGMA, LOWER_TAU, LOWER_UPSILON, LOWER_PHI, LOWER_CHI, LOWER_PSI, LOWER_OMEGA ];

    #[test]
    fn uppercase_letters_are_uppercase() {
        for char in GREEK_UPPERS {
            assert!(is_uppercase_greek(&char), "[{}, {}]", &char.0[0], &char.0[1]);
        }
    }

    #[test]
    fn reserved_block_char_is_not_uppercase() {
        let reserved_char = GreekChar([0xCE, 0xA2]); // unicode \03A2
        assert!(!is_uppercase_greek(&reserved_char), "failed for reserved char");
    }

    #[test]
    fn lowercase_letters_are_not_uppercase() {
        for char in GREEK_LOWERS {
            assert!(!is_uppercase_greek(&char), "[{}, {}]", &char.0[0], &char.0[1]);
        }
    }

    #[test]
    fn lowercase_letters_are_lowercase() {
        for char in GREEK_LOWERS {
            assert!(is_lowercase_greek(&char), "[{}, {}]", &char.0[0], &char.0[1]);
        }
    }

    #[test]
    fn reserved_block_char_is_not_lowercase() {
        let reserved_char = GreekChar([0xCE, 0xA2]); // unicode \03A2
        assert!(!is_lowercase_greek(&reserved_char), "failed for reserved char");
    }

    #[test]
    fn uppercase_letters_are_not_lowercase() {
        for char in GREEK_UPPERS {
            assert!(!is_lowercase_greek(&char), "[{}, {}]", &char.0[0], &char.0[1]);
        }
    }

    #[test]
    fn uppercase_letters_can_be_lowered() {
        for char in GREEK_UPPERS {
            assert!(char.to_lower().is_ok(), "[0x{:02x}, 0x{:02x}]", &char.0[0], &char.0[1]);
        }
    }

    #[test]
    fn lowercase_letters_can_be_uppered() {
        for char in GREEK_LOWERS {
            assert!(char.to_upper().is_ok(), "[0x{:02x}, 0x{:02x}]", &char.0[0], &char.0[1]);
        }
    }
}
