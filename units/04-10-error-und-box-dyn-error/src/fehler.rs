//! Deutsch: Der eigene Fehlertyp der Crate. Zwei fremde Fehlerarten erreichen
//! ihn über `From`, und `Error` macht ihn zu einem Fehler wie jeder andere.
//!
//! English: the crate's own error type. Two foreign error kinds reach it
//! through `From`, and `Error` makes it an error like any other.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

/// Was in dieser Crate schiefgehen kann.
///
/// Jede Variante trägt den fremden Fehler mit, statt ihn wegzuwerfen. Wer die
/// Meldung liest, sieht dann beides: die eigene Einordnung und den Grund.
///
/// What can go wrong in this crate.
///
/// Every variant carries the foreign error along instead of throwing it away.
/// Whoever reads the message then sees both: the classification of its own and
/// the reason.
#[derive(Debug, PartialEq)]
pub enum AppFehler {
    /// Der Text war keine ganze Zahl / the text was not a whole number.
    KeineZahl(ParseIntError),
    /// Der Text war keine Kommazahl / the text was not a decimal number.
    KeineKommazahl(ParseFloatError),
}

// Deutsch: Aufgabe 1: Schreibe die Meldung für Menschen. Für `KeineZahl` steht
// dort "keine ganze Zahl: " und dahinter die Meldung des fremden Fehlers, für
// `KeineKommazahl` "keine Kommazahl: " und dieselbe Form.
// English: Exercise 1: write the message for people. For `KeineZahl` it reads
// "keine ganze Zahl: " followed by the message of the foreign error, for
// `KeineKommazahl` "keine Kommazahl: " and the same shape.
impl fmt::Display for AppFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

// Deutsch: `Error` verlangt `Debug` und `Display` und braucht selbst keinen
// Rumpf. Damit passt der Typ in ein `Box<dyn Error>`.
// English: `Error` demands `Debug` and `Display` and needs no body of its own.
// With it the type fits into a `Box<dyn Error>`.
impl Error for AppFehler {}

// Deutsch: Die erste Umwandlung steht fertig da und ist die Vorlage für die
// zweite.
// English: the first conversion stands there finished and is the model for the
// second one.
impl From<ParseIntError> for AppFehler {
    fn from(fehler: ParseIntError) -> Self {
        AppFehler::KeineZahl(fehler)
    }
}

// Deutsch: Aufgabe 2: Dieselbe Umwandlung für die Kommazahl.
// English: Exercise 2: the same conversion for the decimal number.
impl From<ParseFloatError> for AppFehler {
    fn from(fehler: ParseFloatError) -> Self {
        todo!("Aufgabe 2 / Exercise 2")
    }
}
