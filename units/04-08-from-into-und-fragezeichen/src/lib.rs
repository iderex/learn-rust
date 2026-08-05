//! 04-08 From, Into und der Operator ? / From, Into and the ? operator
//!
//! Deutsch: `?` gibt einen Fehler sofort zurück und wandelt ihn dabei mit
//! `From` um. Wer `From` schreibt, bekommt `into` dazu.
//!
//! English: `?` returns an error at once and converts it with `From` on the
//! way. Whoever writes `From` gets `into` along with it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Was beim Lesen einer Zahl schiefgehen kann.
///
/// What can go wrong while reading a number.
#[derive(Debug, PartialEq)]
pub enum EingabeFehler {
    /// Der Text ist keine Zahl / the text is not a number.
    KeineZahl,
}

/// Was beim Rechnen schiefgehen kann.
///
/// Die erste Variante trägt den Fehler des Lesens weiter. Genau diese Form
/// braucht `?`, um einen Fehler umzuwandeln.
///
/// What can go wrong while calculating.
///
/// The first variant carries the error of the reading onwards. That is exactly
/// the shape `?` needs to convert an error.
#[derive(Debug, PartialEq)]
pub enum Fehler {
    /// Beim Lesen ging etwas schief / something went wrong while reading.
    Eingabe(EingabeFehler),
    /// Der Nenner war null / the denominator was zero.
    DurchNull,
}

/// Liest eine Zahl aus einem Text.
///
/// Diese Funktion steht fertig da. Sie gibt den Fehler des Lesens zurück, und
/// die Aufgaben wandeln ihn um.
///
/// Reads a number out of a text.
///
/// This function stands there finished. It returns the error of the reading,
/// and the exercises convert it.
///
/// ```
/// use unit_04_08_from_into_und_fragezeichen::{EingabeFehler, parsed};
///
/// assert_eq!(parsed(" 42 "), Ok(42));
/// assert_eq!(parsed("zwei"), Err(EingabeFehler::KeineZahl));
/// ```
pub fn parsed(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

// Deutsch: Aufgabe 1: Sag, wie aus einem `EingabeFehler` ein `Fehler` wird.
// Der Rumpf von `from` ist eine Zeile, und danach kann `?` umwandeln.
// English: Exercise 1: say how an `EingabeFehler` becomes a `Fehler`. The body
// of `from` is one line, and afterwards `?` can convert.
impl From<EingabeFehler> for Fehler {
    fn from(fehler: EingabeFehler) -> Self {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

/// Aufgabe 2: Lies zwei Texte als Zahlen und teile sie.
///
/// Beide Male wird `parsed` gerufen und mit `?` weitergegangen; die Umwandlung
/// macht Aufgabe 1. Ist die zweite Zahl null, ist das Ergebnis
/// `Err(Fehler::DurchNull)`.
///
/// Exercise 2: read two texts as numbers and divide them.
///
/// Both times `parsed` is called and carried on with `?`; the conversion is
/// exercise 1. If the second number is zero, the result is
/// `Err(Fehler::DurchNull)`.
pub fn divided_text(links: &str, rechts: &str) -> Result<i32, Fehler> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Wandle einen `EingabeFehler` in einen `Fehler` um.
///
/// Hier steht kein zweites `impl`. `into` gibt es geschenkt, sobald `From`
/// dasteht.
///
/// Exercise 3: convert an `EingabeFehler` into a `Fehler`.
///
/// No second `impl` stands here. `into` comes as a gift as soon as `From` is
/// there.
pub fn as_fehler(fehler: EingabeFehler) -> Fehler {
    todo!("Aufgabe 3 / Exercise 3")
}
