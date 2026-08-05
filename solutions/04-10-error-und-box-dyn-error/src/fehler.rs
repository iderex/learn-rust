//! Deutsch: Der eigene Fehlertyp der Crate.
//!
//! English: the crate's own error type.

use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

/// Was in dieser Crate schiefgehen kann.
///
/// What can go wrong in this crate.
#[derive(Debug, PartialEq)]
pub enum AppFehler {
    /// Der Text war keine ganze Zahl / the text was not a whole number.
    KeineZahl(ParseIntError),
    /// Der Text war keine Kommazahl / the text was not a decimal number.
    KeineKommazahl(ParseFloatError),
}

impl fmt::Display for AppFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppFehler::KeineZahl(fehler) => write!(f, "keine ganze Zahl: {fehler}"),
            AppFehler::KeineKommazahl(fehler) => write!(f, "keine Kommazahl: {fehler}"),
        }
    }
}

impl Error for AppFehler {}

impl From<ParseIntError> for AppFehler {
    fn from(fehler: ParseIntError) -> Self {
        AppFehler::KeineZahl(fehler)
    }
}

impl From<ParseFloatError> for AppFehler {
    fn from(fehler: ParseFloatError) -> Self {
        AppFehler::KeineKommazahl(fehler)
    }
}
