//! 09-04 Das Newtype-Muster / The newtype pattern, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/09-04-newtype-muster/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/09-04-newtype-muster/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

use std::fmt;

/// Eine Länge in Zentimetern.
///
/// A length in centimetres.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Zentimeter(pub u32);

/// Eine Masse in Gramm.
///
/// A mass in grams.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Gramm(pub u32);

/// Eine Länge in Kilometern.
///
/// A length in kilometres.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Kilometer(pub u32);

/// Ein Newtype um einen fremden Typ.
///
/// A newtype around a foreign type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Liste(pub Vec<String>);

/// Addiert zwei Längen.
///
/// Adds two lengths.
///
/// ```
/// use unit_09_04_newtype_muster::{Zentimeter, addiere};
///
/// assert_eq!(addiere(Zentimeter(80), Zentimeter(120)), Zentimeter(200));
/// ```
///
/// ```compile_fail
/// use unit_09_04_newtype_muster::{Gramm, Zentimeter, addiere};
///
/// let falsch = addiere(Zentimeter(80), Gramm(120));
/// ```
pub fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

/// Zählt die Längen zusammen.
///
/// Adds the lengths up.
pub fn summe(werte: &[Zentimeter]) -> Zentimeter {
    let mut zusammen = 0;
    for wert in werte {
        zusammen += wert.0;
    }

    Zentimeter(zusammen)
}

/// `Display` für `Liste`, geschrieben um den fremden Typ herum.
///
/// `Display` for `Liste`, written around the foreign type.
impl fmt::Display for Liste {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/// Rechnet Kilometer in Zentimeter um.
///
/// Converts kilometres into centimetres.
impl From<Kilometer> for Zentimeter {
    fn from(wert: Kilometer) -> Self {
        Zentimeter(wert.0 * 100_000)
    }
}
