//! 10-07 Miri / Miri
//!
//! Deutsch: Miri führt ein Programm Schritt für Schritt aus und sieht dabei
//! jedem Zugriff zu. Was der Übersetzer nicht prüfen kann, weil es erst beim
//! Laufen entsteht, fällt hier auf. Was nie ausgeführt wird, fällt nicht auf.
//!
//! English: Miri runs a program step by step and watches every access while
//! doing so. What the compiler cannot check, because it only arises while
//! running, shows up here. What is never executed does not show up.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Liest ein Element hinter einem rohen Zeiger.
///
/// Diese Funktion steht fertig da und ist die Form, die jede Aufgabe dieser
/// Einheit wiederholt: erst die Grenze prüfen, dann den Zeiger benutzen. Die
/// Prüfung steht in sicherem Code, und der `unsafe`-Block ist so klein, dass
/// er nur noch den Zugriff enthält.
///
/// Der `SAFETY`-Kommentar sagt, warum der Zugriff erlaubt ist. Er ist keine
/// Zusage an den Übersetzer, sondern die Rechnung für den nächsten Leser, und
/// Miri prüft nicht ihn, sondern was der Block tut.
///
/// Reads an element behind a raw pointer.
///
/// This function stands there finished and is the shape every exercise of this
/// unit repeats: check the bound first, then use the pointer. The check stands
/// in safe code, and the `unsafe` block is small enough to hold nothing but the
/// access.
///
/// The `SAFETY` comment says why the access is allowed. It is not a promise to
/// the compiler but the reasoning for the next reader, and Miri does not check
/// it but what the block does.
///
/// ```
/// use unit_10_07_miri::lese;
///
/// assert_eq!(lese(&[7, 8, 9], 0), Some(7));
/// assert_eq!(lese(&[7, 8, 9], 2), Some(9));
/// assert_eq!(lese(&[7, 8, 9], 3), None);
/// assert_eq!(lese(&[], 0), None);
/// ```
pub fn lese(werte: &[i64], stelle: usize) -> Option<i64> {
    if stelle >= werte.len() {
        return None;
    }

    // SAFETY: `stelle` ist kleiner als die Länge, also liegt `add(stelle)` in
    // derselben Zuteilung wie `werte` und zeigt auf ein gültiges `i64`.
    // SAFETY: `stelle` is smaller than the length, so `add(stelle)` lies in the
    // same allocation as `werte` and points at a valid `i64`.
    unsafe { Some(*werte.as_ptr().add(stelle)) }
}

/// Aufgabe 1: Zähl die Werte zusammen, ohne den Index zu benutzen.
///
/// Gelesen wird über einen rohen Zeiger auf den Anfang und `add`, und zwar
/// genau so oft, wie die Länge sagt. Aus einer leeren Liste kommt 0.
///
/// Der eine Schritt zu viel ist der Fehler, um den es hier geht: Ein Zeiger
/// darf bis hinter das letzte Element zeigen, aber nicht mehr gelesen werden.
/// Der Übersetzer sagt dazu nichts, der Lauf meistens auch nicht, und Miri
/// schon.
///
/// Exercise 1: add the values up without using an index.
///
/// Reading goes over a raw pointer to the beginning and `add`, and exactly as
/// often as the length says. Out of an empty list comes 0.
///
/// The one step too many is the mistake this is about: a pointer may point one
/// past the last element, but may not be read there. The compiler says nothing
/// about it, the run mostly says nothing either, and Miri does.
pub fn summe_ueber_zeiger(werte: &[i64]) -> i64 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Tausch zwei Werte über rohe Zeiger.
///
/// Aus jeder der beiden Referenzen wird ein roher Zeiger, und getauscht wird
/// mit `std::ptr::swap`. Zwei `&mut` können nicht auf dieselbe Stelle zeigen,
/// und deshalb ist der Tausch hier erlaubt.
///
/// In echtem Code nimmt man `std::mem::swap` und keinen rohen Zeiger. Diese
/// Aufgabe ist der kleine Fall, an dem sich ansehen lässt, was Miri beim
/// Schreiben durch einen Zeiger prüft.
///
/// Exercise 2: swap two values over raw pointers.
///
/// Each of the two references becomes a raw pointer, and the swap happens with
/// `std::ptr::swap`. Two `&mut` cannot point at the same place, and that is why
/// the swap is allowed here.
///
/// In real code one takes `std::mem::swap` and no raw pointer. This exercise is
/// the small case on which it can be seen what Miri checks about writing
/// through a pointer.
pub fn tauschen(links: &mut i64, rechts: &mut i64) {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Erhöhe jeden Wert um `um`, geschrieben über einen rohen Zeiger.
///
/// Der Zeiger kommt aus `as_mut_ptr` und wird `len` mal weitergeschoben.
/// Danach steht in jedem Feld sein alter Wert plus `um`.
///
/// Solange die Schleife läuft, greift nichts anderes auf `werte` zu. Was der
/// Rumpf über die Liste wissen muss, holt er deshalb, bevor er anfängt zu
/// schreiben.
///
/// Exercise 3: raise every value by `um`, written through a raw pointer.
///
/// The pointer comes out of `as_mut_ptr` and gets pushed on `len` times.
/// Afterwards every slot holds its old value plus `um`.
///
/// As long as the loop runs, nothing else reaches for `werte`. What the body
/// needs to know about the list it therefore fetches before it starts writing.
pub fn erhoehen_ueber_zeiger(werte: &mut [i64], um: i64) {
    todo!("Aufgabe 3 / Exercise 3")
}
