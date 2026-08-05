// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_02_02_stack_und_heap::{copies_on_assignment, twice, with_exclamation};

#[test]
fn twice_doubles_the_number() {
    assert_eq!(twice(21), 42);
    assert_eq!(twice(0), 0);
}

#[test]
fn twice_leaves_the_number_with_the_caller() {
    let zahl = 21;

    assert_eq!(twice(zahl), 42);

    // Deutsch: `zahl` ist nach dem Aufruf noch da, denn `i32` ist `Copy`.
    // English: `zahl` is still there after the call, because `i32` is `Copy`.
    assert_eq!(zahl, 21);
}

#[test]
fn with_exclamation_appends_the_mark() {
    assert_eq!(with_exclamation(String::from("hallo")), "hallo!");
}

#[test]
fn with_exclamation_on_an_empty_string() {
    assert_eq!(with_exclamation(String::new()), "!");
}

#[test]
fn copies_on_assignment_says_yes_for_the_copy_types() {
    assert!(copies_on_assignment("i32"));
    assert!(copies_on_assignment("u8"));
    assert!(copies_on_assignment("f64"));
    assert!(copies_on_assignment("bool"));
    assert!(copies_on_assignment("char"));
    assert!(copies_on_assignment("&str"));
    assert!(copies_on_assignment("(i32, bool)"));
}

#[test]
fn copies_on_assignment_says_no_for_the_others() {
    assert!(!copies_on_assignment("String"));
    assert!(!copies_on_assignment("Vec<u8>"));
    assert!(!copies_on_assignment("(i32, String)"));
}
