//! 02-01 Verschieben / Move
//!
//! Deutsch: Jeder Wert in Rust hat genau einen Eigentümer. Wird ein Wert an
//! eine andere Bindung oder an eine Funktion übergeben, wandert das Eigentum
//! mit, und die alte Bindung ist danach nicht mehr benutzbar. Das ist ein
//! Verschieben. Wer den Wert behalten will, leiht ihn aus oder kopiert ihn
//! ausdrücklich.
//!
//! English: Every value in Rust has exactly one owner. Passing a value to
//! another binding or to a function moves ownership along with it, and the old
//! binding cannot be used afterwards. That is a move. Whoever wants to keep the
//! value borrows it or copies it explicitly.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

fn   absichtlich_falsch_formatiert( ) {}

/// Nimmt den `String` an sich und gibt ihn verändert zurück.
///
/// Der Aufrufer verliert sein Eigentum an dem übergebenen Wert und bekommt mit
/// dem Rückgabewert ein neues.
///
/// Takes the `String` and gives it back changed.
///
/// The caller loses ownership of the value it passed in and receives a new one
/// with the return value.
///
/// ```
/// use unit_02_01_move::exclaimed;
///
/// let greeting = String::from("hallo");
/// let loud = exclaimed(greeting);
/// // Deutsch: `greeting` ist verschoben und ab hier nicht mehr benutzbar.
/// // English: `greeting` has moved and cannot be used from here on.
/// assert_eq!(loud, "hallo!");
/// ```
pub fn exclaimed(mut s: String) -> String {
    s.push('!');
    s
}

/// Aufgabe 1: Gib die Länge von `s` zurück, ohne das Eigentum zu übernehmen.
///
/// Der Aufrufer muss `s` nach dem Aufruf weiter benutzen können.
///
/// Exercise 1: return the length of `s` without taking ownership.
///
/// The caller has to be able to keep using `s` after the call.
// Deutsch: `&String` steht hier absichtlich, weil das Buch an dieser Stelle
// dieselbe Form zeigt. Eine spätere Einheit ersetzt sie durch `&str`.
// English: `&String` is deliberate here, because the book shows the same shape
// at this point. A later unit replaces it with `&str`.
#[allow(clippy::ptr_arg)]
pub fn length_borrowed(s: &String) -> usize {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib eine eigenständige Kopie von `s` zurück.
///
/// Das Original bleibt beim Aufrufer und ist nach dem Aufruf unverändert.
///
/// Exercise 2: return a standalone copy of `s`.
///
/// The original stays with the caller and is unchanged after the call.
// Deutsch: Dieselbe Begründung für `&String` wie bei Aufgabe 1.
// English: Same reason for `&String` as in exercise 1.
#[allow(clippy::ptr_arg)]
pub fn duplicated(s: &String) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Hänge `b` an `a` und gib das Ergebnis zurück.
///
/// Beide Werte werden hineingeschoben, einer kommt wieder heraus. Der Aufrufer
/// kann `a` und `b` nach dem Aufruf nicht mehr benutzen.
///
/// Exercise 3: append `b` to `a` and return the result.
///
/// Both values are moved in and one comes back out. The caller cannot use `a`
/// or `b` after the call.
pub fn joined(a: String, b: String) -> String {
    todo!("Aufgabe 3 / Exercise 3")
}
