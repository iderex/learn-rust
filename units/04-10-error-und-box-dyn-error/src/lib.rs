//! 04-10 std::error::Error und Box<dyn Error> / std::error::Error and Box<dyn Error>
//!
//! Deutsch: Diese Datei ist die Wurzel der Crate und die oberste Ebene. Der
//! eigene Fehlertyp steht in `src/fehler.rs`, das Lesen in `src/eingabe.rs`,
//! und hier läuft beides als `Box<dyn Error>` zusammen.
//!
//! English: this file is the root of the crate and the top level. The error
//! type of its own stands in `src/fehler.rs`, the reading in `src/eingabe.rs`,
//! and here both come together as `Box<dyn Error>`.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

pub mod eingabe;
pub mod fehler;

use std::error::Error;

/// Aufgabe 3: Rechne die beiden Texte zusammen, an der obersten Ebene.
///
/// Gerufen wird `eingabe::summe`, und der Fehler von dort wird mit `?`
/// weitergegeben. Der Rückgabetyp ist `Box<dyn Error>`, und die Umwandlung
/// dorthin macht `?` von selbst, weil der eigene Fehlertyp `Error` kann.
///
/// Exercise 3: add the two texts together, at the top level.
///
/// `eingabe::summe` is called, and the error from there is passed on with `?`.
/// The return type is `Box<dyn Error>`, and the conversion into it is done by
/// `?` itself, because the error type of its own implements `Error`.
pub fn summe_aus_texten(ganz: &str, komma: &str) -> Result<f64, Box<dyn Error>> {
    todo!("Aufgabe 3 / Exercise 3")
}
