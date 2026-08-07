//! 07-01 Box / Box
//!
//! Deutsch: `Box<T>` legt einen Wert auf den Heap und lässt einen Zeiger auf
//! dem Stack. Damit bekommt ein Typ, der sich selbst enthält, eine bekannte
//! Größe.
//!
//! English: `Box<T>` puts a value on the heap and leaves a pointer on the
//! stack. With it a type containing itself gets a known size.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Eine Liste aus Gliedern, jedes mit einer Zahl und dem Rest dahinter.
///
/// Der Rest steht hinter einer `Box`. Ohne sie enthielte `Glied` wieder eine
/// ganze `Liste`, und der Übersetzer könnte nicht ausrechnen, wie groß ein
/// Wert dieses Typs ist.
///
/// A list of links, each with a number and the rest behind it.
///
/// The rest stands behind a `Box`. Without it `Glied` would contain a whole
/// `Liste` again, and the compiler could not work out how large a value of this
/// type is.
#[derive(Debug, PartialEq)]
pub enum Liste {
    Glied(i64, Box<Liste>),
    Ende,
}

/// Zählt die Glieder einer Liste.
///
/// Diese Funktion steht fertig da und zeigt die Form: ein `match` über die
/// beiden Fälle, und im ersten geht es mit dem Rest weiter. `rest` ist eine
/// `&Box<Liste>`, und der Aufruf nimmt sie an, weil eine `Box` sich wie das
/// verhält, worauf sie zeigt.
///
/// Counts the links of a list.
///
/// This function stands there finished and shows the shape: a `match` over the
/// two cases, and in the first one it carries on with the rest. `rest` is a
/// `&Box<Liste>`, and the call takes it, because a `Box` behaves like the thing
/// it points at.
///
/// ```
/// use unit_07_01_box::{Liste, length};
///
/// let liste = Liste::Glied(1, Box::new(Liste::Glied(2, Box::new(Liste::Ende))));
///
/// assert_eq!(length(&liste), 2);
/// assert_eq!(length(&Liste::Ende), 0);
/// ```
pub fn length(liste: &Liste) -> usize {
    match liste {
        Liste::Glied(_, rest) => 1 + length(rest),
        Liste::Ende => 0,
    }
}

/// Aufgabe 1: Bau aus einem Slice eine Liste.
///
/// Die erste Zahl des Slices wird das erste Glied, danach kommt der Rest.
/// Ein leeres Slice ergibt `Liste::Ende`. Jedes `Glied` braucht eine `Box`
/// um den Rest herum, und `Box::new` ist es, die den Rest auf den Heap legt.
///
/// Exercise 1: build a list out of a slice.
///
/// The first number of the slice becomes the first link, and after it comes the
/// rest. An empty slice gives `Liste::Ende`. Every `Glied` needs a `Box` around
/// the rest, and `Box::new` is what puts that rest on the heap.
pub fn from_slice(zahlen: &[i64]) -> Liste {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Addiere alle Zahlen einer Liste.
///
/// Eine Liste ohne Glieder hat die Summe null.
///
/// Exercise 2: add up every number of a list.
///
/// A list without links has the sum zero.
pub fn sum(liste: &Liste) -> i64 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag, ob eine Zahl in der Liste vorkommt.
///
/// In `Liste::Ende` kommt nichts vor.
///
/// Exercise 3: say whether a number turns up in the list.
///
/// Nothing turns up in `Liste::Ende`.
pub fn contains(liste: &Liste, gesucht: i64) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}
