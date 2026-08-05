//! 02-04 Veränderbares Ausleihen / Mutable borrowing, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/02-04-veraenderbares-ausleihen/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/02-04-veraenderbares-ausleihen/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Hängt ein Ausrufezeichen an den geliehenen Text an.
///
/// Appends an exclamation mark to the borrowed text.
pub fn push_exclamation(text: &mut String) {
    text.push('!');
}

/// Verdoppelt die geliehene Zahl an Ort und Stelle.
///
/// Doubles the borrowed number in place.
pub fn double_in_place(zahl: &mut i32) {
    *zahl *= 2;
}

/// Hängt `zusatz` zweimal an `text` an.
///
/// Appends `zusatz` twice to `text`.
pub fn append_twice(text: &mut String, zusatz: &str) {
    text.push_str(zusatz);
    text.push_str(zusatz);
}

/// Addiert `summand` auf `ziel`.
///
/// Adds `summand` onto `ziel`.
pub fn add_into(ziel: &mut i32, summand: &i32) {
    *ziel += *summand;
}
