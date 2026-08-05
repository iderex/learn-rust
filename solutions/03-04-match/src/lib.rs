//! 03-04 match / match, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/03-04-match/README.md`. Hier stehen
//! nur der Typ und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/03-04-match/README.md`. What is
//! here is only the type and the bodies that turn the unit's tests green.

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
/// Describes a reading as text.
pub fn as_text(messwert: &Reading) -> String {
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
        Reading::Range { von, bis } => format!("von {von} bis {bis} Grad"),
    }
}

/// Gibt die höchste Zahl eines Messwerts zurück.
///
/// Returns the highest number of a reading.
pub fn highest(messwert: &Reading) -> i32 {
    match messwert {
        Reading::Missing => 0,
        Reading::Temperature(grad) => *grad,
        Reading::Range { bis, .. } => *bis,
    }
}

/// Gibt zu jedem Fall ein Wort zurück.
///
/// Returns one word for each case.
pub fn label(messwert: &Reading) -> &'static str {
    match messwert {
        Reading::Missing => "leer",
        Reading::Temperature(_) => "einzeln",
        Reading::Range { .. } => "bereich",
    }
}

/// Sagt, wie viele Zahlen ein Fall trägt.
///
/// Says how many numbers a case carries.
pub fn carried_values(messwert: &Reading) -> u32 {
    match messwert {
        Reading::Missing => 0,
        Reading::Temperature(_) => 1,
        _ => 2,
    }
}
