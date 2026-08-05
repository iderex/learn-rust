// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_02_zahlen_und_einfache_typen::{fits_in_u8, half, is_hex_letter, seconds_of, widened};

#[test]
fn fits_in_u8_knows_the_upper_end_of_the_range() {
    assert!(fits_in_u8(255));
    assert!(!fits_in_u8(256));
}

#[test]
fn fits_in_u8_knows_the_lower_end_of_the_range() {
    assert!(fits_in_u8(0));
}

#[test]
fn widened_keeps_the_value() {
    assert_eq!(widened(7), 7_u32);
    assert_eq!(widened(255), 255_u32);
}

#[test]
fn half_divides_without_a_remainder() {
    assert_eq!(half(5.0), 2.5);
    assert_eq!(half(0.0), 0.0);
}

#[test]
fn is_hex_letter_separates_a_to_f_from_the_rest() {
    assert!(is_hex_letter('a'));
    assert!(is_hex_letter('f'));
    assert!(!is_hex_letter('g'));
    assert!(!is_hex_letter('7'));
}

#[test]
fn the_example_counts_a_minute() {
    assert_eq!(seconds_of(1), 60);
}
