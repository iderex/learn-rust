//! 01-02 Zahlen und andere einfache Typen / Numbers and other simple types,
//! gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/01-02-zahlen-und-einfache-typen/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/01-02-zahlen-und-einfache-typen/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Gibt zurück, wie viele Sekunden `minuten` Minuten sind.
///
/// Returns how many seconds `minuten` minutes are.
pub fn seconds_of(minuten: u32) -> u32 {
    minuten * 60
}

/// Gibt zurück, ob `wert` in ein `u8` passt.
///
/// Returns whether `wert` fits into a `u8`.
pub fn fits_in_u8(wert: u32) -> bool {
    wert <= 255
}

/// Gibt `wert` als `u32` zurück.
///
/// Returns `wert` as a `u32`.
pub fn widened(wert: u8) -> u32 {
    u32::from(wert)
}

/// Gibt die Hälfte von `wert` zurück.
///
/// Returns half of `wert`.
pub fn half(wert: f64) -> f64 {
    wert / 2.0
}

/// Gibt zurück, ob `zeichen` ein Buchstabe einer Hexadezimalziffer ist, also a
/// bis f.
///
/// Returns whether `zeichen` is a letter of a hexadecimal digit, meaning a to f.
pub fn is_hex_letter(zeichen: char) -> bool {
    ('a'..='f').contains(&zeichen)
}
