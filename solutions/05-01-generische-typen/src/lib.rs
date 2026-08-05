//! 05-01 Generische Typen / Generic types, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-01-generische-typen/README.md`.
//! Hier stehen nur der Typ und die Rümpfe, die die Tests der Einheit grün
//! machen.
//!
//! English: the explanation lives in
//! `units/05-01-generische-typen/README.md`. What is here is only the type and
//! the bodies that turn the unit's tests green.

/// Ein Paar aus zwei Werten desselben Typs.
///
/// A pair of two values of the same type.
#[derive(Debug, PartialEq)]
pub struct Paar<T> {
    /// Die linke Seite / the left side.
    pub links: T,
    /// Die rechte Seite / the right side.
    pub rechts: T,
}

/// Gibt den ersten Wert einer Liste zurück, falls es einen gibt.
///
/// Returns the first value of a list, if there is one.
pub fn first_of<T>(werte: &[T]) -> Option<&T> {
    werte.first()
}

/// Gibt den letzten Wert einer Liste zurück.
///
/// Returns the last value of a list.
pub fn last_of<T>(werte: &[T]) -> Option<&T> {
    werte.last()
}

impl<T> Paar<T> {
    /// Legt ein Paar aus zwei Werten an.
    ///
    /// Creates a pair out of two values.
    pub fn new(links: T, rechts: T) -> Self {
        Paar { links, rechts }
    }
}

/// Vertauscht die beiden Seiten eines Paars.
///
/// Swaps the two sides of a pair.
pub fn swapped<T>(paar: Paar<T>) -> Paar<T> {
    Paar {
        links: paar.rechts,
        rechts: paar.links,
    }
}
