//! 01-04 Kommentare und cargo fmt / Comments and cargo fmt
//!
//! Deutsch: Ein Kommentar mit `//` erklärt einer Person etwas. Ein
//! Doku-Kommentar mit `///` gehört zu dem, was darunter steht, und seine
//! Beispiele werden mitgetestet. `cargo fmt` legt die Form fest, damit niemand
//! darüber streiten muss.
//!
//! English: a comment with `//` explains something to a person. A doc comment
//! with `///` belongs to whatever stands below it, and its examples are tested
//! along with everything else. `cargo fmt` fixes the shape so that nobody has to
//! argue about it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Bruttobetrag zu einem Nettobetrag in Cent zurück.
///
/// Der Satz ist 19 Prozent. Gerechnet wird in Cent, damit keine
/// Fließkommazahlen ins Spiel kommen.
///
/// Returns the gross amount for a net amount in cents.
///
/// The rate is 19 percent. The calculation runs in cents so that no floating
/// point numbers come into play.
///
/// ```
/// use unit_01_04_kommentare_und_cargo_fmt::gross_of;
///
/// assert_eq!(gross_of(100), 119);
/// ```
pub fn gross_of(netto_cent: u32) -> u32 {
    netto_cent + netto_cent * 19 / 100
}

/// Aufgabe 1: Gib die Mehrwertsteuer auf `netto_cent` zurück, 19 Prozent.
///
/// Ganze Zahlen teilen ohne Rest, der Rest fällt weg. Das ist hier gewollt.
///
/// Exercise 1: return the value added tax on `netto_cent`, 19 percent.
///
/// Whole numbers divide without a remainder, the remainder falls away. That is
/// intended here.
pub fn vat_of(netto_cent: u32) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib `betrag_cent` mit zehn Prozent Nachlass zurück.
///
/// Exercise 2: return `betrag_cent` with ten percent taken off.
pub fn discounted(betrag_cent: u32) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Runde `betrag_cent` auf den nächsten vollen Euro auf.
///
/// Ein voller Euro sind 100 Cent. Ein Betrag, der schon voll ist, bleibt, wie
/// er ist.
///
/// Exercise 3: round `betrag_cent` up to the next full euro.
///
/// A full euro is 100 cents. An amount that is already full stays as it is.
pub fn rounded_up_to_full_euro(betrag_cent: u32) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
