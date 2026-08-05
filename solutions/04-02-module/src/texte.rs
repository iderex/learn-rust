//! Deutsch: Das Modul `texte`, ein zweiter Ast neben `zahlen`.
//!
//! English: the module `texte`, a second branch beside `zahlen`.

/// Gibt den Text in Großbuchstaben mit einem Ausrufezeichen zurück.
///
/// Returns the text in capitals with an exclamation mark.
pub fn shouted(text: &str) -> String {
    format!("{}!", text.to_uppercase())
}
