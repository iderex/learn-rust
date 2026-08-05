//! 01-03 Funktionen / Functions, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/01-03-funktionen/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/01-03-funktionen/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Gibt den Umfang eines Rechtecks zurück.
///
/// Returns the perimeter of a rectangle.
pub fn perimeter(breite: u32, hoehe: u32) -> u32 {
    2 * (breite + hoehe)
}

/// Gibt die Fläche eines Rechtecks zurück.
///
/// Returns the area of a rectangle.
pub fn area(breite: u32, hoehe: u32) -> u32 {
    breite * hoehe
}

/// Rechnet eine Temperatur von Fahrenheit in Celsius um.
///
/// Converts a temperature from Fahrenheit to Celsius.
pub fn celsius_from(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Gibt die Fläche eines Quadrats zurück.
///
/// Returns the area of a square.
pub fn square_area(seite: u32) -> u32 {
    area(seite, seite)
}
