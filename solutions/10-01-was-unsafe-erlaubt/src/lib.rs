//! 10-01 Was unsafe erlaubt und was es nicht abschaltet / What unsafe allows
//! and what it does not switch off, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/10-01-was-unsafe-erlaubt/README.md`. Hier stehen nur die Rümpfe, die
//! die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/10-01-was-unsafe-erlaubt/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

use std::slice;

/// Gibt die Adresse eines Wertes als rohen Zeiger.
///
/// Returns the address of a value as a raw pointer.
///
/// ```
/// use unit_10_01_was_unsafe_erlaubt::adresse;
///
/// let wert = 7;
/// let zeiger = adresse(&wert);
///
/// assert_eq!(unsafe { *zeiger }, 7);
/// ```
pub fn adresse(wert: &i32) -> *const i32 {
    wert
}

/// Liest den Wert, auf den ein roher Zeiger zeigt.
///
/// Reads the value a raw pointer points at.
///
/// # Safety
///
/// `zeiger` zeigt auf einen gültigen, ausgerichteten `i32`, der so lange lebt,
/// wie der Aufruf dauert.
///
/// `zeiger` points at a valid, aligned `i32` living as long as the call takes.
pub unsafe fn lesen(zeiger: *const i32) -> i32 {
    // Deutsch: In der Ausgabe 2024 ist der Rumpf einer `unsafe fn` nicht von
    // selbst unsicher, der Block gehört also hierher.
    // English: in edition 2024 the body of an `unsafe fn` is not unsafe by
    // itself, so the block belongs here.
    unsafe { *zeiger }
}

/// Teilt einen Slice in zwei veränderbare Hälften.
///
/// Splits a slice into two mutable halves.
pub fn teilen(werte: &mut [i32], mitte: usize) -> (&mut [i32], &mut [i32]) {
    let laenge = werte.len();
    let zeiger = werte.as_mut_ptr();

    assert!(mitte <= laenge);

    // Deutsch: Die beiden Hälften überlappen einander nicht. Das sieht der
    // Ausleihprüfer nicht, und genau dafür steht der Block hier.
    // English: the two halves do not overlap. The borrow checker does not see
    // that, and that is exactly what the block stands here for.
    unsafe {
        (
            slice::from_raw_parts_mut(zeiger, mitte),
            slice::from_raw_parts_mut(zeiger.add(mitte), laenge - mitte),
        )
    }
}

/// Gibt das erste und das letzte Element veränderbar heraus.
///
/// Hands out the first and the last element mutably.
pub fn erstes_und_letztes(werte: &mut [i32]) -> Option<(&mut i32, &mut i32)> {
    let laenge = werte.len();
    if laenge < 2 {
        return None;
    }
    let zeiger = werte.as_mut_ptr();

    // Deutsch: Zwei verschiedene Stellen desselben Slices, also keine
    // Überschneidung.
    // English: two different places of the same slice, so no overlap.
    unsafe { Some((&mut *zeiger, &mut *zeiger.add(laenge - 1))) }
}
