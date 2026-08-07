//! 07-02 Rc / Rc
//!
//! Deutsch: `Rc<T>` erlaubt mehreren Stellen, denselben Wert zu besitzen. Es
//! zählt dazu mit, wie viele es sind, und räumt den Wert weg, wenn die letzte
//! wegfällt. Ändern lässt sich durch ein `Rc` nichts.
//!
//! English: `Rc<T>` allows several places to own the same value. It counts along
//! how many there are and clears the value away when the last one falls away.
//! Nothing can be changed through an `Rc`.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::rc::Rc;

/// Ein Knoten, der auf seinen Elternknoten zeigt.
///
/// Die Wurzel hat kein Eltern, deshalb `Option`. Der Zeiger ist ein `Rc`, denn
/// mehrere Kinder sollen auf dasselbe Eltern zeigen dürfen, ohne es zu
/// kopieren.
///
/// A node pointing at its parent node.
///
/// The root has no parent, hence `Option`. The pointer is an `Rc`, because
/// several children are meant to be allowed to point at the same parent without
/// copying it.
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub eltern: Option<Rc<Node>>,
}

/// Sagt, wie viele Stellen diesen Wert gerade besitzen.
///
/// Diese Funktion steht fertig da. `Rc::strong_count` ist die einzige Stelle, an
/// der die Zahl im Programm sichtbar wird. Sie steigt mit jedem `Rc::clone` und
/// fällt, sobald ein `Rc` weggeräumt wird, also am Ende seines Bereichs.
///
/// Says how many places own this value right now.
///
/// This function stands there finished. `Rc::strong_count` is the only place
/// where the number becomes visible inside the program. It goes up with every
/// `Rc::clone` and down as soon as an `Rc` is cleared away, meaning at the end of
/// its scope.
///
/// ```
/// use std::rc::Rc;
/// use unit_07_02_rc::owners;
///
/// let wert = Rc::new(String::from("Ada"));
/// assert_eq!(owners(&wert), 1);
///
/// let zweiter = Rc::clone(&wert);
/// assert_eq!(owners(&wert), 2);
///
/// drop(zweiter);
/// assert_eq!(owners(&wert), 1);
/// ```
pub fn owners<T>(wert: &Rc<T>) -> usize {
    Rc::strong_count(wert)
}

/// Aufgabe 1: Baue eine Kette aus den Namen.
///
/// Der erste Name wird die Wurzel, jeder weitere hängt sich an den vorigen als
/// Kind. Zurück kommt der letzte Knoten, denn von ihm aus führt der Weg nach
/// oben. Aus einer leeren Liste wird `None`.
///
/// Aus `["a", "b", "c"]` wird also `c`, dessen Eltern `b` ist, dessen Eltern
/// `a` ist, und `a` hat kein Eltern.
///
/// Exercise 1: build a chain out of the names.
///
/// The first name becomes the root, every further one attaches itself to the
/// previous one as a child. What comes back is the last node, because from it
/// the way leads upwards. An empty list becomes `None`.
///
/// Out of `["a", "b", "c"]` comes `c`, whose parent is `b`, whose parent is `a`,
/// and `a` has no parent.
pub fn chain(namen: &[&str]) -> Option<Rc<Node>> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Sag, wie viele Knoten von hier bis zur Wurzel stehen.
///
/// Der Knoten selbst zählt mit. Die Wurzel allein ergibt 1.
///
/// Exercise 2: say how many nodes stand from here up to the root.
///
/// The node itself counts along. The root on its own gives 1.
pub fn depth(knoten: &Rc<Node>) -> usize {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib den Namen der Wurzel zurück.
///
/// Von `knoten` aus wird nach oben gegangen, bis einer kein Eltern mehr hat.
/// Dessen Name kommt zurück. Hat `knoten` selbst kein Eltern, ist er die
/// Wurzel.
///
/// Exercise 3: return the name of the root.
///
/// From `knoten` the way goes upwards until one has no parent any more. Its name
/// comes back. If `knoten` itself has no parent, it is the root.
pub fn root_name(knoten: &Rc<Node>) -> &str {
    todo!("Aufgabe 3 / Exercise 3")
}
