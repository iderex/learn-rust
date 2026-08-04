//! <nn-nn> <Titel deutsch> / <title english>
//!
//! Deutsch: <ein Absatz, der sagt, worum es in dieser Einheit geht>
//!
//! English: <one paragraph saying what this unit is about>

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// <Was diese Beispielfunktion zeigt.>
///
/// <What this example function shows.>
///
/// ```
/// use unit_<nn>_<nn>_<name>::example;
///
/// assert_eq!(example(2), 4);
/// ```
pub fn example(n: u32) -> u32 {
    n * 2
}

/// Aufgabe 1: <was zu tun ist>
///
/// Exercise 1: <what to do>
pub fn exercise(n: u32) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}
