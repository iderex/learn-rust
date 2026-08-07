//! 10-02 Rohe Zeiger / Raw pointers
//!
//! Deutsch: Ein roher Zeiger ist eine Adresse ohne Regeln. Ihn anzulegen und
//! ihn zu vergleichen geht ohne `unsafe`. Durch ihn zu lesen oder zu schreiben
//! geht nur im `unsafe`-Block, und dort gehört die Begründung daneben.
//!
//! English: a raw pointer is an address without rules. Creating it and
//! comparing it works without `unsafe`. Reading or writing through it works
//! only inside an `unsafe` block, and there the justification belongs next to
//! it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Die Adresse eines Wertes als `*const i32`.
///
/// Diese Funktion steht fertig da und zeigt die erste Hälfte der Sache. Sie
/// trägt kein `unsafe`, weil sie nichts liest: Aus einer Referenz einen rohen
/// Zeiger zu machen ist erlaubt, und der Zeiger allein tut nichts.
///
/// The address of a value as a `*const i32`.
///
/// This function stands there finished and shows the first half of the thing.
/// It carries no `unsafe` because it reads nothing: turning a reference into a
/// raw pointer is allowed, and the pointer on its own does nothing.
///
/// ```
/// use unit_10_02_rohe_zeiger::adresse_von;
///
/// let zahl = 5;
/// let zeiger = adresse_von(&zahl);
///
/// // Deutsch: Der Vergleich braucht kein `unsafe`, das Lesen schon.
/// // English: the comparison needs no `unsafe`, the reading does.
/// assert!(!zeiger.is_null());
///
/// // Sicher, weil: `zeiger` kommt aus einer Referenz auf `zahl`, und `zahl`
/// // lebt bis zum Ende dieses Blocks. Er ist damit nicht null, ausgerichtet
/// // und gültig, und auf dieselbe Stelle schreibt hier nichts.
/// //
/// // Safe because: `zeiger` comes from a reference to `zahl`, and `zahl` lives
/// // until the end of this block. It is therefore not null, aligned and valid,
/// // and nothing writes to the same place here.
/// let gelesen = unsafe { *zeiger };
///
/// assert_eq!(gelesen, 5);
/// ```
pub fn adresse_von(wert: &i32) -> *const i32 {
    wert as *const i32
}

/// Aufgabe 1: Lies den Wert hinter einem `*const i32`.
///
/// Der Rumpf braucht einen `unsafe`-Block, denn das Dereferenzieren eines rohen
/// Zeigers ist nicht erlaubt, solange keiner darum steht. Die Meldung dazu
/// steht in der README.
///
/// In der Ausgabe 2024 reicht das `unsafe` am Kopf der Funktion dafür nicht.
/// Auch innen gehört ein Block darum, und daneben gehört die Begründung.
///
/// # Sicherheit
///
/// Der Aufrufer sagt zu, dass `zeiger` nicht null, ausgerichtet und gültig ist
/// und auf einen lesbaren `i32` zeigt. Diese Funktion sieht das nicht nach und
/// kann es nicht nachsehen.
///
/// Exercise 1: read the value behind a `*const i32`.
///
/// The body needs an `unsafe` block, because dereferencing a raw pointer is not
/// allowed while none stands around it. The message for that is in the README.
///
/// In edition 2024 the `unsafe` at the head of the function does not reach for
/// this. A block belongs around it inside as well, and the justification
/// belongs next to it.
///
/// # Safety
///
/// The caller promises that `zeiger` is not null, aligned and valid and points
/// at a readable `i32`. This function does not look that up and cannot look it
/// up.
pub unsafe fn lies(zeiger: *const i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Schreibe einen neuen Wert hinter den Zeiger.
///
/// Zurück kommt der Wert, der vorher dort stand. Danach steht dort der neue.
/// Der alte Wert muss also geholt werden, bevor geschrieben wird.
///
/// # Sicherheit
///
/// Der Aufrufer sagt zu, dass `zeiger` nicht null, ausgerichtet und gültig ist
/// und auf einen les- und beschreibbaren `i32` zeigt.
///
/// Exercise 2: write a new value behind the pointer.
///
/// What comes back is the value that stood there before. Afterwards the new one
/// stands there. The old value therefore has to be fetched before the write
/// happens.
///
/// # Safety
///
/// The caller promises that `zeiger` is not null, aligned and valid and points
/// at a readable and writable `i32`.
pub unsafe fn ersetzen(zeiger: *mut i32, neu: i32) -> i32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag, ob zwei Zeiger auf dieselbe Stelle zeigen.
///
/// Diese Funktion trägt kein `unsafe` und darf keines brauchen. Verglichen
/// werden zwei Adressen, und dabei wird nichts gelesen. Wer hier einen
/// `unsafe`-Block schreibt, hat die Aufgabe nicht gelöst, sondern eine andere.
///
/// Exercise 3: say whether two pointers point at the same place.
///
/// This function carries no `unsafe` and may not need one. What is compared are
/// two addresses, and nothing is read while doing it. Whoever writes an
/// `unsafe` block here has not solved this exercise but a different one.
pub fn zeigen_auf_dasselbe(a: *const i32, b: *const i32) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}
