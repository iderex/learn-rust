//! 09-02 Assoziierte Typen / Associated types
//!
//! Deutsch: Ein Trait kann einen Typ mitbringen, den erst die Implementierung
//! festlegt. Er steht dann nicht mehr am Aufruf, und ein Typ kann den Trait
//! genau einmal erfüllen.
//!
//! English: a trait can bring along a type that only the implementation fixes.
//! It then no longer stands at the call, and a type can meet the trait exactly
//! once.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Etwas, das Werte einen nach dem anderen herausgibt.
///
/// Something that hands out values one after another.
pub trait Quelle {
    /// Der Typ, den diese Quelle liefert. Er gehört zur Implementierung und
    /// nicht zum Aufruf.
    ///
    /// The type this source delivers. It belongs to the implementation and not
    /// to the call.
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
    /// Diese Quelle steht fertig da und ist das Muster für die Aufgaben. Ihr
    /// assoziierter Typ ist `u32`, und am Aufruf steht davon nichts.
    ///
    /// A counter that starts at 1 and stops after `bis`.
    ///
    /// This source stands there finished and is the model for the exercises.
    /// Its associated type is `u32`, and nothing of that stands at the call.
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

/// Aufgabe 1: Gib die Zeichen des Wortes heraus, eines nach dem anderen.
///
/// Der assoziierte Typ ist hier `char` und steht schon da. Zu schreiben ist
/// `naechstes`. `stelle` ist eine Stelle in Bytes und nicht in Zeichen, denn
/// ein `&str` wird über Bytes angesprochen. Wer um `len_utf8` herumrechnet,
/// trifft bei `ä` neben die Zeichengrenze und bekommt einen Abbruch. Ist das
/// Wort zu Ende, kommt `None`.
///
/// Exercise 1: hand out the characters of the word, one after another.
///
/// The associated type is `char` here and already stands there. What is to be
/// written is `naechstes`. `stelle` is a position in bytes and not in
/// characters, because a `&str` is addressed over bytes. Whoever counts around
/// `len_utf8` lands beside a character boundary at `ä` and gets an abort. When
/// the word is over, `None` comes.
impl Quelle for Buchstaben {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        todo!("Aufgabe 1 / Exercise 1")
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

/// Aufgabe 2: Gib die Wörter des Satzes heraus, eines nach dem anderen.
///
/// Derselbe Trait, ein anderer assoziierter Typ: hier ist es `String`. Getrennt
/// wird am Leerzeichen, und mehrere Leerzeichen hintereinander sind kein leeres
/// Wort. Ein Satz ohne Wörter gibt sofort `None`.
///
/// Exercise 2: hand out the words of the sentence, one after another.
///
/// The same trait, a different associated type: here it is `String`. The split
/// is at the space, and several spaces in a row are not an empty word. A
/// sentence without words gives `None` straight away.
impl Quelle for Woerter {
    type Item = String;

    fn naechstes(&mut self) -> Option<String> {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Aufgabe 3: Sammle alles ein, was eine Quelle noch hergibt.
///
/// Diese Funktion kennt den gelieferten Typ nicht und nennt ihn `Q::Item`. Das
/// ist die Stelle, an der der assoziierte Typ seinen Namen bekommt, ohne dass
/// ein weiterer Typparameter dazukommt. Eine Quelle, die nichts mehr hergibt,
/// gibt eine leere Liste.
///
/// Exercise 3: collect everything a source still hands out.
///
/// This function does not know the delivered type and names it `Q::Item`. That
/// is the place where the associated type gets its name without a second type
/// parameter coming along. A source that hands out nothing more gives an empty
/// list.
pub fn einsammeln<Q: Quelle>(quelle: &mut Q) -> Vec<Q::Item> {
    todo!("Aufgabe 3 / Exercise 3")
}
