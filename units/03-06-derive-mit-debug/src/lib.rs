//! 03-06 derive mit Debug / derive with Debug
//!
//! Deutsch: `#[derive(...)]` lässt den Übersetzer eine Implementierung
//! schreiben. `Debug` gehört zu `{:?}` und `{:#?}`, `PartialEq` zu `==`,
//! `Clone` und `Copy` zum Kopieren.
//!
//! English: `#[derive(...)]` lets the compiler write an implementation.
//! `Debug` belongs to `{:?}` and `{:#?}`, `PartialEq` to `==`, `Clone` and
//! `Copy` to copying.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Rechteck mit vier abgeleiteten Implementierungen.
///
/// A rectangle with four derived implementations.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    /// Die Breite / the width.
    pub breite: u32,
    /// Die Höhe / the height.
    pub hoehe: u32,
}

/// Ein Messwert, wie in `03-03`, hier mit `Debug` und `PartialEq`.
///
/// A reading, as in `03-03`, here with `Debug` and `PartialEq`.
#[derive(Debug, PartialEq)]
pub enum Reading {
    /// Kein Wert / no value.
    Missing,
    /// Ein einzelner Wert in Grad / a single value in degrees.
    Temperature(i32),
}

/// Gibt einen Messwert als Text für die Fehlersuche zurück.
///
/// Das ist die Ausgabe, die `derive(Debug)` erzeugt. Der Name der Variante
/// steht mit darin, und bei `Temperature` auch die Zahl.
///
/// Returns a reading as text for fault finding.
///
/// This is the output `derive(Debug)` produces. The name of the variant stands
/// in it, and for `Temperature` the number as well.
///
/// ```
/// use unit_03_06_derive_mit_debug::{Reading, reading_debug};
///
/// assert_eq!(reading_debug(&Reading::Missing), "Missing");
/// assert_eq!(reading_debug(&Reading::Temperature(17)), "Temperature(17)");
/// ```
pub fn reading_debug(messwert: &Reading) -> String {
    format!("{messwert:?}")
}

/// Aufgabe 1: Gib das Rechteck als eine Zeile zurück.
///
/// Das ist `{:?}`. Erwartet wird genau die Zeile, die `derive(Debug)` erzeugt.
///
/// Exercise 1: return the rectangle as one line.
///
/// That is `{:?}`. Expected is exactly the line `derive(Debug)` produces.
pub fn debug_line(rechteck: &Rectangle) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib das Rechteck mit einer Zeile je Feld zurück.
///
/// Das ist `{:#?}`. Eingerückt wird mit vier Leerzeichen, und hinter dem
/// letzten Feld steht ein Komma.
///
/// Exercise 2: return the rectangle with one line per field.
///
/// That is `{:#?}`. The indentation is four spaces, and a comma stands behind
/// the last field.
pub fn debug_block(rechteck: &Rectangle) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag, ob zwei Rechtecke gleich sind.
///
/// Möglich ist das nur, weil `PartialEq` abgeleitet ist. Verglichen wird Feld
/// für Feld, und das macht die abgeleitete Implementierung.
///
/// Exercise 3: say whether two rectangles are equal.
///
/// That is only possible because `PartialEq` is derived. The comparison goes
/// field by field, and the derived implementation does it.
pub fn same(a: &Rectangle, b: &Rectangle) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}
