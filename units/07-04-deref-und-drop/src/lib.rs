//! 07-04 Deref und Drop / Deref and Drop
//!
//! Deutsch: Zwei Traits, die einen eigenen Typ wie einen Zeiger aussehen lassen
//! und ihm einen Abgang geben. `Deref` sagt, worauf ein `*` führt, `Drop` sagt,
//! was geschieht, wenn ein Wert wegfällt.
//!
//! English: two traits that make a type of your own look like a pointer and give
//! it an exit. `Deref` says what a `*` leads to, `Drop` says what happens when a
//! value falls away.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::ops::Deref;

/// Ein Etikett, das sich wie ein `str` benutzen lässt.
///
/// Dieser Typ steht fertig da und zeigt das Muster, das Aufgabe 1 dann selbst
/// schreibt. `Target` ist `str` und nicht `String`, denn das ist der Typ, auf
/// den die Methoden gehören, die ein Aufrufer hier erwartet.
///
/// A label that can be used like a `str`.
///
/// This type stands there finished and shows the pattern that exercise 1 then
/// writes itself. `Target` is `str` and not `String`, because that is the type
/// the methods belong to which a caller expects here.
///
/// ```
/// use unit_07_04_deref_und_drop::Etikett;
///
/// let etikett = Etikett(String::from("Ada"));
///
/// // Deutsch: Kein Stern noetig. Der Uebersetzer geht durch `Deref` hindurch,
/// // um `to_uppercase` zu finden.
/// assert_eq!(etikett.to_uppercase(), "ADA");
/// assert_eq!(etikett.len(), 3);
///
/// // Deutsch: Mit Stern geht es auch, und dann steht da ein `str`.
/// assert_eq!(&*etikett, "Ada");
/// ```
pub struct Etikett(pub String);

impl Deref for Etikett {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Ein Behälter, der genau einen Wert hält.
///
/// A container holding exactly one value.
pub struct Karton<T>(pub T);

/// Aufgabe 1: Lass `Karton<T>` sich wie ein `&T` benutzen.
///
/// `Target` ist `T`, und `deref` gibt eine Referenz auf den gehaltenen Wert
/// zurück. Danach findet `karton.len()` an einem `Karton<String>` die Methode
/// von `String`, ohne dass jemand einen Stern hinschreibt.
///
/// Exercise 1: let `Karton<T>` be used like a `&T`.
///
/// `Target` is `T`, and `deref` returns a reference to the held value.
/// Afterwards `karton.len()` on a `Karton<String>` finds the method of `String`,
/// without anybody writing a star.
impl<T> Deref for Karton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

/// Aufgabe 2: Sag, wie lang der Text in einem `Karton<String>` ist.
///
/// Gezählt wird in Bytes, also mit `len`. Zu schreiben ist der Aufruf ohne
/// Stern, denn genau das ist es, was Aufgabe 1 möglich gemacht hat.
///
/// Exercise 2: say how long the text in a `Karton<String>` is.
///
/// Counting is in bytes, so with `len`. What is to be written is the call
/// without a star, because that is exactly what exercise 1 made possible.
pub fn length(karton: &Karton<String>) -> usize {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Ein Wächter, der beim Wegfallen ein Kreuz macht.
///
/// Er hält eine veränderbare Referenz auf ein `bool`. Solange er lebt, steht
/// dort `false`, und sein Abgang setzt es auf `true`.
///
/// A guard that ticks a box when it falls away.
///
/// It holds a mutable reference to a `bool`. While it lives, `false` stands
/// there, and its exit sets it to `true`.
pub struct Wachhund<'a> {
    pub gefallen: &'a mut bool,
}

/// Aufgabe 3: Gib dem Wächter seinen Abgang.
///
/// `drop` läuft, wenn der Wert wegfällt, also am Ende seines Bereichs oder bei
/// `drop(wert)`. Zu tun ist eine Zuweisung: `*self.gefallen` wird `true`.
///
/// Aufrufen lässt sich `drop` nicht von Hand, und die Meldung dazu steht in der
/// README.
///
/// Exercise 3: give the guard its exit.
///
/// `drop` runs when the value falls away, meaning at the end of its scope or at
/// `drop(wert)`. What is to be done is one assignment: `*self.gefallen` becomes
/// `true`.
///
/// `drop` cannot be called by hand, and the message for that is in the README.
impl Drop for Wachhund<'_> {
    fn drop(&mut self) {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
