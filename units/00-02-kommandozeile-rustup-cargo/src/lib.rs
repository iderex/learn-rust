//! 00-02 Kommandozeile, rustup und cargo / The command line, rustup and cargo
//!
//! Deutsch: Auf der Kommandozeile tippt man einen Befehl und bekommt Text
//! zurück. rustup verwaltet die Übersetzer, cargo verwaltet die Projekte. Diese
//! Einheit zeigt beide Werkzeuge und den Ordnerbau, den cargo anlegt.
//!
//! English: on the command line you type a command and get text back. rustup
//! manages the compilers, cargo manages the projects. This unit shows both
//! tools and the folder layout cargo creates.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Befehl zurück, mit dem cargo ein neues Projekt anlegt.
///
/// Returns the command with which cargo creates a new project.
///
/// ```
/// use unit_00_02_kommandozeile_rustup_cargo::new_project_command;
///
/// assert_eq!(new_project_command("hallo"), "cargo new hallo");
/// ```
pub fn new_project_command(name: &str) -> String {
    format!("cargo new {name}")
}

/// Aufgabe 1: Gib den Pfad der Datei zurück, in der ein frisches Projekt seinen
/// Quelltext hat.
///
/// Für ein Projekt `hallo` ist das `hallo/src/main.rs`. Die Trennzeichen sind
/// Schrägstriche, auch unter Windows.
///
/// Exercise 1: return the path of the file where a fresh project keeps its
/// source text.
///
/// For a project `hallo` that is `hallo/src/main.rs`. The separators are forward
/// slashes, on Windows as well.
pub fn main_file(project: &str) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib den Pfad der Datei zurück, in der ein Projekt seinen Namen und
/// seine Abhängigkeiten stehen hat.
///
/// Für ein Projekt `hallo` ist das `hallo/Cargo.toml`.
///
/// Exercise 2: return the path of the file where a project keeps its name and
/// its dependencies.
///
/// For a project `hallo` that is `hallo/Cargo.toml`.
pub fn manifest_file(project: &str) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}
