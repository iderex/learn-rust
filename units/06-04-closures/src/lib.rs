//! 06-04 Closures / Closures
//!
//! Deutsch: Eine Closure ist eine Funktion, die etwas aus ihrer Umgebung
//! mitnimmt. Wie sie es mitnimmt, entscheidet, wie oft sie sich aufrufen lässt,
//! und genau das sagen die drei Traits `Fn`, `FnMut` und `FnOnce`.
//!
//! English: a closure is a function that takes something along from its
//! surroundings. How it takes it along decides how often it can be called, and
//! that is exactly what the three traits `Fn`, `FnMut` and `FnOnce` say.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Wendet `f` einmal auf `wert` an.
///
/// Diese Funktion steht fertig da und zeigt die Form, in der eine Closure
/// entgegengenommen wird. `F` ist ein eigener Typ je Closure, denn zwei
/// Closures mit demselben Rumpf sind trotzdem zwei Typen. Deshalb steht hier
/// ein generischer Parameter mit einer Schranke und kein fester Typ.
///
/// `Fn(i32) -> i32` ist die schwächste Anforderung, die hier reicht: Es genügt,
/// die Closure einmal aufzurufen, ohne sie zu verändern und ohne sie
/// aufzubrauchen.
///
/// Applies `f` once to `wert`.
///
/// This function stands there finished and shows the shape in which a closure is
/// taken. `F` is a type of its own per closure, because two closures with the
/// same body are still two types. That is why there is a generic parameter with
/// a bound here and not a fixed type.
///
/// `Fn(i32) -> i32` is the weakest requirement that is enough here: calling the
/// closure once is sufficient, without changing it and without using it up.
///
/// ```
/// use unit_06_04_closures::apply;
///
/// assert_eq!(apply(|zahl| zahl + 1, 41), 42);
///
/// let faktor = 3;
/// assert_eq!(apply(|zahl| zahl * faktor, 5), 15);
/// ```
pub fn apply<F: Fn(i32) -> i32>(f: F, wert: i32) -> i32 {
    f(wert)
}

/// Aufgabe 1: Wende `f` zweimal an.
///
/// Heraus kommt `f(f(wert))`. Die Schranke steht schon da und ist der Grund,
/// warum das geht: `Fn` heißt, dass die Closure sich beliebig oft aufrufen
/// lässt. Mit `FnOnce` wäre der zweite Aufruf zurückgewiesen, und die
/// Fehlermeldung dazu steht in der README.
///
/// Exercise 1: apply `f` twice.
///
/// What comes out is `f(f(wert))`. The bound already stands there and is the
/// reason this works: `Fn` means the closure can be called any number of times.
/// With `FnOnce` the second call would be refused, and the message for that is
/// in the README.
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, wert: i32) -> i32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Melde jede gerade Zahl aus `werte` an `melde`.
///
/// Für jede gerade Zahl in `werte`, in der Reihenfolge der Liste, wird `melde`
/// mit dieser Zahl aufgerufen. Ungerade Zahlen werden übergangen. Zurück kommt
/// nichts, denn was mit den Zahlen geschieht, entscheidet der Aufrufer.
///
/// Die Schranke ist `FnMut`, denn eine Closure, die etwas in eine Liste legt,
/// verändert die Liste und damit sich selbst. Der Parameter ist deshalb `mut`.
///
/// Exercise 2: report every even number out of `werte` to `melde`.
///
/// For every even number in `werte`, in the order of the list, `melde` is called
/// with that number. Odd numbers are skipped. Nothing comes back, because what
/// happens to the numbers is the caller's decision.
///
/// The bound is `FnMut`, because a closure that puts something into a list
/// changes the list and therefore itself. The parameter is `mut` for that
/// reason.
// Deutsch: Das `mut` am Parameter gehoert zur Aufgabe und wird erst gebraucht,
// wenn der Rumpf steht. Solange er `todo!()` ist, meldet clippy es als
// ueberfluessig, und deshalb steht die Ausnahme hier statt eines `mut`, das
// beim Loesen wieder dazukommen muesste.
// English: the `mut` on the parameter belongs to the exercise and is only needed
// once the body stands. While it is `todo!()`, clippy reports it as superfluous,
// which is why the exception stands here instead of a `mut` that would have to
// come back when solving.
#[allow(unused_mut)]
pub fn for_each_even<F: FnMut(i32)>(werte: &[i32], mut melde: F) {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib eine Closure zurück, die `summand` addiert.
///
/// Der Rückgabetyp ist `impl Fn(i32) -> i32`, also "irgendein Typ, der dieses
/// Trait erfüllt". Ein fester Name geht nicht, denn der Typ einer Closure hat
/// keinen, den man hinschreiben könnte.
///
/// `summand` gehört der Funktion und ist weg, sobald sie zurückkehrt. Die
/// Closure muss ihn deshalb übernehmen und nicht ausleihen, und das schreibt
/// man mit `move` vor die Closure.
///
/// Exercise 3: return a closure that adds `summand`.
///
/// The return type is `impl Fn(i32) -> i32`, meaning "some type that fulfils
/// this trait". A fixed name does not work, because the type of a closure has
/// none that could be written down.
///
/// `summand` belongs to the function and is gone as soon as it returns. The
/// closure therefore has to take it over rather than borrow it, and that is
/// written with `move` in front of the closure.
// Deutsch: Hier steht das `todo!()` im Rumpf der Closure und nicht im Rumpf der
// Funktion. Ein `todo!()` an der Stelle der Funktion hat den Typ `!`, und `!`
// erfuellt kein `Fn`, also uebersetzt die Einheit dann gar nicht erst. Der
// Aufruf bricht trotzdem ab, und der Test bleibt rot.
// English: here the `todo!()` stands in the body of the closure and not in the
// body of the function. A `todo!()` in the function's place has type `!`, and
// `!` fulfils no `Fn`, so the unit would not even compile. The call still
// aborts, and the test stays red.
pub fn make_adder(summand: i32) -> impl Fn(i32) -> i32 {
    move |zahl| todo!("Aufgabe 3 / Exercise 3")
}
