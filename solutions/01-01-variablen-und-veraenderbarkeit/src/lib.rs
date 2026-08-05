//! 01-01 Variablen und Veränderbarkeit / Variables and mutability, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/01-01-variablen-und-veraenderbarkeit/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/01-01-variablen-und-veraenderbarkeit/README.md`. What is here is only
//! the bodies that turn the unit's tests green.

/// Wie viele Versuche es höchstens gibt.
///
/// How many attempts there are at most.
pub const MAX_ATTEMPTS: u32 = 3;

/// Gibt zurück, wie viele Versuche noch übrig sind.
///
/// Returns how many attempts are left.
pub fn attempts_left(used: u32) -> u32 {
    MAX_ATTEMPTS - used
}

/// Zählt `start` zweimal um eins hoch.
///
/// Counts `start` up by one twice.
pub fn twice_incremented(start: u32) -> u32 {
    let mut zahl = start;
    zahl += 1;
    zahl += 1;
    zahl
}

/// Gibt die Länge von `text` in Anführungszeichen zurück, über eine
/// Beschattung.
///
/// Returns the length of `text` in quotation marks, through a shadowing.
pub fn quoted_length(text: &str) -> usize {
    let text = text.len();
    text + 2
}
