//! 03-02 Methoden / Methods, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/03-02-methoden/README.md`. Hier
//! stehen nur der Typ und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/03-02-methoden/README.md`. What is
//! here is only the type and the bodies that turn the unit's tests green.

/// Ein Rechteck aus Breite und Höhe.
///
/// A rectangle made of a width and a height.
pub struct Rectangle {
    /// Die Breite / the width.
    pub breite: u32,
    /// Die Höhe / the height.
    pub hoehe: u32,
}

impl Rectangle {
    /// Gibt die Fläche zurück.
    ///
    /// Returns the area.
    pub fn area(&self) -> u32 {
        self.breite * self.hoehe
    }

    /// Legt ein Rechteck aus `breite` und `hoehe` an.
    ///
    /// Creates a rectangle out of `breite` and `hoehe`.
    pub fn new(breite: u32, hoehe: u32) -> Self {
        Self { breite, hoehe }
    }

    /// Gibt den Umfang zurück.
    ///
    /// Returns the perimeter.
    pub fn perimeter(&self) -> u32 {
        2 * (self.breite + self.hoehe)
    }

    /// Verdoppelt beide Seiten an Ort und Stelle.
    ///
    /// Doubles both sides in place.
    pub fn double(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }
}
