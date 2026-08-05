//! Deutsch: Ein Untermodul in `src/zahlen/intern.rs`. Sein Pfad ist
//! `zahlen::intern`, und beide Module tragen `pub`, sonst käme niemand
//! heran.
//!
//! English: a submodule in `src/zahlen/intern.rs`. Its path is
//! `zahlen::intern`, and both modules carry `pub`, otherwise nobody would get
//! to it.

// Deutsch: Die Aufgabe ist offen, ihr Rumpf ist `todo!()`, und der Parameter
// bleibt deshalb ungenutzt, bis jemand sie löst.
// English: The exercise is open, its body is `todo!()`, and its parameter
// therefore stays unused until somebody solves it.
#![allow(unused_variables)]

/// Gibt die Summe der beiden Zahlen und ihr Doppeltes zurück.
///
/// Der Pfad `super::summed` geht einen Schritt nach oben, `crate::zahlen::summed`
/// beginnt an der Wurzel, und beide meinen dieselbe Funktion.
///
/// Returns the sum of the two numbers and its double.
///
/// The path `super::summed` goes one step up, `crate::zahlen::summed` starts at
/// the root, and both mean the same function.
///
/// ```
/// use unit_04_02_module::zahlen::intern::summed_twice;
///
/// assert_eq!(summed_twice(20, 22), 84);
/// ```
pub fn summed_twice(a: i32, b: i32) -> i32 {
    super::summed(a, b) + crate::zahlen::summed(a, b)
}

/// Aufgabe 2: Runde `zahl` auf die nächste kleinere volle Zehn ab.
///
/// Aus 47 wird 40, aus 40 wird 40. Gelöst wird das in `src/zahlen/intern.rs`.
///
/// Exercise 2: round `zahl` down to the next lower full ten.
///
/// Out of 47 comes 40, out of 40 comes 40. This is solved in
/// `src/zahlen/intern.rs`.
pub fn rounded_down(zahl: u32) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}
