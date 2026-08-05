//! Deutsch: Ein Untermodul in `src/zahlen/intern.rs`.
//!
//! English: a submodule in `src/zahlen/intern.rs`.

/// Gibt die Summe der beiden Zahlen und ihr Doppeltes zurück.
///
/// Returns the sum of the two numbers and its double.
pub fn summed_twice(a: i32, b: i32) -> i32 {
    super::summed(a, b) + crate::zahlen::summed(a, b)
}

/// Rundet `zahl` auf die nächste kleinere volle Zehn ab.
///
/// Rounds `zahl` down to the next lower full ten.
pub fn rounded_down(zahl: u32) -> u32 {
    zahl / 10 * 10
}
