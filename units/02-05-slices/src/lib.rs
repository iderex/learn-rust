//! 02-05 Slices / Slices
//!
//! Deutsch: Ein Slice ist eine Ausleihe auf einen Teil. Er merkt sich die
//! Stelle und die Länge und kopiert nichts. Auf einem Text heißt er `&str`, auf
//! einem Feld `&[T]`. Die Grenzen zählen Bytes.
//!
//! English: a slice is a loan on a part. It remembers the place and the length
//! and copies nothing. Over a text it is called `&str`, over an array `&[T]`.
//! The bounds count bytes.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt die ersten beiden Zahlen als Slice zurück.
///
/// Der Rückgabewert zeigt in dasselbe Feld hinein. `as_ptr` gibt die Adresse
/// her, und beide Adressen sind gleich, weil nichts kopiert wurde.
///
/// Returns the first two numbers as a slice.
///
/// The return value points into the same array. `as_ptr` hands out the address,
/// and both addresses are equal because nothing was copied.
///
/// ```
/// use unit_02_05_slices::first_two;
///
/// let zahlen = [1, 2, 3, 4];
/// let anfang = first_two(&zahlen);
///
/// assert_eq!(anfang, [1, 2]);
/// assert_eq!(anfang.as_ptr(), zahlen.as_ptr());
/// ```
pub fn first_two(zahlen: &[i32]) -> &[i32] {
    &zahlen[..2]
}

/// Aufgabe 1: Gib den Teil bis zum ersten Leerzeichen zurück.
///
/// Steht kein Leerzeichen im Text, ist die Antwort der ganze Text.
/// `text.as_bytes()` gibt die Bytes her, und `b' '` ist das Byte des
/// Leerzeichens.
///
/// Exercise 1: return the part up to the first space.
///
/// If no space stands in the text, the answer is the whole text.
/// `text.as_bytes()` hands out the bytes, and `b' '` is the byte of the space.
pub fn first_word(text: &str) -> &str {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib alles außer der ersten Zahl zurück.
///
/// Bei einem Slice mit einer einzigen Zahl bleibt ein leerer Slice übrig.
///
/// Exercise 2: return everything except the first number.
///
/// For a slice holding a single number an empty slice is left.
pub fn without_first(zahlen: &[i32]) -> &[i32] {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Addiere die Zahlen des Slice.
///
/// Dieselbe Funktion nimmt ein ganzes Feld und einen Teil davon, denn beide
/// sind derselbe Typ.
///
/// Exercise 3: add up the numbers of the slice.
///
/// The same function takes a whole array and a part of one, because both are
/// the same type.
pub fn sum_of(zahlen: &[i32]) -> i32 {
    todo!("Aufgabe 3 / Exercise 3")
}
