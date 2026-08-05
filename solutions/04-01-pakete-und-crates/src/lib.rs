//! 04-01 Pakete und Crates / Packages and crates, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/04-01-pakete-und-crates/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/04-01-pakete-und-crates/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

/// Sagt zu einer Wurzel, welche Art von Crate dort anfängt.
///
/// Says for a root which kind of crate begins there.
pub fn crate_kind(wurzel: &str) -> Option<&'static str> {
    if wurzel == "src/lib.rs" {
        Some("bibliothek")
    } else if wurzel == "src/main.rs" || wurzel.starts_with("src/bin/") {
        Some("programm")
    } else {
        None
    }
}

/// Gibt zu einer Art von Crate ihre Wurzel zurück.
///
/// Returns the root for a kind of crate.
pub fn crate_root(art: &str) -> Option<&'static str> {
    if art == "bibliothek" {
        Some("src/lib.rs")
    } else if art == "programm" {
        Some("src/main.rs")
    } else {
        None
    }
}

/// Zählt die Crates eines Pakets.
///
/// Counts the crates of a package.
pub fn crate_count(hat_bibliothek: bool, programme: u32) -> u32 {
    if hat_bibliothek {
        programme + 1
    } else {
        programme
    }
}

/// Gibt den Pfad eines weiteren Programms zurück.
///
/// Returns the path of a further program.
pub fn binary_root(name: &str) -> String {
    format!("src/bin/{name}.rs")
}
