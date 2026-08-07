//! 05-05 Tests und ihr Aufbau / Tests and how they are organised, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/05-05-tests-und-ihr-aufbau/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen, dazu das
//! Testmodul neben dem Code, das die Einheit an dieser Stelle auch hat.
//!
//! English: the explanation lives in
//! `units/05-05-tests-und-ihr-aufbau/README.md`. What is here is only the
//! bodies that turn the unit's tests green, plus the test module beside the
//! code that the unit carries at this place too.

/// Sagt, ob eine Note bestanden ist.
///
/// Says whether a grade has passed.
///
/// ```
/// use unit_05_05_tests_und_ihr_aufbau::passed;
///
/// assert!(passed('A'));
/// assert!(passed('D'));
/// assert!(!passed('F'));
/// ```
pub fn passed(note: char) -> bool {
    matches!(note, 'A' | 'B' | 'C' | 'D')
}

/// Gibt zu einer Punktzahl die Note als Buchstaben.
///
/// Gives the grade as a letter for a number of points.
pub fn grade(punkte: u32) -> char {
    match punkte {
        101.. => panic!("mehr als 100 Punkte gibt es nicht: {punkte}"),
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

/// Setzt Punkte, Note und Ausgang zu einer Zeile zusammen.
///
/// Puts points, grade and outcome together into one line.
pub fn describe(punkte: u32) -> String {
    let note = grade(punkte);
    let ausgang = if passed(note) {
        "bestanden"
    } else {
        "durchgefallen"
    };

    format!("{punkte} Punkte, Note {note}, {ausgang}")
}

/// Sagt, ob jede Punktzahl der Liste bestanden ist.
///
/// Says whether every score in the list has passed.
pub fn all_passed(punkte: &[u32]) -> bool {
    punkte.iter().all(|zahl| passed(grade(*zahl)))
}

// Deutsch: Tests neben dem Code, dieselben wie in der Einheit.
// English: tests beside the code, the same ones as in the unit.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passed_is_true_from_a_to_d() {
        assert!(passed('A'));
        assert!(passed('B'));
        assert!(passed('C'));
        assert!(passed('D'));
    }

    #[test]
    fn passed_is_false_for_f() {
        assert!(!passed('F'));
        assert_ne!(passed('F'), passed('D'));
    }
}
