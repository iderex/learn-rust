//! 09-03 Operatorüberladung / Operator overloading, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/09-03-operatorueberladung/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-03-operatorueberladung/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

use std::ops::{Add, Index, Neg};

/// Ein Punkt auf der Fläche.
///
/// A point on the surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Das Gegenteil eines Punktes.
///
/// The opposite of a point.
///
/// ```
/// use unit_09_03_operatorueberladung::Point;
///
/// let punkt = Point { x: 1, y: -2 };
///
/// assert_eq!(-punkt, Point { x: -1, y: 2 });
/// assert_eq!(punkt, Point { x: 1, y: -2 });
/// ```
impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, andere: Point) -> Point {
        Point {
            x: self.x + andere.x,
            y: self.y + andere.y,
        }
    }
}

/// Eine Woche mit sieben Kürzeln.
///
/// A week with seven abbreviations.
pub struct Week {
    pub tage: [&'static str; 7],
}

impl Index<usize> for Week {
    type Output = str;

    fn index(&self, stelle: usize) -> &str {
        self.tage[stelle]
    }
}

/// Zählt eine Liste von Punkten zusammen.
///
/// Adds a list of points up.
pub fn sum(punkte: &[Point]) -> Point {
    let mut summe = Point { x: 0, y: 0 };

    for punkt in punkte {
        summe = summe + *punkt;
    }

    summe
}
