//! 09-02 Assoziierte Typen / Associated types, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/09-02-assoziierte-typen/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-02-assoziierte-typen/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

/// Etwas, das Werte einen nach dem anderen herausgibt.
///
/// Something that hands out values one after another.
pub trait Quelle {
    /// Der Typ, den diese Quelle liefert.
    ///
    /// The type this source delivers.
    type Item;

    /// Der nächste Wert, oder `None`, wenn keiner mehr kommt.
    ///
    /// The next value, or `None` when no more come.
    fn naechstes(&mut self) -> Option<Self::Item>;
}

/// Zählt von 1 bis zu einer Grenze.
///
/// Counts from 1 up to a limit.
pub struct Zaehler {
    pub stand: u32,
    pub bis: u32,
}

impl Zaehler {
    /// Ein Zähler, der bei 1 anfängt und nach `bis` aufhört.
    ///
    /// A counter that starts at 1 and stops after `bis`.
    ///
    /// ```
    /// use unit_09_02_assoziierte_typen::{Quelle, Zaehler};
    ///
    /// let mut zaehler = Zaehler::neu(3);
    ///
    /// assert_eq!(zaehler.naechstes(), Some(1));
    /// assert_eq!(zaehler.naechstes(), Some(2));
    /// assert_eq!(zaehler.naechstes(), Some(3));
    /// assert_eq!(zaehler.naechstes(), None);
    /// ```
    pub fn neu(bis: u32) -> Self {
        Zaehler { stand: 0, bis }
    }
}

impl Quelle for Zaehler {
    type Item = u32;

    fn naechstes(&mut self) -> Option<u32> {
        if self.stand >= self.bis {
            return None;
        }
        self.stand += 1;
        Some(self.stand)
    }
}

/// Gibt die Zeichen eines Wortes heraus.
///
/// Hands out the characters of a word.
pub struct Buchstaben {
    pub wort: String,
    pub stelle: usize,
}

impl Buchstaben {
    /// Fängt am Anfang des Wortes an.
    ///
    /// Starts at the beginning of the word.
    pub fn neu(wort: &str) -> Self {
        Buchstaben {
            wort: wort.to_string(),
            stelle: 0,
        }
    }
}

impl Quelle for Buchstaben {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        let zeichen = self.wort[self.stelle..].chars().next()?;
        self.stelle += zeichen.len_utf8();
        Some(zeichen)
    }
}

/// Gibt die Wörter eines Satzes heraus.
///
/// Hands out the words of a sentence.
pub struct Woerter {
    pub satz: String,
    pub stelle: usize,
}

impl Woerter {
    /// Fängt am Anfang des Satzes an.
    ///
    /// Starts at the beginning of the sentence.
    pub fn neu(satz: &str) -> Self {
        Woerter {
            satz: satz.to_string(),
            stelle: 0,
        }
    }
}

impl Quelle for Woerter {
    type Item = String;

    fn naechstes(&mut self) -> Option<String> {
        let rest = &self.satz[self.stelle..];
        let vorlauf = rest.find(|zeichen: char| zeichen != ' ')?;
        let anfang = self.stelle + vorlauf;
        let laenge = match self.satz[anfang..].find(' ') {
            Some(stelle) => stelle,
            None => self.satz.len() - anfang,
        };
        self.stelle = anfang + laenge;
        Some(self.satz[anfang..anfang + laenge].to_string())
    }
}

/// Sammelt alles ein, was eine Quelle noch hergibt.
///
/// Collects everything a source still hands out.
pub fn einsammeln<Q: Quelle>(quelle: &mut Q) -> Vec<Q::Item> {
    let mut gesammelt = Vec::new();
    while let Some(wert) = quelle.naechstes() {
        gesammelt.push(wert);
    }
    gesammelt
}
