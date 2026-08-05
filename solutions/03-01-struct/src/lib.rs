//! 03-01 struct / struct, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/03-01-struct/README.md`. Hier stehen
//! nur die Typen und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/03-01-struct/README.md`. What is
//! here is only the types and the bodies that turn the unit's tests green.

/// Ein Rechteck aus Breite und Höhe.
///
/// A rectangle made of a width and a height.
pub struct Rectangle {
    /// Die Breite / the width.
    pub breite: u32,
    /// Die Höhe / the height.
    pub hoehe: u32,
}

/// Eine Länge in Metern, als Tupel-Struct.
///
/// A length in metres, as a tuple struct.
pub struct Meter(pub u32);

/// Ein Struct ohne Feld.
///
/// A struct without a field.
pub struct Marker;

/// Gibt ein Rechteck zurück, dessen Seiten mit `faktor` vervielfacht sind.
///
/// Returns a rectangle whose sides are multiplied by `faktor`.
pub fn scaled(rechteck: &Rectangle, faktor: u32) -> Rectangle {
    Rectangle {
        breite: rechteck.breite * faktor,
        hoehe: rechteck.hoehe * faktor,
    }
}

/// Legt ein `Rectangle` aus `breite` und `hoehe` an.
///
/// Creates a `Rectangle` out of `breite` and `hoehe`.
pub fn new_rectangle(breite: u32, hoehe: u32) -> Rectangle {
    Rectangle { breite, hoehe }
}

/// Gibt die Fläche des geliehenen Rechtecks zurück.
///
/// Returns the area of the borrowed rectangle.
pub fn area_of(rechteck: &Rectangle) -> u32 {
    rechteck.breite * rechteck.hoehe
}

/// Gibt die Zahl aus einem `Meter` zurück.
///
/// Returns the number out of a `Meter`.
pub fn in_meters(strecke: &Meter) -> u32 {
    strecke.0
}
