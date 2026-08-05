// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_00_03_erstes_projekt_und_fehlermeldung::{explain_command, explain_url, points_at};

#[test]
fn explain_url_leads_to_the_page_of_a_number() {
    assert_eq!(
        explain_url("E0308"),
        "https://doc.rust-lang.org/error_codes/E0308.html"
    );
}

#[test]
fn explain_url_uses_the_number_it_was_given() {
    assert_eq!(
        explain_url("E0425"),
        "https://doc.rust-lang.org/error_codes/E0425.html"
    );
}

#[test]
fn points_at_writes_line_and_column_the_way_the_compiler_does() {
    assert_eq!(points_at(2, 21), "2:21");
}

#[test]
fn points_at_keeps_both_numbers_apart() {
    assert_eq!(points_at(21, 2), "21:2");
}

#[test]
fn the_command_and_the_page_name_the_same_number() {
    assert!(explain_command("E0425").ends_with("E0425"));
    assert!(explain_url("E0425").contains("E0425"));
}
