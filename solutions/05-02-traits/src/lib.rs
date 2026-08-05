//! 05-02 Traits / Traits, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-02-traits/README.md`. Hier stehen
//! nur der Trait, die Typen und die Rümpfe, die die Tests der Einheit grün
//! machen.
//!
//! English: the explanation lives in `units/05-02-traits/README.md`. What is
//! here is only the trait, the types and the bodies that turn the unit's tests
//! green.

/// Was eine Form können muss.
///
/// What a shape has to be able to do.
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

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    fn beschreibung(&self) -> String {
        format!("Quadrat mit Seite {}", self.seite)
    }
}

impl Flaeche for u32 {
    fn flaeche(&self) -> u32 {
        *self
    }
}
