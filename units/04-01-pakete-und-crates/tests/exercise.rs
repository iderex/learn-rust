// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_01_pakete_und_crates::{binary_root, crate_count, crate_kind, crate_root};

#[test]
fn crate_root_of_a_library_and_of_a_program() {
    assert_eq!(crate_root("bibliothek"), Some("src/lib.rs"));
    assert_eq!(crate_root("programm"), Some("src/main.rs"));
}

#[test]
fn crate_root_of_something_that_is_neither() {
    assert_eq!(crate_root("paket"), None);
    assert_eq!(crate_root(""), None);
}

#[test]
fn crate_count_of_a_package_from_cargo_new() {
    // Deutsch: `cargo new` legt ein Programm an und keine Bibliothek.
    // English: `cargo new` creates a program and no library.
    assert_eq!(crate_count(false, 1), 1);
}

#[test]
fn crate_count_of_a_package_with_a_library_and_two_programs() {
    assert_eq!(crate_count(true, 2), 3);
}

#[test]
fn crate_count_of_a_package_with_nothing_in_it() {
    assert_eq!(crate_count(false, 0), 0);
}

#[test]
fn binary_root_puts_the_program_under_src_bin() {
    assert_eq!(binary_root("zweites"), "src/bin/zweites.rs");
    assert_eq!(binary_root("werkzeug"), "src/bin/werkzeug.rs");
}

#[test]
fn a_root_and_its_kind_belong_together() {
    // Deutsch: Was `crate_root` zurückgibt, erkennt `crate_kind` wieder.
    // English: what `crate_root` returns is recognised again by `crate_kind`.
    assert_eq!(
        crate_kind(crate_root("bibliothek").unwrap_or("")),
        Some("bibliothek")
    );
    assert_eq!(
        crate_kind(crate_root("programm").unwrap_or("")),
        Some("programm")
    );
    assert_eq!(crate_kind(&binary_root("zweites")), Some("programm"));
}
