//! 00-01 Was ein Programm ist und was ein Compiler tut / What a program is and
//! what a compiler does
//!
//! Deutsch: Ein Programm ist zuerst Text. Der Compiler liest diesen Text,
//! prüft ihn und macht daraus eine Datei, die der Rechner ausführen kann. Bei
//! Rust passiert das Prüfen vollständig vor dem Ausführen, und deshalb bleibt
//! ein ganzer Haufen Fehler dort liegen, wo er billig ist.
//!
//! English: a program is text first. The compiler reads that text, checks it,
//! and makes a file the machine can run. In Rust the checking happens entirely
//! before the running, and that is why a whole pile of mistakes stays where it
//! is cheap.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Text zurück, den das erste Programm ausgibt.
///
/// Der Rückgabetyp `&'static str` heißt: ein Textstück, das im Programm selbst
/// liegt und so lange da ist wie das Programm.
///
/// Returns the text the first program prints.
///
/// The return type `&'static str` means: a piece of text that lives in the
/// program itself and stays there as long as the program does.
///
/// ```
/// use unit_00_01_programm_und_compiler::hello;
///
/// assert_eq!(hello(), "Hallo, Welt!");
/// ```
pub fn hello() -> &'static str {
    "Hallo, Welt!"
}

/// Aufgabe 1: Gib `Hallo, <name>!` zurück, mit dem übergebenen Namen darin.
///
/// Für `"Welt"` kommt genau der Text heraus, den `hello` zurückgibt.
///
/// Exercise 1: return `Hallo, <name>!` with the name that was passed in.
///
/// For `"Welt"` the result is exactly the text `hello` returns.
pub fn greeting(name: &str) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib das Doppelte von `n` zurück.
///
/// Exercise 2: return twice `n`.
pub fn doubled(n: u32) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}
