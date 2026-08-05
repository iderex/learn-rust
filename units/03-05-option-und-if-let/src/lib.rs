//! 03-05 Option und if let / Option and if let
//!
//! Deutsch: `Option<T>` ist ein `enum` aus der Standardbibliothek mit den
//! Varianten `Some(wert)` und `None`. Gelesen wird es mit `match`, `if let`
//! oder `let ... else`. `unwrap` behandelt den leeren Fall nicht, es hält an.
//!
//! English: `Option<T>` is an `enum` from the standard library with the
//! variants `Some(wert)` and `None`. It is read with `match`, `if let` or
//! `let ... else`. `unwrap` does not treat the empty case, it stops.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt die erste Zahl eines Slice zurück, falls es eine gibt.
///
/// `first` aus der Standardbibliothek antwortet selbst mit einem `Option`, denn
/// ein leerer Slice hat kein erstes Element.
///
/// Returns the first number of a slice, if there is one.
///
/// `first` from the standard library answers with an `Option` itself, because
/// an empty slice has no first element.
///
/// ```
/// use unit_03_05_option_und_if_let::first_of;
///
/// assert_eq!(first_of(&[7, 8, 9]), Some(7));
/// assert_eq!(first_of(&[]), None);
/// ```
pub fn first_of(zahlen: &[i32]) -> Option<i32> {
    zahlen.first().copied()
}

/// Aufgabe 1: Gib zu einer Punktzahl ein Urteil zurück.
///
/// Ab 60 Punkten "bestanden", darunter "nicht bestanden". Über 100 Punkte gibt
/// es nicht, und die Antwort ist dort `None`.
///
/// Exercise 1: return a verdict for a score.
///
/// From 60 points "bestanden", below that "nicht bestanden". More than 100
/// points do not exist, and the answer there is `None`.
pub fn grade_for(punkte: u32) -> Option<&'static str> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Beschreibe ein `Option` als Text, mit `if let`.
///
/// Aus `Some(17)` wird "Wert 17", aus `None` wird "kein Wert".
///
/// Exercise 2: describe an `Option` as text, with `if let`.
///
/// Out of `Some(17)` comes "Wert 17", out of `None` comes "kein Wert".
pub fn describe(wert: Option<i32>) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Verdopple den Wert oder gib null zurück, mit `let ... else`.
///
/// Der `else`-Zweig muss die Funktion verlassen, hier mit `return 0;`.
///
/// Exercise 3: double the value or return zero, with `let ... else`.
///
/// The `else` arm has to leave the function, here with `return 0;`.
pub fn doubled_or_zero(wert: Option<i32>) -> i32 {
    todo!("Aufgabe 3 / Exercise 3")
}
