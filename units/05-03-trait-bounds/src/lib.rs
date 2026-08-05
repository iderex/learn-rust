//! 05-03 Trait Bounds / Trait bounds
//!
//! Deutsch: Eine Schranke sagt, was ein Typparameter können muss. Mehrere
//! werden mit `+` verbunden, und `where` schreibt dieselben Schranken unter den
//! Kopf. Geprüft wird an der Aufrufstelle.
//!
//! English: a bound says what a type parameter has to be able to do. Several
//! are joined with `+`, and `where` writes the same bounds below the head. The
//! check happens at the call site.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::fmt::Display;

/// Gibt den kleinsten Wert einer Liste zurück.
///
/// Diese Funktion steht fertig da und zeigt die Form: die Schranke im Kopf,
/// der Vergleich im Rumpf.
///
/// Returns the smallest value of a list.
///
/// This function stands there finished and shows the shape: the bound in the
/// head, the comparison in the body.
///
/// ```
/// use unit_05_03_trait_bounds::smallest;
///
/// assert_eq!(smallest(&[3, 9, 4]), Some(&3));
/// assert_eq!(smallest(&["neun", "drei"]), Some(&"drei"));
/// assert_eq!(smallest::<i32>(&[]), None);
/// ```
pub fn smallest<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    let mut kleinster = werte.first()?;

    for wert in werte {
        if wert < kleinster {
            kleinster = wert;
        }
    }

    Some(kleinster)
}

/// Aufgabe 1: Gib den größten Wert einer Liste zurück.
///
/// Die Schranke `PartialOrd` steht schon im Kopf; ohne sie wäre der Vergleich
/// `E0369` wie in `05-01`. Eine leere Liste hat keinen größten Wert.
///
/// Exercise 1: return the biggest value of a list.
///
/// The bound `PartialOrd` already stands in the head; without it the comparison
/// would be `E0369` as in `05-01`. An empty list has no biggest value.
pub fn largest<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib einen Wert als Text zurück.
///
/// Heraus kommt "Wert " und dahinter der Wert. Möglich ist das nur mit der
/// Schranke `Display`.
///
/// Exercise 2: return a value as text.
///
/// What comes out is "Wert " followed by the value. That is only possible with
/// the bound `Display`.
pub fn reported<T: Display>(wert: T) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib den größten Wert einer Liste als Text zurück.
///
/// Hier werden beide Schranken gebraucht, und sie stehen in der Schreibweise
/// mit `where`. Bei einer leeren Liste steht dort "keine Werte".
///
/// Exercise 3: return the biggest value of a list as text.
///
/// Both bounds are needed here, and they stand in the form with `where`. For an
/// empty list it reads "keine Werte".
pub fn largest_reported<T>(werte: &[T]) -> String
where
    T: PartialOrd + Display,
{
    todo!("Aufgabe 3 / Exercise 3")
}
