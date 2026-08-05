//! 03-03 enum / enum
//!
//! Deutsch: Ein `enum` zählt die Fälle auf, die es gibt, und jede Variante darf
//! eigene Daten tragen. Ein Zustand, den es nicht geben darf, lässt sich damit
//! nicht mehr hinschreiben.
//!
//! English: an `enum` lists the cases there are, and every variant may carry
//! data of its own. A state that must not exist can no longer be written down
//! with it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Messwert, in drei Fällen.
///
/// `Missing` trägt nichts, `Temperature` eine Zahl, `Range` zwei benannte
/// Felder. Mehr Fälle gibt es nicht.
///
/// A reading, in three cases.
///
/// `Missing` carries nothing, `Temperature` one number, `Range` two named
/// fields. There are no further cases.
pub enum Reading {
    /// Kein Wert / no value.
    Missing,
    /// Ein einzelner Wert in Grad / a single value in degrees.
    Temperature(i32),
    /// Ein Bereich von `von` bis `bis` / a range from `von` to `bis`.
    Range {
        /// Die untere Grenze / the lower bound.
        von: i32,
        /// Die obere Grenze / the upper bound.
        bis: i32,
    },
}

/// Beschreibt einen Messwert als Text.
///
/// Diese Funktion steht schon fertig da. Sie nimmt die Variante mit `match`
/// auseinander, und `match` wird in `03-04` erklärt. Hier ist sie nur da, damit
/// die Tests die angelegten Werte ansehen können.
///
/// Describes a reading as text.
///
/// This function already stands there finished. It takes the variant apart with
/// `match`, and `match` is explained in `03-04`. Here it is only there so that
/// the tests can look at the values created.
///
/// ```
/// use unit_03_03_enum::{Reading, as_text};
///
/// assert_eq!(as_text(&Reading::Missing), "kein Wert");
/// assert_eq!(as_text(&Reading::Temperature(17)), "17 Grad");
/// assert_eq!(as_text(&Reading::Range { von: 3, bis: 9 }), "von 3 bis 9 Grad");
/// ```
pub fn as_text(messwert: &Reading) -> String {
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
        Reading::Range { von, bis } => format!("von {von} bis {bis} Grad"),
    }
}

/// Aufgabe 1: Gib die Variante ohne Daten zurück.
///
/// Exercise 1: return the variant without data.
pub fn missing() -> Reading {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib die Variante mit einer einzelnen Zahl zurück.
///
/// Exercise 2: return the variant carrying a single number.
pub fn single(grad: i32) -> Reading {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die Variante mit zwei benannten Feldern zurück.
///
/// `von` darf nie größer als `bis` sein. Kommen die beiden verkehrt herum
/// herein, werden sie getauscht.
///
/// Exercise 3: return the variant with two named fields.
///
/// `von` may never be bigger than `bis`. If the two come in the wrong way
/// round, they get swapped.
pub fn range(von: i32, bis: i32) -> Reading {
    todo!("Aufgabe 3 / Exercise 3")
}
