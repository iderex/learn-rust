//! 06-07 #[cfg] und Features / #[cfg] and features, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/06-07-cfg-und-features/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/06-07-cfg-und-features/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Die Teile, die in dieser Übersetzung eingebaut sind, mit Feature.
///
/// The parts built into this compilation, with the feature.
#[cfg(feature = "zusammenfassung")]
pub fn eingebaute_teile() -> Vec<&'static str> {
    vec!["kern", "zusammenfassung"]
}

/// Die Teile, die in dieser Übersetzung eingebaut sind, ohne Feature.
///
/// The parts built into this compilation, without the feature.
#[cfg(not(feature = "zusammenfassung"))]
pub fn eingebaute_teile() -> Vec<&'static str> {
    vec!["kern"]
}

/// Sagt, ob das Feature an ist.
///
/// Says whether the feature is on.
///
/// ```
/// use unit_06_07_cfg_und_features::{eingebaute_teile, zusammenfassung_an};
///
/// assert_eq!(eingebaute_teile()[0], "kern");
///
/// assert_eq!(
///     eingebaute_teile().contains(&"zusammenfassung"),
///     zusammenfassung_an()
/// );
/// ```
pub fn zusammenfassung_an() -> bool {
    cfg!(feature = "zusammenfassung")
}

/// Der Bericht mit Zusammenfassung.
///
/// The report with the summary.
#[cfg(feature = "zusammenfassung")]
pub fn bericht(zeilen: &[&str]) -> String {
    let mut text = zeilen.join("\n");
    if !text.is_empty() {
        text.push('\n');
    }
    text.push_str(&format!("Zeilen: {}", zeilen.len()));
    text
}

/// Der Bericht ohne Zusammenfassung.
///
/// The report without the summary.
#[cfg(not(feature = "zusammenfassung"))]
pub fn bericht(zeilen: &[&str]) -> String {
    zeilen.join("\n")
}

/// Beschreibt die Übersetzung in einem Satz.
///
/// Describes the compilation in one sentence.
pub fn beschreibung() -> String {
    if cfg!(feature = "zusammenfassung") {
        String::from("Bericht mit Zusammenfassung")
    } else {
        String::from("Bericht ohne Zusammenfassung")
    }
}

/// Ein Bericht als Struktur, deren Feld am Feature hängt.
///
/// A report as a struct whose field hangs on the feature.
#[derive(Debug, PartialEq, Eq)]
pub struct Bericht {
    /// Die Zeilen des Berichts.
    ///
    /// The lines of the report.
    pub zeilen: Vec<String>,

    /// Die Anzahl der Zeilen, nur mit dem Feature.
    ///
    /// The number of lines, only with the feature.
    #[cfg(feature = "zusammenfassung")]
    pub anzahl: usize,
}

/// Baut den Bericht als Struktur.
///
/// Builds the report as a struct.
pub fn neuer_bericht(zeilen: &[&str]) -> Bericht {
    Bericht {
        zeilen: zeilen.iter().map(|zeile| zeile.to_string()).collect(),
        #[cfg(feature = "zusammenfassung")]
        anzahl: zeilen.len(),
    }
}
