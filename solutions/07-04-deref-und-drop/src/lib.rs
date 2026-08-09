//! 07-04 Deref und Drop / Deref and Drop, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-04-deref-und-drop/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-04-deref-und-drop/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

use std::ops::Deref;

/// Ein Etikett, das sich wie ein `str` benutzen lässt.
///
/// A label that can be used like a `str`.
///
/// ```
/// use unit_07_04_deref_und_drop::Etikett;
///
/// let etikett = Etikett(String::from("Ada"));
///
/// assert_eq!(etikett.to_uppercase(), "ADA");
/// assert_eq!(etikett.len(), 3);
/// assert_eq!(&*etikett, "Ada");
/// ```
pub struct Etikett(pub String);

impl Deref for Etikett {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Ein Behälter, der genau einen Wert hält.
///
/// A container holding exactly one value.
pub struct Karton<T>(pub T);

impl<T> Deref for Karton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Sagt, wie lang der Text in einem `Karton<String>` ist.
///
/// Says how long the text in a `Karton<String>` is.
pub fn length(karton: &Karton<String>) -> usize {
    karton.len()
}

/// Ein Wächter, der beim Wegfallen ein Kreuz macht.
///
/// A guard that ticks a box when it falls away.
pub struct Wachhund<'a> {
    pub gefallen: &'a mut bool,
}

impl Drop for Wachhund<'_> {
    fn drop(&mut self) {
        *self.gefallen = true;
    }
}
