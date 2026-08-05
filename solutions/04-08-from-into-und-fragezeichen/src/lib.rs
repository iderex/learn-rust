//! 04-08 From, Into und der Operator ? / From, Into and the ? operator, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/04-08-from-into-und-fragezeichen/README.md`. Hier stehen nur die
//! Typen und die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/04-08-from-into-und-fragezeichen/README.md`. What is here is only the
//! types and the bodies that turn the unit's tests green.

/// Was beim Lesen einer Zahl schiefgehen kann.
///
/// What can go wrong while reading a number.
#[derive(Debug, PartialEq)]
pub enum EingabeFehler {
    /// Der Text ist keine Zahl / the text is not a number.
    KeineZahl,
}

/// Was beim Rechnen schiefgehen kann.
///
/// What can go wrong while calculating.
#[derive(Debug, PartialEq)]
pub enum Fehler {
    /// Beim Lesen ging etwas schief / something went wrong while reading.
    Eingabe(EingabeFehler),
    /// Der Nenner war null / the denominator was zero.
    DurchNull,
}

/// Liest eine Zahl aus einem Text.
///
/// Reads a number out of a text.
pub fn parsed(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

impl From<EingabeFehler> for Fehler {
    fn from(fehler: EingabeFehler) -> Self {
        Fehler::Eingabe(fehler)
    }
}

/// Liest zwei Texte als Zahlen und teilt sie.
///
/// Reads two texts as numbers and divides them.
pub fn divided_text(links: &str, rechts: &str) -> Result<i32, Fehler> {
    let a = parsed(links)?;
    let b = parsed(rechts)?;

    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a / b)
}

/// Wandelt einen `EingabeFehler` in einen `Fehler` um.
///
/// Converts an `EingabeFehler` into a `Fehler`.
pub fn as_fehler(fehler: EingabeFehler) -> Fehler {
    fehler.into()
}
