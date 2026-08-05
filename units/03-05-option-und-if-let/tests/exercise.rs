// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_05_option_und_if_let::{describe, doubled_or_zero, grade_for};

#[test]
fn grade_for_a_score_that_exists() {
    assert_eq!(grade_for(60), Some("bestanden"));
    assert_eq!(grade_for(100), Some("bestanden"));
    assert_eq!(grade_for(0), Some("nicht bestanden"));
}

#[test]
fn grade_for_a_score_that_cannot_exist() {
    assert_eq!(grade_for(101), None);
    assert_eq!(grade_for(1000), None);
}

#[test]
fn describe_a_value_that_is_there() {
    assert_eq!(describe(Some(17)), "Wert 17");
    assert_eq!(describe(Some(-3)), "Wert -3");
}

#[test]
fn describe_a_value_that_is_missing() {
    assert_eq!(describe(None), "kein Wert");
}

#[test]
fn doubled_or_zero_with_a_value() {
    assert_eq!(doubled_or_zero(Some(21)), 42);
    assert_eq!(doubled_or_zero(Some(0)), 0);
}

#[test]
fn doubled_or_zero_without_a_value() {
    assert_eq!(doubled_or_zero(None), 0);
}
