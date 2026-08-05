// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_05_if_und_else::{answer_to, grade_of, larger};

#[test]
fn larger_takes_the_bigger_number() {
    assert_eq!(larger(3, 5), 5);
    assert_eq!(larger(5, 3), 5);
}

#[test]
fn larger_with_two_equal_numbers_takes_that_number() {
    assert_eq!(larger(4, 4), 4);
    assert_eq!(larger(-2, -2), -2);
}

#[test]
fn grade_of_names_the_top() {
    assert_eq!(grade_of(90), "sehr gut");
    assert_eq!(grade_of(100), "sehr gut");
}

#[test]
fn grade_of_names_the_middle() {
    assert_eq!(grade_of(60), "bestanden");
    assert_eq!(grade_of(89), "bestanden");
}

#[test]
fn grade_of_names_the_bottom() {
    assert_eq!(grade_of(59), "nicht bestanden");
    assert_eq!(grade_of(0), "nicht bestanden");
}

#[test]
fn answer_to_takes_yes() {
    assert_eq!(answer_to("ja"), "weiter");
    assert_eq!(answer_to("j"), "weiter");
}

#[test]
fn answer_to_takes_everything_else() {
    assert_eq!(answer_to("nein"), "abbruch");
    assert_eq!(answer_to(""), "abbruch");
}

#[test]
fn answer_to_ignores_the_line_break() {
    assert_eq!(answer_to("ja\n"), "weiter");
    assert_eq!(answer_to(" ja \r\n"), "weiter");
}
