//! 03-06 derive mit Debug / derive with Debug, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/03-06-derive-mit-debug/README.md`.
//! Hier stehen nur die Typen und die Rümpfe, die die Tests der Einheit grün
//! machen.
//!
//! English: the explanation lives in
//! `units/03-06-derive-mit-debug/README.md`. What is here is only the types and
//! the bodies that turn the unit's tests green.

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

/// Ein Messwert mit `Debug` und `PartialEq`.
///
/// A reading with `Debug` and `PartialEq`.
#[derive(Debug, PartialEq)]
pub enum Reading {
    /// Kein Wert / no value.
    Missing,
    /// Ein einzelner Wert in Grad / a single value in degrees.
    Temperature(i32),
}

/// Gibt einen Messwert als Text für die Fehlersuche zurück.
///
/// Returns a reading as text for fault finding.
pub fn reading_debug(messwert: &Reading) -> String {
    format!("{messwert:?}")
}

/// Gibt das Rechteck als eine Zeile zurück.
///
/// Returns the rectangle as one line.
pub fn debug_line(rechteck: &Rectangle) -> String {
    format!("{rechteck:?}")
}

/// Gibt das Rechteck mit einer Zeile je Feld zurück.
///
/// Returns the rectangle with one line per field.
pub fn debug_block(rechteck: &Rectangle) -> String {
    format!("{rechteck:#?}")
}

/// Sagt, ob zwei Rechtecke gleich sind.
///
/// Says whether two rectangles are equal.
pub fn same(a: &Rectangle, b: &Rectangle) -> bool {
    a == b
}
