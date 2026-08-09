//! 07-05 Threads / Threads, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-05-threads/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-05-threads/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

use std::thread;

/// Verdoppelt `wert` in einem eigenen Faden.
///
/// Doubles `wert` in a thread of its own.
///
/// ```
/// use unit_07_05_threads::in_einem_faden;
///
/// assert_eq!(in_einem_faden(21), 42);
/// assert_eq!(in_einem_faden(0), 0);
/// ```
///
/// ```
/// use std::thread;
///
/// let faden = thread::spawn(|| 6 * 7);
///
/// assert_eq!(faden.join().unwrap(), 42);
/// ```
pub fn in_einem_faden(wert: i32) -> i32 {
    let faden = thread::spawn(move || wert * 2);

    faden.join().unwrap()
}

/// Die Summe der Zahlen, jede in ihrem eigenen Faden.
///
/// The sum of the numbers, each in a thread of its own.
pub fn summe_in_faeden(werte: Vec<i32>) -> i32 {
    let mut faeden = Vec::new();
    for wert in werte {
        faeden.push(thread::spawn(move || wert));
    }

    let mut summe = 0;
    for faden in faeden {
        summe += faden.join().unwrap();
    }

    summe
}

/// Die Quadrate der Zahlen, jedes in seinem eigenen Faden, in der Reihenfolge
/// der Liste.
///
/// The squares of the numbers, each in a thread of its own, in the order of the
/// list.
pub fn quadrate_in_faeden(werte: Vec<i32>) -> Vec<i32> {
    let mut faeden = Vec::new();
    for wert in werte {
        faeden.push(thread::spawn(move || wert * wert));
    }

    let mut quadrate = Vec::new();
    for faden in faeden {
        quadrate.push(faden.join().unwrap());
    }

    quadrate
}

/// Die Summe der Textlängen, jeder Text in seinem eigenen Faden.
///
/// The sum of the text lengths, each text in a thread of its own.
pub fn zeichen_in_faeden(texte: Vec<String>) -> usize {
    let mut faeden = Vec::new();
    for text in texte {
        faeden.push(thread::spawn(move || text.len()));
    }

    let mut zeichen = 0;
    for faden in faeden {
        zeichen += faden.join().unwrap();
    }

    zeichen
}
