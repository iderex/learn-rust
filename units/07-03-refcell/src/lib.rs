//! 07-03 RefCell / RefCell
//!
//! Deutsch: `RefCell` verschiebt die Ausleihregel vom Übersetzen in die
//! Laufzeit. Die Regel bleibt dieselbe, nur wird sie jetzt gezählt statt
//! gelesen, und wer sie bricht, bekommt einen Abbruch statt einer Meldung.
//!
//! English: `RefCell` moves the borrowing rule from compile time into run time.
//! The rule stays the same, it is only counted now rather than read, and
//! whoever breaks it gets an abort instead of a message.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::cell::RefCell;

/// Ein Protokoll, das sich hinter einem `&` füllen lässt.
///
/// Die Zeilen liegen in einer `RefCell`. Deshalb nehmen alle Methoden hier
/// `&self` und keine `&mut self`, obwohl sie den Inhalt ändern.
///
/// A log that can be filled behind a `&`.
///
/// The lines lie in a `RefCell`. That is why every method here takes `&self`
/// and not `&mut self`, although they change the content.
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
    /// Diese Methode steht fertig da. Sie ist der ganze Punkt von `RefCell`:
    /// `&self` genügt, und die Veränderung geht durch die Zelle. `borrow_mut`
    /// gibt eine Ausleihe, die bis zum Ende des Ausdrucks lebt und danach
    /// wieder frei ist.
    ///
    /// Writes a line into the log.
    ///
    /// This method stands there finished. It is the whole point of `RefCell`:
    /// `&self` is enough, and the change goes through the cell. `borrow_mut`
    /// gives a borrow that lives to the end of the expression and is free again
    /// afterwards.
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
    /// Steht fertig da, damit ein Doku-Test und ein Test hineinsehen können,
    /// ohne die Aufgaben zu lösen. Solange der Rückgabewert lebt, ist die Zelle
    /// geliehen.
    ///
    /// Borrows the lines for reading.
    ///
    /// Stands there finished, so that a doc test and a test can look inside
    /// without solving the exercises. As long as the returned value lives, the
    /// cell is borrowed.
    pub fn zeilen_geliehen(&self) -> std::cell::Ref<'_, Vec<String>> {
        self.zeilen.borrow()
    }

    /// Leiht dieselbe Zelle zweimal veränderbar aus und bricht deshalb ab.
    ///
    /// Steht mit Absicht fertig und mit Absicht falsch da. Der Übersetzer nimmt
    /// diese Methode an, denn `borrow_mut` nimmt ein `&self`, und zwei `&self`
    /// sind erlaubt. Erst die Zelle selbst zählt mit und bricht beim zweiten
    /// Mal ab. Ein Test hält diesen Abbruch fest.
    ///
    /// Borrows the same cell mutably twice and therefore aborts.
    ///
    /// Stands there finished on purpose and wrong on purpose. The compiler
    /// accepts this method, because `borrow_mut` takes a `&self`, and two
    /// `&self` are allowed. Only the cell itself counts along and aborts the
    /// second time. A test holds this abort down.
    pub fn zwei_veraenderbare_ausleihen(&self) -> usize {
        let erste = self.zeilen.borrow_mut();
        let zweite = self.zeilen.borrow_mut();

        erste.len() + zweite.len()
    }

    /// Aufgabe 1: Gib zurück, wie viele Zeilen im Protokoll stehen.
    ///
    /// Gelesen wird mit `borrow`, das eine Ausleihe zum Lesen gibt. Davon darf
    /// es mehrere gleichzeitig geben, genau wie bei `&`.
    ///
    /// Exercise 1: return how many lines stand in the log.
    ///
    /// Reading goes through `borrow`, which gives a borrow for reading. There
    /// may be several of those at once, exactly as with `&`.
    pub fn anzahl(&self) -> usize {
        todo!("Aufgabe 1 / Exercise 1")
    }

    /// Aufgabe 2: Gib die zuletzt geschriebene Zeile zurück.
    ///
    /// Bei einem leeren Protokoll kommt `None` zurück. Sonst kommt eine Kopie
    /// der letzten Zeile, denn eine Ausleihe darf die Methode nicht überleben:
    /// die Zelle wäre danach dauerhaft geliehen und der nächste `borrow_mut`
    /// bräche ab.
    ///
    /// Exercise 2: return the line written last.
    ///
    /// For an empty log `None` comes back. Otherwise a copy of the last line
    /// comes back, because a borrow may not outlive the method: the cell would
    /// be borrowed for good afterwards and the next `borrow_mut` would abort.
    pub fn letzte(&self) -> Option<String> {
        todo!("Aufgabe 2 / Exercise 2")
    }

    /// Aufgabe 3: Schreib eine Zeile und gib zurück, wie viele es danach sind.
    ///
    /// Das ist die Stelle, an der der Abbruch wartet. Wer die Ausleihe aus
    /// `borrow_mut` noch in der Hand hält und dann `anzahl` aufruft, leiht
    /// dieselbe Zelle ein zweites Mal, und die Zelle bricht ab. Die Ausleihe
    /// muss vorher zu Ende sein.
    ///
    /// Exercise 3: write a line and return how many there are afterwards.
    ///
    /// This is the place where the abort waits. Whoever still holds the borrow
    /// from `borrow_mut` and then calls `anzahl` borrows the same cell a second
    /// time, and the cell aborts. The borrow has to be over before that.
    pub fn notieren_und_zaehlen(&self, zeile: &str) -> usize {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
