//! 02-04 Veränderbares Ausleihen / Mutable borrowing
//!
//! Deutsch: `&mut wert` leiht einen Wert zum Verändern aus. Zur selben Zeit
//! darf es entweder eine veränderbare Ausleihe geben oder beliebig viele
//! geteilte, nie beides. `*` schreibt durch die Referenz hindurch.
//!
//! English: `&mut wert` lends a value out for changing. At the same time there
//! may be either one mutable loan or any number of shared ones, never both. `*`
//! writes through the reference.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Hängt ein Ausrufezeichen an den geliehenen Text an.
///
/// Der Aufrufer behält seinen `String`, und er ist danach verändert. Zurück
/// kommt nichts, denn der Aufrufer hat den Wert ja schon.
///
/// Appends an exclamation mark to the borrowed text.
///
/// The caller keeps its `String`, and it is changed afterwards. Nothing comes
/// back, because the caller already has the value.
///
/// ```
/// use unit_02_04_veraenderbares_ausleihen::push_exclamation;
///
/// let mut text = String::from("hallo");
///
/// push_exclamation(&mut text);
///
/// assert_eq!(text, "hallo!");
/// ```
pub fn push_exclamation(text: &mut String) {
    text.push('!');
}

/// Aufgabe 1: Verdopple die geliehene Zahl an Ort und Stelle.
///
/// Geschrieben wird durch die Referenz, also mit `*`.
///
/// Exercise 1: double the borrowed number in place.
///
/// The writing goes through the reference, so with `*`.
pub fn double_in_place(zahl: &mut i32) {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Hänge `zusatz` zweimal an `text` an.
///
/// Aus "hallo" und "!" wird "hallo!!".
///
/// Exercise 2: append `zusatz` twice to `text`.
///
/// Out of "hallo" and "!" comes "hallo!!".
// Deutsch: Solange der Rumpf `todo!()` ist, sieht clippy nicht, dass hier
// angehängt wird, und schlägt `&mut str` vor. Das Anhängen braucht den
// `String`, deshalb steht die Meldung hier ausdrücklich still.
// English: while the body is `todo!()`, clippy cannot see that something is
// appended here and suggests `&mut str`. Appending needs the `String`, so the
// message is silenced here on purpose.
#[allow(clippy::ptr_arg)]
pub fn append_twice(text: &mut String, zusatz: &str) {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Addiere `summand` auf `ziel`.
///
/// `ziel` ist veränderbar geliehen, `summand` nur geteilt. Zwei Ausleihen auf
/// zwei verschiedene Werte stoßen nicht zusammen.
///
/// Exercise 3: add `summand` onto `ziel`.
///
/// `ziel` is borrowed mutably, `summand` only shared. Two loans on two
/// different values do not collide.
pub fn add_into(ziel: &mut i32, summand: &i32) {
    todo!("Aufgabe 3 / Exercise 3")
}
