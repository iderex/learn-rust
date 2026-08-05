//! 09-01 Muster im Detail / Patterns in detail, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/09-01-muster-im-detail/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/09-01-muster-im-detail/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Ein Punkt auf der Fläche.
///
/// A point on the surface.
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Ein Ereignis, das einen Punkt enthalten kann.
///
/// An event that may contain a point.
#[derive(Debug, PartialEq)]
pub enum Event {
    Click { punkt: Point, taste: char },
    Key(char),
    Nothing,
}

/// Sagt, um welche Art Ereignis es sich handelt.
///
/// Says what kind of event this is.
pub fn kind(ereignis: &Event) -> &'static str {
    match ereignis {
        Event::Click { .. } => "Klick",
        Event::Key(_) => "Taste",
        Event::Nothing => "nichts",
    }
}

/// Beschreibt ein Ereignis in einem Satz.
///
/// Describes an event in one sentence.
pub fn describe(ereignis: &Event) -> String {
    match ereignis {
        Event::Click {
            punkt: Point { x, y },
            taste,
        } if x == y => format!("{taste} auf der Diagonalen bei {x}"),
        Event::Click {
            punkt: Point { x: 0, y },
            ..
        } => format!("am linken Rand, {y} tief"),
        Event::Click {
            punkt: Point { x, y },
            ..
        } => format!("bei {x} und {y}"),
        Event::Key(zeichen @ '0'..='9') => format!("Ziffer {zeichen}"),
        Event::Key(zeichen) => format!("Taste {zeichen}"),
        Event::Nothing => String::from("nichts"),
    }
}

/// Räumt einen Stapel von oben nach unten ab.
///
/// Clears a stack from the top down.
pub fn drain_stack(stapel: Vec<i32>) -> Vec<i32> {
    let mut stapel = stapel;
    let mut heraus = Vec::new();

    while let Some(oben) = stapel.pop() {
        heraus.push(oben);
    }

    heraus
}

/// Findet den Punkt des ersten Klicks in einer Liste.
///
/// Finds the point of the first click in a list.
pub fn first_click(ereignisse: &[Event]) -> Option<&Point> {
    for ereignis in ereignisse {
        if let Event::Click { punkt, .. } = ereignis {
            return Some(punkt);
        }
    }

    None
}
