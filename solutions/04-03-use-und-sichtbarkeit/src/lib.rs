//! 04-03 use und Sichtbarkeit / use and visibility, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/04-03-use-und-sichtbarkeit/README.md`. Hier stehen nur der Baum und
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/04-03-use-und-sichtbarkeit/README.md`. What is here is only the tree
//! and the bodies that turn the unit's tests green.

/// Die Umrechnungen, in zwei Modulen.
///
/// The conversions, in two modules.
pub mod messwerte {
    /// Alles, was von Celsius ausgeht / everything starting from Celsius.
    pub mod celsius {
        /// Rechnet Grad Celsius in Grad Fahrenheit um.
        ///
        /// Converts degrees Celsius into degrees Fahrenheit.
        ///
        /// ```
        /// use unit_04_03_use_und_sichtbarkeit::messwerte::celsius::to_fahrenheit;
        ///
        /// assert_eq!(to_fahrenheit(100), 212);
        /// assert_eq!(to_fahrenheit(0), 32);
        /// ```
        pub fn to_fahrenheit(grad: i32) -> i32 {
            grad * 9 / 5 + 32
        }

        /// Rechnet Grad Fahrenheit zurück in Grad Celsius.
        ///
        /// Converts degrees Fahrenheit back into degrees Celsius.
        pub fn from_fahrenheit(grad: i32) -> i32 {
            (grad - 32) * 5 / 9
        }
    }

    /// Alles, was mit Kelvin zu tun hat / everything to do with Kelvin.
    pub mod kelvin {
        /// Rechnet Grad Celsius in Kelvin um, gerundet auf ganze Grad.
        ///
        /// Converts degrees Celsius into Kelvin, rounded to whole degrees.
        pub fn from_celsius(grad: i32) -> i32 {
            grad + 273
        }
    }
}

pub use messwerte::celsius::to_fahrenheit;

use messwerte::celsius;
use messwerte::kelvin::from_celsius as kelvin_aus_celsius;

/// Gibt den Siedepunkt von Wasser in Grad Fahrenheit zurück.
///
/// Returns the boiling point of water in degrees Fahrenheit.
pub fn boiling_in_fahrenheit() -> i32 {
    celsius::to_fahrenheit(100)
}

/// Rechnet nach Fahrenheit und wieder zurück.
///
/// Converts to Fahrenheit and back again.
pub fn round_trip(grad: i32) -> i32 {
    celsius::from_fahrenheit(celsius::to_fahrenheit(grad))
}

/// Rechnet von Celsius nach Kelvin.
///
/// Converts from Celsius to Kelvin.
pub fn in_kelvin(grad: i32) -> i32 {
    kelvin_aus_celsius(grad)
}
