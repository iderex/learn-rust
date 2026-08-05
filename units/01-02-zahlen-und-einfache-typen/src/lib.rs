//! 01-02 Zahlen und andere einfache Typen / Numbers and other simple types
//!
//! Deutsch: Ganze Zahlen mit und ohne Vorzeichen, Fließkommazahlen, `bool` und
//! `char`. Jeder dieser Typen hat einen festen Bereich, und Rust rechnet nie
//! stillschweigend von einem in den anderen um.
//!
//! English: whole numbers with and without a sign, floating point numbers,
//! `bool` and `char`. Each of these types has a fixed range, and Rust never
//! converts silently from one into another.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt zurück, wie viele Sekunden `minuten` Minuten sind.
///
/// Returns how many seconds `minuten` minutes are.
///
/// ```
/// use unit_01_02_zahlen_und_einfache_typen::seconds_of;
///
/// assert_eq!(seconds_of(2), 120);
/// ```
pub fn seconds_of(minuten: u32) -> u32 {
    minuten * 60
}

/// Aufgabe 1: Gib zurück, ob `wert` in ein `u8` passt.
///
/// Der Bereich von `u8` steht in der Erklärung. Vergleiche, statt umzuwandeln.
///
/// Exercise 1: return whether `wert` fits into a `u8`.
///
/// The range of `u8` is in the explanation. Compare rather than convert.
pub fn fits_in_u8(wert: u32) -> bool {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib `wert` als `u32` zurück.
///
/// Von `u8` nach `u32` geht jeder Wert verlustfrei. Die Umwandlung wird
/// trotzdem hingeschrieben, denn Rust macht sie nicht von selbst.
///
/// Exercise 2: return `wert` as a `u32`.
///
/// From `u8` to `u32` every value fits without loss. The conversion is written
/// out all the same, because Rust does not make it by itself.
pub fn widened(wert: u8) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die Hälfte von `wert` zurück.
///
/// `f64` teilt anders als eine ganze Zahl: aus 5 wird 2.5 und nicht 2.
///
/// Exercise 3: return half of `wert`.
///
/// `f64` divides differently from a whole number: 5 becomes 2.5 and not 2.
pub fn half(wert: f64) -> f64 {
    todo!("Aufgabe 3 / Exercise 3")
}

/// Aufgabe 4: Gib zurück, ob `zeichen` einer der Buchstaben einer
/// Hexadezimalziffer ist, also a bis f.
///
/// Ein `char` lässt sich vergleichen wie eine Zahl, also mit `>=` und `<=`.
/// Diese Fassung ist richtig; `cargo clippy` schlägt später die kürzere mit
/// einem Bereich vor, und die Lösung zeigt sie.
///
/// Exercise 4: return whether `zeichen` is one of the letters of a hexadecimal
/// digit, meaning a to f.
///
/// A `char` compares like a number, meaning with `>=` and `<=`. That version is
/// correct; `cargo clippy` later proposes the shorter one with a range, and the
/// solution shows it.
pub fn is_hex_letter(zeichen: char) -> bool {
    todo!("Aufgabe 4 / Exercise 4")
}
