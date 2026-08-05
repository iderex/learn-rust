//! 04-05 String / String, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/04-05-string/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/04-05-string/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Gibt die Zahl der Bytes eines Textes zurück.
///
/// Returns the number of bytes of a text.
pub fn byte_count(text: &str) -> usize {
    text.len()
}

/// Zählt die Zeichen eines Textes.
///
/// Counts the characters of a text.
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}

/// Setzt zwei Texte mit einem Leerzeichen dazwischen zusammen.
///
/// Puts two texts together with a space in between.
pub fn joined(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

/// Gibt die ersten `zeichen` Zeichen des Textes zurück.
///
/// Returns the first `zeichen` characters of the text.
pub fn shortened(text: &str, zeichen: usize) -> String {
    let mut kurz = String::new();

    for (stelle, buchstabe) in text.chars().enumerate() {
        if stelle == zeichen {
            break;
        }

        kurz.push(buchstabe);
    }

    kurz
}
