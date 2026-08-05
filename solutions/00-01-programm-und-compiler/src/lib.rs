//! 00-01 Was ein Programm ist und was ein Compiler tut / What a program is and
//! what a compiler does, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/00-01-programm-und-compiler/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/00-01-programm-und-compiler/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Gibt den Text zurück, den das erste Programm ausgibt.
///
/// Returns the text the first program prints.
pub fn hello() -> &'static str {
    "Hallo, Welt!"
}

/// Gibt `Hallo, <name>!` zurück.
///
/// Returns `Hallo, <name>!`.
pub fn greeting(name: &str) -> String {
    format!("Hallo, {name}!")
}

/// Gibt das Doppelte von `n` zurück.
///
/// Returns twice `n`.
pub fn doubled(n: u32) -> u32 {
    n * 2
}
