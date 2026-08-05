//! 03-07 Display selbst schreiben / Writing Display by hand
//!
//! Deutsch: `{:?}` kommt von `derive(Debug)`, `{}` von `Display`, und die
//! schreibt man von Hand. `Debug` zeigt den Aufbau, `Display` den Satz für
//! einen Menschen.
//!
//! English: `{:?}` comes from `derive(Debug)`, `{}` from `Display`, and that one
//! is written by hand. `Debug` shows the build, `Display` the sentence for a
//! person.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und der
// Schreibplatz bleibt deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and the place to
// write therefore stays unused until somebody solves them.
#![allow(unused_variables)]

use std::fmt;

/// Ein Anteil in Prozent.
///
/// Hier ist die Form einmal ausgefüllt, damit die Aufgaben eine Vorlage haben.
///
/// A share in percent.
///
/// Here the shape is filled in once, so that the exercises have a model.
#[derive(Debug, PartialEq)]
pub struct Percent(pub u8);

impl fmt::Display for Percent {
    /// Schreibt "42 %" statt "Percent(42)".
    ///
    /// Writes "42 %" instead of "Percent(42)".
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
    /// Aufgabe 1: Schreibe den Messwert für einen Menschen.
    ///
    /// "kein Wert" für `Missing`, "17 Grad" für einen einzelnen Wert, "3 bis 9
    /// Grad" für einen Bereich. Jeder Fall wird behandelt, `unwrap` kommt nicht
    /// vor.
    ///
    /// Exercise 1: write the reading for a person.
    ///
    /// "kein Wert" for `Missing`, "17 Grad" for a single value, "3 bis 9 Grad"
    /// for a range. Every case is treated, `unwrap` does not appear.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

impl fmt::Display for Sensor {
    /// Aufgabe 2: Schreibe das Gerät für einen Menschen.
    ///
    /// "Flur: 17 Grad", also der Name, ein Doppelpunkt und die Ausgabe des
    /// Messwerts. Die kommt aus Aufgabe 1 und wird nicht ein zweites Mal
    /// geschrieben.
    ///
    /// Exercise 2: write the device for a person.
    ///
    /// "Flur: 17 Grad", so the name, a colon and the output of the reading.
    /// That one comes from exercise 1 and is not written a second time.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Gibt die Ausgabe für die Fehlersuche zurück.
///
/// Das ist `{:?}` und kommt von `derive(Debug)`. Sie steht hier fertig da,
/// damit der Unterschied zur Ausgabe für Menschen in einem Test sichtbar wird.
///
/// Returns the output for fault finding.
///
/// That is `{:?}` and comes from `derive(Debug)`. It stands here finished so
/// that the difference from the output for people becomes visible in a test.
///
/// ```
/// use unit_03_07_display_selbst_schreiben::{Reading, Sensor, for_debugging};
///
/// let sensor = Sensor {
///     name: String::from("Flur"),
///     wert: Reading::Temperature(17),
/// };
///
/// assert_eq!(
///     for_debugging(&sensor),
///     "Sensor { name: \"Flur\", wert: Temperature(17) }"
/// );
/// ```
pub fn for_debugging(sensor: &Sensor) -> String {
    format!("{sensor:?}")
}

/// Aufgabe 3: Gib die Ausgabe für Menschen als `String` zurück.
///
/// Das ist `{}` und damit Aufgabe 2. Ein Typ mit `Display` bekommt außerdem
/// `to_string` geschenkt; beides ist richtig.
///
/// Exercise 3: return the output for people as a `String`.
///
/// That is `{}` and therefore exercise 2. A type with `Display` also gets
/// `to_string` for free; both are right.
pub fn for_people(sensor: &Sensor) -> String {
    todo!("Aufgabe 3 / Exercise 3")
}
