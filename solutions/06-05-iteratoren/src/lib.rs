//! 06-05 Iteratoren / Iterators, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/06-05-iteratoren/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/06-05-iteratoren/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

/// Verdoppelt jede Zahl.
///
/// Doubles every number.
///
/// ```
/// use unit_06_05_iteratoren::verdoppelt;
///
/// assert_eq!(verdoppelt(&[1, 2, 3]), vec![2, 4, 6]);
/// assert_eq!(verdoppelt(&[]), Vec::<i32>::new());
/// ```
///
/// ```
/// let werte = [1, 2, 3, 4];
///
/// let kette = werte.iter().map(|wert| wert * 2);
/// let ergebnis: Vec<i32> = kette.take(2).collect();
///
/// assert_eq!(ergebnis, vec![2, 4]);
/// ```
pub fn verdoppelt(werte: &[i32]) -> Vec<i32> {
    werte.iter().map(|wert| wert * 2).collect()
}

/// Die Quadrate der geraden Zahlen, in der Reihenfolge der Liste.
///
/// The squares of the even numbers, in the order of the list.
pub fn quadrate_der_geraden(werte: &[i32]) -> Vec<i32> {
    werte
        .iter()
        .filter(|wert| *wert % 2 == 0)
        .map(|wert| wert * wert)
        .collect()
}

/// Ein eigener Iterator, der von 1 bis 5 zählt.
///
/// An iterator of your own counting from 1 to 5.
#[derive(Debug)]
pub struct Zaehler {
    stand: u32,
}

impl Zaehler {
    /// Ein frischer Zähler, der noch nichts herausgegeben hat.
    ///
    /// A fresh counter that has handed out nothing yet.
    pub fn neu() -> Zaehler {
        Zaehler { stand: 0 }
    }
}

/// `Zaehler` als Iterator, mit `next` als der einen verlangten Methode.
///
/// `Zaehler` as an iterator, with `next` as the one required method.
impl Iterator for Zaehler {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.stand < 5 {
            self.stand += 1;
            Some(self.stand)
        } else {
            None
        }
    }
}

/// Die erste Zahl, die größer als `schwelle` ist.
///
/// The first number greater than `schwelle`.
pub fn erste_ueber(werte: &[i32], schwelle: i32) -> Option<i32> {
    werte.iter().copied().find(|wert| *wert > schwelle)
}
