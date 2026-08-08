//! 09-05 Fortgeschrittene Typen / Advanced types, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/09-05-fortgeschrittene-typen/README.md`. Hier stehen nur die Rümpfe,
//! die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-05-fortgeschrittene-typen/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

use std::fmt::Debug;

/// Ein Ergebnis, dessen Fehler ein Text ist.
///
/// A result whose error is a text.
///
/// ```
/// use unit_09_05_fortgeschrittene_typen::Ergebnis;
///
/// let ueber_alias: Ergebnis<u32> = Ok(3);
/// let ausgeschrieben: Result<u32, String> = ueber_alias;
///
/// assert_eq!(ausgeschrieben, Ok(3));
/// ```
pub type Ergebnis<T> = Result<T, String>;

/// Beschreibt einen Wert so, wie `{:?}` ihn ausgeben würde.
///
/// Describes a value the way `{:?}` would print it.
///
/// ```
/// use unit_09_05_fortgeschrittene_typen::beschreibe;
///
/// assert_eq!(beschreibe("hallo"), "\"hallo\"");
/// assert_eq!(beschreibe(&[1, 2, 3][..]), "[1, 2, 3]");
/// assert_eq!(beschreibe(&7), "7");
/// ```
pub fn beschreibe<T: Debug + ?Sized>(wert: &T) -> String {
    format!("{wert:?}")
}

/// Setzt zwei Teile zu einem Text zusammen.
///
/// Puts two parts together into one text.
pub fn zusammen(links: &str, rechts: &str) -> Ergebnis<String> {
    if links.is_empty() || rechts.is_empty() {
        return Err(String::from("ein Teil ist leer"));
    }

    Ok(format!("{links} {rechts}"))
}

/// Bricht das Programm mit einem Grund ab.
///
/// Aborts the program with a reason.
pub fn abbruch(grund: &str) -> ! {
    panic!("Abbruch: {grund}");
}

/// Liest eine Zahl oder bricht ab.
///
/// Reads a number or aborts.
pub fn zahl_oder_abbruch(text: &str) -> u32 {
    match text.parse::<u32>() {
        Ok(gelesen) => gelesen,
        Err(_) => abbruch(text),
    }
}

/// Gibt das erste und das letzte Element heraus.
///
/// Hands the first and the last element out.
pub fn erstes_und_letztes(werte: &[i32]) -> Option<(i32, i32)> {
    let erstes = werte.first()?;
    let letztes = werte.last()?;

    Some((*erstes, *letztes))
}
