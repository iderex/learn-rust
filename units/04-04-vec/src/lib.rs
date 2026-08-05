//! 04-04 Vec / Vec
//!
//! Deutsch: Ein `Vec<T>` ist eine Liste, die wachsen kann. `push` hängt an,
//! der eckige Klammergriff bricht bei einer Stelle ab, die es nicht gibt, und
//! `get` antwortet dort mit `Option`.
//!
//! English: a `Vec<T>` is a list that can grow. `push` appends, the square
//! brackets break off on a place that does not exist, and `get` answers with an
//! `Option` there.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Nimmt eine Liste, hängt einen Wert an und gibt sie zurück.
///
/// Die Liste wird übernommen und wieder herausgegeben, denn ein `Vec` ist nicht
/// `Copy`. Das ist derselbe Fall wie in `02-01`.
///
/// Takes a list, appends a value and gives it back.
///
/// The list is taken over and handed back out, because a `Vec` is not `Copy`.
/// That is the same case as in `02-01`.
///
/// ```
/// use unit_04_04_vec::pushed;
///
/// assert_eq!(pushed(vec![1, 2], 3), vec![1, 2, 3]);
/// assert_eq!(pushed(Vec::new(), 1), vec![1]);
/// ```
pub fn pushed(zahlen: Vec<i32>, neu: i32) -> Vec<i32> {
    let mut zahlen = zahlen;
    zahlen.push(neu);
    zahlen
}

/// Aufgabe 1: Baue eine Liste von 1 bis einschließlich `bis` auf.
///
/// Bei `bis` gleich null bleibt die Liste leer. Angelegt wird sie mit
/// `Vec::new`, gefüllt mit `push`.
///
/// Exercise 1: build a list from 1 up to and including `bis`.
///
/// For `bis` equal to zero the list stays empty. It is created with `Vec::new`
/// and filled with `push`.
pub fn built(bis: u32) -> Vec<u32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib den größten Wert der Liste zurück.
///
/// Eine leere Liste hat keinen größten Wert, und die Antwort ist dort `None`.
/// Gelesen wird ohne den eckigen Klammergriff.
///
/// Exercise 2: return the biggest value of the list.
///
/// An empty list has no biggest value, and the answer there is `None`. The
/// reading goes without the square brackets.
pub fn largest(zahlen: &[i32]) -> Option<i32> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib eine neue Liste mit verdoppelten Werten zurück.
///
/// Die alte Liste ist nur geliehen und bleibt, wie sie ist.
///
/// Exercise 3: return a new list with doubled values.
///
/// The old list is only borrowed and stays as it is.
pub fn doubled_all(zahlen: &[i32]) -> Vec<i32> {
    todo!("Aufgabe 3 / Exercise 3")
}
