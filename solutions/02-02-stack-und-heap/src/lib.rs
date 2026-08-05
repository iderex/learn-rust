//! 02-02 Stack und Heap / Stack and heap, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/02-02-stack-und-heap/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/02-02-stack-und-heap/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Gibt die Summe zweier Zahlen zurück.
///
/// Returns the sum of two numbers.
pub fn sum_of(a: i32, b: i32) -> i32 {
    a + b
}

/// Gibt das Doppelte von `zahl` zurück.
///
/// Returns the double of `zahl`.
pub fn twice(zahl: i32) -> i32 {
    zahl * 2
}

/// Hängt ein Ausrufezeichen an und gibt den `String` zurück.
///
/// Appends an exclamation mark and returns the `String`.
pub fn with_exclamation(text: String) -> String {
    let mut text = text;
    text.push('!');
    text
}

/// Sagt zu einem Typnamen, ob eine Zuweisung ihn kopiert.
///
/// Says for a type name whether an assignment copies it.
pub fn copies_on_assignment(typ: &str) -> bool {
    typ == "i32"
        || typ == "u8"
        || typ == "f64"
        || typ == "bool"
        || typ == "char"
        || typ == "&str"
        || typ == "(i32, bool)"
}
