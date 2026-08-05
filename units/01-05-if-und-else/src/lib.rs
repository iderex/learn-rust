//! 01-05 if und else / if and else
//!
//! Deutsch: `if` prüft eine Bedingung, und die Bedingung muss ein `bool` sein.
//! `if` ist außerdem ein Ausdruck, hat also einen Wert und darf rechts von einem
//! `let` stehen. Beide Zweige liefern dann denselben Typ.
//!
//! English: `if` checks a condition, and the condition has to be a `bool`. `if`
//! is also an expression, so it has a value and may stand on the right of a
//! `let`. Both branches then deliver the same type.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt das Vorzeichen einer Zahl als Wort zurück.
///
/// Drei Fälle, also `if`, `else if` und `else`. Alle drei Zweige liefern einen
/// `&'static str`, sonst hätte die Verzweigung keinen Typ.
///
/// Returns the sign of a number as a word.
///
/// Three cases, so `if`, `else if` and `else`. All three branches deliver a
/// `&'static str`, otherwise the branch would have no type.
///
/// ```
/// use unit_01_05_if_und_else::sign_of;
///
/// assert_eq!(sign_of(-3), "negativ");
/// assert_eq!(sign_of(0), "null");
/// assert_eq!(sign_of(7), "positiv");
/// ```
pub fn sign_of(zahl: i32) -> &'static str {
    if zahl < 0 {
        "negativ"
    } else if zahl == 0 {
        "null"
    } else {
        "positiv"
    }
}

/// Aufgabe 1: Gib die größere der beiden Zahlen zurück.
///
/// Sind beide gleich groß, ist die Antwort diese Zahl.
///
/// Exercise 1: return the bigger of the two numbers.
///
/// If both are the same size, the answer is that number.
pub fn larger(a: i32, b: i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib zu einer Punktzahl das Urteil zurück.
///
/// Ab 90 Punkten "sehr gut", ab 60 "bestanden", darunter "nicht bestanden".
///
/// Exercise 2: return the verdict for a score.
///
/// From 90 points "sehr gut", from 60 "bestanden", below that
/// "nicht bestanden".
pub fn grade_of(punkte: u32) -> &'static str {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Verzweige auf eine gelesene Zeile.
///
/// "ja" und "j" heißen "weiter", alles andere heißt "abbruch". Die Zeile kommt
/// so, wie `read_line` sie liefert, also mit dem Zeilenumbruch am Ende.
///
/// Exercise 3: branch on a line that was read.
///
/// "ja" and "j" mean "weiter", everything else means "abbruch". The line comes
/// as `read_line` delivers it, so with the line break at the end.
pub fn answer_to(zeile: &str) -> &'static str {
    todo!("Aufgabe 3 / Exercise 3")
}
