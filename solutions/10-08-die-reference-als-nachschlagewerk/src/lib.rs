//! 10-08 Die Reference als Nachschlagewerk / The Reference as a lookup, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/10-08-die-reference-als-nachschlagewerk/README.md`. Hier stehen nur
//! die Rümpfe, die die Tests der Einheit grün machen, und an jedem die Stelle
//! der Reference, aus der die Antwort kommt.
//!
//! English: the explanation lives in
//! `units/10-08-die-reference-als-nachschlagewerk/README.md`. What is here is
//! only the bodies that turn the unit's tests green, and at each of them the
//! place in the Reference the answer comes from.

/// Wandelt eine Kommazahl mit `as` in ein `u8`.
///
/// Converts a floating point number into a `u8` with `as`.
///
/// ```
/// use unit_10_08_die_reference_als_nachschlagewerk::saturating_to_u8;
///
/// assert_eq!(saturating_to_u8(42.9), 42);
/// assert_eq!(saturating_to_u8(300.0), 255);
/// assert_eq!(saturating_to_u8(-1.5), 0);
/// assert_eq!(saturating_to_u8(f64::NAN), 0);
/// ```
pub fn saturating_to_u8(zahl: f64) -> u8 {
    // Deutsch: Reference 8.2.4 "Operator expressions": zur Null hin gerundet,
    // NaN wird 0, zu grosse Werte saettigen an der Grenze.
    // English: Reference 8.2.4 "Operator expressions": rounded towards zero,
    // NaN becomes 0, values too large saturate at the bound.
    zahl as u8
}

/// Wandelt eine ganze Zahl mit `as` in ein `u8`.
///
/// Converts an integer into a `u8` with `as`.
///
/// ```
/// use unit_10_08_die_reference_als_nachschlagewerk::truncating_to_u8;
///
/// assert_eq!(truncating_to_u8(300), 44);
/// assert_eq!(truncating_to_u8(-1), 255);
/// ```
pub fn truncating_to_u8(zahl: i32) -> u8 {
    // Deutsch: Dieselbe Stelle, andere Regel. Zwischen ganzen Zahlen behaelt
    // `as` die unteren Bits, es saettigt nicht. Deshalb ist 300 hier 44 und
    // nicht 255, und deshalb traegt die Funktion darueber einen anderen Namen.
    // English: same place, other rule. Between integers `as` keeps the lower
    // bits, it does not saturate. That is why 300 is 44 here and not 255, and
    // why the function above carries a different name.
    zahl as u8
}

/// Addiert zwei `u8`, ohne dass ein Überlauf das Programm anhält.
///
/// Adds two `u8` without an overflow stopping the program.
///
/// ```
/// use unit_10_08_die_reference_als_nachschlagewerk::sum_without_panic;
///
/// assert_eq!(sum_without_panic(250, 5), Some(255));
/// assert_eq!(sum_without_panic(250, 10), None);
/// ```
pub fn sum_without_panic(a: u8, b: u8) -> Option<u8> {
    // Deutsch: Reference 8.2.4, Abschnitt "Overflow". `a + b` haelt im
    // Debug-Bau an, `checked_add` fragt stattdessen.
    // English: Reference 8.2.4, section "Overflow". `a + b` panics in a debug
    // build, `checked_add` asks instead.
    a.checked_add(b)
}

/// Sagt, ob an dieser Adresse ein `u32` liegen dürfte.
///
/// Says whether a `u32` would be allowed to sit at this address.
///
/// ```
/// use unit_10_08_die_reference_als_nachschlagewerk::is_aligned_for_u32;
///
/// assert!(is_aligned_for_u32(8));
/// assert!(!is_aligned_for_u32(6));
/// ```
pub fn is_aligned_for_u32(adresse: usize) -> bool {
    // Deutsch: Reference 17.2 "Behavior considered undefined" fuehrt den
    // Zugriff ueber einen falsch ausgerichteten Zeiger auf. Was noetig ist,
    // sagt `align_of` und nicht eine Vier im Quelltext.
    // English: Reference 17.2 "Behavior considered undefined" lists an access
    // through a misaligned pointer. What is needed is said by `align_of` and
    // not by a four written into the source.
    adresse.is_multiple_of(align_of::<u32>())
}
