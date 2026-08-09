//! 07-06 Kanäle / Channels, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-06-kanaele/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-06-kanaele/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

/// Schickt einen Wert durch einen Kanal und nimmt ihn wieder entgegen.
///
/// Sends a value through a channel and takes it back.
///
/// ```
/// use unit_07_06_kanaele::echo;
///
/// assert_eq!(echo(42), 42);
/// ```
pub fn echo(wert: i32) -> i32 {
    let (sender, empfaenger) = mpsc::channel();

    sender.send(wert).expect("der Empfaenger lebt noch");

    empfaenger.recv().expect("ein Sender hat etwas geschickt")
}

/// Lässt einen zweiten Faden alle Werte schicken und sammelt sie ein.
///
/// Lets a second thread send all values and collects them.
pub fn send_all(werte: Vec<i32>) -> Vec<i32> {
    let (sender, empfaenger) = mpsc::channel();

    thread::spawn(move || {
        for wert in werte {
            sender.send(wert).expect("der Empfaenger lebt noch");
        }
    });

    empfaenger.iter().collect()
}

/// Lässt zwei Fäden gleichzeitig durch denselben Kanal schicken.
///
/// Lets two threads send through the same channel at the same time.
pub fn merge_two(links: Vec<i32>, rechts: Vec<i32>) -> Vec<i32> {
    let (sender, empfaenger) = mpsc::channel();
    let zweiter = sender.clone();

    thread::spawn(move || {
        for wert in links {
            sender.send(wert).expect("der Empfaenger lebt noch");
        }
    });
    thread::spawn(move || {
        for wert in rechts {
            zweiter.send(wert).expect("der Empfaenger lebt noch");
        }
    });

    empfaenger.iter().collect()
}

/// Nimmt alles entgegen, was noch kommt.
///
/// Takes everything that still comes.
pub fn drain(empfaenger: Receiver<i32>) -> Vec<i32> {
    let mut gesammelt = Vec::new();

    while let Ok(wert) = empfaenger.recv() {
        gesammelt.push(wert);
    }

    gesammelt
}
