//! 06-08 build.rs / build.rs, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/06-08-build-rs/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen. Das Build-Skript und
//! die Datendatei liegen hier noch einmal, denn ein Build-Skript gehört zu genau
//! einem Paket.
//!
//! English: the explanation lives in `units/06-08-build-rs/README.md`. What is
//! here is only the bodies that turn the unit's tests green. The build script and
//! the data file stand here a second time, because a build script belongs to
//! exactly one package.

include!(concat!(env!("OUT_DIR"), "/farben.rs"));

/// Gibt die erzeugte Liste zurück.
///
/// Returns the generated list.
///
/// ```
/// use unit_06_08_build_rs::colours;
///
/// assert_eq!(colours().len(), 5);
/// assert_eq!(colours()[0], "rot");
/// ```
pub fn colours() -> &'static [&'static str] {
    &FARBEN
}

/// Sagt, ob `name` in der erzeugten Liste steht.
///
/// Says whether `name` is in the generated list.
pub fn contains(name: &str) -> bool {
    FARBEN.contains(&name)
}

/// Gibt den längsten Namen aus der erzeugten Liste zurück.
///
/// Returns the longest name out of the generated list.
pub fn longest() -> &'static str {
    let mut bester = FARBEN[0];

    for farbe in &FARBEN[1..] {
        if farbe.len() > bester.len() {
            bester = farbe;
        }
    }

    bester
}

/// Baut eine Zeile aus allen Namen, durch Komma und Leerzeichen getrennt.
///
/// Builds one line out of all names, separated by a comma and a space.
pub fn as_line() -> String {
    FARBEN.join(", ")
}
