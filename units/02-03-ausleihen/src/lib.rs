//! 02-03 Ausleihen / Borrowing
//!
//! Deutsch: `&wert` leiht einen Wert aus, statt ihn zu übergeben. Die Ausleihe
//! darf lesen und nichts ändern, der Aufrufer behält sein Eigentum, und die
//! Ausleihe endet bei ihrer letzten Benutzung. `*` liest den Wert hinter einer
//! Referenz.
//!
//! English: `&wert` lends a value out instead of handing it over. The loan may
//! read and change nothing, the caller keeps its ownership, and the loan ends at
//! its last use. `*` reads the value behind a reference.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Text in Großbuchstaben zurück, ohne ihn zu nehmen.
///
/// Der Aufrufer behält seinen `String` und kann ihn danach unverändert
/// weiterbenutzen.
///
/// Returns the text in capitals without taking it.
///
/// The caller keeps its `String` and can go on using it unchanged afterwards.
///
/// ```
/// use unit_02_03_ausleihen::shout;
///
/// let text = String::from("hallo");
///
/// assert_eq!(shout(&text), "HALLO");
///
/// // Deutsch: `text` gehört immer noch dem Aufrufer.
/// // English: `text` still belongs to the caller.
/// assert_eq!(text, "hallo");
/// ```
// Deutsch: `&String` steht hier absichtlich, weil der Lernende an dieser Stelle
// einen `String` in der Hand hat. Der Vorschlag von clippy ist `&str`, und der
// ist ein Slice; Slices stehen in `02-05`.
// English: `&String` is deliberate here, because the learner holds a `String` at
// this point. The suggestion from clippy is `&str`, which is a slice; slices
// stand in `02-05`.
#[allow(clippy::ptr_arg)]
pub fn shout(text: &String) -> String {
    text.to_uppercase()
}

/// Aufgabe 1: Zähle die Selbstlaute in `text`.
///
/// Gezählt werden `a`, `e`, `i`, `o` und `u`, kleingeschrieben. `text.chars()`
/// gibt die Zeichen der Reihe nach her.
///
/// Exercise 1: count the vowels in `text`.
///
/// Counted are `a`, `e`, `i`, `o` and `u`, in lower case. `text.chars()` hands
/// out the characters one after the other.
// Deutsch: Dieselbe Begründung für `&String` wie oben.
// English: Same reason for `&String` as above.
#[allow(clippy::ptr_arg)]
pub fn vowel_count(text: &String) -> usize {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib das Doppelte der geliehenen Zahl zurück.
///
/// Hier wird `*` gebraucht, denn gerechnet wird mit der Zahl und nicht mit der
/// Referenz.
///
/// Exercise 2: return the double of the borrowed number.
///
/// Here `*` is needed, because the calculation runs on the number and not on
/// the reference.
pub fn doubled_through(zahl: &i32) -> i32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Addiere die Längen von `a` und `b`.
///
/// Beide sind Ausleihen, und beide dürfen zur selben Zeit bestehen. Ein Test
/// leiht denselben Text zweimal aus.
///
/// Exercise 3: add the lengths of `a` and `b`.
///
/// Both are loans, and both may exist at the same time. One test lends the same
/// text out twice.
// Deutsch: Dieselbe Begründung für `&String` wie oben.
// English: Same reason for `&String` as above.
#[allow(clippy::ptr_arg)]
pub fn total_length(a: &String, b: &String) -> usize {
    todo!("Aufgabe 3 / Exercise 3")
}
