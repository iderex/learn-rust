//! 04-05 String / String
//!
//! Deutsch: Ein `&str` ist geliehener Text, ein `String` gehört sich selbst und
//! kann wachsen. `len` zählt Bytes, `chars()` zählt Zeichen, und über eine Zahl
//! kommt man an kein Zeichen heran.
//!
//! English: a `&str` is borrowed text, a `String` owns itself and can grow.
//! `len` counts bytes, `chars()` counts characters, and through a number no
//! character can be reached.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt die Zahl der Bytes eines Textes zurück.
///
/// Das ist `len`, und es ist bei einem Wort mit Umlauten eine andere Zahl als
/// die der Zeichen.
///
/// Returns the number of bytes of a text.
///
/// That is `len`, and for a word with umlauts in it that is a different number
/// from the number of characters.
///
/// ```
/// use unit_04_05_string::byte_count;
///
/// assert_eq!(byte_count("Grüße"), 7);
/// assert_eq!(byte_count("Gruesse"), 7);
/// ```
pub fn byte_count(text: &str) -> usize {
    text.len()
}

/// Aufgabe 1: Zähle die Zeichen eines Textes.
///
/// Gemeint sind Zeichen und nicht Bytes. `text.chars()` gibt sie der Reihe nach
/// her.
///
/// Exercise 1: count the characters of a text.
///
/// Characters are meant and not bytes. `text.chars()` hands them out one after
/// the other.
pub fn char_count(text: &str) -> usize {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Setze zwei Texte mit einem Leerzeichen dazwischen zusammen.
///
/// Beide kommen geliehen herein, heraus kommt ein eigener `String`.
///
/// Exercise 2: put two texts together with a space in between.
///
/// Both come in borrowed, and what comes out is a `String` of its own.
pub fn joined(a: &str, b: &str) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die ersten `zeichen` Zeichen des Textes zurück.
///
/// Gezählt werden Zeichen. Ist der Text kürzer, kommt er ganz zurück. Ein
/// Ausschnitt über Bytes wäre hier falsch, denn er kann mitten in ein Zeichen
/// fallen.
///
/// Exercise 3: return the first `zeichen` characters of the text.
///
/// Characters are counted. If the text is shorter it comes back whole. A cut
/// over bytes would be wrong here, because it can fall into the middle of a
/// character.
pub fn shortened(text: &str, zeichen: usize) -> String {
    todo!("Aufgabe 3 / Exercise 3")
}
