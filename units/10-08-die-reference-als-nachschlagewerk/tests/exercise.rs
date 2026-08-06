// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_08_die_reference_als_nachschlagewerk::{
    is_aligned_for_u32, saturating_to_u8, sum_without_panic, truncating_to_u8,
};

#[test]
fn truncating_to_u8_keeps_the_lower_bits() {
    // Deutsch: 300 ist 0b1_0010_1100. Die unteren acht Bit sind 0b0010_1100,
    // also 44. Gesaettigt waere die Antwort 255, und genau das ist sie nicht.
    // English: 300 is 0b1_0010_1100. The lower eight bits are 0b0010_1100,
    // meaning 44. Saturated the answer would be 255, and that is exactly what
    // it is not.
    assert_eq!(truncating_to_u8(300), 44);
    assert_eq!(truncating_to_u8(256), 0);
}

#[test]
fn truncating_to_u8_reads_a_negative_number_as_two_s_complement() {
    assert_eq!(truncating_to_u8(-1), 255);
    assert_eq!(truncating_to_u8(-256), 0);
}

#[test]
fn truncating_to_u8_leaves_a_fitting_number_alone() {
    assert_eq!(truncating_to_u8(0), 0);
    assert_eq!(truncating_to_u8(42), 42);
    assert_eq!(truncating_to_u8(255), 255);
}

#[test]
fn sum_without_panic_adds_while_it_fits() {
    assert_eq!(sum_without_panic(0, 0), Some(0));
    assert_eq!(sum_without_panic(250, 5), Some(255));
    assert_eq!(sum_without_panic(1, 2), Some(3));
}

#[test]
fn sum_without_panic_says_none_instead_of_stopping() {
    // Deutsch: 250 + 10 passt nicht in ein u8. Im Debug-Bau haelt `+` hier an,
    // und diese Funktion soll stattdessen antworten.
    // English: 250 + 10 does not fit into a u8. In a debug build `+` panics
    // here, and this function is meant to answer instead.
    assert_eq!(sum_without_panic(250, 10), None);
    assert_eq!(sum_without_panic(255, 1), None);
    assert_eq!(sum_without_panic(255, 255), None);
}

#[test]
fn is_aligned_for_u32_accepts_a_multiple_of_four() {
    assert!(is_aligned_for_u32(0));
    assert!(is_aligned_for_u32(4));
    assert!(is_aligned_for_u32(8));
    assert!(is_aligned_for_u32(1024));
}

#[test]
fn is_aligned_for_u32_refuses_everything_else() {
    assert!(!is_aligned_for_u32(1));
    assert!(!is_aligned_for_u32(2));
    assert!(!is_aligned_for_u32(6));
    assert!(!is_aligned_for_u32(1023));
}

#[test]
fn is_aligned_for_u32_asks_align_of_and_does_not_hard_code_two() {
    // Deutsch: Eine Adresse, die durch 2 teilbar ist und nicht durch 4, trennt
    // die nachgeschlagene Antwort von der geratenen.
    // English: an address divisible by 2 and not by 4 separates the looked up
    // answer from the guessed one.
    assert!(!is_aligned_for_u32(2));
    assert!(!is_aligned_for_u32(14));
}

#[test]
fn the_finished_function_shows_the_same_shape() {
    assert_eq!(saturating_to_u8(42.9), 42);
    assert_eq!(saturating_to_u8(300.0), 255);
    assert_eq!(saturating_to_u8(-1.5), 0);
    assert_eq!(saturating_to_u8(f64::NAN), 0);
}
