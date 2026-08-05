//! 04-10 std::error::Error und Box<dyn Error>, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/04-10-error-und-box-dyn-error/README.md`. Die Aufteilung auf Module
//! ist dieselbe wie in der Einheit.
//!
//! English: the explanation lives in
//! `units/04-10-error-und-box-dyn-error/README.md`. The split over modules is
//! the same as in the unit.

pub mod eingabe;
pub mod fehler;

use std::error::Error;

/// Rechnet die beiden Texte zusammen, an der obersten Ebene.
///
/// Adds the two texts together, at the top level.
pub fn summe_aus_texten(ganz: &str, komma: &str) -> Result<f64, Box<dyn Error>> {
    let ergebnis = eingabe::summe(ganz, komma)?;

    Ok(ergebnis)
}
