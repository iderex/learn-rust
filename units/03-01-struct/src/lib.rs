//! 03-01 struct / struct
//!
//! Deutsch: Ein `struct` fasst mehrere Werte zu einem zusammen. Es gibt die
//! Form mit benannten Feldern, das Tupel-Struct mit Nummern statt Namen und das
//! Struct ohne Feld. Beim Anlegen müssen alle Felder dastehen.
//!
//! English: a `struct` gathers several values into one. There is the form with
//! named fields, the tuple struct with numbers instead of names, and the struct
//! without a field. When a value is created every field has to be there.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

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
/// Das Feld hat keinen Namen und heißt `.0`. Eine Funktion, die `Meter`
/// erwartet, nimmt keine beliebige Zahl entgegen.
///
/// A length in metres, as a tuple struct.
///
/// The field has no name and is called `.0`. A function expecting `Meter` does
/// not accept just any number.
pub struct Meter(pub u32);

/// Ein Struct ohne Feld.
///
/// Es trägt keine Daten und belegt keinen Platz. Hier steht es, damit die
/// dritte Form einmal dagestanden hat.
///
/// A struct without a field.
///
/// It carries no data and takes up no space. It stands here so that the third
/// form has been seen once.
pub struct Marker;

/// Gibt ein Rechteck zurück, dessen Seiten mit `faktor` vervielfacht sind.
///
/// Das geliehene Rechteck bleibt unverändert, und das neue entsteht aus seinen
/// Feldern.
///
/// Returns a rectangle whose sides are multiplied by `faktor`.
///
/// The borrowed rectangle stays unchanged, and the new one comes out of its
/// fields.
///
/// ```
/// use unit_03_01_struct::{Rectangle, scaled};
///
/// let rechteck = Rectangle {
///     breite: 3,
///     hoehe: 4,
/// };
///
/// let doppelt = scaled(&rechteck, 2);
///
/// assert_eq!(doppelt.breite, 6);
/// assert_eq!(doppelt.hoehe, 8);
/// assert_eq!(rechteck.breite, 3);
/// ```
pub fn scaled(rechteck: &Rectangle, faktor: u32) -> Rectangle {
    Rectangle {
        breite: rechteck.breite * faktor,
        hoehe: rechteck.hoehe * faktor,
    }
}

/// Aufgabe 1: Lege ein `Rectangle` aus `breite` und `hoehe` an.
///
/// Beide Felder müssen dastehen, sonst ist es `E0063`.
///
/// Exercise 1: create a `Rectangle` out of `breite` and `hoehe`.
///
/// Both fields have to be there, otherwise it is `E0063`.
pub fn new_rectangle(breite: u32, hoehe: u32) -> Rectangle {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib die Fläche des geliehenen Rechtecks zurück.
///
/// Gelesen wird mit dem Punkt, und das Rechteck gehört weiter dem Aufrufer.
///
/// Exercise 2: return the area of the borrowed rectangle.
///
/// Reading goes with the dot, and the rectangle still belongs to the caller.
pub fn area_of(rechteck: &Rectangle) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die Zahl aus einem `Meter` zurück.
///
/// Das Feld eines Tupel-Structs heißt `.0`.
///
/// Exercise 3: return the number out of a `Meter`.
///
/// The field of a tuple struct is called `.0`.
pub fn in_meters(strecke: &Meter) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
