//! 07-09 Trait-Objekte / Trait objects
//!
//! Deutsch: Ein `dyn Trait` hinter einem Zeiger lässt eine Liste mehrere Typen
//! tragen. Der Übersetzer weiß dann nicht mehr, welcher Typ da liegt, und sucht
//! die Methode erst beim Aufruf. Dieselbe Aufgabe geht auch mit einem `enum`,
//! und welche Fassung wann passt, steht in der README.
//!
//! English: a `dyn Trait` behind a pointer lets one list carry several types.
//! The compiler then no longer knows which type lies there and looks the method
//! up at the call. The same task also works with an `enum`, and which version
//! fits when is written in the README.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Was eine Form können muss.
///
/// `name` kommt zu `flaeche` dazu, damit eine Liste hinterher sagen kann, was
/// in ihr das Größte war. Beide Methoden nehmen `&self` und geben nichts
/// zurück, das `Self` heißt; genau das macht diesen Trait hinter `dyn`
/// benutzbar.
///
/// What a shape has to be able to do.
///
/// `name` comes along with `flaeche` so that a list can say afterwards what the
/// largest thing in it was. Both methods take `&self` and return nothing called
/// `Self`; that is exactly what makes this trait usable behind `dyn`.
pub trait Flaeche {
    /// Die Fläche der Form / the area of the shape.
    fn flaeche(&self) -> u32;

    /// Der Name der Form / the name of the shape.
    fn name(&self) -> &'static str;
}

/// Ein Rechteck aus Breite und Höhe.
///
/// A rectangle made of a width and a height.
pub struct Rechteck {
    /// Die Breite / the width.
    pub breite: u32,
    /// Die Höhe / the height.
    pub hoehe: u32,
}

/// Ein Quadrat mit einer Seitenlänge.
///
/// A square with one side length.
pub struct Quadrat {
    /// Die Seitenlänge / the side length.
    pub seite: u32,
}

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    fn name(&self) -> &'static str {
        "Rechteck"
    }
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    fn name(&self) -> &'static str {
        "Quadrat"
    }
}

/// Dieselben zwei Formen, diesmal als ein einziger Typ.
///
/// Ein `enum` zählt seine Fälle auf, und damit steht beim Übersetzen fest,
/// welche es gibt. Eine Liste von `Form` braucht deshalb keinen Zeiger und
/// keine Suche beim Aufruf.
///
/// The same two shapes, this time as a single type.
///
/// An `enum` lists its cases, so at compile time it is settled which ones there
/// are. A list of `Form` therefore needs no pointer and no lookup at the call.
pub enum Form {
    /// Ein Rechteck / a rectangle.
    Rechteck {
        /// Die Breite / the width.
        breite: u32,
        /// Die Höhe / the height.
        hoehe: u32,
    },
    /// Ein Quadrat / a square.
    Quadrat {
        /// Die Seitenlänge / the side length.
        seite: u32,
    },
}

impl Form {
    /// Aufgabe 2: Gib die Fläche dieses Falls zurück.
    ///
    /// Ein `match` über die beiden Fälle, und in jedem Fall die Rechnung, die
    /// dort gilt. Wer einen Fall vergisst, erfährt es vom Übersetzer und nicht
    /// von einem Test: Ein `match` muss jeden Fall treffen.
    ///
    /// Exercise 2: return the area of this case.
    ///
    /// A `match` over the two cases, and in every case the calculation that
    /// holds there. Whoever forgets a case hears it from the compiler and not
    /// from a test: a `match` has to meet every case.
    pub fn flaeche(&self) -> u32 {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Nimmt irgendeine Form und gibt ihre Fläche zurück, generisch.
///
/// Diese Funktion steht fertig da und ist der Vergleichsfall. Sie ist
/// generisch, also schreibt der Übersetzer sie für jeden Typ, mit dem sie
/// aufgerufen wird, ein zweites Mal hin. Beim Aufruf steht dann fest, welcher
/// Rumpf gemeint ist, und nichts muss gesucht werden.
///
/// Der Preis steht an derselben Stelle: Eine Liste kann nicht `Vec<F>` sein und
/// zwei Typen tragen, denn `F` ist ein einziger Typ. Dafür ist `dyn` da.
///
/// Takes any shape and returns its area, generically.
///
/// This function stands there finished and is the case to compare against. It is
/// generic, so the compiler writes it out a second time for every type it is
/// called with. At the call it is then settled which body is meant, and nothing
/// has to be looked up.
///
/// The price sits at the same place: a list cannot be `Vec<F>` and carry two
/// types, because `F` is one single type. That is what `dyn` is for.
///
/// ```
/// use unit_07_09_trait_objekte::{Flaeche, Quadrat, Rechteck, flaeche_von};
///
/// let rechteck = Rechteck {
///     breite: 3,
///     hoehe: 4,
/// };
/// let quadrat = Quadrat { seite: 5 };
///
/// assert_eq!(flaeche_von(&rechteck), 12);
/// assert_eq!(flaeche_von(&quadrat), 25);
///
/// // Deutsch: Der Trait ist eingebunden, also steht auch `name` bereit.
/// // English: the trait is in scope, so `name` stands ready as well.
/// assert_eq!(rechteck.name(), "Rechteck");
/// assert_eq!(quadrat.name(), "Quadrat");
/// ```
pub fn flaeche_von<F: Flaeche>(form: &F) -> u32 {
    form.flaeche()
}

/// Aufgabe 1: Zähle die Flächen einer gemischten Liste zusammen.
///
/// In der Liste liegen mehrere Typen nebeneinander, jeder hinter einem `Box`.
/// Was sie gemeinsam haben, ist der Trait, und mehr braucht der Rumpf auch
/// nicht zu wissen. Eine leere Liste ergibt 0.
///
/// Exercise 1: add up the areas of a mixed list.
///
/// Several types lie side by side in the list, each behind a `Box`. What they
/// have in common is the trait, and the body does not need to know more than
/// that. An empty list gives 0.
pub fn gesamt_dyn(formen: &[Box<dyn Flaeche>]) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 3: Dieselbe Aufgabe, diesmal über das `enum`.
///
/// Zurück kommt dieselbe Zahl wie aus `gesamt_dyn` für dieselben Formen. Was
/// sich ändert, ist der Weg dorthin, und darum geht es in dieser Einheit. Eine
/// leere Liste ergibt 0.
///
/// Exercise 3: the same task, this time over the `enum`.
///
/// What comes back is the same number as out of `gesamt_dyn` for the same
/// shapes. What changes is the way there, and that is what this unit is about.
/// An empty list gives 0.
pub fn gesamt_enum(formen: &[Form]) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}

/// Aufgabe 4: Sag, wie die größte Form heißt.
///
/// Zurück kommt der Name der Form mit der größten Fläche. Bei Gleichstand
/// gewinnt die erste, und aus einer leeren Liste kommt `None`.
///
/// Beachte, dass `max_by_key` hier nicht ohne Weiteres passt: Bei Gleichstand
/// gibt es das letzte Element zurück und nicht das erste.
///
/// Exercise 4: say what the largest shape is called.
///
/// What comes back is the name of the shape with the largest area. On a tie the
/// first one wins, and out of an empty list comes `None`.
///
/// Note that `max_by_key` does not fit here without more thought: on a tie it
/// gives back the last element and not the first.
pub fn groesste_dyn(formen: &[Box<dyn Flaeche>]) -> Option<&'static str> {
    todo!("Aufgabe 4 / Exercise 4")
}
