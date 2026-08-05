//! 03-07 Display selbst schreiben / Writing Display by hand, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/03-07-display-selbst-schreiben/README.md`. Hier stehen nur die Typen
//! und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/03-07-display-selbst-schreiben/README.md`. What is here is only the
//! types and the bodies that turn the unit's tests green.

use std::fmt;

/// Ein Anteil in Prozent.
///
/// A share in percent.
#[derive(Debug, PartialEq)]
pub struct Percent(pub u8);

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} %", self.0)
    }
}

/// Ein Messwert, in drei Fällen.
///
/// A reading, in three cases.
#[derive(Debug, PartialEq)]
pub enum Reading {
    /// Kein Wert / no value.
    Missing,
    /// Ein einzelner Wert in Grad / a single value in degrees.
    Temperature(i32),
    /// Ein Bereich von `von` bis `bis` / a range from `von` to `bis`.
    Range {
        /// Die untere Grenze / the lower bound.
        von: i32,
        /// Die obere Grenze / the upper bound.
        bis: i32,
    },
}

/// Ein Messgerät mit Namen und Messwert.
///
/// A measuring device with a name and a reading.
#[derive(Debug, PartialEq)]
pub struct Sensor {
    /// Der Name des Geräts / the name of the device.
    pub name: String,
    /// Was es gemessen hat / what it measured.
    pub wert: Reading,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Reading::Missing => write!(f, "kein Wert"),
            Reading::Temperature(grad) => write!(f, "{grad} Grad"),
            Reading::Range { von, bis } => write!(f, "{von} bis {bis} Grad"),
        }
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.wert)
    }
}

/// Gibt die Ausgabe für die Fehlersuche zurück.
///
/// Returns the output for fault finding.
pub fn for_debugging(sensor: &Sensor) -> String {
    format!("{sensor:?}")
}

/// Gibt die Ausgabe für Menschen als `String` zurück.
///
/// Returns the output for people as a `String`.
pub fn for_people(sensor: &Sensor) -> String {
    sensor.to_string()
}
