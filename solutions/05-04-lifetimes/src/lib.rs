//! 05-04 Lifetimes / Lifetimes, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-04-lifetimes/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/05-04-lifetimes/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Gibt den kürzeren von zwei Texten zurück.
///
/// Returns the shorter of two texts.
pub fn shorter<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if links.len() < rechts.len() {
        links
    } else {
        rechts
    }
}

/// Gibt den längeren von zwei Texten zurück.
///
/// Returns the longer of two texts.
pub fn longest<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if links.len() > rechts.len() {
        links
    } else {
        rechts
    }
}

/// Gibt das erste Wort eines Satzes zurück.
///
/// Returns the first word of a sentence.
pub fn first_word(satz: &str) -> &str {
    match satz.find(' ') {
        Some(stelle) => &satz[..stelle],
        None => satz,
    }
}

/// Ein Auszug aus einem Text, der den Text nicht überleben kann.
///
/// An excerpt from a text that cannot outlive the text.
#[derive(Debug, PartialEq)]
pub struct Excerpt<'a> {
    pub teil: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Nimmt den ersten Satz des Textes als Auszug.
    ///
    /// Takes the first sentence of the text as an excerpt.
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        let ende = text.find('.').unwrap_or(text.len());

        Excerpt {
            teil: &text[..ende],
        }
    }
}
