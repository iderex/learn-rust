//! 00-03 Das erste Projekt und eine Fehlermeldung lesen / The first project and
//! reading an error message, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/00-03-erstes-projekt-und-fehlermeldung/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/00-03-erstes-projekt-und-fehlermeldung/README.md`. What is here is
//! only the bodies that turn the unit's tests green.

/// Gibt den Befehl zurück, den der Übersetzer am Ende jeder Meldung vorschlägt.
///
/// Returns the command the compiler suggests at the end of every message.
pub fn explain_command(code: &str) -> String {
    format!("rustc --explain {code}")
}

/// Gibt die Adresse der Seite zurück, auf der eine Fehlernummer erklärt wird.
///
/// Returns the address of the page explaining an error number.
pub fn explain_url(code: &str) -> String {
    format!("https://doc.rust-lang.org/error_codes/{code}.html")
}

/// Gibt Zeile und Spalte in der Schreibweise des Übersetzers zurück.
///
/// Returns line and column in the compiler's spelling.
pub fn points_at(line: u32, column: u32) -> String {
    format!("{line}:{column}")
}
