//! 10-03 Undefiniertes Verhalten / Undefined behaviour
//!
//! Deutsch: Undefiniertes Verhalten ist kein Absturz und keine falsche Zahl. Es
//! ist der Zustand, in dem die Sprache über das Programm nichts mehr aussagt.
//! Welche Schritte dorthin führen, steht in der Reference unter "Behavior
//! considered undefined", und was daneben nur unerwünscht ist, steht eine Liste
//! weiter unter "Behavior not considered unsafe".
//!
//! English: undefined behaviour is not a crash and not a wrong number. It is
//! the state in which the language says nothing about the program any more.
//! Which steps lead there stands in the Reference under "Behavior considered
//! undefined", and what next to it is merely undesirable stands one list on
//! under "Behavior not considered unsafe".

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein Schritt, den ein Programm tun kann.
///
/// Die acht stehen hier, weil sie sich auf drei Fälle verteilen: vier sind
/// undefiniert, zwei sind ausdrücklich nicht unsicher, und zwei stehen in
/// keiner der beiden Listen, weil an ihnen nichts zu sagen ist.
///
/// A step a program can take.
///
/// The eight stand here because they fall into three cases: four are undefined,
/// two are expressly not unsafe, and two stand in neither of the two lists,
/// because there is nothing to say about them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Schritt {
    /// Zwei rohe Zeiger vergleichen.
    ///
    /// Comparing two raw pointers.
    ZeigerVergleichen,
    /// Über den Rand einer Liste hinaus indizieren.
    ///
    /// Indexing past the end of a list.
    IndexUeberDenRand,
    /// Eine Addition, die überläuft.
    ///
    /// An addition that overflows.
    UeberlaufBeimAddieren,
    /// Zwei Fäden, die aufeinander warten.
    ///
    /// Two threads waiting for each other.
    Verklemmung,
    /// Durch einen Zeiger lesen, dessen Wert es nicht mehr gibt.
    ///
    /// Reading through a pointer whose value is gone.
    HaengendenZeigerLesen,
    /// Durch einen falsch ausgerichteten Zeiger lesen.
    ///
    /// Reading through a misaligned pointer.
    FalschAusgerichtetLesen,
    /// Zwei Fäden schreiben ohne Absprache auf dieselbe Stelle.
    ///
    /// Two threads writing to the same place without arranging it.
    Wettlauf,
    /// Aus einer 2 ein `bool` bauen.
    ///
    /// Building a `bool` out of a 2.
    UngueltigerWert,
}

/// Woher ein Schritt kommt, wenn man ihn in der Reference nachschlägt.
///
/// Where a step comes from when you look it up in the Reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunft {
    /// Aus der Liste "Behavior considered undefined", mit dem Punkt, unter dem
    /// er dort steht.
    ///
    /// From the list "Behavior considered undefined", with the item it stands
    /// under there.
    Undefiniert(&'static str),
    /// Aus der Liste "Behavior not considered unsafe", mit dem Eintrag, unter
    /// dem er dort steht. Unerwünscht ist er trotzdem.
    ///
    /// From the list "Behavior not considered unsafe", with the entry it stands
    /// under there. It is undesirable all the same.
    NichtUnsicher(&'static str),
    /// In keiner der beiden Listen, weil der Schritt erlaubt und gewöhnlich
    /// ist.
    ///
    /// In neither of the two lists, because the step is allowed and ordinary.
    InKeinerListe,
}

/// Ob dieser Schritt das Programm undefiniert macht.
///
/// Diese Funktion steht fertig da und ist die kurze Antwort. Sie sagt nur ja
/// oder nein und nicht, wo das nachzulesen ist; das ist Aufgabe 1.
///
/// Zwei der acht Schritte sind hier mit Absicht dabei, obwohl sie nichts
/// undefiniert machen. Ein Überlauf und eine Verklemmung sind Fehler, und die
/// Reference führt sie ausdrücklich als nicht unsicher. Wer "schlecht" und
/// "undefiniert" für dasselbe hält, kommt bei ihnen zum falschen Ergebnis.
///
/// Whether this step makes the program undefined.
///
/// This function stands there finished and is the short answer. It says only
/// yes or no and not where that is to be read up; that is exercise 1.
///
/// Two of the eight steps are deliberately among them although they make
/// nothing undefined. An overflow and a deadlock are faults, and the Reference
/// lists them expressly as not unsafe. Whoever takes "bad" and "undefined" to
/// be the same thing arrives at the wrong result on them.
///
/// ```
/// use unit_10_03_undefiniertes_verhalten::{Schritt, ist_undefiniert};
///
/// assert!(ist_undefiniert(Schritt::HaengendenZeigerLesen));
///
/// // Deutsch: Ein Überlauf ist ein Fehler und trotzdem nicht undefiniert.
/// // English: an overflow is a fault and undefined all the same it is not.
/// assert!(!ist_undefiniert(Schritt::UeberlaufBeimAddieren));
/// assert!(!ist_undefiniert(Schritt::Verklemmung));
/// ```
pub fn ist_undefiniert(schritt: Schritt) -> bool {
    matches!(
        schritt,
        Schritt::HaengendenZeigerLesen
            | Schritt::FalschAusgerichtetLesen
            | Schritt::Wettlauf
            | Schritt::UngueltigerWert
    )
}

/// Der Punkt der Reference, unter dem "Accessing" steht.
///
/// The item of the Reference that "Accessing" stands under.
pub const ZUGRIFF: &str = "Accessing (loading from or storing to) a place that is dangling or based on a misaligned pointer.";

/// Der Punkt der Reference, unter dem ein Wettlauf steht.
///
/// The item of the Reference that a data race stands under.
pub const WETTLAUF: &str = "Data races.";

/// Der Punkt der Reference, unter dem ein ungültiger Wert steht.
///
/// The item of the Reference that an invalid value stands under.
pub const UNGUELTIGER_WERT: &str = "Producing an invalid value.";

/// Der Eintrag der anderen Liste für einen Überlauf.
///
/// The entry of the other list for an overflow.
pub const UEBERLAUF: &str = "Integer overflow";

/// Der Eintrag der anderen Liste für eine Verklemmung.
///
/// The entry of the other list for a deadlock.
pub const VERKLEMMUNG: &str = "Deadlocks";

/// Aufgabe 1: Schlag den Schritt in der Reference nach.
///
/// Für die vier undefinierten Schritte kommt `Herkunft::Undefiniert` mit dem
/// Punkt zurück, unter dem er in "Behavior considered undefined" steht. Die
/// beiden Punkte sind als Konstanten da: `ZUGRIFF` deckt beide Zeiger-Fälle ab,
/// weil dieselbe Zeile hängende und falsch ausgerichtete Zugriffe nennt, dazu
/// `WETTLAUF` und `UNGUELTIGER_WERT`.
///
/// Für den Überlauf und die Verklemmung kommt `Herkunft::NichtUnsicher` mit
/// `UEBERLAUF` und `VERKLEMMUNG`, denn die stehen in der anderen Liste, in
/// "Behavior not considered unsafe". Für die beiden übrigen kommt
/// `Herkunft::InKeinerListe`.
///
/// Exercise 1: look the step up in the Reference.
///
/// For the four undefined steps `Herkunft::Undefiniert` comes back with the
/// item it stands under in "Behavior considered undefined". The items are there
/// as constants: `ZUGRIFF` covers both pointer cases, because the same line
/// names dangling and misaligned accesses, plus `WETTLAUF` and
/// `UNGUELTIGER_WERT`.
///
/// For the overflow and the deadlock `Herkunft::NichtUnsicher` comes with
/// `UEBERLAUF` and `VERKLEMMUNG`, because those stand in the other list, in
/// "Behavior not considered unsafe". For the two remaining ones
/// `Herkunft::InKeinerListe` comes.
pub fn herkunft(schritt: Schritt) -> Herkunft {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Nenn die Stelle, an der ein Programm undefiniert wird.
///
/// Zurück kommt der Index des ersten Schritts, der es undefiniert macht, und
/// `None`, wenn keiner das tut. Der erste zählt und nicht der schlimmste: ab
/// ihm sagt die Sprache über den Rest nichts mehr, also ist alles danach keine
/// Verschlechterung, sondern schon dieselbe Lage.
///
/// Exercise 2: name the point at which a program becomes undefined.
///
/// What comes back is the index of the first step making it undefined, and
/// `None` where none does. The first one counts and not the worst one: from it
/// on the language says nothing about the rest any more, so everything after it
/// is not a worsening but already the same situation.
pub fn erster_undefinierter(programm: &[Schritt]) -> Option<usize> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Nenn die Stelle und den Punkt der Reference dazu.
///
/// Zurück kommt das Paar aus dem Index aus Aufgabe 2 und dem Punkt aus
/// Aufgabe 1, und `None`, wenn das Programm definiert bleibt.
///
/// Das ist die Auskunft, um die es in dieser Einheit geht. "Irgendwo ist es
/// undefiniert" hilft niemandem; "an Schritt 2, und zwar unter dieser Zeile der
/// Reference" ist eine Aussage, die jemand nachschlagen kann.
///
/// Exercise 3: name the point and the item of the Reference for it.
///
/// What comes back is the pair of the index from exercise 2 and the item from
/// exercise 1, and `None` where the program stays defined.
///
/// That is the answer this unit is about. "Somewhere it is undefined" helps
/// nobody; "at step 2, and under this line of the Reference" is a statement
/// somebody can look up.
pub fn stelle_und_abschnitt(programm: &[Schritt]) -> Option<(usize, &'static str)> {
    todo!("Aufgabe 3 / Exercise 3")
}
