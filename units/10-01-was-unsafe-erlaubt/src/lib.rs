//! 10-01 Was unsafe erlaubt und was es nicht abschaltet / What unsafe allows
//! and what it does not switch off
//!
//! Deutsch: `unsafe` schaltet fünf Dinge frei und nichts sonst. Der
//! Ausleihprüfer, die Typprüfung und alles andere laufen weiter.
//!
//! English: `unsafe` unlocks five things and nothing else. The borrow checker,
//! the type checking and everything else keep running.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt die Adresse eines Wertes als rohen Zeiger.
///
/// Diese Funktion steht fertig da, und sie hat kein `unsafe` im Rumpf. Einen
/// rohen Zeiger zu bauen ist erlaubt, denn dabei kann nichts schiefgehen: er
/// wird nur hingelegt. Gefährlich ist erst das Lesen, und das steht deshalb im
/// Doku-Test und nicht hier.
///
/// Returns the address of a value as a raw pointer.
///
/// This function stands there finished, and it has no `unsafe` in its body.
/// Building a raw pointer is allowed, because nothing can go wrong doing it: it
/// is only put down. Only the reading is dangerous, and that is why the reading
/// stands in the doc test and not here.
///
/// ```
/// use unit_10_01_was_unsafe_erlaubt::adresse;
///
/// let wert = 7;
/// let zeiger = adresse(&wert);
///
/// // Deutsch: Das Bauen war sicher, das Lesen braucht `unsafe`.
/// // English: building it was safe, reading it needs `unsafe`.
/// assert_eq!(unsafe { *zeiger }, 7);
/// ```
pub fn adresse(wert: &i32) -> *const i32 {
    wert
}

/// Aufgabe 1: Lies den Wert, auf den ein roher Zeiger zeigt.
///
/// Diese Funktion ist selbst `unsafe`, denn sie kann den Aufrufer nicht daran
/// hindern, ihr einen Zeiger ins Nichts zu geben. Wer sie ruft, verspricht,
/// dass der Zeiger auf einen gültigen `i32` zeigt.
///
/// Der Rumpf braucht trotzdem einen eigenen `unsafe`-Block. In der Ausgabe 2024
/// ist der Rumpf einer `unsafe fn` nicht mehr von selbst unsicher, und ohne den
/// Block wird der Prüflauf rot. Das ist gewollt: die Zusage an den Aufrufer und
/// die Stelle, an der etwas Rohes geschieht, sind zwei verschiedene Dinge.
///
/// # Safety
///
/// `zeiger` zeigt auf einen gültigen, ausgerichteten `i32`, der so lange lebt,
/// wie der Aufruf dauert.
///
/// Exercise 1: read the value a raw pointer points at.
///
/// This function is `unsafe` itself, because it cannot stop the caller from
/// handing it a pointer into nothing. Whoever calls it promises that the
/// pointer points at a valid `i32`.
///
/// The body still needs an `unsafe` block of its own. In edition 2024 the body
/// of an `unsafe fn` is no longer unsafe by itself, and without the block the
/// check run turns red. That is intended: the promise to the caller and the
/// place where something raw happens are two different things.
pub unsafe fn lesen(zeiger: *const i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Teile einen Slice in zwei veränderbare Hälften.
///
/// Sicheres Rust lässt das nicht zu, denn zwei veränderbare Ausleihen auf
/// dasselbe wären das, was der Ausleihprüfer verbietet. Er sieht nur nicht,
/// dass die beiden Hälften einander nicht überlappen.
///
/// Der Weg führt über `werte.as_mut_ptr()` und zweimal
/// `std::slice::from_raw_parts_mut`, das zweite Mal ab `zeiger.add(mitte)`. Ist
/// `mitte` größer als die Länge, bricht die Funktion mit `assert!` ab, und zwar
/// bevor gerechnet wird.
///
/// Exercise 2: split a slice into two mutable halves.
///
/// Safe Rust does not allow that, because two mutable borrows of the same thing
/// are what the borrow checker forbids. It just does not see that the two
/// halves do not overlap.
///
/// The way there is `werte.as_mut_ptr()` and twice
/// `std::slice::from_raw_parts_mut`, the second time from `zeiger.add(mitte)`.
/// Where `mitte` is larger than the length the function aborts with `assert!`,
/// and it does so before any arithmetic.
pub fn teilen(werte: &mut [i32], mitte: usize) -> (&mut [i32], &mut [i32]) {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib das erste und das letzte Element veränderbar heraus.
///
/// Dieselbe Sperre wie in Aufgabe 2 und derselbe Weg daran vorbei, nur ohne
/// Slices: zwei veränderbare Ausleihen aus einem `as_mut_ptr()`, die zweite ab
/// `.add(len - 1)`. Hat der Slice weniger als zwei Elemente, gibt es nichts
/// herauszugeben und die Antwort ist `None`.
///
/// Exercise 3: hand out the first and the last element mutably.
///
/// The same barrier as in exercise 2 and the same way past it, only without
/// slices: two mutable borrows out of one `as_mut_ptr()`, the second from
/// `.add(len - 1)`. Where the slice has fewer than two elements there is
/// nothing to hand out and the answer is `None`.
pub fn erstes_und_letztes(werte: &mut [i32]) -> Option<(&mut i32, &mut i32)> {
    todo!("Aufgabe 3 / Exercise 3")
}
