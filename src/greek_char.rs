#![allow(dead_code)]

#[derive(Debug)]
pub struct InvalidCharError;

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

pub const UPPER_ALPHA: GreekChar = GreekChar::TwoByte(0xCE, 0x91);
pub const UPPER_BETA: GreekChar = GreekChar::TwoByte(0xCE, 0x92);
pub const UPPER_GAMMA: GreekChar = GreekChar::TwoByte(0xCE, 0x93);
pub const UPPER_DELTA: GreekChar = GreekChar::TwoByte(0xCE, 0x94);
pub const UPPER_EPSILON: GreekChar = GreekChar::TwoByte(0xCE, 0x95);
pub const UPPER_ZETA: GreekChar = GreekChar::TwoByte(0xCE, 0x96);
pub const UPPER_ETA: GreekChar = GreekChar::TwoByte(0xCE, 0x97);
pub const UPPER_THETA: GreekChar = GreekChar::TwoByte(0xCE, 0x98);
pub const UPPER_IOTA: GreekChar = GreekChar::TwoByte(0xCE, 0x99);
pub const UPPER_KAPPA: GreekChar = GreekChar::TwoByte(0xCE, 0x9A);
pub const UPPER_LAMBDA: GreekChar = GreekChar::TwoByte(0xCE, 0x9B);
pub const UPPER_MU: GreekChar = GreekChar::TwoByte(0xCE, 0x9C);
pub const UPPER_NU: GreekChar = GreekChar::TwoByte(0xCE, 0x9D);
pub const UPPER_XI: GreekChar = GreekChar::TwoByte(0xCE, 0x9E);
pub const UPPER_OMICRON: GreekChar = GreekChar::TwoByte(0xCE, 0x9F);
pub const UPPER_PI: GreekChar = GreekChar::TwoByte(0xCE, 0xA0);
pub const UPPER_RHO: GreekChar = GreekChar::TwoByte(0xCE, 0xA1);
pub const UPPER_SIGMA: GreekChar = GreekChar::TwoByte(0xCE, 0xA3);
pub const UPPER_TAU: GreekChar = GreekChar::TwoByte(0xCE, 0xA4);
pub const UPPER_UPSILON: GreekChar = GreekChar::TwoByte(0xCE, 0xA5);
pub const UPPER_PHI: GreekChar = GreekChar::TwoByte(0xCE, 0xA6);
pub const UPPER_CHI: GreekChar = GreekChar::TwoByte(0xCE, 0xA7);
pub const UPPER_PSI: GreekChar = GreekChar::TwoByte(0xCE, 0xA8);
pub const UPPER_OMEGA: GreekChar = GreekChar::TwoByte(0xCE, 0xA9);

pub const UPPER_ALPHA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x86);
pub const UPPER_EPSILON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x88);
pub const UPPER_ETA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x89);
pub const UPPER_IOTA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x8A);
pub const UPPER_OMICRON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x8C);
pub const UPPER_UPSILON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x8E);
pub const UPPER_OMEGA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x8F);

pub const UPPER_IOTA_WITH_DIALYTIKA: GreekChar = GreekChar::TwoByte(0xCE, 0xAA);
pub const UPPER_UPSILON_WITH_DIALYTIKA: GreekChar = GreekChar::TwoByte(0xCE, 0xAB);

pub const LOWER_ALPHA: GreekChar = GreekChar::TwoByte(0xCE, 0xB1);
pub const LOWER_BETA: GreekChar = GreekChar::TwoByte(0xCE, 0xB2);
pub const LOWER_GAMMA: GreekChar = GreekChar::TwoByte(0xCE, 0xB3);
pub const LOWER_DELTA: GreekChar = GreekChar::TwoByte(0xCE, 0xB4);
pub const LOWER_EPSILON: GreekChar = GreekChar::TwoByte(0xCE, 0xB5);
pub const LOWER_ZETA: GreekChar = GreekChar::TwoByte(0xCE, 0xB6);
pub const LOWER_ETA: GreekChar = GreekChar::TwoByte(0xCE, 0xB7);
pub const LOWER_THETA: GreekChar = GreekChar::TwoByte(0xCE, 0xB8);
pub const LOWER_IOTA: GreekChar = GreekChar::TwoByte(0xCE, 0xB9);
pub const LOWER_KAPPA: GreekChar = GreekChar::TwoByte(0xCE, 0xBA);
pub const LOWER_LAMBDA: GreekChar = GreekChar::TwoByte(0xCE, 0xBB);
pub const LOWER_MU: GreekChar = GreekChar::TwoByte(0xCE, 0xBC);
pub const LOWER_NU: GreekChar = GreekChar::TwoByte(0xCE, 0xBD);
pub const LOWER_XI: GreekChar = GreekChar::TwoByte(0xCE, 0xBE);
pub const LOWER_OMICRON: GreekChar = GreekChar::TwoByte(0xCE, 0xBF);
pub const LOWER_PI: GreekChar = GreekChar::TwoByte(0xCF, 0x80);
pub const LOWER_RHO: GreekChar = GreekChar::TwoByte(0xCF, 0x81);
pub const LOWER_SIGMA_FINAL: GreekChar = GreekChar::TwoByte(0xCF, 0x82);
pub const LOWER_SIGMA: GreekChar = GreekChar::TwoByte(0xCF, 0x83);
pub const LOWER_TAU: GreekChar = GreekChar::TwoByte(0xCF, 0x84);
pub const LOWER_UPSILON: GreekChar = GreekChar::TwoByte(0xCF, 0x85);
pub const LOWER_PHI: GreekChar = GreekChar::TwoByte(0xCF, 0x86);
pub const LOWER_CHI: GreekChar = GreekChar::TwoByte(0xCF, 0x87);
pub const LOWER_PSI: GreekChar = GreekChar::TwoByte(0xCF, 0x88);
pub const LOWER_OMEGA: GreekChar = GreekChar::TwoByte(0xCF, 0x89);

pub const LOWER_ALPHA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0xAC);
pub const LOWER_EPSILON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0xAD);
pub const LOWER_ETA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0xAE);
pub const LOWER_IOTA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0xAF);
pub const LOWER_OMICRON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCF, 0x8C);
pub const LOWER_UPSILON_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCF, 0x8D);
pub const LOWER_OMEGA_WITH_TONOS: GreekChar = GreekChar::TwoByte(0xCF, 0x8E);

pub const LOWER_IOTA_WITH_DIALYTIKA: GreekChar = GreekChar::TwoByte(0xCF, 0x8A);
pub const LOWER_UPSILON_WITH_DIALYTIKA: GreekChar = GreekChar::TwoByte(0xCF, 0x8B);

pub const LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0x90);
pub const LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS: GreekChar = GreekChar::TwoByte(0xCE, 0xB0);

#[cfg(test)]
mod tests {
    use super::*;

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
}
