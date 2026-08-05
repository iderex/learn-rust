//! 04-07 panic! und Result / panic! and Result, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/04-07-panic-und-result/README.md`.
//! Hier stehen nur der Fehlertyp und die Rümpfe, die die Tests der Einheit grün
//! machen. `unwrap` kommt in keinem davon vor.
//!
//! English: the explanation lives in
//! `units/04-07-panic-und-result/README.md`. What is here is only the error
//! type and the bodies that turn the unit's tests green. `unwrap` appears in
//! none of them.

/// Was in dieser Einheit schiefgehen kann.
///
/// What can go wrong in this unit.
#[derive(Debug, PartialEq)]
pub enum Fehler {
    /// Der Nenner war null / the denominator was zero.
    DurchNull,
    /// Die Zahl kann kein Alter sein / the number cannot be an age.
    KeinAlter,
    /// Der Text war leer / the text was empty.
    LeererText,
}

/// Gibt den Rest der Division zurück, oder einen Fehler bei null.
///
/// Returns the remainder of the division, or an error on zero.
pub fn remainder(a: i32, b: i32) -> Result<i32, Fehler> {
    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a % b)
}

/// Teilt `a` durch `b`.
///
/// Divides `a` by `b`.
pub fn divided(a: i32, b: i32) -> Result<i32, Fehler> {
    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a / b)
}

/// Nimmt eine Zahl als Alter an.
///
/// Accepts a number as an age.
pub fn checked_age(jahre: u32) -> Result<u32, Fehler> {
    if jahre > 130 {
        return Err(Fehler::KeinAlter);
    }

    Ok(jahre)
}

/// Gibt die erste Zeile eines Textes zurück.
///
/// Returns the first line of a text.
pub fn first_line(text: &str) -> Result<String, Fehler> {
    if text.is_empty() {
        return Err(Fehler::LeererText);
    }

    let mut zeile = String::new();

    for zeichen in text.chars() {
        if zeichen == '\n' {
            break;
        }

        zeile.push(zeichen);
    }

    Ok(zeile)
}
