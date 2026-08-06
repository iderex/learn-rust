//! 09-03 Operatorüberladung / Operator overloading
//!
//! Deutsch: Ein Operator ist in Rust ein Trait. `+` ruft `Add::add` auf, die
//! eckigen Klammern rufen `Index::index` auf. Wer diese Traits für einen eigenen
//! Typ schreibt, gibt dem Operator dort eine Bedeutung, und nur dort.
//!
//! English: an operator in Rust is a trait. `+` calls `Add::add`, the square
//! brackets call `Index::index`. Whoever writes these traits for a type of their
//! own gives the operator a meaning there, and only there.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::ops::{Add, Index, Neg};

/// Ein Punkt auf der Fläche.
///
/// `Copy` steht hier mit, damit ein `a + b` die beiden nicht aufbraucht. Ohne
/// das nähme `add` seine Argumente an sich, und danach wären sie weg.
///
/// A point on the surface.
///
/// `Copy` stands here as well so that an `a + b` does not use the two up.
/// Without it `add` would take its arguments for itself, and afterwards they
/// would be gone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Das Gegenteil eines Punktes, also beide Zahlen mit umgedrehtem Vorzeichen.
///
/// Diese Implementierung steht fertig da und zeigt die Form. `Neg` gehört zum
/// einstelligen Minus, `Output` sagt, was herauskommt, und `self` heißt, dass
/// der Operator seinen Wert an sich nimmt.
///
/// The opposite of a point, meaning both numbers with the sign turned around.
///
/// This implementation stands there finished and shows the shape. `Neg` belongs
/// to the unary minus, `Output` says what comes out, and `self` means the
/// operator takes its value for itself.
///
/// ```
/// use unit_09_03_operatorueberladung::Point;
///
/// let punkt = Point { x: 1, y: -2 };
///
/// assert_eq!(-punkt, Point { x: -1, y: 2 });
///
/// // Deutsch: `Point` ist `Copy`, also steht `punkt` danach noch da.
/// // English: `Point` is `Copy`, so `punkt` is still there afterwards.
/// assert_eq!(punkt, Point { x: 1, y: -2 });
/// ```
impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// Aufgabe 1: Gib dem Plus eine Bedeutung für `Point`.
///
/// Zusammengezählt wird je Achse: `x` mit `x`, `y` mit `y`. Heraus kommt wieder
/// ein `Point`, also ist `Output` gleich `Point`.
///
/// Ohne diese Implementierung weist der Übersetzer `a + b` zurück, und die
/// Meldung dazu steht in der README.
///
/// Exercise 1: give the plus a meaning for `Point`.
///
/// Adding happens per axis: `x` with `x`, `y` with `y`. What comes out is a
/// `Point` again, so `Output` equals `Point`.
///
/// Without this implementation the compiler refuses `a + b`, and the message for
/// that is in the README.
impl Add for Point {
    type Output = Point;

    fn add(self, andere: Point) -> Point {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

/// Eine Woche mit sieben Kürzeln.
///
/// A week with seven abbreviations.
pub struct Week {
    pub tage: [&'static str; 7],
}

/// Aufgabe 2: Gib den eckigen Klammern eine Bedeutung für `Week`.
///
/// `woche[0]` ist `"Mo"`, `woche[6]` ist `"So"`. Zurück kommt eine Referenz und
/// keine Kopie, denn der Wert bleibt in der Woche liegen. `Output` ist deshalb
/// `str` und nicht `&str`: Das kaufmännische Und steht schon in der Signatur von
/// `index`.
///
/// Eine Stelle über 6 bricht ab. Das muss hier nicht geschrieben werden, denn
/// der Zugriff auf das Feld tut es von selbst.
///
/// Exercise 2: give the square brackets a meaning for `Week`.
///
/// `woche[0]` is `"Mo"`, `woche[6]` is `"So"`. What comes back is a reference and
/// not a copy, because the value stays lying in the week. `Output` is therefore
/// `str` and not `&str`: the ampersand already stands in the signature of
/// `index`.
///
/// A place above 6 aborts. That need not be written here, because reaching into
/// the field does it by itself.
impl Index<usize> for Week {
    type Output = str;

    fn index(&self, stelle: usize) -> &str {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Zähle eine Liste von Punkten zusammen.
///
/// Angefangen wird bei `Point { x: 0, y: 0 }`, und jeder Punkt der Liste kommt
/// dazu. Aus einer leeren Liste wird der Punkt bei null.
///
/// Zu benutzen ist das `+` aus Aufgabe 1. Das ist der Sinn der Sache: Von hier
/// aus sieht `Point` aus wie eine Zahl, und dieser Rumpf muss nicht wissen, wie
/// das Zusammenzählen gemacht ist.
///
/// Exercise 3: add a list of points up.
///
/// The start is `Point { x: 0, y: 0 }`, and every point of the list comes on
/// top. An empty list becomes the point at zero.
///
/// What is to be used is the `+` from exercise 1. That is the point of the
/// thing: from here `Point` looks like a number, and this body need not know how
/// the adding is done.
pub fn sum(punkte: &[Point]) -> Point {
    todo!("Aufgabe 3 / Exercise 3")
}
