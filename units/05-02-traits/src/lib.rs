//! 05-02 Traits / Traits
//!
//! Deutsch: Ein Trait sagt, was ein Typ können muss. Eine Methode ohne Rumpf
//! schreibt jeder Typ selbst, eine mit Rumpf ist die Standardfassung, und ein
//! eigener Trait darf auch für einen fremden Typ geschrieben werden.
//!
//! English: a trait says what a type has to be able to do. A method without a
//! body is written by every type itself, one with a body is the default
//! version, and a trait of your own may be written for a foreign type as well.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Was eine Form können muss.
///
/// `flaeche` hat keinen Rumpf und wird von jedem Typ selbst geschrieben.
/// `beschreibung` hat einen und ist die Standardfassung.
///
/// What a shape has to be able to do.
///
/// `flaeche` has no body and is written by every type itself. `beschreibung`
/// has one and is the default version.
pub trait Flaeche {
    /// Die Fläche der Form / the area of the shape.
    fn flaeche(&self) -> u32;

    /// Eine Beschreibung für Menschen / a description for people.
    fn beschreibung(&self) -> String {
        format!("Flaeche {}", self.flaeche())
    }
}

/// Ein Rechteck aus Breite und Höhe.
///
/// A rectangle made of a width and a height.
pub struct Rechteck {
    /// Die Breite / the width.
    pub breite: u32,
    /// Die Höhe / the height.
    pub hoehe: u32,
}

/// Ein Quadrat mit einer Seitenlänge.
///
/// A square with one side length.
pub struct Quadrat {
    /// Die Seitenlänge / the side length.
    pub seite: u32,
}

/// Diese Implementierung steht fertig da und ist die Vorlage für die Aufgaben.
///
/// This implementation stands there finished and is the model for the
/// exercises.
///
/// ```
/// use unit_05_02_traits::{Flaeche, Rechteck};
///
/// let rechteck = Rechteck {
///     breite: 3,
///     hoehe: 4,
/// };
///
/// assert_eq!(rechteck.flaeche(), 12);
///
/// // Deutsch: Die Standardfassung aus dem Trait, ungeschrieben übernommen.
/// // English: the default version from the trait, taken over unwritten.
/// assert_eq!(rechteck.beschreibung(), "Flaeche 12");
/// ```
impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }
}

// Deutsch: Aufgabe 1 und 2 stehen in diesem Block. Aufgabe 1 ist die verlangte
// Methode, Aufgabe 2 überschreibt die Standardfassung mit
// "Quadrat mit Seite <zahl>".
// English: exercises 1 and 2 stand in this block. Exercise 1 is the method that
// is demanded, exercise 2 overrides the default version with
// "Quadrat mit Seite <zahl>".
impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        todo!("Aufgabe 1 / Exercise 1")
    }

    fn beschreibung(&self) -> String {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

// Deutsch: Aufgabe 3: Der eigene Trait für einen fremden Typ. Die Zahl ist ihre
// eigene Fläche.
// English: Exercise 3: the trait of your own for a foreign type. The number is
// its own area.
impl Flaeche for u32 {
    fn flaeche(&self) -> u32 {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
