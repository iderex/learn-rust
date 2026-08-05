//! 04-06 HashMap / HashMap
//!
//! Deutsch: Eine `HashMap<K, V>` legt Werte unter Schlüsseln ab. `get`
//! antwortet mit `Option`, und `entry(...).or_insert(...)` legt den Eintrag an,
//! der noch fehlt.
//!
//! English: a `HashMap<K, V>` stores values under keys. `get` answers with an
//! `Option`, and `entry(...).or_insert(...)` creates the entry that is still
//! missing.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::collections::HashMap;

/// Baut eine Karte aus Paaren von Schlüssel und Wert.
///
/// Diese Funktion steht fertig da; die Tests bauen ihre Karten damit.
///
/// Builds a map out of pairs of a key and a value.
///
/// This function stands there finished; the tests build their maps with it.
///
/// ```
/// use unit_04_06_hashmap::from_pairs;
///
/// let karte = from_pairs(&[("hallo", 1), ("welt", 2)]);
///
/// assert_eq!(karte.get("hallo").copied(), Some(1));
/// assert_eq!(karte.get("fehlt").copied(), None);
/// ```
pub fn from_pairs(paare: &[(&str, u32)]) -> HashMap<String, u32> {
    let mut karte = HashMap::new();

    for (schluessel, wert) in paare {
        karte.insert(String::from(*schluessel), *wert);
    }

    karte
}

/// Aufgabe 1: Zähle, wie oft jedes Wort vorkommt.
///
/// Der Schlüssel ist das Wort, der Wert seine Anzahl. `entry` legt den Eintrag
/// an, der noch fehlt.
///
/// Exercise 1: count how often each word appears.
///
/// The key is the word, the value its count. `entry` creates the entry that is
/// still missing.
pub fn counted(woerter: &[&str]) -> HashMap<String, u32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Schlage ein Wort nach und gib für ein fehlendes null zurück.
///
/// Ohne `unwrap`. `get` antwortet mit `Option`, und für den leeren Fall gibt es
/// eine Antwort statt eines Abbruchs.
///
/// Exercise 2: look a word up and return zero for a missing one.
///
/// Without `unwrap`. `get` answers with an `Option`, and for the empty case
/// there is an answer instead of a break.
pub fn count_of(karte: &HashMap<String, u32>, wort: &str) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Erhöhe den Wert eines Schlüssels um eins.
///
/// Fehlt der Schlüssel, wird er mit eins angelegt. Die Karte ist veränderbar
/// geliehen, und zurück kommt nichts.
///
/// Exercise 3: raise the value of a key by one.
///
/// If the key is missing it is created with a one. The map is borrowed mutably,
/// and nothing comes back.
pub fn increment(karte: &mut HashMap<String, u32>, wort: &str) {
    todo!("Aufgabe 3 / Exercise 3")
}
