// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_05_tests_und_ihr_aufbau::{all_passed, describe, grade, passed};

#[test]
fn grade_names_a_letter_for_every_band() {
    assert_eq!(grade(100), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(80), 'B');
    assert_eq!(grade(70), 'C');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(0), 'F');
}

#[test]
fn grade_falls_to_the_lower_band_one_point_below_the_step() {
    // Deutsch: `assert_ne!` sagt, dass zwei Werte verschieden sein sollen, und
    // hier ist genau die Grenze gemeint.
    // English: `assert_ne!` says two values should differ, and here it is
    // exactly the boundary that is meant.
    assert_ne!(grade(89), 'A');
    assert_eq!(grade(89), 'B');
    assert_eq!(grade(59), 'F');
}

#[test]
#[should_panic(expected = "mehr als 100 Punkte gibt es nicht")]
fn grade_panics_above_a_hundred_points() {
    grade(101);
}

#[test]
fn describe_puts_points_grade_and_outcome_in_one_line() {
    assert_eq!(describe(85), "85 Punkte, Note B, bestanden");
    assert_eq!(describe(100), "100 Punkte, Note A, bestanden");
}

#[test]
fn describe_ends_on_durchgefallen_below_sixty_points() {
    assert_eq!(describe(59), "59 Punkte, Note F, durchgefallen");
    assert_eq!(describe(0), "0 Punkte, Note F, durchgefallen");
}

#[test]
fn all_passed_is_true_when_every_score_reaches_sixty() {
    assert!(all_passed(&[60, 75, 100]));
}

#[test]
fn all_passed_is_false_when_one_score_misses() {
    assert!(!all_passed(&[60, 59, 100]));
}

#[test]
fn all_passed_of_an_empty_list_is_true() {
    // Deutsch: Keine Punktzahl fällt durch, also ist nichts durchgefallen.
    // English: no score fails, so nothing has failed.
    assert!(all_passed(&[]));
}

#[test]
fn the_finished_function_shows_the_same_shape() {
    assert!(passed('A'));
    assert!(!passed('F'));
}
