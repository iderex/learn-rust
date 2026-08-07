//! 07-02 Rc / Rc, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-02-rc/README.md`. Hier stehen nur
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-02-rc/README.md`. What is here is
//! only the bodies that turn the unit's tests green.

use std::rc::Rc;

/// Ein Knoten, der auf seinen Elternknoten zeigt.
///
/// A node pointing at its parent node.
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub eltern: Option<Rc<Node>>,
}

/// Sagt, wie viele Stellen diesen Wert gerade besitzen.
///
/// Says how many places own this value right now.
///
/// ```
/// use std::rc::Rc;
/// use unit_07_02_rc::owners;
///
/// let wert = Rc::new(String::from("Ada"));
/// assert_eq!(owners(&wert), 1);
///
/// let zweiter = Rc::clone(&wert);
/// assert_eq!(owners(&wert), 2);
///
/// drop(zweiter);
/// assert_eq!(owners(&wert), 1);
/// ```
pub fn owners<T>(wert: &Rc<T>) -> usize {
    Rc::strong_count(wert)
}

/// Baut eine Kette aus den Namen und gibt den letzten Knoten zurück.
///
/// Builds a chain out of the names and returns the last node.
pub fn chain(namen: &[&str]) -> Option<Rc<Node>> {
    let mut letzter: Option<Rc<Node>> = None;

    for name in namen {
        letzter = Some(Rc::new(Node {
            name: (*name).to_string(),
            eltern: letzter,
        }));
    }

    letzter
}

/// Sagt, wie viele Knoten von hier bis zur Wurzel stehen.
///
/// Says how many nodes stand from here up to the root.
pub fn depth(knoten: &Rc<Node>) -> usize {
    let mut gezaehlt = 1;
    let mut aktuell = knoten;

    while let Some(eltern) = &aktuell.eltern {
        gezaehlt += 1;
        aktuell = eltern;
    }

    gezaehlt
}

/// Gibt den Namen der Wurzel zurück.
///
/// Returns the name of the root.
pub fn root_name(knoten: &Rc<Node>) -> &str {
    let mut aktuell = knoten;

    while let Some(eltern) = &aktuell.eltern {
        aktuell = eltern;
    }

    &aktuell.name
}
