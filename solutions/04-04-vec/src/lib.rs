//! 04-04 Vec / Vec, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/04-04-vec/README.md`. Hier stehen nur
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/04-04-vec/README.md`. What is here
//! is only the bodies that turn the unit's tests green.

/// Nimmt eine Liste, hängt einen Wert an und gibt sie zurück.
///
/// Takes a list, appends a value and gives it back.
pub fn pushed(zahlen: Vec<i32>, neu: i32) -> Vec<i32> {
    let mut zahlen = zahlen;
    zahlen.push(neu);
    zahlen
}

/// Baut eine Liste von 1 bis einschließlich `bis` auf.
///
/// Builds a list from 1 up to and including `bis`.
pub fn built(bis: u32) -> Vec<u32> {
    let mut zahlen = Vec::new();

    for zahl in 1..=bis {
        zahlen.push(zahl);
    }

    zahlen
}

/// Gibt den größten Wert der Liste zurück.
///
/// Returns the biggest value of the list.
pub fn largest(zahlen: &[i32]) -> Option<i32> {
    let mut groesster = None;

    for zahl in zahlen {
        if let Some(bisher) = groesster {
            if *zahl > bisher {
                groesster = Some(*zahl);
            }
        } else {
            groesster = Some(*zahl);
        }
    }

    groesster
}

/// Gibt eine neue Liste mit verdoppelten Werten zurück.
///
/// Returns a new list with doubled values.
pub fn doubled_all(zahlen: &[i32]) -> Vec<i32> {
    let mut neue = Vec::new();

    for zahl in zahlen {
        neue.push(zahl * 2);
    }

    neue
}
