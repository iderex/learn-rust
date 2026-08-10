//! 07-09 Trait-Objekte / Trait objects, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-09-trait-objekte/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-09-trait-objekte/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Was eine Form können muss.
///
/// What a shape has to be able to do.
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
/// The same two shapes, this time as a single type.
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
    /// Gibt die Fläche dieses Falls zurück.
    ///
    /// Returns the area of this case.
    pub fn flaeche(&self) -> u32 {
        match self {
            Form::Rechteck { breite, hoehe } => breite * hoehe,
            Form::Quadrat { seite } => seite * seite,
        }
    }
}

/// Nimmt irgendeine Form und gibt ihre Fläche zurück, generisch.
///
/// Takes any shape and returns its area, generically.
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

/// Zählt die Flächen einer gemischten Liste zusammen.
///
/// Adds up the areas of a mixed list.
pub fn gesamt_dyn(formen: &[Box<dyn Flaeche>]) -> u32 {
    formen.iter().map(|form| form.flaeche()).sum()
}

/// Dieselbe Aufgabe über das `enum`.
///
/// The same task over the `enum`.
pub fn gesamt_enum(formen: &[Form]) -> u32 {
    formen.iter().map(Form::flaeche).sum()
}

/// Sagt, wie die größte Form heißt.
///
/// Says what the largest shape is called.
pub fn groesste_dyn(formen: &[Box<dyn Flaeche>]) -> Option<&'static str> {
    let mut groesste = 0;
    let mut name = None;

    for form in formen {
        // Deutsch: Nur echtes Größersein zählt, deshalb bleibt bei Gleichstand
        // die erste Form stehen.
        // English: only being genuinely larger counts, so on a tie the first
        // shape stays.
        if name.is_none() || form.flaeche() > groesste {
            groesste = form.flaeche();
            name = Some(form.name());
        }
    }

    name
}
