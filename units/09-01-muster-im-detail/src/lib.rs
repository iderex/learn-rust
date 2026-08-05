//! 09-01 Muster im Detail / Patterns in detail
//!
//! Deutsch: Muster stehen nicht nur in `match`. Sie können scheitern oder
//! nicht, ein Wächter engt einen Zweig weiter ein, `@` hält fest, was gerade
//! geprüft wurde, und ein verschachtelter Wert geht in einem Zug auseinander.
//!
//! English: patterns do not only stand in `match`. They can fail or not, a
//! guard narrows an arm further, `@` holds on to what was just checked, and a
//! nested value comes apart in one go.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Punkt auf der Fläche.
///
/// A point on the surface.
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Ein Ereignis, das einen Punkt enthalten kann.
///
/// An event that may contain a point.
#[derive(Debug, PartialEq)]
pub enum Event {
    Click { punkt: Point, taste: char },
    Key(char),
    Nothing,
}

/// Sagt, um welche Art Ereignis es sich handelt.
///
/// Diese Funktion steht fertig da und zeigt die einfachste Form: ein Muster je
/// Variante, ohne Wächter und ohne Bindung. `..` heißt, dass der Rest der
/// Variante hier nicht interessiert.
///
/// Says what kind of event this is.
///
/// This function stands there finished and shows the simplest form: one pattern
/// per variant, without a guard and without a binding. `..` means that the rest
/// of the variant is of no interest here.
///
/// ```
/// use unit_09_01_muster_im_detail::{Event, Point, kind};
///
/// let klick = Event::Click {
///     punkt: Point { x: 2, y: 2 },
///     taste: 'L',
/// };
///
/// assert_eq!(kind(&klick), "Klick");
/// assert_eq!(kind(&Event::Key('7')), "Taste");
/// assert_eq!(kind(&Event::Nothing), "nichts");
/// ```
pub fn kind(ereignis: &Event) -> &'static str {
    match ereignis {
        Event::Click { .. } => "Klick",
        Event::Key(_) => "Taste",
        Event::Nothing => "nichts",
    }
}

/// Aufgabe 1: Beschreibe ein Ereignis in einem Satz.
///
/// Sechs Zweige, in dieser Reihenfolge, denn der erste passende gewinnt.
///
/// 1. Klick, dessen Punkt `x == y` erfüllt: `"<taste> auf der Diagonalen bei
///    <x>"`, mit einem Wächter.
/// 2. Klick mit `x` gleich 0: `"am linken Rand, <y> tief"`.
/// 3. Jeder andere Klick: `"bei <x> und <y>"`.
/// 4. Taste, deren Zeichen eine Ziffer ist: `"Ziffer <zeichen>"`, mit einer
///    Bindung `@` auf den Bereich `'0'..='9'`.
/// 5. Jede andere Taste: `"Taste <zeichen>"`.
/// 6. `Nothing`: `"nichts"`.
///
/// Exercise 1: describe an event in one sentence.
///
/// Six arms, in this order, because the first matching one wins.
///
/// 1. A click whose point fulfils `x == y`: `"<taste> auf der Diagonalen bei
///    <x>"`, with a guard.
/// 2. A click with `x` equal to 0: `"am linken Rand, <y> tief"`.
/// 3. Any other click: `"bei <x> und <y>"`.
/// 4. A key whose character is a digit: `"Ziffer <zeichen>"`, with a binding
///    `@` on the range `'0'..='9'`.
/// 5. Any other key: `"Taste <zeichen>"`.
/// 6. `Nothing`: `"nichts"`.
pub fn describe(ereignis: &Event) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Räume einen Stapel von oben nach unten ab.
///
/// Heraus kommt die Liste in umgekehrter Reihenfolge. Zu lösen ist das mit
/// `while let Some(oben) = stapel.pop()`, also mit einem Muster, das scheitern
/// darf, denn sein Scheitern beendet die Schleife.
///
/// Exercise 2: clear a stack from the top down.
///
/// What comes out is the list in reverse order. Solve it with
/// `while let Some(oben) = stapel.pop()`, meaning with a pattern that is
/// allowed to fail, because its failure is what ends the loop.
pub fn drain_stack(stapel: Vec<i32>) -> Vec<i32> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Finde den Punkt des ersten Klicks in einer Liste.
///
/// Gibt es keinen Klick, kommt `None` heraus. Hier steht das Muster in einem
/// `if let` innerhalb einer Schleife, also wieder an einer Stelle, an der es
/// scheitern darf.
///
/// Exercise 3: find the point of the first click in a list.
///
/// If there is no click, `None` comes out. Here the pattern stands in an
/// `if let` inside a loop, so again at a place where it may fail.
pub fn first_click(ereignisse: &[Event]) -> Option<&Point> {
    todo!("Aufgabe 3 / Exercise 3")
}
