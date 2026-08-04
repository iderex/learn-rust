//! 02-01 Verschieben / Move, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/02-01-move/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/02-01-move/README.md`. What is here
//! is only the bodies that turn the unit's tests green.

/// Nimmt den `String` an sich und gibt ihn verändert zurück.
///
/// Takes the `String` and gives it back changed.
pub fn exclaimed(mut s: String) -> String {
    s.push('!');
    s
}

/// Gibt die Länge von `s` zurück, ohne das Eigentum zu übernehmen.
///
/// Returns the length of `s` without taking ownership.
// Deutsch: `&String` steht hier absichtlich, weil die Einheit dieselbe Form
// zeigt und die eingebundene Testdatei sie aufruft.
// English: `&String` is deliberate here, because the unit shows the same shape
// and the included test file calls it.
#[allow(clippy::ptr_arg)]
pub fn length_borrowed(s: &String) -> usize {
    s.len()
}

/// Gibt eine eigenständige Kopie von `s` zurück.
///
/// Returns a standalone copy of `s`.
// Deutsch: Dieselbe Begründung für `&String` wie oben.
// English: Same reason for `&String` as above.
#[allow(clippy::ptr_arg)]
pub fn duplicated(s: &String) -> String {
    s.clone()
}

/// Hängt `b` an `a` und gibt das Ergebnis zurück.
///
/// Appends `b` to `a` and returns the result.
pub fn joined(a: String, b: String) -> String {
    a + &b
}
