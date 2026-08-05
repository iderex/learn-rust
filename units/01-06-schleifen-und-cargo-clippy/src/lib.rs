//! 01-06 Schleifen und cargo clippy / Loops and cargo clippy
//!
//! Deutsch: `for` läuft über einen Bereich, `while` läuft, solange eine
//! Bedingung zutrifft, und `loop` läuft, bis ein `break` ihn beendet. Am `break`
//! darf ein Wert stehen, und dann hat die Schleife einen Wert. `cargo clippy`
//! sagt zusätzlich, wo etwas umständlich geschrieben ist.
//!
//! English: `for` runs over a range, `while` runs as long as a condition holds,
//! and `loop` runs until a `break` ends it. A value may stand at the `break`,
//! and then the loop has a value. `cargo clippy` says on top of that where
//! something is written the long way round.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Addiert mit `for` alle Zahlen von 1 bis einschließlich `n`.
///
/// `1..=n` nimmt die `n` mit. Bei `n` gleich null läuft die Schleife keinen
/// Durchgang, und die Summe bleibt null.
///
/// Adds every number from 1 up to and including `n` with `for`.
///
/// `1..=n` takes the `n` along. For `n` equal to zero the loop runs no pass at
/// all, and the sum stays zero.
///
/// ```
/// use unit_01_06_schleifen_und_cargo_clippy::sum_to;
///
/// assert_eq!(sum_to(5), 15);
/// assert_eq!(sum_to(0), 0);
/// ```
pub fn sum_to(n: u32) -> u32 {
    let mut summe = 0;

    for zahl in 1..=n {
        summe += zahl;
    }

    summe
}

/// Aufgabe 1: Multipliziere mit `for` alle Zahlen von 1 bis einschließlich `n`.
///
/// Das leere Produkt ist eins, also sind `product_to(0)` und `product_to(1)`
/// beide eins.
///
/// Exercise 1: multiply every number from 1 up to and including `n` with `for`.
///
/// The empty product is one, so `product_to(0)` and `product_to(1)` are both
/// one.
pub fn product_to(n: u32) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Zähle mit `while` die Stellen von `zahl`.
///
/// Die Null hat eine Stelle. Teilen durch zehn nimmt bei ganzen Zahlen je eine
/// Stelle weg.
///
/// Exercise 2: count the digits of `zahl` with `while`.
///
/// Zero has one digit. Dividing by ten takes one digit away at a time for whole
/// numbers.
pub fn digit_count(zahl: u32) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Suche mit `loop` die erste Quadratzahl über `grenze`.
///
/// Gesucht ist das kleinste Quadrat, das echt größer als `grenze` ist. Der Wert
/// steht am `break`.
///
/// Exercise 3: look for the first square above `grenze` with `loop`.
///
/// Wanted is the smallest square strictly greater than `grenze`. The value
/// stands at the `break`.
pub fn first_square_over(grenze: u32) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
