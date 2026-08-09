//! 07-08 Send, Sync und die atomaren Typen / Send, Sync and the atomic types,
//! gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/07-08-send-sync-und-atomare-typen/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/07-08-send-sync-und-atomare-typen/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

/// Zählt eins hoch und gibt den Wert von vorher zurück.
///
/// Counts one up and returns the value from before.
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
    zaehler.fetch_add(1, Ordering::Relaxed)
}

/// Lässt `faeden` Fäden je `je` mal hochzählen.
///
/// Lets `faeden` threads count up `je` times each.
pub fn count_up(faeden: usize, je: usize) -> usize {
    let zaehler = Arc::new(AtomicUsize::new(0));

    let mut laufende = Vec::new();
    for _ in 0..faeden {
        let meiner = Arc::clone(&zaehler);
        laufende.push(thread::spawn(move || {
            for _ in 0..je {
                meiner.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for faden in laufende {
        faden.join().expect("der Faden ist durchgelaufen");
    }

    zaehler.load(Ordering::Relaxed)
}

/// Findet das Größte, mit einem Faden je Wert.
///
/// Finds the largest one, with one thread per value.
pub fn max_of(werte: Vec<usize>) -> usize {
    let groesstes = Arc::new(AtomicUsize::new(0));

    let mut laufende = Vec::new();
    for wert in werte {
        let meines = Arc::clone(&groesstes);
        laufende.push(thread::spawn(move || {
            meines.fetch_max(wert, Ordering::Relaxed);
        }));
    }

    for faden in laufende {
        faden.join().expect("der Faden ist durchgelaufen");
    }

    groesstes.load(Ordering::Relaxed)
}

/// Lässt genau einen von `faeden` Fäden gewinnen.
///
/// Lets exactly one of `faeden` threads win.
pub fn only_one_wins(faeden: usize) -> usize {
    let genommen = Arc::new(AtomicBool::new(false));
    let gewinner = Arc::new(AtomicUsize::new(0));

    let mut laufende = Vec::new();
    for _ in 0..faeden {
        let meiner = Arc::clone(&genommen);
        let zaehler = Arc::clone(&gewinner);
        laufende.push(thread::spawn(move || {
            if meiner
                .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                zaehler.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for faden in laufende {
        faden.join().expect("der Faden ist durchgelaufen");
    }

    gewinner.load(Ordering::Relaxed)
}
