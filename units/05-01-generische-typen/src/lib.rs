//! 05-01 Generische Typen / Generic types
//!
//! Deutsch: Ein Typparameter ist ein Platzhalter für einen Typ. Er steht an
//! Funktionen, an Structs und an Enums, und der Übersetzer schreibt je
//! benutztem Typ eine eigene Fassung.
//!
//! English: a type parameter is a placeholder for a type. It stands on
//! functions, on structs and on enums, and the compiler writes a version of its
//! own per type used.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Paar aus zwei Werten desselben Typs.
///
/// A pair of two values of the same type.
#[derive(Debug, PartialEq)]
pub struct Paar<T> {
    /// Die linke Seite / the left side.
    pub links: T,
    /// Die rechte Seite / the right side.
    pub rechts: T,
}

/// Gibt den ersten Wert einer Liste zurück, falls es einen gibt.
///
/// Diese Funktion steht fertig da und arbeitet für jeden Typ, denn sie tut
/// nichts mit dem Wert außer ihn zurückzugeben.
///
/// Returns the first value of a list, if there is one.
///
/// This function stands there finished and works for every type, because it
/// does nothing with the value except return it.
///
/// ```
/// use unit_05_01_generische_typen::first_of;
///
/// assert_eq!(first_of(&[3, 9, 4]), Some(&3));
/// assert_eq!(first_of(&["drei", "neun"]), Some(&"drei"));
/// assert_eq!(first_of::<i32>(&[]), None);
/// ```
pub fn first_of<T>(werte: &[T]) -> Option<&T> {
    werte.first()
}

/// Aufgabe 1: Gib den letzten Wert einer Liste zurück.
///
/// Eine leere Liste hat keinen letzten Wert. Eine Schranke braucht es nicht,
/// denn der Wert wird nur weitergereicht.
///
/// Exercise 1: return the last value of a list.
///
/// An empty list has no last value. No bound is needed, because the value is
/// only handed on.
pub fn last_of<T>(werte: &[T]) -> Option<&T> {
    todo!("Aufgabe 1 / Exercise 1")
}

impl<T> Paar<T> {
    /// Aufgabe 2: Lege ein Paar aus zwei Werten an.
    ///
    /// Das `T` hinter `impl` führt den Typparameter ein, das `T` hinter `Paar`
    /// benutzt ihn.
    ///
    /// Exercise 2: create a pair out of two values.
    ///
    /// The `T` behind `impl` introduces the type parameter, the `T` behind
    /// `Paar` uses it.
    pub fn new(links: T, rechts: T) -> Self {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Vertausche die beiden Seiten eines Paars.
///
/// Das Paar wird übernommen und ein neues zurückgegeben. Auch hier reicht das
/// freie `T`, denn die Werte werden nur verschoben.
///
/// Exercise 3: swap the two sides of a pair.
///
/// The pair is taken over and a new one returned. Here too the free `T` is
/// enough, because the values are only moved.
pub fn swapped<T>(paar: Paar<T>) -> Paar<T> {
    todo!("Aufgabe 3 / Exercise 3")
}
