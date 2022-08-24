use crate::words::*;

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