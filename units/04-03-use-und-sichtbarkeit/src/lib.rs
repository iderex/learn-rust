//! 04-03 use und Sichtbarkeit / use and visibility
//!
//! Deutsch: `use` kürzt einen Pfad, `as` gibt ihm einen zweiten Namen, und
//! `pub use` gibt ihn nach außen weiter. An der Sichtbarkeit ändert keines der
//! drei etwas.
//!
//! English: `use` shortens a path, `as` gives it a second name, and `pub use`
//! hands it on outwards. None of the three changes anything about visibility.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

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

// Deutsch: `pub use` gibt den Namen nach außen weiter. Von außen heißt die
// Funktion danach `to_fahrenheit`, und der lange Pfad geht weiterhin auch.
// English: `pub use` hands the name on outwards. From outside the function is
// called `to_fahrenheit` afterwards, and the long path keeps working too.
pub use messwerte::celsius::to_fahrenheit;

/// Aufgabe 1: Gib den Siedepunkt von Wasser in Grad Fahrenheit zurück.
///
/// Der Siedepunkt sind 100 Grad Celsius. Gerechnet wird mit der Funktion aus
/// dem Baum, nicht mit einer eigenen Formel.
///
/// Exercise 1: return the boiling point of water in degrees Fahrenheit.
///
/// The boiling point is 100 degrees Celsius. Use the function from the tree and
/// not a formula of your own.
pub fn boiling_in_fahrenheit() -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Rechne nach Fahrenheit und wieder zurück.
///
/// Beide Funktionen stehen im Modul `celsius`. Durch das Teilen ganzer Zahlen
/// kommt nicht immer dieselbe Zahl heraus, und die Tests sagen, welche.
///
/// Exercise 2: convert to Fahrenheit and back again.
///
/// Both functions stand in the module `celsius`. Because whole numbers divide
/// without a remainder the same number does not always come back, and the tests
/// say which one does.
pub fn round_trip(grad: i32) -> i32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Rechne von Celsius nach Kelvin.
///
/// Die Funktion dafür steht im anderen Ast des Baums.
///
/// Exercise 3: convert from Celsius to Kelvin.
///
/// The function for it stands in the other branch of the tree.
pub fn in_kelvin(grad: i32) -> i32 {
    todo!("Aufgabe 3 / Exercise 3")
}
