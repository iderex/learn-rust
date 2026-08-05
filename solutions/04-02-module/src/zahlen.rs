//! Deutsch: Das Modul `zahlen`, in einer eigenen Datei.
//!
//! English: the module `zahlen`, in a file of its own.

pub mod intern;

/// Gibt die Summe zweier Zahlen zurück.
///
/// Returns the sum of two numbers.
pub fn summed(a: i32, b: i32) -> i32 {
    a + b
}

/// Gibt das Doppelte von `zahl` zurück.
///
/// Returns the double of `zahl`.
pub fn doubled(zahl: i32) -> i32 {
    zahl * 2
}
