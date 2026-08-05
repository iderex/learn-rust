//! 04-09 TryFrom und ein eigener Fehlertyp / TryFrom and an error type of your own
//!
//! Deutsch: `TryFrom` ist `From` für Umwandlungen, die scheitern können. Der
//! Fehlertyp steht als `type Error` daneben und trägt `Display`, damit die
//! Meldung für Menschen an ihm steht.
//!
//! English: `TryFrom` is `From` for conversions that can fail. The error type
//! stands beside it as `type Error` and carries `Display`, so that the message
//! for people stands on it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::fmt;

/// Ein Alter in Jahren.
///
/// Ein Wert dieses Typs entsteht nur über die Prüfung, und deshalb muss ihn
/// niemand ein zweites Mal prüfen.
///
/// An age in years.
///
/// A value of this type only comes into being through the check, and therefore
/// nobody has to check it a second time.
#[derive(Debug, PartialEq)]
pub struct Alter(pub u32);

/// Warum eine Zahl kein Alter ist.
///
/// Why a number is not an age.
#[derive(Debug, PartialEq)]
pub enum AlterFehler {
    /// Die Zahl war negativ / the number was negative.
    Negativ,
    /// Die Zahl war größer als 130 / the number was bigger than 130.
    ZuGross,
}

/// Gibt die Zahl aus einem geprüften Alter zurück.
///
/// Diese Funktion steht fertig da. Sie braucht keine Prüfung mehr, denn ein
/// `Alter` gibt es nur mit einer gültigen Zahl darin.
///
/// Returns the number out of a checked age.
///
/// This function stands there finished. It needs no check any more, because an
/// `Alter` only exists with a valid number in it.
///
/// ```
/// use unit_04_09_tryfrom_und_fehlertyp::{Alter, years_of};
///
/// assert_eq!(years_of(&Alter(42)), 42);
/// ```
pub fn years_of(alter: &Alter) -> u32 {
    alter.0
}

// Deutsch: Aufgabe 1: Prüfe die Zahl und gib sonst einen Fehler zurück.
// Negativ ist `AlterFehler::Negativ`, über 130 ist `AlterFehler::ZuGross`,
// alles dazwischen ist ein `Alter`.
// English: Exercise 1: check the number and otherwise return an error. Negative
// is `AlterFehler::Negativ`, above 130 is `AlterFehler::ZuGross`, everything in
// between is an `Alter`.
impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

// Deutsch: Aufgabe 2: Schreibe die Meldung für Menschen. Für `Negativ` steht
// dort "ein Alter ist nicht negativ", für `ZuGross` "ein Alter über 130 gibt es
// nicht".
// English: Exercise 2: write the message for people. For `Negativ` it reads
// "ein Alter ist nicht negativ", for `ZuGross` "ein Alter über 130 gibt es
// nicht".
impl fmt::Display for AlterFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Wandle eine Zahl in ein Alter um, mit `try_into`.
///
/// `Alter::try_from(zahl)` ginge auch. Gemeint ist hier die andere Seite,
/// nämlich `zahl.try_into()`, die es mit `TryFrom` geschenkt gibt.
///
/// Exercise 3: convert a number into an age, with `try_into`.
///
/// `Alter::try_from(zahl)` would work too. What is meant here is the other
/// side, namely `zahl.try_into()`, which comes as a gift with `TryFrom`.
pub fn age_from(zahl: i32) -> Result<Alter, AlterFehler> {
    todo!("Aufgabe 3 / Exercise 3")
}
