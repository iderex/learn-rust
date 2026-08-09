//! 09-05 Fortgeschrittene Typen / Advanced types
//!
//! Deutsch: Ein Typalias gibt einem Typ einen zweiten Namen und macht keinen
//! neuen daraus. `!` ist der Typ, zu dem es keinen Wert gibt. Ein Typ ohne
//! feste Größe steht in einer Signatur hinter einem `&`, und `?Sized` nimmt die
//! Forderung nach fester Größe zurück.
//!
//! English: a type alias gives a type a second name and makes no new one out of
//! it. `!` is the type that has no value. A type without a fixed size stands in
//! a signature behind a `&`, and `?Sized` takes the demand for a fixed size
//! back.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::fmt::Debug;

/// Ein Ergebnis, dessen Fehler ein Text ist.
///
/// Das ist ein Typalias und kein eigener Typ. `Ergebnis<T>` und
/// `Result<T, String>` sind derselbe Typ, und der Doku-Test unten zeigt genau
/// das: Der Wert wechselt die Schreibweise, ohne umgewandelt zu werden.
///
/// A result whose error is a text.
///
/// This is a type alias and not a type of its own. `Ergebnis<T>` and
/// `Result<T, String>` are the same type, and the doc test below shows exactly
/// that: the value changes its spelling without being converted.
///
/// ```
/// use unit_09_05_fortgeschrittene_typen::Ergebnis;
///
/// let ueber_alias: Ergebnis<u32> = Ok(3);
///
/// // Deutsch: Dieselbe Sache, ausgeschrieben. Es wird nichts umgewandelt.
/// // English: the same thing, written out. Nothing is converted.
/// let ausgeschrieben: Result<u32, String> = ueber_alias;
///
/// assert_eq!(ausgeschrieben, Ok(3));
/// ```
pub type Ergebnis<T> = Result<T, String>;

/// Beschreibt einen Wert so, wie `{:?}` ihn ausgeben würde.
///
/// Diese Funktion steht fertig da und zeigt die Form. `?Sized` in der Schranke
/// nimmt die Forderung nach fester Größe zurück, die sonst still mitgilt.
/// Deshalb darf `T` hier auch `str` oder `[i32]` sein, und deshalb steht der
/// Wert hinter einem `&`.
///
/// Describes a value the way `{:?}` would print it.
///
/// This function stands there finished and shows the shape. `?Sized` in the
/// bound takes back the demand for a fixed size that otherwise holds silently.
/// That is why `T` may be `str` or `[i32]` here, and that is why the value
/// stands behind a `&`.
///
/// ```
/// use unit_09_05_fortgeschrittene_typen::beschreibe;
///
/// // Deutsch: `str` und `[i32]` haben keine feste Größe, `i32` hat eine.
/// // English: `str` and `[i32]` have no fixed size, `i32` has one.
/// assert_eq!(beschreibe("hallo"), "\"hallo\"");
/// assert_eq!(beschreibe(&[1, 2, 3][..]), "[1, 2, 3]");
/// assert_eq!(beschreibe(&7), "7");
/// ```
pub fn beschreibe<T: Debug + ?Sized>(wert: &T) -> String {
    format!("{wert:?}")
}

/// Aufgabe 1: Setze zwei Teile zu einem Text zusammen.
///
/// Sind beide Teile nicht leer, kommt `Ok` mit den beiden Teilen und einem
/// Leerzeichen dazwischen heraus. Ist einer von beiden leer, kommt `Err` mit dem
/// Text `ein Teil ist leer` heraus.
///
/// Der Rückgabetyp ist `Ergebnis<String>`, also `Result<String, String>`. Beide
/// Schreibweisen sind erlaubt, weil sie dasselbe meinen.
///
/// Exercise 1: put two parts together into one text.
///
/// If neither part is empty, `Ok` comes out with the two parts and a space
/// between them. If one of the two is empty, `Err` comes out with the text
/// `ein Teil ist leer`.
///
/// The return type is `Ergebnis<String>`, meaning `Result<String, String>`.
/// Both spellings are allowed, because they mean the same.
pub fn zusammen(links: &str, rechts: &str) -> Ergebnis<String> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Brich das Programm mit einem Grund ab.
///
/// Der Rückgabetyp ist `!`, also der Typ, zu dem es keinen Wert gibt. Eine
/// Funktion mit diesem Rückgabetyp darf nicht zurückkehren, und deshalb bleibt
/// hier nur der Abbruch.
///
/// Die Meldung lautet `Abbruch: ` und dahinter der Grund.
///
/// Exercise 2: abort the program with a reason.
///
/// The return type is `!`, the type that has no value. A function with this
/// return type may not return, and that is why only the abort is left here.
///
/// The message reads `Abbruch: ` followed by the reason.
pub fn abbruch(grund: &str) -> ! {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Liest eine Zahl oder bricht ab.
///
/// Diese Funktion steht fertig da. Sie ist der Grund, warum `!` in dieser
/// Einheit steht: Der zweite Arm liefert nie einen Wert, und trotzdem passt er
/// neben einen Arm, der `u32` liefert. Das liegt an `!` und nicht an `panic!`.
///
/// Reads a number or aborts.
///
/// This function stands there finished. It is the reason why `!` is in this
/// unit: the second arm never delivers a value, and it still fits next to an arm
/// delivering `u32`. That comes from `!` and not from `panic!`.
pub fn zahl_oder_abbruch(text: &str) -> u32 {
    match text.parse::<u32>() {
        Ok(gelesen) => gelesen,
        Err(_) => abbruch(text),
    }
}

/// Aufgabe 3: Gib das erste und das letzte Element heraus.
///
/// Aus einem leeren Ausschnitt wird `None`. Sonst kommt `Some` mit dem ersten
/// und dem letzten Element heraus, und bei genau einem Element ist das zweimal
/// dasselbe.
///
/// `[i32]` hat keine feste Größe, deshalb steht in der Signatur `&[i32]`. Ein
/// `werte: [i32]` wiese der Übersetzer zurück, und die Meldung dazu steht in der
/// README.
///
/// Exercise 3: hand the first and the last element out.
///
/// An empty slice becomes `None`. Otherwise `Some` comes out with the first and
/// the last element, and at exactly one element that is the same one twice.
///
/// `[i32]` has no fixed size, which is why the signature carries `&[i32]`. A
/// `werte: [i32]` would be refused by the compiler, and the message for that is
/// in the README.
pub fn erstes_und_letztes(werte: &[i32]) -> Option<(i32, i32)> {
    todo!("Aufgabe 3 / Exercise 3")
}
