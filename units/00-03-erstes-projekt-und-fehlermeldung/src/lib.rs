//! 00-03 Das erste Projekt und eine Fehlermeldung lesen / The first project and
//! reading an error message
//!
//! Deutsch: Ein frisches cargo-Projekt hat wenige Teile, und `cargo run` und
//! `cargo test` sind die beiden Befehle, mit denen man es benutzt. Wenn etwas
//! nicht stimmt, antwortet der Übersetzer, und diese Antwort hat immer denselben
//! Aufbau: eine Nummer, eine Stelle, ein Pfeil und ein Hinweis.
//!
//! English: a fresh cargo project has few parts, and `cargo run` and
//! `cargo test` are the two commands you use it with. When something is wrong
//! the compiler answers, and that answer always has the same shape: a number, a
//! place, an arrow and a note.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Befehl zurück, den der Übersetzer am Ende jeder Meldung vorschlägt.
///
/// Returns the command the compiler suggests at the end of every message.
///
/// ```
/// use unit_00_03_erstes_projekt_und_fehlermeldung::explain_command;
///
/// assert_eq!(explain_command("E0308"), "rustc --explain E0308");
/// ```
pub fn explain_command(code: &str) -> String {
    format!("rustc --explain {code}")
}

/// Aufgabe 1: Gib die Adresse der Seite zurück, auf der eine Fehlernummer
/// erklärt wird.
///
/// Für `E0308` ist das `https://doc.rust-lang.org/error_codes/E0308.html`.
///
/// Exercise 1: return the address of the page explaining an error number.
///
/// For `E0308` that is `https://doc.rust-lang.org/error_codes/E0308.html`.
pub fn explain_url(code: &str) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib Zeile und Spalte in der Schreibweise zurück, die der
/// Übersetzer benutzt.
///
/// Für Zeile 2 und Spalte 21 ist das `2:21`.
///
/// Exercise 2: return line and column in the spelling the compiler uses.
///
/// For line 2 and column 21 that is `2:21`.
pub fn points_at(line: u32, column: u32) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}
