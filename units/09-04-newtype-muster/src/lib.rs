//! 09-04 Das Newtype-Muster / The newtype pattern
//!
//! Deutsch: Ein Newtype ist ein eigener Typ, der genau einen fremden Wert
//! umschließt. Er kostet zur Laufzeit nichts und trennt trotzdem zwei Zahlen,
//! die sonst dieselbe wären.
//!
//! English: a newtype is a type of your own wrapping exactly one foreign value.
//! It costs nothing at run time and still keeps apart two numbers that would
//! otherwise be the same.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::fmt;

/// Eine Länge in Zentimetern.
///
/// A length in centimetres.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Zentimeter(pub u32);

/// Eine Masse in Gramm.
///
/// Steht hier, damit es einen zweiten Typ über demselben `u32` gibt. Genau
/// dessen Verwechslung mit `Zentimeter` lehnt der Übersetzer ab.
///
/// A mass in grams.
///
/// It stands here so that there is a second type over the same `u32`. Its
/// mix-up with `Zentimeter` is exactly what the compiler refuses.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Gramm(pub u32);

/// Eine Länge in Kilometern.
///
/// A length in kilometres.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Kilometer(pub u32);

/// Ein Newtype um einen fremden Typ.
///
/// `Vec<String>` gehört der Standardbibliothek und `Display` auch. Deshalb darf
/// hier niemand `Display` für `Vec<String>` schreiben. Um diesen eigenen Typ
/// herum darf er es.
///
/// A newtype around a foreign type.
///
/// `Vec<String>` belongs to the standard library and so does `Display`. That is
/// why nobody may write `Display` for `Vec<String>` here. Around this type of
/// your own they may.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Liste(pub Vec<String>);

/// Addiert zwei Längen.
///
/// Diese Funktion steht fertig da. Sie zeigt beide Hälften des Musters: das
/// Auspacken mit `.0`, und dass eine `Gramm` hier nicht hineinpasst, obwohl
/// beide Typen dasselbe `u32` tragen.
///
/// Adds two lengths.
///
/// This function stands there finished. It shows both halves of the pattern:
/// the unwrapping with `.0`, and that a `Gramm` does not fit in here although
/// both types carry the same `u32`.
///
/// ```
/// use unit_09_04_newtype_muster::{Zentimeter, addiere};
///
/// assert_eq!(addiere(Zentimeter(80), Zentimeter(120)), Zentimeter(200));
/// ```
///
/// Deutsch: Und dies übersetzt nicht. Der Doku-Test zeigt, dass die
/// Verwechslung zurückgewiesen wird; welche Meldung dabei entsteht, zeigt er
/// nicht, und sie steht in der README unter "Häufige Fehler".
///
/// English: and this does not compile. The doc test shows that the mix-up is
/// refused; which message comes out of it, it does not show, and that stands in
/// the README under "Common mistakes".
///
/// ```compile_fail
/// use unit_09_04_newtype_muster::{Gramm, Zentimeter, addiere};
///
/// let falsch = addiere(Zentimeter(80), Gramm(120));
/// ```
pub fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

/// Aufgabe 1: Zähle die Längen zusammen.
///
/// Bei einer leeren Liste kommt `Zentimeter(0)` zurück. Die Summe wird über die
/// ausgepackten Zahlen gebildet und am Ende wieder eingepackt, denn ein `u32`
/// ist keine Länge, solange niemand das sagt.
///
/// Exercise 1: add the lengths up.
///
/// For an empty list `Zentimeter(0)` comes back. The sum is formed over the
/// unwrapped numbers and wrapped again at the end, because a `u32` is not a
/// length as long as nobody says so.
pub fn summe(werte: &[Zentimeter]) -> Zentimeter {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Schreib `Display` für `Liste`.
///
/// Die Ausgabe ist eine eckige Klammer auf, die Einträge mit Komma und
/// Leerzeichen dazwischen, und eine eckige Klammer zu. Eine leere Liste gibt
/// `[]`.
///
/// Das ist die zweite Hälfte des Musters. Ein fremdes Trait auf einen fremden
/// Typ zu schreiben ist verboten, und der eigene Typ um den fremden herum hebt
/// genau dieses Verbot auf.
///
/// Exercise 2: write `Display` for `Liste`.
///
/// The output is an opening square bracket, the entries with a comma and a
/// space between them, and a closing square bracket. An empty list gives `[]`.
///
/// That is the second half of the pattern. Writing a foreign trait onto a
/// foreign type is forbidden, and the type of your own around the foreign one
/// lifts exactly that ban.
impl fmt::Display for Liste {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Rechne Kilometer in Zentimeter um.
///
/// Ein Kilometer sind 100000 Zentimeter. Über `From` geschrieben, damit
/// `Zentimeter::from` und `.into()` beide gehen.
///
/// Das ist der Ausweg aus der Strenge von oben. Die Verwechslung bleibt
/// verboten, der Übergang wird einmal aufgeschrieben und ist danach überall
/// derselbe.
///
/// Exercise 3: convert kilometres into centimetres.
///
/// One kilometre is 100000 centimetres. Written through `From`, so that
/// `Zentimeter::from` and `.into()` both work.
///
/// This is the way out of the strictness above. The mix-up stays forbidden, the
/// crossing is written down once and is the same everywhere afterwards.
impl From<Kilometer> for Zentimeter {
    fn from(wert: Kilometer) -> Self {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
