// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_00_02_kommandozeile_rustup_cargo::{main_file, manifest_file, new_project_command};

#[test]
fn main_file_sits_under_src() {
    assert_eq!(main_file("hallo"), "hallo/src/main.rs");
}

#[test]
fn main_file_uses_the_project_it_was_given() {
    assert_eq!(main_file("rechner"), "rechner/src/main.rs");
}

#[test]
fn manifest_file_sits_next_to_src() {
    assert_eq!(manifest_file("hallo"), "hallo/Cargo.toml");
}

#[test]
fn the_new_project_command_names_the_same_project() {
    assert_eq!(new_project_command("hallo"), "cargo new hallo");
    assert!(manifest_file("hallo").starts_with("hallo/"));
}
