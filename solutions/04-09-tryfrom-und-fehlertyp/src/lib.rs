//! 04-09 TryFrom und ein eigener Fehlertyp / TryFrom and an error type of your
//! own, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/04-09-tryfrom-und-fehlertyp/README.md`. Hier stehen nur die Typen und
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/04-09-tryfrom-und-fehlertyp/README.md`. What is here is only the
//! types and the bodies that turn the unit's tests green.

use std::fmt;

/// Ein Alter in Jahren.
///
/// An age in years.
#[derive(Debug, PartialEq)]
pub struct Alter(pub u32);

/// Warum eine Zahl kein Alter ist.
///
/// Why a number is not an age.
#[derive(Debug, PartialEq)]
pub enum AlterFehler {
    /// Die Zahl war negativ / the number was negative.
    Negativ,
    /// Die Zahl war größer als 130 / the number was bigger than 130.
    ZuGross,
}

/// Gibt die Zahl aus einem geprüften Alter zurück.
///
/// Returns the number out of a checked age.
pub fn years_of(alter: &Alter) -> u32 {
    alter.0
}

impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        if zahl < 0 {
            return Err(AlterFehler::Negativ);
        }

        if zahl > 130 {
            return Err(AlterFehler::ZuGross);
        }

        Ok(Alter(zahl as u32))
    }
}

impl fmt::Display for AlterFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlterFehler::Negativ => write!(f, "ein Alter ist nicht negativ"),
            AlterFehler::ZuGross => write!(f, "ein Alter über 130 gibt es nicht"),
        }
    }
}

/// Wandelt eine Zahl in ein Alter um.
///
/// Converts a number into an age.
pub fn age_from(zahl: i32) -> Result<Alter, AlterFehler> {
    zahl.try_into()
}
