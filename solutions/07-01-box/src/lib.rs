//! 07-01 Box / Box, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-01-box/README.md`. Hier stehen nur
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-01-box/README.md`. What is here
//! is only the bodies that turn the unit's tests green.

/// Eine Liste aus Gliedern, jedes mit einer Zahl und dem Rest dahinter.
///
/// A list of links, each with a number and the rest behind it.
#[derive(Debug, PartialEq)]
pub enum Liste {
    Glied(i64, Box<Liste>),
    Ende,
}

/// Zählt die Glieder einer Liste.
///
/// Counts the links of a list.
///
/// ```
/// use unit_07_01_box::{Liste, length};
///
/// let liste = Liste::Glied(1, Box::new(Liste::Glied(2, Box::new(Liste::Ende))));
///
/// assert_eq!(length(&liste), 2);
/// assert_eq!(length(&Liste::Ende), 0);
/// ```
pub fn length(liste: &Liste) -> usize {
    match liste {
        Liste::Glied(_, rest) => 1 + length(rest),
        Liste::Ende => 0,
    }
}

/// Baut aus einem Slice eine Liste.
///
/// Builds a list out of a slice.
pub fn from_slice(zahlen: &[i64]) -> Liste {
    match zahlen.split_first() {
        Some((erste, rest)) => Liste::Glied(*erste, Box::new(from_slice(rest))),
        None => Liste::Ende,
    }
}

/// Addiert alle Zahlen einer Liste.
///
/// Adds up every number of a list.
pub fn sum(liste: &Liste) -> i64 {
    match liste {
        Liste::Glied(zahl, rest) => zahl + sum(rest),
        Liste::Ende => 0,
    }
}

/// Sagt, ob eine Zahl in der Liste vorkommt.
///
/// Says whether a number turns up in the list.
pub fn contains(liste: &Liste, gesucht: i64) -> bool {
    match liste {
        Liste::Glied(zahl, rest) => *zahl == gesucht || contains(rest, gesucht),
        Liste::Ende => false,
    }
}
