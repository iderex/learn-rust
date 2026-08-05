//! 04-07 panic! und Result / panic! and Result
//!
//! Deutsch: `panic!` hält das Programm an, `Result<T, E>` gibt den Fehler als
//! Wert zurück. Geworfen wird nichts, und der Aufrufer behandelt beide Fälle
//! mit `match`.
//!
//! English: `panic!` stops the program, `Result<T, E>` returns the fault as a
//! value. Nothing is thrown, and the caller treats both cases with `match`.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Was in dieser Einheit schiefgehen kann.
///
/// Ein eigenes `enum`, ein Fall je Aufgabe. Ein Fehlertyp darf alles sein;
/// `04-09` und `04-10` bauen ihn weiter aus.
///
/// What can go wrong in this unit.
///
/// An `enum` of its own, one case per exercise. An error type may be anything;
/// `04-09` and `04-10` build it out further.
#[derive(Debug, PartialEq)]
pub enum Fehler {
    /// Der Nenner war null / the denominator was zero.
    DurchNull,
    /// Die Zahl kann kein Alter sein / the number cannot be an age.
    KeinAlter,
    /// Der Text war leer / the text was empty.
    LeererText,
}

/// Gibt den Rest der Division zurück, oder einen Fehler bei null.
///
/// Diese Funktion steht fertig da und zeigt die Form: der schlechte Fall zuerst
/// mit `return Err(...)`, danach der gute mit `Ok(...)`.
///
/// Returns the remainder of the division, or an error on zero.
///
/// This function stands there finished and shows the shape: the bad case first
/// with `return Err(...)`, then the good one with `Ok(...)`.
///
/// ```
/// use unit_04_07_panic_und_result::{Fehler, remainder};
///
/// assert_eq!(remainder(10, 3), Ok(1));
/// assert_eq!(remainder(10, 0), Err(Fehler::DurchNull));
/// ```
pub fn remainder(a: i32, b: i32) -> Result<i32, Fehler> {
    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a % b)
}

/// Aufgabe 1: Teile `a` durch `b`.
///
/// Bei `b` gleich null ist das Ergebnis `Err(Fehler::DurchNull)`, sonst
/// `Ok(...)`. Kein `panic!`, denn die Null kommt von außen.
///
/// Exercise 1: divide `a` by `b`.
///
/// For `b` equal to zero the result is `Err(Fehler::DurchNull)`, otherwise
/// `Ok(...)`. No `panic!`, because the zero comes from outside.
pub fn divided(a: i32, b: i32) -> Result<i32, Fehler> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Nimm eine Zahl als Alter an.
///
/// Über 130 ist kein Alter mehr, und die Antwort ist dann
/// `Err(Fehler::KeinAlter)`.
///
/// Exercise 2: accept a number as an age.
///
/// Above 130 it is no age any more, and the answer is then
/// `Err(Fehler::KeinAlter)`.
pub fn checked_age(jahre: u32) -> Result<u32, Fehler> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die erste Zeile eines Textes zurück.
///
/// Ein leerer Text hat keine erste Zeile, und die Antwort ist dann
/// `Err(Fehler::LeererText)`. Eine Zeile endet am ersten Zeilenumbruch; steht
/// keiner darin, ist der ganze Text die erste Zeile.
///
/// Exercise 3: return the first line of a text.
///
/// An empty text has no first line, and the answer is then
/// `Err(Fehler::LeererText)`. A line ends at the first line break; if none
/// stands in the text, the whole text is the first line.
pub fn first_line(text: &str) -> Result<String, Fehler> {
    todo!("Aufgabe 3 / Exercise 3")
}
