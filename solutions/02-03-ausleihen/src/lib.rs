//! 02-03 Ausleihen / Borrowing, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/02-03-ausleihen/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/02-03-ausleihen/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Gibt den Text in Großbuchstaben zurück, ohne ihn zu nehmen.
///
/// Returns the text in capitals without taking it.
// Deutsch: `&String` steht hier absichtlich, weil die Einheit dieselbe Form
// zeigt und die eingebundene Testdatei sie aufruft.
// English: `&String` is deliberate here, because the unit shows the same shape
// and the included test file calls it.
#[allow(clippy::ptr_arg)]
pub fn shout(text: &String) -> String {
    text.to_uppercase()
}

/// Zählt die Selbstlaute in `text`.
///
/// Counts the vowels in `text`.
// Deutsch: Dieselbe Begründung für `&String` wie oben.
// English: Same reason for `&String` as above.
#[allow(clippy::ptr_arg)]
pub fn vowel_count(text: &String) -> usize {
    let mut anzahl = 0;

    for zeichen in text.chars() {
        if zeichen == 'a' || zeichen == 'e' || zeichen == 'i' || zeichen == 'o' || zeichen == 'u' {
            anzahl += 1;
        }
    }

    anzahl
}

/// Gibt das Doppelte der geliehenen Zahl zurück.
///
/// Returns the double of the borrowed number.
pub fn doubled_through(zahl: &i32) -> i32 {
    *zahl * 2
}

/// Addiert die Längen von `a` und `b`.
///
/// Adds the lengths of `a` and `b`.
// Deutsch: Dieselbe Begründung für `&String` wie oben.
// English: Same reason for `&String` as above.
#[allow(clippy::ptr_arg)]
pub fn total_length(a: &String, b: &String) -> usize {
    a.len() + b.len()
}
