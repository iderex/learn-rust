//! 02-02 Stack und Heap / Stack and heap
//!
//! Deutsch: Werte mit fester Größe liegen auf dem Stack, Werte mit erst beim
//! Laufen bekannter Größe auf dem Heap. Ist ein Typ `Copy`, legt eine Zuweisung
//! eine zweite Kopie an und die alte Bindung bleibt benutzbar. Sonst wird
//! verschoben.
//!
//! English: values with a fixed size lie on the stack, values whose size is
//! known only while the program runs lie on the heap. If a type is `Copy`, an
//! assignment makes a second copy and the old binding stays usable. Otherwise
//! it is moved.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt die Summe zweier Zahlen zurück.
///
/// `i32` ist `Copy`, deshalb behält der Aufrufer seine beiden Zahlen, obwohl
/// sie ohne `&` übergeben werden.
///
/// Returns the sum of two numbers.
///
/// `i32` is `Copy`, so the caller keeps both of its numbers although they are
/// handed over without a `&`.
///
/// ```
/// use unit_02_02_stack_und_heap::sum_of;
///
/// let a = 20;
/// let b = 22;
///
/// assert_eq!(sum_of(a, b), 42);
///
/// // Deutsch: a und b stehen nach dem Aufruf noch da.
/// // English: a and b are still there after the call.
/// assert_eq!(a + b, 42);
/// ```
pub fn sum_of(a: i32, b: i32) -> i32 {
    a + b
}

/// Aufgabe 1: Gib das Doppelte von `zahl` zurück.
///
/// Der Aufrufer behält seine Zahl, denn `i32` ist `Copy`. Ein Test sieht genau
/// das nach.
///
/// Exercise 1: return the double of `zahl`.
///
/// The caller keeps its number, because `i32` is `Copy`. One test checks
/// exactly that.
pub fn twice(zahl: i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Hänge ein Ausrufezeichen an und gib den `String` zurück.
///
/// Der `String` wird hereingegeben und wieder herausgegeben, denn er ist nicht
/// `Copy`. Wer ihn nur hereingibt, hat ihn danach nicht mehr.
///
/// Exercise 2: append an exclamation mark and return the `String`.
///
/// The `String` is handed in and handed back out, because it is not `Copy`.
/// Whoever only hands it in does not have it afterwards.
pub fn with_exclamation(text: String) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag zu einem Typnamen, ob eine Zuweisung ihn kopiert.
///
/// Kopiert werden die ganzen Zahlen, die Fließkommazahlen, `bool`, `char`,
/// geteilte Referenzen und Tupel, deren Teile alle kopiert werden. Alles, was
/// Platz auf dem Heap besitzt, wird verschoben. Die Namen kommen so herein, wie
/// sie geschrieben werden, also `"i32"`, `"String"` oder `"(i32, bool)"`.
///
/// Exercise 3: say for a type name whether an assignment copies it.
///
/// Copied are the whole numbers, the floating point numbers, `bool`, `char`,
/// shared references and tuples all of whose parts are copied. Everything that
/// owns space on the heap is moved. The names come in the way they are written,
/// so `"i32"`, `"String"` or `"(i32, bool)"`.
pub fn copies_on_assignment(typ: &str) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}
