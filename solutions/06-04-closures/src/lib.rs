//! 06-04 Closures / Closures, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/06-04-closures/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/06-04-closures/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Wendet `f` einmal auf `wert` an.
///
/// Applies `f` once to `wert`.
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

/// Wendet `f` zweimal an.
///
/// Applies `f` twice.
pub fn apply_twice<F: Fn(i32) -> i32>(f: F, wert: i32) -> i32 {
    f(f(wert))
}

/// Meldet jede gerade Zahl aus `werte` an `melde`.
///
/// Reports every even number out of `werte` to `melde`.
pub fn for_each_even<F: FnMut(i32)>(werte: &[i32], mut melde: F) {
    for wert in werte {
        if wert % 2 == 0 {
            melde(*wert);
        }
    }
}

/// Gibt eine Closure zurück, die `summand` addiert.
///
/// Returns a closure that adds `summand`.
pub fn make_adder(summand: i32) -> impl Fn(i32) -> i32 {
    move |zahl| zahl + summand
}
