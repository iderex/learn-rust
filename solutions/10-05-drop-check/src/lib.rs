//! 10-05 Drop check / Drop check, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/10-05-drop-check/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/10-05-drop-check/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

use std::cell::RefCell;

/// Ein Buch, in das beim Aufräumen Namen eingetragen werden.
///
/// A book that names get written into while things are cleaned up.
pub type Buch = RefCell<Vec<String>>;

/// Ein leeres Buch.
///
/// An empty book.
pub fn neues_buch() -> Buch {
    RefCell::new(Vec::new())
}

/// Die Einträge des Buchs in der Reihenfolge, in der sie hineingekommen sind.
///
/// The entries of the book in the order they came in.
pub fn eintraege(buch: &Buch) -> Vec<String> {
    buch.borrow().clone()
}

/// Eine Spur, die sich beim Aufräumen in ein geliehenes Buch einträgt.
///
/// A trace that writes itself into a borrowed book while being cleaned up.
///
/// ```
/// use unit_10_05_drop_check::{Spur, eintraege, neues_buch};
///
/// let buch = neues_buch();
///
/// {
///     let _eine = Spur::neu("eine", &buch);
///
///     assert!(eintraege(&buch).is_empty());
/// }
///
/// assert_eq!(eintraege(&buch), vec![String::from("eine")]);
/// ```
pub struct Spur<'a> {
    name: String,
    buch: &'a Buch,
}

impl<'a> Spur<'a> {
    /// Eine neue Spur mit diesem Namen, die sich in dieses Buch einträgt.
    ///
    /// A new trace with this name that writes itself into this book.
    pub fn neu(name: &str, buch: &'a Buch) -> Spur<'a> {
        Spur {
            name: name.to_string(),
            buch,
        }
    }

    /// Der Name dieser Spur.
    ///
    /// The name of this trace.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Spur<'_> {
    fn drop(&mut self) {
        self.buch.borrow_mut().push(self.name.clone());
    }
}

/// Zeigt, in welcher Reihenfolge drei Spuren fallen.
///
/// Shows the order three traces fall in.
pub fn reihenfolge() -> Vec<String> {
    // Deutsch: Das Buch zuerst, sonst uebersetzt der Rumpf nicht.
    // English: the book first, otherwise this body does not compile.
    let buch = neues_buch();

    {
        let _eins = Spur::neu("eins", &buch);
        let _zwei = Spur::neu("zwei", &buch);
        let _drei = Spur::neu("drei", &buch);
    }

    eintraege(&buch)
}

/// Lässt die mittlere Spur früher fallen, durch einen eigenen Block.
///
/// Lets the middle trace fall earlier, through a block of its own.
pub fn mit_eigenem_block() -> Vec<String> {
    let buch = neues_buch();

    {
        let _eins = Spur::neu("eins", &buch);
        {
            let _zwei = Spur::neu("zwei", &buch);
        }
        let _drei = Spur::neu("drei", &buch);
    }

    eintraege(&buch)
}

/// Lässt die erste Spur von Hand früher fallen.
///
/// Lets the first trace fall earlier by hand.
pub fn frueh_fallen_lassen() -> Vec<String> {
    let buch = neues_buch();

    {
        let eins = Spur::neu("eins", &buch);
        let _zwei = Spur::neu("zwei", &buch);
        let _drei = Spur::neu("drei", &buch);

        drop(eins);
    }

    eintraege(&buch)
}
