//! 01-05 if und else / if and else, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/01-05-if-und-else/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/01-05-if-und-else/README.md`. What
//! is here is only the bodies that turn the unit's tests green.

/// Gibt das Vorzeichen einer Zahl als Wort zurück.
///
/// Returns the sign of a number as a word.
pub fn sign_of(zahl: i32) -> &'static str {
    if zahl < 0 {
        "negativ"
    } else if zahl == 0 {
        "null"
    } else {
        "positiv"
    }
}

/// Gibt die größere der beiden Zahlen zurück.
///
/// Returns the bigger of the two numbers.
pub fn larger(a: i32, b: i32) -> i32 {
    if a >= b { a } else { b }
}

/// Gibt zu einer Punktzahl das Urteil zurück.
///
/// Returns the verdict for a score.
pub fn grade_of(punkte: u32) -> &'static str {
    if punkte >= 90 {
        "sehr gut"
    } else if punkte >= 60 {
        "bestanden"
    } else {
        "nicht bestanden"
    }
}

/// Verzweigt auf eine gelesene Zeile.
///
/// Branches on a line that was read.
pub fn answer_to(zeile: &str) -> &'static str {
    let antwort = zeile.trim();

    if antwort == "ja" || antwort == "j" {
        "weiter"
    } else {
        "abbruch"
    }
}
