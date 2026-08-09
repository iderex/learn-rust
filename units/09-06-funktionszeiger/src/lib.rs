//! 09-06 Funktionszeiger / Function pointers
//!
//! Deutsch: `fn` mit kleinem f ist ein Typ und kein Trait. Ein Wert dieses Typs
//! zeigt auf eine Funktion, trägt aber nichts mit sich herum, und genau daran
//! entscheidet sich, wann er reicht und wann nicht.
//!
//! English: `fn` with a small f is a type and not a trait. A value of that type
//! points at a function but carries nothing around with it, and exactly that
//! decides when it is enough and when it is not.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Eine Zahl, die in einen eigenen Typ eingepackt ist.
///
/// Deutsch: Der Konstruktor `Marke` ist selbst eine Funktion von `u32` nach
/// `Marke`. Er passt deshalb überall dorthin, wo `fn(u32) -> Marke` steht, und
/// Aufgabe 3 lebt davon.
///
/// A number packed into a type of its own.
///
/// English: the constructor `Marke` is itself a function from `u32` to `Marke`.
/// It therefore fits everywhere `fn(u32) -> Marke` stands, and exercise 3 lives
/// off that.
#[derive(Debug, PartialEq, Eq)]
pub struct Marke(pub u32);

/// Verdoppelt eine Zahl.
///
/// Doubles a number.
pub fn verdoppeln(x: i32) -> i32 {
    x * 2
}

/// Dreht das Vorzeichen einer Zahl um.
///
/// Turns the sign of a number around.
pub fn negieren(x: i32) -> i32 {
    -x
}

/// Wendet `f` zweimal hintereinander auf `wert` an.
///
/// Deutsch: Diese Funktion steht fertig da. Sie nimmt einen Funktionszeiger und
/// nicht ein `impl Fn`, und deshalb geht jede Closure durch, die nichts
/// einfängt, und keine, die etwas einfängt.
///
/// Applies `f` twice in a row to `wert`.
///
/// English: this function stands there finished. It takes a function pointer
/// and not an `impl Fn`, and therefore every closure that captures nothing gets
/// through and none that captures something does.
///
/// # Beispiele / Examples
///
/// ```
/// use unit_09_06_funktionszeiger::{negieren, verdoppeln, zweimal};
///
/// assert_eq!(zweimal(verdoppeln, 3), 12);
/// assert_eq!(zweimal(negieren, 3), 3);
///
/// let ohne_fang: fn(i32) -> i32 = |x| x + 1;
///
/// assert_eq!(zweimal(ohne_fang, 3), 5);
/// ```
pub fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

/// Aufgabe 1: Wende einen Funktionszeiger auf jeden Wert an.
///
/// Zurück kommt eine neue Liste, in der an jeder Stelle `f` vom Wert an
/// derselben Stelle steht. Eine leere Liste ergibt eine leere Liste.
///
/// `f` steht hier als Wert da und nicht als generischer Typ. Wer den Rumpf über
/// einen Iterator schreibt, kann `f` unmittelbar an `map` übergeben, denn `map`
/// erwartet etwas, das `FnMut` erfüllt, und ein `fn` erfüllt das.
///
/// Exercise 1: apply a function pointer to every value.
///
/// What comes back is a new list holding, at every place, `f` of the value at
/// the same place. An empty list gives an empty list.
///
/// `f` stands here as a value and not as a generic type. Whoever writes the body
/// over an iterator can hand `f` straight to `map`, because `map` expects
/// something fulfilling `FnMut`, and an `fn` fulfils that.
pub fn anwenden(werte: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib zu einem Namen den passenden Funktionszeiger heraus.
///
/// Zu `"verdoppeln"` kommt `verdoppeln` zurück, zu `"negieren"` kommt
/// `negieren` zurück, zu allem anderen `None`.
///
/// Das ist die Stelle, an der ein Funktionszeiger mehr kann als eine Closure:
/// Der Rückgabetyp steht ausgeschrieben da, ohne `Box` und ohne Lebenszeit. Mit
/// `Box<dyn Fn(i32) -> i32>` ginge dieselbe Tabelle auch, sie würde nur bei
/// jedem Griff etwas auf dem Heap anlegen.
///
/// Exercise 2: hand out the function pointer matching a name.
///
/// For `"verdoppeln"` back comes `verdoppeln`, for `"negieren"` back comes
/// `negieren`, for everything else `None`.
///
/// This is the place where a function pointer can do more than a closure: the
/// return type stands written out, without a `Box` and without a lifetime. The
/// same table would work with `Box<dyn Fn(i32) -> i32>` as well, it would only
/// put something on the heap at every reach.
pub fn waehle(name: &str) -> Option<fn(i32) -> i32> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Steck jede Zahl in eine `Marke`, ohne eine Closure zu schreiben.
///
/// Zurück kommt eine Liste mit einer `Marke` je Zahl, in derselben Reihenfolge.
///
/// Die Lösung braucht kein `|wert| Marke(wert)`, denn `Marke` ist bereits eine
/// Funktion mit genau dieser Wirkung. clippy weist die Closure an dieser Stelle
/// auch zurück, und der Prüflauf dieses Repositories fährt clippy mit
/// `-D warnings`.
///
/// Exercise 3: put every number into a `Marke`, without writing a closure.
///
/// What comes back is a list with one `Marke` per number, in the same order.
///
/// The solution needs no `|wert| Marke(wert)`, because `Marke` already is a
/// function with exactly that effect. clippy also refuses the closure at this
/// place, and the check run of this repository drives clippy with `-D warnings`.
pub fn einpacken(werte: &[u32]) -> Vec<Marke> {
    todo!("Aufgabe 3 / Exercise 3")
}
