//! Deutsch: Das Modul `zahlen`, in einer eigenen Datei. Es nennt sein
//! Untermodul und trägt eine Aufgabe.
//!
//! English: the module `zahlen`, in a file of its own. It names its submodule
//! and carries one exercise.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

pub mod intern;

/// Gibt die Summe zweier Zahlen zurück.
///
/// Diese Funktion steht fertig da, damit `intern` einen Pfad nach oben zeigen
/// kann.
///
/// Returns the sum of two numbers.
///
/// This function stands there finished, so that `intern` has a path upwards to
/// show.
///
/// ```
/// use unit_04_02_module::zahlen::summed;
///
/// assert_eq!(summed(20, 22), 42);
/// ```
pub fn summed(a: i32, b: i32) -> i32 {
    a + b
}

/// Aufgabe 1: Gib das Doppelte von `zahl` zurück.
///
/// Diese Aufgabe wird in `src/zahlen.rs` gelöst und nicht in `src/lib.rs`.
///
/// Exercise 1: return the double of `zahl`.
///
/// This exercise is solved in `src/zahlen.rs` and not in `src/lib.rs`.
pub fn doubled(zahl: i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}
