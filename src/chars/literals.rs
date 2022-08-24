use crate::chars::GreekChar;

/////////////////////////////////////////////////////////
//                                                     //
//    GREEK UTF-8 CODE POINTS (TWO-BYTE CHARACTERS)    //
//                                                     //
/////////////////////////////////////////////////////////

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


////////////////////////////////////////////////////////////////////
//                                                                //
//    GREEK EXTENDED UTF-8 CODE POINTS (THREE-BYTE CHARACTERS)    //
//                                                                //
////////////////////////////////////////////////////////////////////


// LOWER ALPHA

pub const LOWER_ALPHA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x80);
pub const LOWER_ALPHA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x81);
pub const LOWER_ALPHA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x82);
pub const LOWER_ALPHA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x83);
pub const LOWER_ALPHA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x84);
pub const LOWER_ALPHA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x85);
pub const LOWER_ALPHA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x86);
pub const LOWER_ALPHA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x87);
pub const LOWER_ALPHA_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB0);
pub const LOWER_ALPHA_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB1);
pub const LOWER_ALPHA_WITH_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB2);
pub const LOWER_ALPHA_WITH_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB3);
pub const LOWER_ALPHA_WITH_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB4);
pub const LOWER_ALPHA_WITH_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB6);
pub const LOWER_ALPHA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB7);
pub const LOWER_ALPHA_WITH_PSILI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x80);
pub const LOWER_ALPHA_WITH_DASIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x81);
pub const LOWER_ALPHA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x82);
pub const LOWER_ALPHA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x83);
pub const LOWER_ALPHA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x84);
pub const LOWER_ALPHA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x85);
pub const LOWER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x86);
pub const LOWER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x87);
pub const LOWER_ALPHA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB0);
pub const LOWER_ALPHA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB1);

// UPPER ALPHA

pub const UPPER_ALPHA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x88);
pub const UPPER_ALPHA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x89);
pub const UPPER_ALPHA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8A);
pub const UPPER_ALPHA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8B);
pub const UPPER_ALPHA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8C);
pub const UPPER_ALPHA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8D);
pub const UPPER_ALPHA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8E);
pub const UPPER_ALPHA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x8F);
pub const UPPER_ALPHA_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB8);
pub const UPPER_ALPHA_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xB9);
pub const UPPER_ALPHA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xBA);
pub const UPPER_ALPHA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xBB);
pub const UPPER_ALPHA_WITH_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xBC);
pub const UPPER_ALPHA_WITH_PSILI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x88);
pub const UPPER_ALPHA_WITH_DASIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x89);
pub const UPPER_ALPHA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8A);
pub const UPPER_ALPHA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8B);
pub const UPPER_ALPHA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8C);
pub const UPPER_ALPHA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8D);
pub const UPPER_ALPHA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8E);
pub const UPPER_ALPHA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x8F);

// LOWER EPSILON

pub const LOWER_EPSILON_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x90);
pub const LOWER_EPSILON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x91);
pub const LOWER_EPSILON_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x92);
pub const LOWER_EPSILON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x93);
pub const LOWER_EPSILON_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x94);
pub const LOWER_EPSILON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x95);
pub const LOWER_EPSILON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB2);
pub const LOWER_EPSILON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB3);

// UPPER EPSILON

pub const UPPER_EPSILON_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x98);
pub const UPPER_EPSILON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x99);
pub const UPPER_EPSILON_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x9A);
pub const UPPER_EPSILON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x9B);
pub const UPPER_EPSILON_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x9C);
pub const UPPER_EPSILON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0x9D);
pub const UPPER_EPSILON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x88);
pub const UPPER_EPSILON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x89);

// LOWER ETA

pub const LOWER_ETA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA0);
pub const LOWER_ETA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA1);
pub const LOWER_ETA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA2);
pub const LOWER_ETA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA3);
pub const LOWER_ETA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA4);
pub const LOWER_ETA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA5);
pub const LOWER_ETA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA6);
pub const LOWER_ETA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA7);
pub const LOWER_ETA_WITH_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x82);
pub const LOWER_ETA_WITH_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x83);
pub const LOWER_ETA_WITH_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x84);
pub const LOWER_ETA_WITH_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x86);
pub const LOWER_ETA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x87);
pub const LOWER_ETA_WITH_PSILI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x90);
pub const LOWER_ETA_WITH_DASIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x91);
pub const LOWER_ETA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x92);
pub const LOWER_ETA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x93);
pub const LOWER_ETA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x94);
pub const LOWER_ETA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x95);
pub const LOWER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x96);
pub const LOWER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x97);
pub const LOWER_ETA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB4);
pub const LOWER_ETA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB5);

// UPPER ETA

pub const UPPER_ETA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA8);
pub const UPPER_ETA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xA9);
pub const UPPER_ETA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAA);
pub const UPPER_ETA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAB);
pub const UPPER_ETA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAC);
pub const UPPER_ETA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAD);
pub const UPPER_ETA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAE);
pub const UPPER_ETA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xAF);
pub const UPPER_ETA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x8A);
pub const UPPER_ETA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x8B);
pub const UPPER_ETA_WITH_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x8C);
pub const UPPER_ETA_WITH_PSILI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x98);
pub const UPPER_ETA_WITH_DASIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x99);
pub const UPPER_ETA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9A);
pub const UPPER_ETA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9B);
pub const UPPER_ETA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9C);
pub const UPPER_ETA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9D);
pub const UPPER_ETA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9E);
pub const UPPER_ETA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0x9F);

// LOWER IOTA

pub const LOWER_IOTA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB0);
pub const LOWER_IOTA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB1);
pub const LOWER_IOTA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB2);
pub const LOWER_IOTA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB3);
pub const LOWER_IOTA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB4);
pub const LOWER_IOTA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB5);
pub const LOWER_IOTA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB6);
pub const LOWER_IOTA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB7);
pub const LOWER_IOTA_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x90);
pub const LOWER_IOTA_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x91);
pub const LOWER_IOTA_WITH_DIALYTIKA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x92);
pub const LOWER_IOTA_WITH_DIALYTIKA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x93);
pub const LOWER_IOTA_WITH_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x96);
pub const LOWER_IOTA_WITH_DIALYTIKA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x97);
pub const LOWER_IOTA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB6);
pub const LOWER_IOTA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB7);

// UPPER IOTA

pub const UPPER_IOTA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB8);
pub const UPPER_IOTA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xB9);
pub const UPPER_IOTA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBA);
pub const UPPER_IOTA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBB);
pub const UPPER_IOTA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBC);
pub const UPPER_IOTA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBD);
pub const UPPER_IOTA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBE);
pub const UPPER_IOTA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBC, 0xBF);
pub const UPPER_IOTA_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x98);
pub const UPPER_IOTA_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x99);
pub const UPPER_IOTA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x9A);
pub const UPPER_IOTA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0x9B);

// LOWER OMICRON

pub const LOWER_OMICRON_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x80);
pub const LOWER_OMICRON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x81);
pub const LOWER_OMICRON_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x82);
pub const LOWER_OMICRON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x83);
pub const LOWER_OMICRON_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x84);
pub const LOWER_OMICRON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x85);
pub const LOWER_OMICRON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB8);
pub const LOWER_OMICRON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xB9);

// UPPER OMICRON

pub const UPPER_OMICRON_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x88);
pub const UPPER_OMICRON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x89);
pub const UPPER_OMICRON_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x8A);
pub const UPPER_OMICRON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x8B);
pub const UPPER_OMICRON_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x8C);
pub const UPPER_OMICRON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x8D);
pub const UPPER_OMICRON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB8);
pub const UPPER_OMICRON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB9);

// LOWER UPSILON

pub const LOWER_UPSILON_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x90);
pub const LOWER_UPSILON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x91);
pub const LOWER_UPSILON_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x92);
pub const LOWER_UPSILON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x93);
pub const LOWER_UPSILON_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x94);
pub const LOWER_UPSILON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x95);
pub const LOWER_UPSILON_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x96);
pub const LOWER_UPSILON_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x97);
pub const LOWER_UPSILON_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA0);
pub const LOWER_UPSILON_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA1);
pub const LOWER_UPSILON_WITH_DIALYTIKA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA2);
pub const LOWER_UPSILON_WITH_DIALYTIKA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA3);
pub const LOWER_UPSILON_WITH_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA6);
pub const LOWER_UPSILON_WITH_DIALYTIKA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA7);
pub const LOWER_UPSILON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xBA);
pub const LOWER_UPSILON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xBB);

// UPPER UPSILON

pub const UPPER_UPSILON_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x99);
pub const UPPER_UPSILON_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x9B);
pub const UPPER_UPSILON_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x9D);
pub const UPPER_UPSILON_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0x9F);
pub const UPPER_UPSILON_WITH_VRACHY: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA8);
pub const UPPER_UPSILON_WITH_MACRON: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA9);
pub const UPPER_UPSILON_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xAA);
pub const UPPER_UPSILON_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xAB);

// LOWER OMEGA

pub const LOWER_OMEGA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA0);
pub const LOWER_OMEGA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA1);
pub const LOWER_OMEGA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA2);
pub const LOWER_OMEGA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA3);
pub const LOWER_OMEGA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA4);
pub const LOWER_OMEGA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA5);
pub const LOWER_OMEGA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA6);
pub const LOWER_OMEGA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA7);
pub const LOWER_OMEGA_WITH_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB2);
pub const LOWER_OMEGA_WITH_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB3);
pub const LOWER_OMEGA_WITH_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB4);
pub const LOWER_OMEGA_WITH_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB6);
pub const LOWER_OMEGA_WITH_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xB7);
pub const LOWER_OMEGA_WITH_PSILI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA0);
pub const LOWER_OMEGA_WITH_DASIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA1);
pub const LOWER_OMEGA_WITH_PSILI_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA2);
pub const LOWER_OMEGA_WITH_DASIA_AND_VARIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA3);
pub const LOWER_OMEGA_WITH_PSILI_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA4);
pub const LOWER_OMEGA_WITH_DASIA_AND_OXIA_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA5);
pub const LOWER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA6);
pub const LOWER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_YPOGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA7);
pub const LOWER_OMEGA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xBC);
pub const LOWER_OMEGA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xBD);

// UPPER OMEGA

pub const UPPER_OMEGA_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA8);
pub const UPPER_OMEGA_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xA9);
pub const UPPER_OMEGA_WITH_PSILI_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAA);
pub const UPPER_OMEGA_WITH_DASIA_AND_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAB);
pub const UPPER_OMEGA_WITH_PSILI_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAC);
pub const UPPER_OMEGA_WITH_DASIA_AND_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAD);
pub const UPPER_OMEGA_WITH_PSILI_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAE);
pub const UPPER_OMEGA_WITH_DASIA_AND_PERISPOMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBD, 0xAF);
pub const UPPER_OMEGA_WITH_VARIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xBA);
pub const UPPER_OMEGA_WITH_OXIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xBB);
pub const UPPER_OMEGA_WITH_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xBC);
pub const UPPER_OMEGA_WITH_PSILI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA8);
pub const UPPER_OMEGA_WITH_DASIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xA9);
pub const UPPER_OMEGA_WITH_PSILI_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAA);
pub const UPPER_OMEGA_WITH_DASIA_AND_VARIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAB);
pub const UPPER_OMEGA_WITH_PSILI_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAC);
pub const UPPER_OMEGA_WITH_DASIA_AND_OXIA_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAD);
pub const UPPER_OMEGA_WITH_PSILI_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAE);
pub const UPPER_OMEGA_WITH_DASIA_AND_PERISPOMENI_AND_PROSGEGRAMMENI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBE, 0xAF);

// RHO WITH BREAKING MARK

pub const LOWER_RHO_WITH_PSILI: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA4);
pub const LOWER_RHO_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xA5);

pub const UPPER_RHO_WITH_DASIA: GreekChar = GreekChar::ThreeByte(0xE1, 0xBF, 0xAC);