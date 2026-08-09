//! 10-04 Varianz / Variance, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/10-04-varianz/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/10-04-varianz/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Eine Notiz, die eine Referenz nur zum Lesen hält.
///
/// A note holding a reference for reading only.
#[derive(Debug, PartialEq)]
pub struct Notiz<'a> {
    pub text: &'a str,
}

/// Liest eine ewig lebende Notiz als eine, die nur kurz lebt.
///
/// Reads a note that lives forever as one that lives only briefly.
///
/// ```
/// use unit_10_04_varianz::{Notiz, kuerzer};
///
/// fn zusammen<'a>(eine: Notiz<'a>, andere: Notiz<'a>) -> usize {
///     eine.text.len() + andere.text.len()
/// }
///
/// let ewig: Notiz<'static> = Notiz { text: "ewig" };
/// let text = String::from("kurz");
///
/// assert_eq!(zusammen(kuerzer(ewig), Notiz { text: &text }), 8);
/// ```
pub fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

/// Wendet einen Zeiger auf eine Funktion auf einen ewig lebenden Text an.
///
/// Applies a function pointer to a text that lives forever.
///
/// ```
/// use unit_10_04_varianz::laenge_unter;
///
/// assert_eq!(laenge_unter(str::len), 4);
/// ```
pub fn laenge_unter<'kurz>(f: fn(&'kurz str) -> usize) -> usize {
    let fuer_ewig: fn(&'static str) -> usize = f;
    fuer_ewig("ewig")
}

/// Gibt den längeren der beiden Texte heraus, bei gleicher Länge den ersten.
///
/// Gives out the longer of the two texts, the first one on a tie.
pub fn laengere<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() { b } else { a }
}

/// Setzt einen neuen Text in eine Notiz und gibt den alten heraus.
///
/// Puts a new text into a note and gives out the old one.
pub fn ersetzen<'a>(notiz: &mut Notiz<'a>, neu: &'a str) -> &'a str {
    let alt = notiz.text;
    notiz.text = neu;
    alt
}

/// Findet den kürzesten Text unter den Notizen.
///
/// Finds the shortest text among the notes.
pub fn kuerzeste<'a>(notizen: &[Notiz<'a>]) -> Option<&'a str> {
    let mut kuerzeste: Option<&'a str> = None;
    for notiz in notizen {
        match kuerzeste {
            Some(bisher) if bisher.len() <= notiz.text.len() => {}
            _ => kuerzeste = Some(notiz.text),
        }
    }
    kuerzeste
}
