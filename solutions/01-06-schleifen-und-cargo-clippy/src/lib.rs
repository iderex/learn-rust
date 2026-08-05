//! 01-06 Schleifen und cargo clippy / Loops and cargo clippy, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/01-06-schleifen-und-cargo-clippy/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/01-06-schleifen-und-cargo-clippy/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Addiert mit `for` alle Zahlen von 1 bis einschließlich `n`.
///
/// Adds every number from 1 up to and including `n` with `for`.
pub fn sum_to(n: u32) -> u32 {
    let mut summe = 0;

    for zahl in 1..=n {
        summe += zahl;
    }

    summe
}

/// Multipliziert mit `for` alle Zahlen von 1 bis einschließlich `n`.
///
/// Multiplies every number from 1 up to and including `n` with `for`.
pub fn product_to(n: u32) -> u32 {
    let mut produkt = 1;

    for faktor in 2..=n {
        produkt *= faktor;
    }

    produkt
}

/// Zählt mit `while` die Stellen von `zahl`.
///
/// Counts the digits of `zahl` with `while`.
pub fn digit_count(zahl: u32) -> u32 {
    let mut rest = zahl / 10;
    let mut stellen = 1;

    while rest > 0 {
        rest /= 10;
        stellen += 1;
    }

    stellen
}

/// Sucht mit `loop` die erste Quadratzahl über `grenze`.
///
/// Looks for the first square above `grenze` with `loop`.
pub fn first_square_over(grenze: u32) -> u32 {
    let mut zahl = 1;

    loop {
        let quadrat = zahl * zahl;

        if quadrat > grenze {
            break quadrat;
        }

        zahl += 1;
    }
}
