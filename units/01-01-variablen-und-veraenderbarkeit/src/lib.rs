//! 01-01 Variablen und Veränderbarkeit / Variables and mutability
//!
//! Deutsch: `let` bindet einen Wert an einen Namen. Diese Bindung ist nicht
//! veränderbar, solange nicht `mut` dabeisteht. Daneben gibt es Konstanten, die
//! nie veränderbar sind, und das Beschatten, bei dem ein neuer Wert unter
//! demselben Namen entsteht.
//!
//! English: `let` binds a value to a name. That binding is not mutable unless
//! `mut` stands with it. Next to it there are constants, which are never
//! mutable, and shadowing, where a new value appears under the same name.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Wie viele Versuche es höchstens gibt.
///
/// Eine Konstante steht mit `const` da, braucht ihren Typ immer, und es gibt
/// kein `mut` dazu.
///
/// How many attempts there are at most.
///
/// A constant stands there with `const`, always needs its type, and there is no
/// `mut` for it.
pub const MAX_ATTEMPTS: u32 = 3;

/// Gibt zurück, wie viele Versuche noch übrig sind.
///
/// Returns how many attempts are left.
///
/// ```
/// use unit_01_01_variablen_und_veraenderbarkeit::attempts_left;
///
/// assert_eq!(attempts_left(1), 2);
/// ```
pub fn attempts_left(used: u32) -> u32 {
    MAX_ATTEMPTS - used
}

/// Aufgabe 1: Zähle `start` zweimal um eins hoch und gib das Ergebnis zurück.
///
/// Nimm dafür eine veränderbare Bindung und zwei einzelne Schritte, nicht
/// `start + 2`. Die Aufgabe ist die Bindung und nicht die Rechnung.
///
/// Exercise 1: count `start` up by one twice and return the result.
///
/// Use a mutable binding and two separate steps for it, not `start + 2`. The
/// exercise is the binding and not the arithmetic.
pub fn twice_incremented(start: u32) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib die Länge zurück, die `text` mit einem Anführungszeichen auf
/// jeder Seite hätte, also seine Länge in Bytes plus zwei.
///
/// Beschatte dafür den Namen `text` mit seiner eigenen Länge, sodass unter
/// demselben Namen danach eine Zahl steht, und rechne mit dieser weiter.
///
/// Exercise 2: return the length `text` would have with a quotation mark on each
/// side, meaning its length in bytes plus two.
///
/// Shadow the name `text` with its own length for it, so that a number stands
/// under the same name afterwards, and go on computing with that.
pub fn quoted_length(text: &str) -> usize {
    todo!("Aufgabe 2 / Exercise 2")
}
