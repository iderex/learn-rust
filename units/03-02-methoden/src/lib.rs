//! 03-02 Methoden / Methods
//!
//! Deutsch: Ein `impl`-Block sammelt, was zu einem Typ gehört. Eine Methode hat
//! `self` als ersten Parameter, in denselben drei Formen wie jeder andere
//! Parameter, und eine zugeordnete Funktion hat kein `self`.
//!
//! English: an `impl` block gathers what belongs to a type. A method has `self`
//! as its first parameter, in the same three forms as any other parameter, and
//! an associated function has no `self`.

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

impl Rectangle {
    /// Gibt die Fläche zurück.
    ///
    /// `&self` liest nur, das Rechteck bleibt beim Aufrufer und unverändert.
    ///
    /// Returns the area.
    ///
    /// `&self` only reads, the rectangle stays with the caller and unchanged.
    ///
    /// ```
    /// use unit_03_02_methoden::Rectangle;
    ///
    /// let rechteck = Rectangle {
    ///     breite: 3,
    ///     hoehe: 4,
    /// };
    ///
    /// assert_eq!(rechteck.area(), 12);
    /// assert_eq!(rechteck.breite, 3);
    /// ```
    pub fn area(&self) -> u32 {
        self.breite * self.hoehe
    }

    /// Aufgabe 1: Lege ein Rechteck aus `breite` und `hoehe` an.
    ///
    /// Kein `self`, denn es gibt noch keinen Wert. Der Rückgabetyp `Self` ist
    /// `Rectangle`.
    ///
    /// Exercise 1: create a rectangle out of `breite` and `hoehe`.
    ///
    /// No `self`, because there is no value yet. The return type `Self` is
    /// `Rectangle`.
    pub fn new(breite: u32, hoehe: u32) -> Self {
        todo!("Aufgabe 1 / Exercise 1")
    }

    /// Aufgabe 2: Gib den Umfang zurück.
    ///
    /// Der Umfang ist zweimal die Breite plus zweimal die Höhe. Gelesen wird
    /// nur, also `&self`.
    ///
    /// Exercise 2: return the perimeter.
    ///
    /// The perimeter is twice the width plus twice the height. Only reading
    /// happens, so `&self`.
    pub fn perimeter(&self) -> u32 {
        todo!("Aufgabe 2 / Exercise 2")
    }

    /// Aufgabe 3: Verdopple beide Seiten an Ort und Stelle.
    ///
    /// Zurück kommt nichts, denn der Aufrufer hat den Wert schon. Verändert
    /// wird durch `&mut self`.
    ///
    /// Exercise 3: double both sides in place.
    ///
    /// Nothing comes back, because the caller already has the value. The change
    /// goes through `&mut self`.
    pub fn double(&mut self) {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
