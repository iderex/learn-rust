//! 06-03 Umgebungsvariablen und Ausgabe nach stderr / Environment variables and
//! output to stderr, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/06-03-umgebungsvariablen-und-stderr/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/06-03-umgebungsvariablen-und-stderr/README.md`. What is here is only
//! the bodies that turn the unit's tests green.

use std::env;
use std::io::{self, Write};

/// Ein Lauf des Werkzeugs: was gefunden wurde und was dazu zu sagen ist.
///
/// One run of the tool: what was found and what there is to say about it.
#[derive(Debug, PartialEq)]
pub struct Bericht {
    pub treffer: Vec<String>,
    pub meldungen: Vec<String>,
}

/// Liest eine Einstellung aus der Umgebung, mit einer Vorgabe dahinter.
///
/// Reads a setting from the environment, with a default behind it.
///
/// ```
/// use unit_06_03_umgebungsvariablen_und_stderr::aus_der_umgebung;
///
/// // Deutsch: Diesen Namen setzt niemand, also greift die Vorgabe.
/// // English: nobody sets this name, so the default takes hold.
/// assert_eq!(aus_der_umgebung("LR_06_03_NICHT_GESETZT", "an"), "an");
/// ```
pub fn aus_der_umgebung(name: &str, vorgabe: &str) -> String {
    match env::var(name) {
        Ok(wert) => wert,
        Err(_) => vorgabe.to_string(),
    }
}

/// Beantwortet die fehlende Variable mit `vorgabe`, sonst nichts.
///
/// Answers the missing variable with `vorgabe`, and nothing else.
pub fn einstellung(
    gelesen: Result<String, env::VarError>,
    vorgabe: &str,
) -> Result<String, env::VarError> {
    match gelesen {
        Ok(wert) => Ok(wert),
        Err(env::VarError::NotPresent) => Ok(vorgabe.to_string()),
        Err(fehler) => Err(fehler),
    }
}

/// Sammelt die passenden Zeilen und sagt dazu, wonach gesucht wurde.
///
/// Collects the matching lines and says what was searched for.
pub fn bericht(zeilen: &[String], muster: &str) -> Bericht {
    let mut treffer = Vec::new();
    for zeile in zeilen {
        if zeile.contains(muster) {
            treffer.push(zeile.clone());
        }
    }

    let mut meldungen = vec![format!("gesucht wird nach {muster}")];
    if treffer.is_empty() {
        meldungen.push(String::from("kein Treffer"));
    }

    Bericht { treffer, meldungen }
}

/// Schreibt die Meldungen nach `fehler` und die Treffer nach `aus`.
///
/// Writes the messages to `fehler` and the hits to `aus`.
pub fn schreiben(
    bericht: &Bericht,
    aus: &mut impl Write,
    fehler: &mut impl Write,
) -> io::Result<()> {
    for meldung in &bericht.meldungen {
        writeln!(fehler, "{meldung}")?;
    }
    for treffer in &bericht.treffer {
        writeln!(aus, "{treffer}")?;
    }

    Ok(())
}
