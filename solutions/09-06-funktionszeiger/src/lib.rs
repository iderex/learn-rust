//! 09-06 Funktionszeiger / Function pointers, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/09-06-funktionszeiger/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/09-06-funktionszeiger/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Eine Zahl, die in einen eigenen Typ eingepackt ist.
///
/// A number packed into a type of its own.
#[derive(Debug, PartialEq, Eq)]
pub struct Marke(pub u32);

/// Verdoppelt eine Zahl.
///
/// Doubles a number.
pub fn verdoppeln(x: i32) -> i32 {
    x * 2
}

/// Dreht das Vorzeichen einer Zahl um.
///
/// Turns the sign of a number around.
pub fn negieren(x: i32) -> i32 {
    -x
}

/// Wendet `f` zweimal hintereinander auf `wert` an.
///
/// Applies `f` twice in a row to `wert`.
///
/// # Beispiele / Examples
///
/// ```
/// use unit_09_06_funktionszeiger::{negieren, verdoppeln, zweimal};
///
/// assert_eq!(zweimal(verdoppeln, 3), 12);
/// assert_eq!(zweimal(negieren, 3), 3);
///
/// let ohne_fang: fn(i32) -> i32 = |x| x + 1;
///
/// assert_eq!(zweimal(ohne_fang, 3), 5);
/// ```
pub fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

/// Wendet einen Funktionszeiger auf jeden Wert an.
///
/// Applies a function pointer to every value.
pub fn anwenden(werte: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    werte.iter().copied().map(f).collect()
}

/// Gibt zu einem Namen den passenden Funktionszeiger heraus.
///
/// Hands out the function pointer matching a name.
pub fn waehle(name: &str) -> Option<fn(i32) -> i32> {
    match name {
        "verdoppeln" => Some(verdoppeln),
        "negieren" => Some(negieren),
        _ => None,
    }
}

/// Steckt jede Zahl in eine `Marke`.
///
/// Puts every number into a `Marke`.
pub fn einpacken(werte: &[u32]) -> Vec<Marke> {
    werte.iter().copied().map(Marke).collect()
}
