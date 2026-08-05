// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_00_01_programm_und_compiler::{doubled, greeting, hello};

#[test]
fn greeting_greets_the_world_like_hello_does() {
    assert_eq!(greeting("Welt"), hello());
}

#[test]
fn greeting_uses_the_name_it_was_given() {
    assert_eq!(greeting("Rust"), "Hallo, Rust!");
}

#[test]
fn doubled_doubles() {
    assert_eq!(doubled(21), 42);
}

#[test]
fn doubled_leaves_zero_alone() {
    assert_eq!(doubled(0), 0);
}
