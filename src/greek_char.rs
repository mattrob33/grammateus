#![allow(dead_code)]

#[derive(Debug)]
pub struct InvalidCharError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreekChar {
    bytes: [u8; 3],
    len: u8
}

impl GreekChar {

    pub fn bytes(&self) -> &[u8] {
        return if self.len == 3 {
            &self.bytes
        } else {
            &self.bytes[0..2]
        }
    }

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
                            let bytes: [u8; 3] = [
                                bytes[0],
                                bytes[1],
                                0
                            ];
                            Ok(GreekChar {bytes, len: 2}) // valid Greek char
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
                            let bytes: [u8; 3] = [
                                bytes[0],
                                bytes[1],
                                0
                            ];
                            Ok(GreekChar {bytes, len: 2}) // valid Greek char
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
                                        let bytes: [u8; 3] = [
                                            bytes[0],
                                            bytes[1],
                                            bytes[2]
                                        ];
                                        Ok(GreekChar {bytes, len: 3}) // valid Greek char
                                    }
                                }
                            },

                            0xBD => {
                                match bytes[2] {
                                    0x86|0x87|0x8E|0x8F|0x98|0x9A|0x9C|0x9E|0xBE|0xBF => Err(InvalidCharError), // reserved chars
                                    _ => {
                                        let bytes: [u8; 3] = [
                                            bytes[0],
                                            bytes[1],
                                            bytes[2]
                                        ];
                                        Ok(GreekChar {bytes, len: 3}) // valid Greek char
                                    }
                                }
                            },

                            0xBE => {
                                match bytes[2] {
                                    0xB5 => Err(InvalidCharError), // reserved char
                                    _ => {
                                        let bytes: [u8; 3] = [
                                            bytes[0],
                                            bytes[1],
                                            bytes[2]
                                        ];
                                        Ok(GreekChar {bytes, len: 3}) // valid Greek char
                                    }
                                }
                            },

                            0xBF => {
                                match bytes[2] {
                                    0x94|0x95|0x9C|0xB0|0xB1|0xB5|0xBF => Err(InvalidCharError), // reserved chars
                                    _ => {
                                        let bytes: [u8; 3] = [
                                            bytes[0],
                                            bytes[1],
                                            bytes[2]
                                        ];
                                        Ok(GreekChar {bytes, len: 3}) // valid Greek char
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
    return match char.bytes().len() {
        2 => {
            if char.bytes()[0] == 0xCE {
                return (char.bytes()[1] >= 0xB1) && (char.bytes()[1] <= 0xBF)
            }
            else if char.bytes()[0] == 0xCF {
                return (char.bytes()[1] >= 0x80) && (char.bytes()[1] <= 0x89)
            }
            false
        },
        
        3 => panic!("Not yet implented"), // TODO
        
        _ => false
    }
}

pub fn is_uppercase_greek(char: &GreekChar) -> bool {
    return match char.bytes().len() {
        2 => {
            if char.bytes()[0] == 0xCE {
                if (char.bytes()[1] >= 0x91) && (char.bytes()[1] <= 0xA9) {
                    return char.bytes()[1] != 0xA2; // unicode \03A2 is reserved
                }
            }
            false
        }

        3 => panic!("Not yet implented"), // TODO

        _ => false
    };
}

pub fn strip_diacritics(char: &GreekChar) -> &GreekChar {

    if is_consonant(char) { return char }

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

        _ => {
            match char.bytes[1] {
                0xBC => {
                    match char.bytes[2] {
                        0x80..=0x87 => LOWER_ALPHA,
                        0x88..=0x8F => UPPER_ALPHA,
                        0x90..=0x95 => LOWER_EPSILON,
                        0x98..=0x9D => UPPER_EPSILON,
                        0xA0..=0xA7 => LOWER_ETA,
                        0xA8..=0xAF => UPPER_ETA,
                        0xB0..=0xB7 => LOWER_IOTA,
                        0xB8..=0xBF => UPPER_IOTA
                    }
                },
                0xBD => {
                    match char.bytes[2] {
                        0x80..=0x85 => LOWER_OMICRON,
                        0x88..=0x8D => UPPER_OMICRON,
                        0x90..=0x97 => LOWER_UPSILON,
                        0x99|0x9B|0x9D|0x9F => UPPER_UPSILON,
                        0xA0..=0xA7 => LOWER_OMEGA,
                        0xA8..=0xAF => UPPER_OMEGA
                    }
                },
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

pub const UPPER_ALPHA: GreekChar = GreekChar { bytes: [0xCE, 0x91, 0], len: 2 };
pub const UPPER_BETA: GreekChar = GreekChar { bytes: [0xCE, 0x92, 0], len: 2 };
pub const UPPER_GAMMA: GreekChar = GreekChar { bytes: [0xCE, 0x93, 0], len: 2 };
pub const UPPER_DELTA: GreekChar = GreekChar { bytes: [0xCE, 0x94, 0], len: 2 };
pub const UPPER_EPSILON: GreekChar = GreekChar { bytes: [0xCE, 0x95, 0], len: 2 };
pub const UPPER_ZETA: GreekChar = GreekChar { bytes: [0xCE, 0x96, 0], len: 2 };
pub const UPPER_ETA: GreekChar = GreekChar { bytes: [0xCE, 0x97, 0], len: 2 };
pub const UPPER_THETA: GreekChar = GreekChar { bytes: [0xCE, 0x98, 0], len: 2 };
pub const UPPER_IOTA: GreekChar = GreekChar { bytes: [0xCE, 0x99, 0], len: 2 };
pub const UPPER_KAPPA: GreekChar = GreekChar { bytes: [0xCE, 0x9A, 0], len: 2 };
pub const UPPER_LAMBDA: GreekChar = GreekChar { bytes: [0xCE, 0x9B, 0], len: 2 };
pub const UPPER_MU: GreekChar = GreekChar { bytes: [0xCE, 0x9C, 0], len: 2 };
pub const UPPER_NU: GreekChar = GreekChar { bytes: [0xCE, 0x9D, 0], len: 2 };
pub const UPPER_XI: GreekChar = GreekChar { bytes: [0xCE, 0x9E, 0], len: 2 };
pub const UPPER_OMICRON: GreekChar = GreekChar { bytes: [0xCE, 0x9F, 0], len: 2 };
pub const UPPER_PI: GreekChar = GreekChar { bytes: [0xCE, 0xA0, 0], len: 2 };
pub const UPPER_RHO: GreekChar = GreekChar { bytes: [0xCE, 0xA1, 0], len: 2 };
pub const UPPER_SIGMA: GreekChar = GreekChar { bytes: [0xCE, 0xA3, 0], len: 2 };
pub const UPPER_TAU: GreekChar = GreekChar { bytes: [0xCE, 0xA4, 0], len: 2 };
pub const UPPER_UPSILON: GreekChar = GreekChar { bytes: [0xCE, 0xA5, 0], len: 2 };
pub const UPPER_PHI: GreekChar = GreekChar { bytes: [0xCE, 0xA6, 0], len: 2 };
pub const UPPER_CHI: GreekChar = GreekChar { bytes: [0xCE, 0xA7, 0], len: 2 };
pub const UPPER_PSI: GreekChar = GreekChar { bytes: [0xCE, 0xA8, 0], len: 2 };
pub const UPPER_OMEGA: GreekChar = GreekChar { bytes: [0xCE, 0xA9, 0], len: 2 };

pub const UPPER_ALPHA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x86, 0], len: 2 };
pub const UPPER_EPSILON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x88, 0], len: 2 };
pub const UPPER_ETA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x89, 0], len: 2 };
pub const UPPER_IOTA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x8A, 0], len: 2 };
pub const UPPER_OMICRON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x8C, 0], len: 2 };
pub const UPPER_UPSILON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x8E, 0], len: 2 };
pub const UPPER_OMEGA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x8F, 0], len: 2 };

pub const UPPER_IOTA_WITH_DIALYTIKA: GreekChar = GreekChar { bytes: [0xCE, 0xAA, 0], len: 2 };
pub const UPPER_UPSILON_WITH_DIALYTIKA: GreekChar = GreekChar { bytes: [0xCE, 0xAB, 0], len: 2 };

pub const LOWER_ALPHA: GreekChar = GreekChar { bytes: [0xCE, 0xB1, 0], len: 2 };
pub const LOWER_BETA: GreekChar = GreekChar { bytes: [0xCE, 0xB2, 0], len: 2 };
pub const LOWER_GAMMA: GreekChar = GreekChar { bytes: [0xCE, 0xB3, 0], len: 2 };
pub const LOWER_DELTA: GreekChar = GreekChar { bytes: [0xCE, 0xB4, 0], len: 2 };
pub const LOWER_EPSILON: GreekChar = GreekChar { bytes: [0xCE, 0xB5, 0], len: 2 };
pub const LOWER_ZETA: GreekChar = GreekChar { bytes: [0xCE, 0xB6, 0], len: 2 };
pub const LOWER_ETA: GreekChar = GreekChar { bytes: [0xCE, 0xB7, 0], len: 2 };
pub const LOWER_THETA: GreekChar = GreekChar { bytes: [0xCE, 0xB8, 0], len: 2 };
pub const LOWER_IOTA: GreekChar = GreekChar { bytes: [0xCE, 0xB9, 0], len: 2 };
pub const LOWER_KAPPA: GreekChar = GreekChar { bytes: [0xCE, 0xBA, 0], len: 2 };
pub const LOWER_LAMBDA: GreekChar = GreekChar { bytes: [0xCE, 0xBB, 0], len: 2 };
pub const LOWER_MU: GreekChar = GreekChar { bytes: [0xCE, 0xBC, 0], len: 2 };
pub const LOWER_NU: GreekChar = GreekChar { bytes: [0xCE, 0xBD, 0], len: 2 };
pub const LOWER_XI: GreekChar = GreekChar { bytes: [0xCE, 0xBE, 0], len: 2 };
pub const LOWER_OMICRON: GreekChar = GreekChar { bytes: [0xCE, 0xBF, 0], len: 2 };
pub const LOWER_PI: GreekChar = GreekChar { bytes: [0xCF, 0x80, 0], len: 2 };
pub const LOWER_RHO: GreekChar = GreekChar { bytes: [0xCF, 0x81, 0], len: 2 };
pub const LOWER_SIGMA_FINAL: GreekChar = GreekChar { bytes: [0xCF, 0x82, 0], len: 2 };
pub const LOWER_SIGMA: GreekChar = GreekChar { bytes: [0xCF, 0x83, 0], len: 2 };
pub const LOWER_TAU: GreekChar = GreekChar { bytes: [0xCF, 0x84, 0], len: 2 };
pub const LOWER_UPSILON: GreekChar = GreekChar { bytes: [0xCF, 0x85, 0], len: 2 };
pub const LOWER_PHI: GreekChar = GreekChar { bytes: [0xCF, 0x86, 0], len: 2 };
pub const LOWER_CHI: GreekChar = GreekChar { bytes: [0xCF, 0x87, 0], len: 2 };
pub const LOWER_PSI: GreekChar = GreekChar { bytes: [0xCF, 0x88, 0], len: 2 };
pub const LOWER_OMEGA: GreekChar = GreekChar { bytes: [0xCF, 0x89, 0], len: 2 };

pub const LOWER_ALPHA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0xAC, 0], len: 2 };
pub const LOWER_EPSILON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0xAD, 0], len: 2 };
pub const LOWER_ETA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0xAE, 0], len: 2 };
pub const LOWER_IOTA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0xAF, 0], len: 2 };
pub const LOWER_OMICRON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCF, 0x8C, 0], len: 2 };
pub const LOWER_UPSILON_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCF, 0x8D, 0], len: 2 };
pub const LOWER_OMEGA_WITH_TONOS: GreekChar = GreekChar { bytes: [0xCF, 0x8E, 0], len: 2 };

pub const LOWER_IOTA_WITH_DIALYTIKA: GreekChar = GreekChar { bytes: [0xCF, 0x8A, 0], len: 2 };
pub const LOWER_UPSILON_WITH_DIALYTIKA: GreekChar = GreekChar { bytes: [0xCF, 0x8B, 0], len: 2 };

pub const LOWER_IOTA_WITH_DIALYTIKA_AND_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0x90, 0], len: 2 };
pub const LOWER_UPSILON_WITH_DIALYTIKA_AND_TONOS: GreekChar = GreekChar { bytes: [0xCE, 0xB0, 0], len: 2 };

#[cfg(test)]
mod tests {
    use super::*;

    const GREEK_UPPERS: [GreekChar; 24] = [UPPER_ALPHA, UPPER_BETA, UPPER_GAMMA, UPPER_DELTA, UPPER_EPSILON, UPPER_ZETA, UPPER_ETA, UPPER_THETA, UPPER_IOTA, UPPER_KAPPA, UPPER_LAMBDA, UPPER_MU, UPPER_NU, UPPER_XI, UPPER_OMICRON, UPPER_PI, UPPER_RHO, UPPER_SIGMA, UPPER_TAU, UPPER_UPSILON, UPPER_PHI, UPPER_CHI, UPPER_PSI, UPPER_OMEGA ];
    const GREEK_LOWERS: [GreekChar; 25] = [LOWER_ALPHA, LOWER_BETA, LOWER_GAMMA, LOWER_DELTA, LOWER_EPSILON, LOWER_ZETA, LOWER_ETA, LOWER_THETA, LOWER_IOTA, LOWER_KAPPA, LOWER_LAMBDA, LOWER_MU, LOWER_NU, LOWER_XI, LOWER_OMICRON, LOWER_PI, LOWER_RHO, LOWER_SIGMA_FINAL, LOWER_SIGMA, LOWER_TAU, LOWER_UPSILON, LOWER_PHI, LOWER_CHI, LOWER_PSI, LOWER_OMEGA ];

    #[test]
    fn uppercase_letters_are_uppercase() {
        for char in GREEK_UPPERS {
            assert!(is_uppercase_greek(&char), "[{}, {}]", &char.bytes()[0], &char.bytes()[1]);
        }
    }

    #[test]
    fn reserved_block_char_is_not_uppercase() {
        let reserved_char = GreekChar { bytes: [0xCE, 0xA2, 0], len: 2 }; // unicode \03A2
        assert!(!is_uppercase_greek(&reserved_char), "failed for reserved char");
    }

    #[test]
    fn lowercase_letters_are_not_uppercase() {
        for char in GREEK_LOWERS {
            assert!(!is_uppercase_greek(&char), "[{}, {}]", &char.bytes()[0], &char.bytes()[1]);
        }
    }

    #[test]
    fn lowercase_letters_are_lowercase() {
        for char in GREEK_LOWERS {
            assert!(is_lowercase_greek(&char), "[{}, {}]", &char.bytes()[0], &char.bytes()[1]);
        }
    }

    #[test]
    fn reserved_block_char_is_not_lowercase() {
        let reserved_char = GreekChar { bytes: [0xCE, 0xA2, 0], len: 2 }; // unicode \03A2
        assert!(!is_lowercase_greek(&reserved_char), "failed for reserved char");
    }

    #[test]
    fn uppercase_letters_are_not_lowercase() {
        for char in GREEK_UPPERS {
            assert!(!is_lowercase_greek(&char), "[{}, {}]", &char.bytes()[0], &char.bytes()[1]);
        }
    }
}
