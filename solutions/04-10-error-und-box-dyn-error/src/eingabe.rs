//! Deutsch: Das Lesen der beiden Texte.
//!
//! English: the reading of the two texts.

use crate::fehler::AppFehler;

/// Liest eine ganze Zahl und eine Kommazahl und addiert sie.
///
/// Reads a whole number and a decimal number and adds them.
pub fn summe(ganz: &str, komma: &str) -> Result<f64, AppFehler> {
    let a: i32 = ganz.trim().parse()?;
    let b: f64 = komma.trim().parse()?;

    Ok(f64::from(a) + b)
}
