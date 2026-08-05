//! 03-05 Option und if let / Option and if let, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/03-05-option-und-if-let/README.md`. Hier stehen nur die Rümpfe, die
//! die Tests der Einheit grün machen. `unwrap` kommt in keinem davon vor.
//!
//! English: the explanation lives in
//! `units/03-05-option-und-if-let/README.md`. What is here is only the bodies
//! that turn the unit's tests green. `unwrap` appears in none of them.

/// Gibt die erste Zahl eines Slice zurück, falls es eine gibt.
///
/// Returns the first number of a slice, if there is one.
pub fn first_of(zahlen: &[i32]) -> Option<i32> {
    zahlen.first().copied()
}

/// Gibt zu einer Punktzahl ein Urteil zurück.
///
/// Returns a verdict for a score.
pub fn grade_for(punkte: u32) -> Option<&'static str> {
    if punkte > 100 {
        None
    } else if punkte >= 60 {
        Some("bestanden")
    } else {
        Some("nicht bestanden")
    }
}

/// Beschreibt ein `Option` als Text.
///
/// Describes an `Option` as text.
pub fn describe(wert: Option<i32>) -> String {
    if let Some(zahl) = wert {
        format!("Wert {zahl}")
    } else {
        String::from("kein Wert")
    }
}

/// Verdoppelt den Wert oder gibt null zurück.
///
/// Doubles the value or returns zero.
pub fn doubled_or_zero(wert: Option<i32>) -> i32 {
    let Some(zahl) = wert else {
        return 0;
    };

    zahl * 2
}
