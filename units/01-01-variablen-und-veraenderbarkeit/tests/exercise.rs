// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_01_variablen_und_veraenderbarkeit::{
    MAX_ATTEMPTS, attempts_left, quoted_length, twice_incremented,
};

#[test]
fn twice_incremented_adds_two() {
    assert_eq!(twice_incremented(0), 2);
    assert_eq!(twice_incremented(40), 42);
}

#[test]
fn twice_incremented_leaves_its_argument_alone() {
    let start = 7;
    assert_eq!(twice_incremented(start), 9);
    assert_eq!(start, 7);
}

#[test]
fn quoted_length_counts_the_two_quotation_marks_as_well() {
    assert_eq!(quoted_length("hallo"), 7);
}

#[test]
fn quoted_length_of_nothing_is_two() {
    assert_eq!(quoted_length(""), 2);
}

#[test]
fn the_constant_and_the_example_agree() {
    assert_eq!(attempts_left(0), MAX_ATTEMPTS);
    assert_eq!(attempts_left(MAX_ATTEMPTS), 0);
}
