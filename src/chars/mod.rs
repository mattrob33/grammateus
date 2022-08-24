#![allow(dead_code)]

use crate::chars::errors::InvalidCharError;
use crate::chars::literals::*;

pub mod errors;
pub mod literals;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GreekChar {
    TwoByte(u8, u8),
    ThreeByte(u8, u8, u8),
}

impl GreekChar {

    pub fn try_new(bytes: &[u8]) -> Result<Self, InvalidCharError> {
        return match bytes.len() {
            2 => {
                match bytes[0] {
                    0xCE => {
                        if bytes[1] < 0x86 {
                            Err(InvalidCharError) // below minimum range
                        } else if bytes[1] > 0xBF {
                            Err(InvalidCharError) // above maximum range
                        } else if bytes[1] == 0xA2 || bytes[1] == 0x87 || bytes[1] == 0x8B || bytes[1] == 0x8D {
                            Err(InvalidCharError) // these are reserved chars
                        } else {
                            Ok(GreekChar::TwoByte(bytes[0], bytes[1])) // valid Greek char
                        }
                    }

                    0xCF => {
                        if bytes[1] < 0x80 {
                            Err(InvalidCharError) // below minimum range
                        }
                        else if bytes[1] > 0x8E {
                            Err(InvalidCharError) // above maximum range
                        }
                        else {
                            Ok(GreekChar::TwoByte(bytes[0], bytes[1])) // valid Greek char
                        }
                    }

                    _ => Err(InvalidCharError)
                }
            },

            3 => {
                match bytes[0] {
                    0xE1 => {
                        match bytes[1] {
                            0xBC => {
                                match bytes[2] {
                                    0x96|0x97|0x9E|0x9F => Err(InvalidCharError), // reserved chars
                                    _ => {
                                        Ok(GreekChar::ThreeByte(bytes[0], bytes[1], bytes[2])) // valid Greek char
                                    }
                                }
                            },

                            0xBD => {
                                match bytes[2] {
                                    0x86|0x87|0x8E|0x8F|0x98|0x9A|0x9C|0x9E|0xBE|0xBF => Err(InvalidCharError), // reserved chars
                                    _ => {
                                        Ok(GreekChar::ThreeByte(bytes[0], bytes[1], bytes[2])) // valid Greek char
                                    }
                                }
                            },

                            0xBE => {
                                match bytes[2] {
                                    0xB5 => Err(InvalidCharError), // reserved char
                                    _ => {
                                        Ok(GreekChar::ThreeByte(bytes[0], bytes[1], bytes[2])) // valid Greek char
                                    }
                                }
                            },

                            0xBF => {
                                match bytes[2] {
                                    0x94|0x95|0x9C|0xB0|0xB1|0xB5|0xBF => Err(InvalidCharError), // reserved chars
                                    _ => {
                                        Ok(GreekChar::ThreeByte(bytes[0], bytes[1], bytes[2])) // valid Greek char
                                    }
                                }
                            },

                            _ => Err(InvalidCharError)
                        }
                    },

                    _ => Err(InvalidCharError)
                }
            },

            _ => Err(InvalidCharError)
        };
    }

    pub fn is_uppercase(&self) -> bool {
        is_uppercase_greek(self)
    }

    pub fn is_lowercase(&self) -> bool {
        is_lowercase_greek(self)
    }

    pub fn to_upper(&self) -> GreekChar {
        match self {
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

            &LOWER_ALPHA_WITH_TONOS => UPPER_ALPHA_WITH_TONOS,
            &LOWER_EPSILON_WITH_TONOS => UPPER_EPSILON_WITH_TONOS,
            &LOWER_ETA_WITH_TONOS => UPPER_ETA_WITH_TONOS,
            &LOWER_IOTA_WITH_TONOS => UPPER_IOTA_WITH_TONOS,
            &LOWER_OMICRON_WITH_TONOS => UPPER_OMICRON_WITH_TONOS,
            &LOWER_UPSILON_WITH_TONOS => UPPER_UPSILON_WITH_TONOS,
            &LOWER_OMEGA_WITH_TONOS => UPPER_OMEGA_WITH_TONOS,
            &LOWER_IOTA_WITH_DIALYTIKA => UPPER_IOTA_WITH_DIALYTIKA,
            &LOWER_UPSILON_WITH_DIALYTIKA => UPPER_UPSILON_WITH_DIALYTIKA,
            &LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS => LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS,
            &LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS => LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS,

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

            &UPPER_ALPHA_WITH_TONOS => UPPER_ALPHA_WITH_TONOS,
            &UPPER_EPSILON_WITH_TONOS => UPPER_EPSILON_WITH_TONOS,
            &UPPER_ETA_WITH_TONOS => UPPER_ETA_WITH_TONOS,
            &UPPER_IOTA_WITH_TONOS => UPPER_IOTA_WITH_TONOS,
            &UPPER_OMICRON_WITH_TONOS => UPPER_OMICRON_WITH_TONOS,
            &UPPER_UPSILON_WITH_TONOS => UPPER_UPSILON_WITH_TONOS,
            &UPPER_OMEGA_WITH_TONOS => UPPER_OMEGA_WITH_TONOS,

            &UPPER_IOTA_WITH_DIALYTIKA => UPPER_IOTA_WITH_DIALYTIKA,
            &UPPER_UPSILON_WITH_DIALYTIKA => UPPER_UPSILON_WITH_DIALYTIKA,

            _ => panic!("Unable to convert char to upper")
        }
    }

    pub fn to_lower(&self) -> GreekChar {
        match self {
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

            &UPPER_ALPHA_WITH_TONOS => LOWER_ALPHA_WITH_TONOS,
            &UPPER_EPSILON_WITH_TONOS => LOWER_EPSILON_WITH_TONOS,
            &UPPER_ETA_WITH_TONOS => LOWER_ETA_WITH_TONOS,
            &UPPER_IOTA_WITH_TONOS => LOWER_IOTA_WITH_TONOS,
            &UPPER_OMICRON_WITH_TONOS => LOWER_OMICRON_WITH_TONOS,
            &UPPER_UPSILON_WITH_TONOS => LOWER_UPSILON_WITH_TONOS,
            &UPPER_OMEGA_WITH_TONOS => LOWER_OMEGA_WITH_TONOS,

            &UPPER_IOTA_WITH_DIALYTIKA => LOWER_IOTA_WITH_DIALYTIKA,
            &UPPER_UPSILON_WITH_DIALYTIKA => LOWER_UPSILON_WITH_DIALYTIKA,

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

            &LOWER_ALPHA_WITH_TONOS => LOWER_ALPHA_WITH_TONOS,
            &LOWER_EPSILON_WITH_TONOS => LOWER_EPSILON_WITH_TONOS,
            &LOWER_ETA_WITH_TONOS => LOWER_ETA_WITH_TONOS,
            &LOWER_IOTA_WITH_TONOS => LOWER_IOTA_WITH_TONOS,
            &LOWER_OMICRON_WITH_TONOS => LOWER_OMICRON_WITH_TONOS,
            &LOWER_UPSILON_WITH_TONOS => LOWER_UPSILON_WITH_TONOS,
            &LOWER_OMEGA_WITH_TONOS => LOWER_OMEGA_WITH_TONOS,
            &LOWER_IOTA_WITH_DIALYTIKA => LOWER_IOTA_WITH_DIALYTIKA,
            &LOWER_UPSILON_WITH_DIALYTIKA => LOWER_UPSILON_WITH_DIALYTIKA,
            &LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS => LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS,
            &LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS => LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS,

            _ => panic!("Unable to convert char to lower")
        }
    }

    /// Returns this `GreekChar` with any diacritics stripped
    pub fn stripped(&self) -> GreekChar {
        strip_diacritics(&self).clone()
    }
}

pub fn is_lowercase_greek(char: &GreekChar) -> bool {
    return match char {
        GreekChar::TwoByte(first_byte, second_byte) => {
            match first_byte {
                0xCE => {
                    return (*second_byte >= 0xB1) && (*second_byte <= 0xBF)
                },
                0xCF => {
                    return (*second_byte >= 0x80) && (*second_byte <= 0x89)
                },
                _ => false
            }
        },

        GreekChar::ThreeByte(_, _, _) => panic!("Not yet implented"), // TODO
    }
}

pub fn is_uppercase_greek(char: &GreekChar) -> bool {
    return match char {
        GreekChar::TwoByte(first_byte, second_byte) => {
            if *first_byte == 0xCE {
                if (*second_byte >= 0x91) && (*second_byte <= 0xA9) {
                    return *second_byte != 0xA2; // unicode \03A2 is reserved
                }
            }
            false
        }

        GreekChar::ThreeByte(_, _, _) => panic!("Not yet implented"), // TODO
    };
}

pub fn strip_diacritics(char: &GreekChar) -> &GreekChar {

    if is_consonant(char) { return char }

    match char {
        GreekChar::TwoByte(_, _) => {
            match char {
                &UPPER_ALPHA|&UPPER_ALPHA_WITH_TONOS => &UPPER_ALPHA,
                &UPPER_EPSILON|&UPPER_EPSILON_WITH_TONOS => &UPPER_EPSILON,
                &UPPER_ETA|&UPPER_ETA_WITH_TONOS => &UPPER_ETA,
                &UPPER_IOTA|&UPPER_IOTA_WITH_TONOS|&UPPER_IOTA_WITH_DIALYTIKA => &UPPER_IOTA,
                &UPPER_OMICRON|&UPPER_OMICRON_WITH_TONOS => &UPPER_OMICRON,
                &UPPER_UPSILON|&UPPER_UPSILON_WITH_TONOS|&UPPER_UPSILON_WITH_DIALYTIKA => &UPPER_UPSILON,
                &UPPER_OMEGA|&UPPER_OMEGA_WITH_TONOS => &UPPER_OMEGA,

                &LOWER_ALPHA|&LOWER_ALPHA_WITH_TONOS => &LOWER_ALPHA,
                &LOWER_EPSILON|&LOWER_EPSILON_WITH_TONOS => &LOWER_EPSILON,
                &LOWER_ETA|&LOWER_ETA_WITH_TONOS => &LOWER_ETA,
                &LOWER_IOTA|&LOWER_IOTA_WITH_TONOS|&LOWER_IOTA_WITH_DIALYTIKA|&LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS => &LOWER_IOTA,
                &LOWER_OMICRON|&LOWER_OMICRON_WITH_TONOS => &LOWER_OMICRON,
                &LOWER_UPSILON|&LOWER_UPSILON_WITH_TONOS|&LOWER_UPSILON_WITH_DIALYTIKA|&LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS => &LOWER_UPSILON,
                &LOWER_OMEGA|&LOWER_OMEGA_WITH_TONOS => &LOWER_OMEGA,
                _ => panic!("Not yet supported - please file an issue")
            }
        },
        GreekChar::ThreeByte(_, second_byte, third_byte) => {
            match second_byte {
                0xBC => {
                    match third_byte {
                        0x80..=0x87 => &LOWER_ALPHA,
                        0x88..=0x8F => &UPPER_ALPHA,
                        0x90..=0x95 => &LOWER_EPSILON,
                        0x98..=0x9D => &UPPER_EPSILON,
                        0xA0..=0xA7 => &LOWER_ETA,
                        0xA8..=0xAF => &UPPER_ETA,
                        0xB0..=0xB7 => &LOWER_IOTA,
                        0xB8..=0xBF => &UPPER_IOTA,
                        _ => panic!("Not yet supported - please file an issue")
                    }
                },
                0xBD => {
                    match third_byte {
                        0x80..=0x85 => &LOWER_OMICRON,
                        0x88..=0x8D => &UPPER_OMICRON,
                        0x90..=0x97 => &LOWER_UPSILON,
                        0x99|0x9B|0x9D|0x9F => &UPPER_UPSILON,
                        0xA0..=0xA7 => &LOWER_OMEGA,
                        0xA8..=0xAF => &UPPER_OMEGA,
                        _ => panic!("Not yet supported - please file an issue")
                    }
                },
                _ => panic!("Invalid char")
            }
        }
    }
}

fn is_consonant(char: &GreekChar) -> bool {
    match char {
        &UPPER_BETA|&UPPER_GAMMA|&UPPER_DELTA|&UPPER_ZETA|&UPPER_THETA|&UPPER_KAPPA|&UPPER_LAMBDA|&UPPER_MU|&UPPER_NU|&UPPER_XI|&UPPER_PI|&UPPER_RHO|&UPPER_SIGMA|&UPPER_TAU|&UPPER_PHI|&UPPER_CHI|&UPPER_PSI|
        &LOWER_BETA|&LOWER_GAMMA|&LOWER_DELTA|&LOWER_ZETA|&LOWER_THETA|&LOWER_KAPPA|&LOWER_LAMBDA|&LOWER_MU|&LOWER_NU|&LOWER_XI|&LOWER_PI|&LOWER_RHO|&LOWER_SIGMA|&LOWER_SIGMA_FINAL|&LOWER_TAU|&LOWER_PHI|&LOWER_CHI|&LOWER_PSI
        => true,
        _ => false
    }
}