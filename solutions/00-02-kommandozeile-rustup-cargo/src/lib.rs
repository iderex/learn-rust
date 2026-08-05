//! 00-02 Kommandozeile, rustup und cargo / The command line, rustup and cargo,
//! gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/00-02-kommandozeile-rustup-cargo/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/00-02-kommandozeile-rustup-cargo/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Gibt den Befehl zurück, mit dem cargo ein neues Projekt anlegt.
///
/// Returns the command with which cargo creates a new project.
pub fn new_project_command(name: &str) -> String {
    format!("cargo new {name}")
}

/// Gibt den Pfad der Quelltextdatei eines frischen Projekts zurück.
///
/// Returns the path of the source file of a fresh project.
pub fn main_file(project: &str) -> String {
    format!("{project}/src/main.rs")
}

/// Gibt den Pfad der Manifestdatei eines Projekts zurück.
///
/// Returns the path of the manifest file of a project.
pub fn manifest_file(project: &str) -> String {
    format!("{project}/Cargo.toml")
}
