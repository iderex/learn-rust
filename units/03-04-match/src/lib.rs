//! 03-04 match / match
//!
//! Deutsch: `match` vergleicht einen Wert mit Mustern, holt die Daten einer
//! Variante gleich heraus und verlangt, dass jeder Fall behandelt wird. `_`
//! fängt den Rest ab und nimmt genau diese Prüfung weg.
//!
//! English: `match` compares a value against patterns, pulls the data of a
//! variant out on the way, and demands that every case is handled. `_` catches
//! the rest and removes exactly that check.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Messwert, in drei Fällen, wie in `03-03`.
///
/// A reading, in three cases, as in `03-03`.
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
/// Ein vollständiges `match` über alle drei Fälle. Alle Zweige liefern einen
/// `String`, sonst hätte der Ausdruck keinen Typ.
///
/// Describes a reading as text.
///
/// One exhaustive `match` over all three cases. Every arm delivers a `String`,
/// otherwise the expression would have no type.
///
/// ```
/// use unit_03_04_match::{Reading, as_text};
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

/// Aufgabe 1: Gib die höchste Zahl eines Messwerts zurück.
///
/// `Missing` trägt keine Zahl, die Antwort ist dort die Null. Bei einem Bereich
/// ist es die obere Grenze.
///
/// Exercise 1: return the highest number of a reading.
///
/// `Missing` carries no number, and the answer there is zero. For a range it is
/// the upper bound.
pub fn highest(messwert: &Reading) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib zu jedem Fall ein Wort zurück.
///
/// "leer" für `Missing`, "einzeln" für einen Wert, "bereich" für einen Bereich.
///
/// Exercise 2: return one word for each case.
///
/// "leer" for `Missing`, "einzeln" for a single value, "bereich" for a range.
pub fn label(messwert: &Reading) -> &'static str {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag, wie viele Zahlen ein Fall trägt.
///
/// `Missing` trägt keine, `Temperature` eine, ein Bereich zwei. Der letzte Fall
/// wird mit `_` abgefangen, und die Zahl in `Temperature` interessiert nicht,
/// also steht auch dort ein `_`.
///
/// Exercise 3: say how many numbers a case carries.
///
/// `Missing` carries none, `Temperature` one, a range two. The last case is
/// caught with `_`, and the number inside `Temperature` is of no interest, so a
/// `_` stands there as well.
pub fn carried_values(messwert: &Reading) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
