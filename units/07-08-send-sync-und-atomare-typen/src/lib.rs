//! 07-08 Send, Sync und die atomaren Typen / Send, Sync and the atomic types
//!
//! Deutsch: Zwei Traits sagen, was zwischen Fäden bewegt und was zwischen ihnen
//! geteilt werden darf. Geschrieben werden sie fast nie, denn der Übersetzer
//! setzt sie selbst. Bemerkt werden sie an der Stelle, an der einer fehlt.
//!
//! English: two traits say what may be moved between threads and what may be
//! shared between them. They are almost never written, because the compiler puts
//! them in itself. They are noticed at the place where one is missing.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::sync::atomic::AtomicUsize;

/// Zählt eins hoch und gibt den Wert von vorher zurück.
///
/// Diese Funktion steht fertig da. Sie zeigt zwei Dinge auf einmal. Erstens
/// braucht sie kein `mut`, obwohl sie den Zähler verändert: Ein atomarer Typ
/// darf durch eine gemeinsame Referenz verändert werden, und genau das macht
/// ihn zwischen Fäden benutzbar.
///
/// Zweitens gibt `fetch_add` den Wert von vorher zurück und nicht den neuen. Wer
/// den neuen will, liest danach mit `load` oder rechnet ihn sich aus.
///
/// `Ordering::Relaxed` heißt hier: Es geht nur um den Zähler selbst, nicht
/// darum, was andere Fäden drumherum sehen. Für einen Zähler reicht das.
///
/// Counts one up and returns the value from before.
///
/// This function stands there finished. It shows two things at once. First, it
/// needs no `mut` although it changes the counter: an atomic type may be changed
/// through a shared reference, and that is exactly what makes it usable between
/// threads.
///
/// Second, `fetch_add` returns the value from before and not the new one.
/// Whoever wants the new one reads it afterwards with `load` or works it out.
///
/// `Ordering::Relaxed` means here: this is only about the counter itself, not
/// about what other threads see around it. For a counter that is enough.
///
/// ```
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use unit_07_08_send_sync_und_atomare_typen::bump;
///
/// let zaehler = AtomicUsize::new(7);
///
/// assert_eq!(bump(&zaehler), 7);
/// assert_eq!(bump(&zaehler), 8);
/// assert_eq!(zaehler.load(Ordering::Relaxed), 9);
/// ```
pub fn bump(zaehler: &AtomicUsize) -> usize {
    zaehler.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Aufgabe 1: Lass `faeden` Fäden je `je` mal hochzählen.
///
/// Alle zählen denselben Zähler hoch. Zurück kommt sein Stand, nachdem jeder
/// Faden durchgelaufen ist, also `faeden * je`.
///
/// Der Zähler wird über ein `Arc<AtomicUsize>` geteilt. `Arc` ist das `Rc` für
/// mehrere Fäden, und `AtomicUsize` ist der Grund, warum das Hochzählen durch
/// eine gemeinsame Referenz überhaupt geht.
///
/// Vor dem Lesen wird auf jeden Faden gewartet, mit `join`. Ohne das steht der
/// Zähler auf irgendeinem Zwischenstand.
///
/// Exercise 1: let `faeden` threads count up `je` times each.
///
/// All of them count the same counter up. What comes back is its reading after
/// every thread has run through, meaning `faeden * je`.
///
/// The counter is shared through an `Arc<AtomicUsize>`. `Arc` is the `Rc` for
/// several threads, and `AtomicUsize` is the reason counting up through a shared
/// reference works at all.
///
/// Before reading, every thread is waited for, with `join`. Without that the
/// counter stands at some reading in between.
pub fn count_up(faeden: usize, je: usize) -> usize {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Finde das Größte, mit einem Faden je Wert.
///
/// Jeder Wert geht an einen eigenen Faden, und jeder Faden schiebt das
/// gemeinsame Größte mit `fetch_max` nach oben. Zurück kommt der Stand danach.
/// Aus einer leeren Liste wird 0.
///
/// `fetch_max` ist ein Schritt und keine zwei. Lesen, vergleichen und schreiben
/// als drei einzelne Schritte wäre genau die Stelle, an der zwei Fäden einander
/// überschreiben.
///
/// Exercise 2: find the largest one, with one thread per value.
///
/// Every value goes to a thread of its own, and every thread pushes the shared
/// largest one up with `fetch_max`. What comes back is the reading afterwards. An
/// empty list becomes 0.
///
/// `fetch_max` is one step and not two. Reading, comparing and writing as three
/// separate steps would be exactly the place where two threads overwrite each
/// other.
pub fn max_of(werte: Vec<usize>) -> usize {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Lass genau einen von `faeden` Fäden gewinnen.
///
/// Alle Fäden greifen nach demselben `AtomicBool`, der auf `false` steht.
/// Genau einer darf ihn auf `true` setzen und gilt damit als Gewinner. Zurück
/// kommt, wie viele gewonnen haben, und das ist 1, sobald `faeden` mindestens 1
/// ist. Bei 0 Fäden kommt 0 zurück.
///
/// Zu benutzen ist `compare_exchange`. Es setzt nur dann, wenn vorher der
/// erwartete Wert dastand, und sagt im Ergebnis, ob es das getan hat. Ein
/// `load`, gefolgt von einem `store`, tut das nicht: Zwischen beiden kann ein
/// anderer Faden dazwischenkommen.
///
/// Exercise 3: let exactly one of `faeden` threads win.
///
/// All threads reach for the same `AtomicBool`, which stands at `false`. Exactly
/// one is allowed to set it to `true` and counts as the winner for it. What comes
/// back is how many won, and that is 1 as soon as `faeden` is at least 1. With 0
/// threads 0 comes back.
///
/// What is to be used is `compare_exchange`. It sets only when the expected value
/// stood there before, and says in its result whether it did. A `load` followed
/// by a `store` does not do that: another thread can come in between the two.
pub fn only_one_wins(faeden: usize) -> usize {
    todo!("Aufgabe 3 / Exercise 3")
}
