//! 10-03 Undefiniertes Verhalten / Undefined behaviour, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/10-03-undefiniertes-verhalten/README.md`. Hier stehen nur die Rümpfe,
//! die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/10-03-undefiniertes-verhalten/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Ein Schritt, den ein Programm tun kann.
///
/// A step a program can take.
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
    /// Aus der Liste "Behavior considered undefined".
    ///
    /// From the list "Behavior considered undefined".
    Undefiniert(&'static str),
    /// Aus der Liste "Behavior not considered unsafe".
    ///
    /// From the list "Behavior not considered unsafe".
    NichtUnsicher(&'static str),
    /// In keiner der beiden Listen.
    ///
    /// In neither of the two lists.
    InKeinerListe,
}

/// Ob dieser Schritt das Programm undefiniert macht.
///
/// Whether this step makes the program undefined.
///
/// ```
/// use unit_10_03_undefiniertes_verhalten::{Schritt, ist_undefiniert};
///
/// assert!(ist_undefiniert(Schritt::HaengendenZeigerLesen));
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

/// Der Schritt, in der Reference nachgeschlagen.
///
/// The step, looked up in the Reference.
pub fn herkunft(schritt: Schritt) -> Herkunft {
    match schritt {
        Schritt::HaengendenZeigerLesen | Schritt::FalschAusgerichtetLesen => {
            Herkunft::Undefiniert(ZUGRIFF)
        }
        Schritt::Wettlauf => Herkunft::Undefiniert(WETTLAUF),
        Schritt::UngueltigerWert => Herkunft::Undefiniert(UNGUELTIGER_WERT),
        Schritt::UeberlaufBeimAddieren => Herkunft::NichtUnsicher(UEBERLAUF),
        Schritt::Verklemmung => Herkunft::NichtUnsicher(VERKLEMMUNG),
        Schritt::ZeigerVergleichen | Schritt::IndexUeberDenRand => Herkunft::InKeinerListe,
    }
}

/// Der Index des ersten Schritts, der das Programm undefiniert macht.
///
/// The index of the first step making the program undefined.
pub fn erster_undefinierter(programm: &[Schritt]) -> Option<usize> {
    programm
        .iter()
        .position(|schritt| ist_undefiniert(*schritt))
}

/// Die Stelle und der Punkt der Reference dazu.
///
/// The point and the item of the Reference for it.
pub fn stelle_und_abschnitt(programm: &[Schritt]) -> Option<(usize, &'static str)> {
    let stelle = erster_undefinierter(programm)?;

    match herkunft(programm[stelle]) {
        Herkunft::Undefiniert(punkt) => Some((stelle, punkt)),
        _ => None,
    }
}
