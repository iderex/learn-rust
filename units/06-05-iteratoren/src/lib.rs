//! 06-05 Iteratoren / Iterators
//!
//! Deutsch: Ein Iterator ist ein Wert, der auf Nachfrage das nächste Stück
//! herausgibt. `Iterator` ist ein Trait mit genau einer verlangten Methode, und
//! alles andere daran kommt geschenkt dazu. Eine Kette aus `map` und `filter`
//! beschreibt nur; gerechnet wird erst, wenn jemand die Kette abschließt.
//!
//! English: an iterator is a value that hands out the next piece when asked.
//! `Iterator` is a trait with exactly one required method, and everything else
//! on it comes for free. A chain of `map` and `filter` only describes; the
//! computing happens once somebody finishes the chain off.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Verdoppelt jede Zahl.
///
/// Diese Funktion steht fertig da und zeigt die drei Teile einer Kette. `iter`
/// macht aus der Liste einen Iterator, `map` beschreibt, was mit jedem Stück
/// geschehen soll, und `collect` fragt so lange nach, bis nichts mehr kommt,
/// und legt das Ergebnis in einen `Vec`.
///
/// Ohne das `collect` stünde hier eine Beschreibung und kein Ergebnis. Was
/// `map` bekommen hat, ist `&i32`, und `wert * 2` gibt daraus ein `i32`.
///
/// Doubles every number.
///
/// This function stands there finished and shows the three parts of a chain.
/// `iter` turns the list into an iterator, `map` describes what should happen
/// to every piece, and `collect` keeps asking until nothing comes any more and
/// puts the result into a `Vec`.
///
/// Without the `collect` there would be a description here and not a result.
/// What `map` got is a `&i32`, and `wert * 2` makes an `i32` out of it.
///
/// ```
/// use unit_06_05_iteratoren::verdoppelt;
///
/// assert_eq!(verdoppelt(&[1, 2, 3]), vec![2, 4, 6]);
/// assert_eq!(verdoppelt(&[]), Vec::<i32>::new());
/// ```
///
/// Deutsch: Und eine Kette, die abgeschlossen wird, rechnet nur so weit, wie
/// der Abschluss nachfragt. `take(2)` fragt zweimal, also entstehen zwei Werte
/// und nicht vier.
///
/// English: and a chain that gets finished off computes only as far as the
/// finisher asks. `take(2)` asks twice, so two values come about and not four.
///
/// ```
/// let werte = [1, 2, 3, 4];
///
/// let kette = werte.iter().map(|wert| wert * 2);
/// let ergebnis: Vec<i32> = kette.take(2).collect();
///
/// assert_eq!(ergebnis, vec![2, 4]);
/// ```
pub fn verdoppelt(werte: &[i32]) -> Vec<i32> {
    werte.iter().map(|wert| wert * 2).collect()
}

/// Aufgabe 1: Gib die Quadrate der geraden Zahlen zurück, in der Reihenfolge
/// der Liste.
///
/// Aus `[1, 2, 3, 4]` wird `[4, 16]`. Eine leere Liste gibt eine leere Liste,
/// und eine Liste ohne gerade Zahl auch.
///
/// Gemeint ist die Kette und nicht die Schleife. `filter` entscheidet, was
/// durchkommt, `map` rechnet, und `collect` schließt ab. Beide Fassungen stehen
/// in der README nebeneinander, damit sichtbar ist, dass sie dasselbe tun.
///
/// Exercise 1: return the squares of the even numbers, in the order of the
/// list.
///
/// Out of `[1, 2, 3, 4]` comes `[4, 16]`. An empty list gives an empty list, and
/// so does a list without an even number.
///
/// What is meant is the chain and not the loop. `filter` decides what gets
/// through, `map` computes, and `collect` finishes off. Both versions stand
/// side by side in the README, so that it is visible they do the same thing.
pub fn quadrate_der_geraden(werte: &[i32]) -> Vec<i32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Ein eigener Iterator, der von 1 bis 5 zählt.
///
/// `stand` ist die letzte herausgegebene Zahl. Angelegt wird er mit
/// [`Zaehler::neu`], und der Zähler gehört zum Wert: zwei Zähler stören
/// einander nicht, und ein aufgebrauchter fängt nicht von vorn an.
///
/// An iterator of your own counting from 1 to 5.
///
/// `stand` is the last number handed out. It is created with [`Zaehler::neu`],
/// and the count belongs to the value: two counters do not disturb each other,
/// and a used-up one does not start over.
#[derive(Debug)]
pub struct Zaehler {
    // Deutsch: Gelesen wird das Feld erst von `next`, und solange dessen Rumpf
    // `todo!()` ist, meldet clippy es als nie gelesen. Die Ausnahme steht
    // deshalb hier und faellt beim Loesen von selbst weg, weil der geloeste
    // Rumpf das Feld liest.
    // English: the field is only read by `next`, and while that body is
    // `todo!()`, clippy reports it as never read. The exception therefore
    // stands here and falls away by itself when solving, because the solved
    // body reads the field.
    #[allow(dead_code)]
    stand: u32,
}

impl Zaehler {
    /// Ein frischer Zähler, der noch nichts herausgegeben hat.
    ///
    /// A fresh counter that has handed out nothing yet.
    pub fn neu() -> Zaehler {
        Zaehler { stand: 0 }
    }
}

/// Aufgabe 2: Mach `Zaehler` zu einem Iterator.
///
/// `next` gibt der Reihe nach `Some(1)` bis `Some(5)` heraus und danach immer
/// `None`. Einmal `None`, immer `None`: ein Zähler, der später wieder zählt,
/// ist nicht gemeint.
///
/// Zu schreiben ist genau diese eine Methode. `map`, `filter`, `sum`, `zip`,
/// `skip` und `take` stehen danach ohne weiteres Zutun auf `Zaehler` bereit,
/// denn sie hängen am Trait und nicht am Typ. Genau das prüfen die Tests, und
/// genau das ist der Grund, ein Trait zu erfüllen statt eigene Methoden zu
/// schreiben.
///
/// Exercise 2: turn `Zaehler` into an iterator.
///
/// `next` hands out `Some(1)` through `Some(5)` in order and `None` from then
/// on. Once `None`, always `None`: a counter that counts again later is not
/// what is meant.
///
/// What is to be written is exactly this one method. `map`, `filter`, `sum`,
/// `zip`, `skip` and `take` stand ready on `Zaehler` afterwards with nothing
/// further done, because they hang on the trait and not on the type. That is
/// exactly what the tests check, and exactly the reason to fulfil a trait
/// instead of writing methods of your own.
impl Iterator for Zaehler {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Gib die erste Zahl heraus, die größer als `schwelle` ist.
///
/// Kommt keine solche Zahl vor, ist die Antwort `None`. Verlangt ist der
/// Abschluss `find`, der die Kette selbst abbricht, sobald er fündig wird.
///
/// Das ist der Punkt, an dem die Faulheit einer Kette zur Ersparnis wird. Wer
/// stattdessen alles einsammelt und danach das erste Stück nimmt, bekommt
/// dieselbe Antwort und hat den Rest der Liste umsonst angesehen.
///
/// Exercise 3: hand out the first number greater than `schwelle`.
///
/// Where no such number occurs, the answer is `None`. What is asked for is the
/// finisher `find`, which breaks the chain off itself as soon as it strikes.
///
/// This is the point where the laziness of a chain turns into a saving. Whoever
/// collects everything instead and takes the first piece afterwards gets the
/// same answer and has looked at the rest of the list for nothing.
pub fn erste_ueber(werte: &[i32], schwelle: i32) -> Option<i32> {
    todo!("Aufgabe 3 / Exercise 3")
}
