//! 01-04 Kommentare und cargo fmt / Comments and cargo fmt, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/01-04-kommentare-und-cargo-fmt/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/01-04-kommentare-und-cargo-fmt/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Gibt den Bruttobetrag zu einem Nettobetrag in Cent zurück.
///
/// Returns the gross amount for a net amount in cents.
pub fn gross_of(netto_cent: u32) -> u32 {
    netto_cent + netto_cent * 19 / 100
}

/// Gibt die Mehrwertsteuer auf `netto_cent` zurück.
///
/// Returns the value added tax on `netto_cent`.
pub fn vat_of(netto_cent: u32) -> u32 {
    netto_cent * 19 / 100
}

/// Gibt `betrag_cent` mit zehn Prozent Nachlass zurück.
///
/// Returns `betrag_cent` with ten percent taken off.
pub fn discounted(betrag_cent: u32) -> u32 {
    betrag_cent - betrag_cent * 10 / 100
}

/// Rundet `betrag_cent` auf den nächsten vollen Euro auf.
///
/// Rounds `betrag_cent` up to the next full euro.
pub fn rounded_up_to_full_euro(betrag_cent: u32) -> u32 {
    betrag_cent.div_ceil(100) * 100
}
