//! 04-06 HashMap / HashMap, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/04-06-hashmap/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen. `unwrap` kommt in
//! keinem davon vor.
//!
//! English: the explanation lives in `units/04-06-hashmap/README.md`. What is
//! here is only the bodies that turn the unit's tests green. `unwrap` appears in
//! none of them.

use std::collections::HashMap;

/// Baut eine Karte aus Paaren von Schlüssel und Wert.
///
/// Builds a map out of pairs of a key and a value.
pub fn from_pairs(paare: &[(&str, u32)]) -> HashMap<String, u32> {
    let mut karte = HashMap::new();

    for (schluessel, wert) in paare {
        karte.insert(String::from(*schluessel), *wert);
    }

    karte
}

/// Zählt, wie oft jedes Wort vorkommt.
///
/// Counts how often each word appears.
pub fn counted(woerter: &[&str]) -> HashMap<String, u32> {
    let mut karte = HashMap::new();

    for wort in woerter {
        *karte.entry(String::from(*wort)).or_insert(0) += 1;
    }

    karte
}

/// Schlägt ein Wort nach und gibt für ein fehlendes null zurück.
///
/// Looks a word up and returns zero for a missing one.
pub fn count_of(karte: &HashMap<String, u32>, wort: &str) -> u32 {
    karte.get(wort).copied().unwrap_or(0)
}

/// Erhöht den Wert eines Schlüssels um eins.
///
/// Raises the value of a key by one.
pub fn increment(karte: &mut HashMap<String, u32>, wort: &str) {
    *karte.entry(String::from(wort)).or_insert(0) += 1;
}
