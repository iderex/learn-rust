//! 07-05 Threads / Threads
//!
//! Deutsch: Ein Faden ist ein zweiter Ablauf im selben Programm. `spawn`
//! startet ihn, `join` wartet auf ihn und holt seinen Rückgabewert ab, und
//! `move` vor der Closure sagt, dass der Faden die eingefangenen Werte
//! übernimmt statt sie auszuleihen.
//!
//! English: a thread is a second course of events inside the same program.
//! `spawn` starts it, `join` waits for it and picks up its return value, and
//! `move` in front of the closure says that the thread takes the captured
//! values over instead of borrowing them.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::thread;

/// Verdoppelt `wert` in einem eigenen Faden.
///
/// Diese Funktion steht fertig da und zeigt die drei Teile. `spawn` nimmt eine
/// Closure entgegen und startet sie nebenher. `move` schiebt `wert` in die
/// Closure hinein, denn der Faden kann länger leben als diese Funktion, und
/// eine ausgeliehene Zahl wäre dann weg. `join` wartet, bis der Faden fertig
/// ist, und gibt zurück, was die Closure zurückgegeben hat.
///
/// Das `unwrap` gehört zu `join`. Ein Faden kann in Panik enden, und dann ist
/// das Ergebnis ein `Err`. Hier bricht das Programm in dem Fall ab, was für
/// eine Übung richtig und für ein Werkzeug zu wenig ist.
///
/// Doubles `wert` in a thread of its own.
///
/// This function stands there finished and shows the three parts. `spawn` takes
/// a closure and starts it alongside. `move` pushes `wert` into the closure,
/// because the thread can live longer than this function, and a borrowed number
/// would be gone by then. `join` waits until the thread is done and returns
/// what the closure returned.
///
/// The `unwrap` belongs to `join`. A thread can end in a panic, and then the
/// result is an `Err`. Here the program stops in that case, which is right for
/// an exercise and too little for a tool.
///
/// ```
/// use unit_07_05_threads::in_einem_faden;
///
/// assert_eq!(in_einem_faden(21), 42);
/// assert_eq!(in_einem_faden(0), 0);
/// ```
///
/// Deutsch: Und das Muster ohne die Funktion drumherum. Der Wert kommt aus dem
/// `join` und nicht aus einer Variablen, die beide Fäden anfassen.
///
/// English: and the pattern without the function around it. The value comes out
/// of the `join` and not out of a variable both threads touch.
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

/// Aufgabe 1: Zähle die Zahlen zusammen, jede in ihrem eigenen Faden.
///
/// Für jede Zahl in `werte` wird ein Faden gestartet, der sie zurückgibt, und
/// die Summe entsteht aus den abgeholten Ergebnissen. Eine leere Liste gibt 0.
///
/// Erst starten, dann warten. Wer im selben Durchlauf startet und sofort
/// `join` aufruft, hat zu jedem Zeitpunkt einen Faden laufen und rechnet
/// nacheinander; das Ergebnis stimmt, und nebeneinander gerechnet wurde nichts.
///
/// Exercise 1: add the numbers up, each in a thread of its own.
///
/// For every number in `werte` a thread is started that gives it back, and the
/// sum comes about from the picked-up results. An empty list gives 0.
///
/// Start first, then wait. Whoever starts and calls `join` right away in the
/// same pass has one thread running at any moment and computes one after the
/// other; the result is right, and nothing was computed side by side.
pub fn summe_in_faeden(werte: Vec<i32>) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Quadriere jede Zahl in ihrem eigenen Faden.
///
/// Zurück kommen die Quadrate in der Reihenfolge von `werte`. Das ist kein
/// Widerspruch dazu, dass die Fäden in beliebiger Reihenfolge fertig werden:
/// Eingesammelt wird in der Reihenfolge der Fäden, und die steht fest, weil sie
/// beim Starten entsteht.
///
/// Exercise 2: square every number in a thread of its own.
///
/// What comes back are the squares in the order of `werte`. That is no
/// contradiction to the threads finishing in any order: the collecting happens
/// in the order of the threads, and that order is fixed, because it comes about
/// while starting them.
pub fn quadrate_in_faeden(werte: Vec<i32>) -> Vec<i32> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Zähle die Zeichen aller Texte, jeden Text in seinem eigenen
/// Faden.
///
/// Gezählt wird mit `len`, also in Bytes, und zurück kommt die Summe über alle
/// Texte. Eine leere Liste gibt 0.
///
/// Hier ist `move` nicht mehr nur eine Formsache. Ein `String` ist kein `i32`:
/// Wo eine Zahl mitkopiert wird, geht der Text in den Faden über, und danach
/// gehört er ihm. Was der Übersetzer sagt, wenn das `move` fehlt, steht in der
/// README unter "Häufige Fehler".
///
/// Exercise 3: count the characters of all texts, each text in a thread of its
/// own.
///
/// The counting goes through `len`, meaning in bytes, and what comes back is
/// the sum over all texts. An empty list gives 0.
///
/// Here `move` is no longer a formality. A `String` is not an `i32`: where a
/// number is copied along, the text passes into the thread, and belongs to it
/// afterwards. What the compiler says when the `move` is missing stands in the
/// README under "Common mistakes".
pub fn zeichen_in_faeden(texte: Vec<String>) -> usize {
    todo!("Aufgabe 3 / Exercise 3")
}
