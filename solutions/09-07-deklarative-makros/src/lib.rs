//! 09-07 Deklarative Makros mit macro_rules! / Declarative macros with
//! macro_rules!, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/09-07-deklarative-makros/README.md`. Hier stehen nur die Rümpfe, die
//! die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-07-deklarative-makros/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

/// Das Quadrat eines Ausdrucks.
///
/// The square of an expression.
///
/// ```
/// use unit_09_07_deklarative_makros::quadrat;
///
/// assert_eq!(quadrat!(4), 16);
/// assert_eq!(quadrat!(1 + 2), 9);
/// ```
#[macro_export]
macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}

/// Gibt von beliebig vielen Werten den größten heraus.
///
/// Hands the largest of any number of values out.
#[macro_export]
macro_rules! groesster {
    ($einziger:expr) => {
        $einziger
    };
    ($erster:expr, $($weitere:expr),+ $(,)?) => {{
        let rest = $crate::groesster!($($weitere),+);

        if $erster > rest { $erster } else { rest }
    }};
}

/// Baut aus beliebig vielen Werten ein `Vec`.
///
/// Builds a `Vec` out of any number of values.
#[macro_export]
macro_rules! vec_von {
    () => {
        Vec::new()
    };
    ($($wert:expr),+ $(,)?) => {{
        let mut liste = Vec::new();

        $(
            liste.push($wert);
        )+

        liste
    }};
}

/// Erzeugt aus einem Namen und einer Grenze eine Funktion.
///
/// Makes a function out of a name and a limit.
#[macro_export]
macro_rules! mach_pruefer {
    ($name:ident, $grenze:expr) => {
        pub fn $name(wert: i32) -> bool {
            wert > $grenze
        }
    };
}

mach_pruefer!(ueber_zehn, 10);
mach_pruefer!(ueber_hundert, 100);
