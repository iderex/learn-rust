//! 07-07 Mutex und Arc / Mutex and Arc
//!
//! Deutsch: Ein `Mutex` macht aus geteilten Daten veränderbare Daten, indem er
//! zu jedem Zeitpunkt genau einen Faden hineinlässt. Ein `Arc` bringt denselben
//! Wert in mehrere Fäden, ohne ihn zu kopieren. Zusammen ergeben sie
//! `Arc<Mutex<T>>`, und einzeln reicht keiner von beiden.
//!
//! English: a `Mutex` turns shared data into changeable data by letting exactly
//! one thread in at a time. An `Arc` brings the same value into several threads
//! without copying it. Together they make `Arc<Mutex<T>>`, and on its own
//! neither of the two is enough.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};

/// Ein neuer Zähler, der bei null steht und geteilt werden kann.
///
/// Diese Funktion steht fertig da. Sie zeigt die beiden Hüllen in der
/// Reihenfolge, in der sie gebraucht werden: der `Mutex` innen, weil er den
/// Wert schützt, und der `Arc` außen, weil er die Hülle in mehrere Fäden
/// bringt. Umgekehrt ergäbe `Mutex<Arc<usize>>` einen geschützten Zeiger auf
/// einen Wert, den weiter niemand ändern darf, und das ist nicht dasselbe.
///
/// A new counter standing at zero that can be shared.
///
/// This function stands there finished. It shows the two wrappers in the order
/// they are needed: the `Mutex` inside, because it guards the value, and the
/// `Arc` outside, because it brings the guard into several threads. The other
/// way round, `Mutex<Arc<usize>>` would be a guarded pointer to a value nobody
/// may change afterwards, and that is not the same thing.
pub fn neuer_zaehler() -> Arc<Mutex<usize>> {
    Arc::new(Mutex::new(0))
}

/// Erhöht den Zähler um `um`.
///
/// Diese Funktion steht ebenfalls fertig da und ist die ganze Bewegung dieser
/// Einheit in drei Zeilen. `lock` wartet, bis niemand sonst drin ist, und gibt
/// eine Wache heraus, durch die der Wert verändert werden darf. Das `unwrap`
/// gehört dazu: Endet ein Faden in Panik, während er die Sperre hält, ist der
/// `Mutex` danach vergiftet, und jedes weitere `lock` gibt ein `Err` zurück.
///
/// Die Wache geht am Ende der Funktion aus dem Gültigkeitsbereich, und damit
/// wird die Sperre freigegeben. Wer sie länger halten will, hält die Wache
/// länger fest; wer sie kürzer halten will, gibt ihr einen engeren Block.
///
/// Der Parameter ist `&Mutex<usize>` und nicht `&Arc<Mutex<usize>>`. Ein `Arc`
/// gibt seinen Inhalt beim Ausleihen von selbst heraus, und die engere Angabe
/// sagt, dass diese Funktion vom Teilen nichts wissen muss.
///
/// Raises the counter by `um`.
///
/// This function stands there finished as well and is the whole movement of
/// this unit in three lines. `lock` waits until nobody else is inside and hands
/// out a guard through which the value may be changed. The `unwrap` belongs to
/// it: if a thread ends in a panic while holding the lock, the `Mutex` is
/// poisoned afterwards, and every further `lock` gives back an `Err`.
///
/// The guard goes out of scope at the end of the function, and with that the
/// lock is released. Whoever wants to hold it longer holds on to the guard for
/// longer; whoever wants to hold it more briefly gives it a narrower block.
///
/// The parameter is `&Mutex<usize>` and not `&Arc<Mutex<usize>>`. An `Arc` hands
/// out its content by itself when borrowed, and the narrower spelling says that
/// this function needs to know nothing about the sharing.
///
/// ```
/// use std::sync::Arc;
/// use std::thread;
/// use unit_07_07_mutex_und_arc::{erhoehen, neuer_zaehler};
///
/// let zaehler = neuer_zaehler();
/// let mut fertig = Vec::new();
///
/// for _ in 0..4 {
///     let meiner = Arc::clone(&zaehler);
///     fertig.push(thread::spawn(move || {
///         for _ in 0..1000 {
///             erhoehen(&meiner, 1);
///         }
///     }));
/// }
///
/// for faden in fertig {
///     faden.join().unwrap();
/// }
///
/// // Deutsch: Vier Faeden, tausend Schritte je Faden, und keiner geht
/// // verloren. Ohne den Mutex waere diese Zahl bei jedem Lauf eine andere.
/// // English: four threads, a thousand steps each, and none gets lost.
/// // Without the mutex this number would be a different one every run.
/// assert_eq!(*zaehler.lock().unwrap(), 4000);
///
/// // Deutsch: Die vier Kopien des Arc sind mit ihren Faeden weggefallen.
/// // English: the four copies of the Arc fell away with their threads.
/// assert_eq!(Arc::strong_count(&zaehler), 1);
/// ```
pub fn erhoehen(zaehler: &Mutex<usize>, um: usize) {
    let mut stand = zaehler.lock().unwrap();
    *stand += um;
}

/// Aufgabe 1: Zähle mit mehreren Fäden auf einen gemeinsamen Stand.
///
/// `faeden` Fäden erhöhen denselben Zähler je `pro_faden` mal um eins. Heraus
/// kommt der Stand, nachdem alle fertig sind, also `faeden * pro_faden`. Bei
/// null Fäden oder null Schritten kommt null heraus.
///
/// Das Ergebnis hängt nicht davon ab, in welcher Reihenfolge die Fäden
/// drankommen. Genau das ist der Unterschied zu einem Zähler ohne Sperre, bei
/// dem zwei Fäden denselben Stand lesen, beide eins dazuzählen und einer der
/// beiden Schritte danach verschwunden ist.
///
/// Exercise 1: count onto one shared total with several threads.
///
/// `faeden` threads raise the same counter `pro_faden` times by one each. What
/// comes out is the total once all of them are done, meaning
/// `faeden * pro_faden`. With zero threads or zero steps, zero comes out.
///
/// The result does not depend on the order the threads get their turn in. That
/// is exactly the difference from a counter without a lock, where two threads
/// read the same total, both add one, and one of the two steps has disappeared
/// afterwards.
pub fn zaehlen(faeden: usize, pro_faden: usize) -> usize {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Sammle die Quadrate ein, jedes in seinem eigenen Faden.
///
/// Jeder Wert bekommt einen Faden, der sein Quadrat ausrechnet und in eine
/// gemeinsame Liste hängt. Heraus kommt die Liste, wie sie gefüllt wurde.
///
/// Die Reihenfolge in dieser Liste ist nicht festgelegt, und der Test behauptet
/// deshalb keine. Er sortiert, bevor er vergleicht. Wer hier eine Reihenfolge
/// herstellt, indem er die Fäden der Reihe nach einsammelt, löst eine andere
/// Aufgabe; das ist `07-05`, und dort geht es ohne geteilte Daten.
///
/// Exercise 2: collect the squares, each in a thread of its own.
///
/// Every value gets a thread that works out its square and hangs it into one
/// shared list. What comes out is the list the way it was filled.
///
/// The order in that list is not fixed, and the test therefore claims none. It
/// sorts before it compares. Whoever produces an order here by picking the
/// threads up one after another is solving a different exercise; that one is
/// `07-05`, and there it works without shared data.
pub fn einsammeln(werte: Vec<u64>) -> Vec<u64> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Finde den größten Wert, verteilt auf mehrere Fäden.
///
/// Faden Nummer `n` sieht sich die Werte an den Stellen `n`, `n + faeden`,
/// `n + 2 * faeden` und so weiter an, und alle Fäden tragen ihr Ergebnis in
/// denselben `Arc<Mutex<Option<i64>>>` ein. Bei einer leeren Liste kommt `None`
/// heraus. `faeden` von null wird wie eins behandelt.
///
/// Die Werte selbst werden nur gelesen, und dafür reicht ein `Arc` ohne
/// `Mutex`. Geschützt wird das Ergebnis und nicht die Eingabe.
///
/// Lesen und Schreiben des Ergebnisses gehören unter dieselbe Sperre. Wer erst
/// `lock` zum Vergleichen nimmt, die Wache fallen lässt und danach ein zweites
/// `lock` zum Schreiben nimmt, hat zwischen beiden eine Lücke, in der ein
/// anderer Faden einen größeren Wert eingetragen haben kann, der dann
/// überschrieben wird.
///
/// Kein Test in dieser Datei weist diese Lücke zurück, und das ist gemessen und
/// nicht vermutet: eine Fassung mit zwei getrennten `lock` wurde zehnmal
/// laufen gelassen und war zehnmal grün. Jeder Faden trägt hier nur einmal
/// ein, und das Fenster zwischen den beiden Sperren ist zu schmal, als dass ein
/// Test es zuverlässig treffen könnte. Die eine Sperre ist deshalb eine Regel,
/// die der Leser einhält, und keine, die der Lauf erzwingt. Bei `zaehlen` liegt
/// das anders: dort trägt jeder Faden tausende Male ein, und dieselbe Lücke
/// fällt dort auf.
///
/// Exercise 3: find the largest value, spread over several threads.
///
/// Thread number `n` looks at the values at the places `n`, `n + faeden`,
/// `n + 2 * faeden` and so on, and every thread writes its result into the same
/// `Arc<Mutex<Option<i64>>>`. With an empty list `None` comes out. A `faeden` of
/// zero is treated as one.
///
/// The values themselves are only read, and an `Arc` without a `Mutex` is enough
/// for that. What is guarded is the result and not the input.
///
/// Reading and writing the result belong under the same lock. Whoever takes a
/// `lock` to compare, drops the guard and then takes a second `lock` to write
/// has a gap between the two in which another thread can have written a larger
/// value that then gets overwritten.
///
/// No test in this file refuses that gap, and that is measured rather than
/// supposed: a version with two separate `lock` calls was run ten times and was
/// green ten times. Every thread writes only once here, and the window between
/// the two locks is too narrow for a test to hit reliably. The single lock is
/// therefore a rule the reader keeps and not one the run enforces. With
/// `zaehlen` it is different: there every thread writes thousands of times, and
/// the same gap does show.
pub fn hoechste(werte: Vec<i64>, faeden: usize) -> Option<i64> {
    todo!("Aufgabe 3 / Exercise 3")
}
