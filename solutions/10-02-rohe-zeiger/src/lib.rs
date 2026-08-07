//! 10-02 Rohe Zeiger / Raw pointers, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/10-02-rohe-zeiger/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/10-02-rohe-zeiger/README.md`. What
//! is here is only the bodies that turn the unit's tests green.

/// Die Adresse eines Wertes als `*const i32`.
///
/// The address of a value as a `*const i32`.
///
/// ```
/// use unit_10_02_rohe_zeiger::adresse_von;
///
/// let zahl = 5;
/// let zeiger = adresse_von(&zahl);
///
/// assert!(!zeiger.is_null());
///
/// // Sicher, weil: `zeiger` kommt aus einer Referenz auf `zahl`, und `zahl`
/// // lebt bis zum Ende dieses Blocks.
/// // Safe because: `zeiger` comes from a reference to `zahl`, and `zahl` lives
/// // until the end of this block.
/// assert_eq!(unsafe { *zeiger }, 5);
/// ```
pub fn adresse_von(wert: &i32) -> *const i32 {
    wert as *const i32
}

/// Liest den Wert hinter einem `*const i32`.
///
/// Reads the value behind a `*const i32`.
///
/// # Safety
///
/// Deutsch: Der Aufrufer sagt zu, dass `zeiger` nicht null, ausgerichtet und
/// gültig ist und auf einen lesbaren `i32` zeigt.
///
/// English: the caller promises that `zeiger` is not null, aligned and valid
/// and points at a readable `i32`.
pub unsafe fn lies(zeiger: *const i32) -> i32 {
    // Sicher, weil: Die Bedingung steht im Abschnitt "Sicherheit" und liegt
    // beim Aufrufer. Hier bleibt nur, sie nicht zu überschreiten, und gelesen
    // wird genau die eine Stelle.
    //
    // Safe because: the condition stands in the "Safety" section and lies with
    // the caller. What is left here is not to go beyond it, and what is read is
    // exactly the one place.
    unsafe { *zeiger }
}

/// Schreibt einen neuen Wert hinter den Zeiger und gibt den alten heraus.
///
/// Writes a new value behind the pointer and hands the old one out.
///
/// # Safety
///
/// Deutsch: Der Aufrufer sagt zu, dass `zeiger` nicht null, ausgerichtet und
/// gültig ist und auf einen les- und beschreibbaren `i32` zeigt.
///
/// English: the caller promises that `zeiger` is not null, aligned and valid
/// and points at a readable and writable `i32`.
pub unsafe fn ersetzen(zeiger: *mut i32, neu: i32) -> i32 {
    // Sicher, weil: Die Bedingung steht im Abschnitt "Safety" und liegt beim
    // Aufrufer. Gelesen und geschrieben wird genau die eine Stelle, und der
    // alte Wert wird vor dem Schreiben geholt.
    //
    // Safe because: the condition stands in the "Safety" section and lies with
    // the caller. What is read and written is exactly the one place, and the
    // old value is fetched before the write.
    unsafe {
        let alt = *zeiger;

        *zeiger = neu;

        alt
    }
}

/// Sagt, ob zwei Zeiger auf dieselbe Stelle zeigen.
///
/// Says whether two pointers point at the same place.
pub fn zeigen_auf_dasselbe(a: *const i32, b: *const i32) -> bool {
    std::ptr::eq(a, b)
}
