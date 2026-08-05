//! 03-03 enum / enum, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/03-03-enum/README.md`. Hier stehen
//! nur der Typ und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/03-03-enum/README.md`. What is here
//! is only the type and the bodies that turn the unit's tests green.

/// Ein Messwert, in drei Fällen.
///
/// A reading, in three cases.
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

/// Gibt die Variante ohne Daten zurück.
///
/// Returns the variant without data.
pub fn missing() -> Reading {
    Reading::Missing
}

/// Gibt die Variante mit einer einzelnen Zahl zurück.
///
/// Returns the variant carrying a single number.
pub fn single(grad: i32) -> Reading {
    Reading::Temperature(grad)
}

/// Gibt die Variante mit zwei benannten Feldern zurück.
///
/// Returns the variant with two named fields.
pub fn range(von: i32, bis: i32) -> Reading {
    if von <= bis {
        Reading::Range { von, bis }
    } else {
        Reading::Range { von: bis, bis: von }
    }
}
