//! 07-06 Kanäle / Channels
//!
//! Deutsch: Ein Kanal bringt Werte von einem Faden zum anderen, ohne dass beide
//! auf dieselbe Stelle im Speicher zeigen. Der Wert wird verschickt und gehört
//! danach der anderen Seite.
//!
//! English: a channel brings values from one thread to another without both of
//! them pointing at the same place in memory. The value is sent and belongs to
//! the other side afterwards.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::sync::mpsc::Receiver;

/// Schickt einen Wert durch einen Kanal und nimmt ihn wieder entgegen.
///
/// Diese Funktion steht fertig da und zeigt die kleinste Form. Sie braucht
/// keinen zweiten Faden: Ein Kanal ist auch innerhalb eines Fadens ein Kanal,
/// nur ohne den Grund, warum es ihn gibt.
///
/// `mpsc` steht für "multiple producer, single consumer": Es darf viele Sender
/// geben und genau einen Empfänger.
///
/// Sends a value through a channel and takes it back.
///
/// This function stands there finished and shows the smallest form. It needs no
/// second thread: a channel is a channel inside one thread as well, only without
/// the reason it exists.
///
/// `mpsc` stands for "multiple producer, single consumer": there may be many
/// senders and exactly one receiver.
///
/// ```
/// use unit_07_06_kanaele::echo;
///
/// assert_eq!(echo(42), 42);
/// ```
pub fn echo(wert: i32) -> i32 {
    let (sender, empfaenger) = std::sync::mpsc::channel();

    sender.send(wert).expect("der Empfaenger lebt noch");

    empfaenger.recv().expect("ein Sender hat etwas geschickt")
}

/// Aufgabe 1: Lass einen zweiten Faden alle Werte schicken und sammle sie ein.
///
/// Ein Faden bekommt die Liste und schickt jeden Wert einzeln durch den Kanal.
/// Hier wird alles entgegengenommen, bis nichts mehr kommt, und als Liste
/// zurückgegeben. Die Reihenfolge bleibt die der Eingabe, denn es schickt nur
/// einer.
///
/// Die Closure für `thread::spawn` braucht `move`, sonst würde sie den Sender
/// nur ausleihen. Die Meldung dazu steht in der README.
///
/// Exercise 1: let a second thread send all values and collect them.
///
/// A thread gets the list and sends every value through the channel one by one.
/// Here everything is taken until nothing comes any more, and given back as a
/// list. The order stays the one of the input, because only one side sends.
///
/// The closure for `thread::spawn` needs `move`, otherwise it would only borrow
/// the sender. The message for that is in the README.
pub fn send_all(werte: Vec<i32>) -> Vec<i32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Lass zwei Fäden gleichzeitig schicken.
///
/// Jede der beiden Listen geht an einen eigenen Faden, und beide schicken durch
/// denselben Kanal. Dafür wird der Sender mit `clone` vervielfältigt.
/// Zurückkommen alle Werte aus beiden Listen.
///
/// In welcher Reihenfolge sie ankommen, ist nicht festgelegt. Der Test sortiert
/// deshalb, bevor er vergleicht, und das ist kein Trick, sondern die einzige
/// Aussage, die hier zu treffen ist.
///
/// Exercise 2: let two threads send at the same time.
///
/// Each of the two lists goes to a thread of its own, and both send through the
/// same channel. For that the sender is multiplied with `clone`. What comes back
/// is every value out of both lists.
///
/// In which order they arrive is not fixed. The test therefore sorts before it
/// compares, and that is not a trick but the only statement to be made here.
pub fn merge_two(links: Vec<i32>, rechts: Vec<i32>) -> Vec<i32> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Nimm alles entgegen, was noch kommt.
///
/// Zu benutzen ist `recv` in einer Schleife, nicht die `for`-Schleife über den
/// Empfänger. `recv` gibt `Ok(wert)` zurück, solange etwas kommt oder noch
/// kommen kann, und `Err`, sobald der letzte Sender weggefallen ist. Genau
/// dieses `Err` beendet die Schleife.
///
/// Exercise 3: take everything that still comes.
///
/// What is to be used is `recv` in a loop, not the `for` loop over the receiver.
/// `recv` gives back `Ok(wert)` for as long as something comes or still can
/// come, and `Err` as soon as the last sender has fallen away. That very `Err`
/// is what ends the loop.
pub fn drain(empfaenger: Receiver<i32>) -> Vec<i32> {
    todo!("Aufgabe 3 / Exercise 3")
}
