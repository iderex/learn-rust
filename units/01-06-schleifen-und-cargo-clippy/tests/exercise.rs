// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_06_schleifen_und_cargo_clippy::{digit_count, first_square_over, product_to};

#[test]
fn product_to_multiplies_up_to_n() {
    assert_eq!(product_to(5), 120);
    assert_eq!(product_to(3), 6);
}

#[test]
fn product_to_of_zero_and_one_is_one() {
    assert_eq!(product_to(0), 1);
    assert_eq!(product_to(1), 1);
}

#[test]
fn digit_count_counts_the_digits() {
    assert_eq!(digit_count(1234), 4);
    assert_eq!(digit_count(7), 1);
}

#[test]
fn digit_count_of_zero_is_one() {
    assert_eq!(digit_count(0), 1);
}

#[test]
fn digit_count_at_the_step_to_the_next_digit() {
    assert_eq!(digit_count(9), 1);
    assert_eq!(digit_count(10), 2);
}

#[test]
fn first_square_over_climbs_past_the_limit() {
    assert_eq!(first_square_over(50), 64);
    assert_eq!(first_square_over(0), 1);
}

#[test]
fn first_square_over_a_square_takes_the_next_one() {
    assert_eq!(first_square_over(64), 81);
    assert_eq!(first_square_over(1), 4);
}
