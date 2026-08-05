//! 02-05 Slices / Slices, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/02-05-slices/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/02-05-slices/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Gibt die ersten beiden Zahlen als Slice zurück.
///
/// Returns the first two numbers as a slice.
pub fn first_two(zahlen: &[i32]) -> &[i32] {
    &zahlen[..2]
}

/// Gibt den Teil bis zum ersten Leerzeichen zurück.
///
/// Returns the part up to the first space.
pub fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();
    let mut stelle = 0;

    while stelle < bytes.len() {
        if bytes[stelle] == b' ' {
            return &text[..stelle];
        }

        stelle += 1;
    }

    text
}

/// Gibt alles außer der ersten Zahl zurück.
///
/// Returns everything except the first number.
pub fn without_first(zahlen: &[i32]) -> &[i32] {
    &zahlen[1..]
}

/// Addiert die Zahlen des Slice.
///
/// Adds up the numbers of the slice.
pub fn sum_of(zahlen: &[i32]) -> i32 {
    let mut summe = 0;

    for zahl in zahlen {
        summe += zahl;
    }

    summe
}
