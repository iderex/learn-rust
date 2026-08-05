//! 05-03 Trait Bounds / Trait bounds, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-03-trait-bounds/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/05-03-trait-bounds/README.md`. What
//! is here is only the bodies that turn the unit's tests green.

use std::fmt::Display;

/// Gibt den kleinsten Wert einer Liste zurück.
///
/// Returns the smallest value of a list.
pub fn smallest<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    let mut kleinster = werte.first()?;

    for wert in werte {
        if wert < kleinster {
            kleinster = wert;
        }
    }

    Some(kleinster)
}

/// Gibt den größten Wert einer Liste zurück.
///
/// Returns the biggest value of a list.
pub fn largest<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    let mut groesster = werte.first()?;

    for wert in werte {
        if wert > groesster {
            groesster = wert;
        }
    }

    Some(groesster)
}

/// Gibt einen Wert als Text zurück.
///
/// Returns a value as text.
pub fn reported<T: Display>(wert: T) -> String {
    format!("Wert {wert}")
}

/// Gibt den größten Wert einer Liste als Text zurück.
///
/// Returns the biggest value of a list as text.
pub fn largest_reported<T>(werte: &[T]) -> String
where
    T: PartialOrd + Display,
{
    match largest(werte) {
        Some(wert) => format!("groesster Wert {wert}"),
        None => String::from("keine Werte"),
    }
}
