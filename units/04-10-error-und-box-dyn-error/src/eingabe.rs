//! Deutsch: Das Lesen der beiden Texte. Hier laufen zwei fremde Fehlerarten
//! zusammen, und `?` wandelt beide in den eigenen Fehlertyp um.
//!
//! English: the reading of the two texts. Two foreign error kinds come
//! together here, and `?` converts both into the error type of its own.

use crate::fehler::AppFehler;

/// Liest eine ganze Zahl und eine Kommazahl und addiert sie.
///
/// Diese Funktion steht fertig da. Sie zeigt, warum die beiden `From` gebraucht
/// werden: `parse` gibt zwei verschiedene fremde Fehler zurück, und beide `?`
/// machen daraus denselben `AppFehler`.
///
/// Reads a whole number and a decimal number and adds them.
///
/// This function stands there finished. It shows why the two `From` are needed:
/// `parse` returns two different foreign errors, and both `?` turn them into
/// the same `AppFehler`.
///
/// ```
/// use unit_04_10_error_und_box_dyn_error::eingabe::summe;
///
/// assert_eq!(summe("2", "0.5"), Ok(2.5));
/// assert!(summe("zwei", "0.5").is_err());
/// ```
pub fn summe(ganz: &str, komma: &str) -> Result<f64, AppFehler> {
    let a: i32 = ganz.trim().parse()?;
    let b: f64 = komma.trim().parse()?;

    Ok(f64::from(a) + b)
}
