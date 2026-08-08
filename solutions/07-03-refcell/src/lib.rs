//! 07-03 RefCell / RefCell, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-03-refcell/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-03-refcell/README.md`. What is
//! here is only the bodies that turn the unit's tests green.

use std::cell::RefCell;

/// Ein Protokoll, das sich hinter einem `&` füllen lässt.
///
/// A log that can be filled behind a `&`.
#[derive(Debug)]
pub struct Protokoll {
    zeilen: RefCell<Vec<String>>,
}

impl Protokoll {
    /// Ein leeres Protokoll.
    ///
    /// An empty log.
    pub fn neu() -> Self {
        Protokoll {
            zeilen: RefCell::new(Vec::new()),
        }
    }

    /// Schreibt eine Zeile ins Protokoll.
    ///
    /// Writes a line into the log.
    ///
    /// ```
    /// use unit_07_03_refcell::Protokoll;
    ///
    /// // Deutsch: `protokoll` ist nicht `mut`, und es wird trotzdem voller.
    /// // English: `protokoll` is not `mut`, and it fills up all the same.
    /// let protokoll = Protokoll::neu();
    ///
    /// protokoll.notieren("erste");
    /// protokoll.notieren("zweite");
    ///
    /// assert_eq!(protokoll.zeilen_geliehen().len(), 2);
    /// ```
    pub fn notieren(&self, zeile: &str) {
        self.zeilen.borrow_mut().push(zeile.to_string());
    }

    /// Leiht die Zeilen zum Lesen aus.
    ///
    /// Borrows the lines for reading.
    pub fn zeilen_geliehen(&self) -> std::cell::Ref<'_, Vec<String>> {
        self.zeilen.borrow()
    }

    /// Leiht dieselbe Zelle zweimal veränderbar aus und bricht deshalb ab.
    ///
    /// Borrows the same cell mutably twice and therefore aborts.
    pub fn zwei_veraenderbare_ausleihen(&self) -> usize {
        let erste = self.zeilen.borrow_mut();
        let zweite = self.zeilen.borrow_mut();

        erste.len() + zweite.len()
    }

    /// Gibt zurück, wie viele Zeilen im Protokoll stehen.
    ///
    /// Returns how many lines stand in the log.
    pub fn anzahl(&self) -> usize {
        self.zeilen.borrow().len()
    }

    /// Gibt die zuletzt geschriebene Zeile zurück.
    ///
    /// Returns the line written last.
    pub fn letzte(&self) -> Option<String> {
        self.zeilen.borrow().last().cloned()
    }

    /// Schreibt eine Zeile und gibt zurück, wie viele es danach sind.
    ///
    /// Writes a line and returns how many there are afterwards.
    pub fn notieren_und_zaehlen(&self, zeile: &str) -> usize {
        self.notieren(zeile);

        self.anzahl()
    }
}
