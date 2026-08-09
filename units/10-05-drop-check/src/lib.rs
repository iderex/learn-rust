//! 10-05 Drop check / Drop check
//!
//! Deutsch: Ein Typ ohne `Drop` darf am Ende auf etwas zeigen, das schon weg
//! ist, denn niemand sieht mehr hin. Ein Typ mit `Drop` darf das nicht, denn
//! sein `drop` läuft und kann hinsehen. Der drop check ist die Rechnung, die
//! das prüft, und ihre Folge ist eine kürzere erlaubte Lebensdauer.
//!
//! English: a type without `Drop` may point at something already gone at the
//! end, because nobody looks any more. A type with `Drop` may not, because its
//! `drop` runs and can look. The drop check is the reasoning that checks this,
//! and its consequence is a shorter allowed lifetime.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::cell::RefCell;

/// Ein Buch, in das beim Aufräumen Namen eingetragen werden.
///
/// A book that names get written into while things are cleaned up.
pub type Buch = RefCell<Vec<String>>;

/// Ein leeres Buch.
///
/// Diese Funktion steht fertig da. Wer eine der Aufgaben löst, legt das Buch
/// als Erstes an, und zwar vor jeder `Spur`. Andersherum übersetzt es nicht,
/// und die Meldung dazu steht in der README unter "Häufige Fehler".
///
/// An empty book.
///
/// This function stands there finished. Whoever solves one of the exercises
/// creates the book first, before any `Spur`. The other way round it does not
/// compile, and the message for that is in the README under "Common mistakes".
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
/// Diese Struktur ist der Grund für die ganze Einheit. Sie hält eine Referenz,
/// und sie hat ein `Drop`. Beides zusammen macht sie zu einem der Typen, die
/// der drop check streng behandelt: Ihr `drop` läuft, es liest das geliehene
/// Buch, und deshalb muss das Buch noch da sein, wenn sie fällt.
///
/// A trace that writes itself into a borrowed book while being cleaned up.
///
/// This struct is the reason for the whole unit. It holds a reference, and it
/// has a `Drop`. The two together make it one of the types the drop check
/// treats strictly: its `drop` runs, it reads the borrowed book, and the book
/// therefore has to still be there when it falls.
///
/// ```
/// use unit_10_05_drop_check::{Spur, eintraege, neues_buch};
///
/// // Deutsch: Das Buch zuerst. Es muss die Spur ueberleben.
/// // English: the book first. It has to outlive the trace.
/// let buch = neues_buch();
///
/// {
///     let _eine = Spur::neu("eine", &buch);
///
///     // Deutsch: Solange die Spur lebt, steht nichts im Buch.
///     // English: as long as the trace lives, nothing stands in the book.
///     assert!(eintraege(&buch).is_empty());
/// }
///
/// // Deutsch: Der Eintrag entsteht erst beim Aufraeumen am Ende des Blocks.
/// // English: the entry appears only while cleaning up at the end of the block.
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

/// Aufgabe 1: Zeige, in welcher Reihenfolge drei Spuren fallen.
///
/// Lege ein Buch an und darin drei Spuren mit den Namen `"eins"`, `"zwei"` und
/// `"drei"`, jede in einer eigenen `let`-Zeile und in dieser Reihenfolge. Gib
/// die Einträge des Buchs zurück, nachdem alle drei gefallen sind.
///
/// Heraus kommt `["drei", "zwei", "eins"]`, denn Werte in einem
/// Gültigkeitsbereich fallen in der umgekehrten Reihenfolge ihrer Vereinbarung.
/// Genau diesen Satz druckt der Übersetzer als Notiz unter die Meldung, die es
/// gibt, wenn das Buch an der falschen Stelle steht.
///
/// Drei Spuren in einem `Vec` sind etwas anderes und geben eine andere
/// Reihenfolge, denn ein `Vec` räumt seine Werte von vorne nach hinten auf.
///
/// Exercise 1: show the order three traces fall in.
///
/// Create a book and inside it three traces with the names `"eins"`, `"zwei"`
/// and `"drei"`, each in a `let` line of its own and in that order. Give back
/// the entries of the book once all three have fallen.
///
/// What comes out is `["drei", "zwei", "eins"]`, because values in a scope fall
/// in the opposite order of their declaration. That is exactly the sentence the
/// compiler prints as a note under the message you get when the book stands in
/// the wrong place.
///
/// Three traces in a `Vec` are a different thing and give a different order,
/// because a `Vec` cleans its values up from front to back.
pub fn reihenfolge() -> Vec<String> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Lass die mittlere Spur früher fallen, durch einen eigenen Block.
///
/// Wieder drei Spuren mit den Namen `"eins"`, `"zwei"` und `"drei"`, wieder in
/// dieser Reihenfolge vereinbart. `"zwei"` steht diesmal in einem eigenen Block
/// und fällt an dessen Ende, also vor den beiden anderen.
///
/// Heraus kommt `["zwei", "drei", "eins"]`. Ein Block ist damit das Mittel,
/// eine Lebensdauer zu verkürzen, ohne die Reihenfolge der Vereinbarungen
/// umzustellen.
///
/// Exercise 2: let the middle trace fall earlier, through a block of its own.
///
/// Three traces again with the names `"eins"`, `"zwei"` and `"drei"`, declared
/// in that order again. This time `"zwei"` stands in a block of its own and
/// falls at the end of it, meaning before the other two.
///
/// What comes out is `["zwei", "drei", "eins"]`. A block is therefore the means
/// of shortening a lifetime without rearranging the order of the declarations.
pub fn mit_eigenem_block() -> Vec<String> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Lass die erste Spur von Hand früher fallen.
///
/// Wieder dieselben drei Namen in derselben Reihenfolge. Diesmal wird `"eins"`
/// mit `drop` von Hand fallen gelassen, bevor der Gültigkeitsbereich endet.
///
/// Heraus kommt `["eins", "drei", "zwei"]`. `drop` ist dabei keine besondere
/// Anweisung des Übersetzers, sondern eine gewöhnliche Funktion, die ihren
/// Wert entgegennimmt und nichts damit tut; das Aufräumen entsteht daraus, dass
/// der Wert am Ende dieser Funktion liegt.
///
/// Exercise 3: let the first trace fall earlier by hand.
///
/// The same three names in the same order again. This time `"eins"` is dropped
/// by hand with `drop` before the scope ends.
///
/// What comes out is `["eins", "drei", "zwei"]`. `drop` is not a special
/// instruction of the compiler here but an ordinary function that takes its
/// value and does nothing with it; the cleaning up comes about because the value
/// lies at the end of that function.
pub fn frueh_fallen_lassen() -> Vec<String> {
    todo!("Aufgabe 3 / Exercise 3")
}
