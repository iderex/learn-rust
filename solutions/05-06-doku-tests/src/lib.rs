//! 05-06 Doku-Tests / Doc tests, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-06-doku-tests/README.md`. Hier
//! stehen die Rümpfe, die die Tests der Einheit grün machen, und dieselben
//! Beispiele wie dort. Diese Einheit handelt von den Beispielen, deshalb werden
//! sie hier nicht weggekürzt: `cargo test --workspace` führt genau sie aus.
//!
//! English: the explanation lives in `units/05-06-doku-tests/README.md`. What is
//! here is the bodies that turn the unit's tests green, and the same examples as
//! there. This unit is about the examples, so they are not trimmed away here:
//! `cargo test --workspace` runs exactly them.

/// Zählt die Wörter in einem Text.
///
/// Counts the words in a text.
///
/// ```
/// use unit_05_06_doku_tests::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// assert_eq!(word_count("   "), 0);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Gibt das Wort an `stelle` zurück und bricht ab, wenn dort keines steht.
///
/// Returns the word at `stelle` and aborts when there is none there.
///
/// ```
/// use unit_05_06_doku_tests::word_at;
///
/// assert_eq!(word_at("ein zwei drei", 1), "zwei");
/// ```
///
/// ```should_panic
/// use unit_05_06_doku_tests::word_at;
///
/// word_at("ein", 5);
/// ```
pub fn word_at(text: &str, stelle: usize) -> &str {
    text.split_whitespace()
        .nth(stelle)
        .expect("an dieser Stelle steht kein Wort")
}

/// Baut die Initialen eines Namens.
///
/// Builds the initials of a name.
///
/// ```
/// use unit_05_06_doku_tests::initials;
///
/// assert_eq!(initials("Ada Lovelace"), "A.L.");
/// assert_eq!(initials("grace hopper"), "G.H.");
/// assert_eq!(initials("   "), "");
/// ```
pub fn initials(name: &str) -> String {
    let mut heraus = String::new();

    for wort in name.split_whitespace() {
        if let Some(erstes) = wort.chars().next() {
            heraus.extend(erstes.to_uppercase());
            heraus.push('.');
        }
    }

    heraus
}

/// Liest eine Prozentzahl aus einem Text.
///
/// Reads a percentage out of a text.
///
/// ```
/// # use unit_05_06_doku_tests::percent;
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// assert_eq!(percent(" 42 ")?, 42);
/// assert!(percent("dreiundvierzig").is_err());
/// assert!(percent("300").is_err());
/// # Ok(())
/// # }
/// ```
pub fn percent(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.trim().parse()
}

/// Gibt den längeren der beiden Texte zurück.
///
/// Returns the longer of the two texts.
///
/// ```
/// use unit_05_06_doku_tests::longest;
///
/// assert_eq!(longest("kurz", "laenger"), "laenger");
/// assert_eq!(longest("gleich", "gleich"), "gleich");
/// ```
pub fn longest<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if rechts.len() > links.len() {
        rechts
    } else {
        links
    }
}
