//! 05-05 Tests und ihr Aufbau / Tests and how they are organised
//!
//! Deutsch: Ein Test ist eine Funktion mit `#[test]` darüber. Sie gilt als
//! bestanden, solange sie nicht in Panik gerät. Tests neben dem Code sehen auch
//! das Private, Tests unter `tests/` nur das Öffentliche.
//!
//! English: a test is a function with `#[test]` above it. It counts as passed
//! for as long as it does not panic. Tests beside the code see the private
//! things too, tests under `tests/` only the public ones.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Sagt, ob eine Note bestanden ist.
///
/// Diese Funktion steht fertig da. Bestanden sind `'A'` bis `'D'`, alles andere
/// nicht. Sie wird von dem Testmodul am Ende dieser Datei geprüft, also neben
/// dem Code, und ihr Beispiel unten ist ein Doku-Test.
///
/// Says whether a grade has passed.
///
/// This function stands there finished. `'A'` to `'D'` have passed, everything
/// else has not. It is checked by the test module at the end of this file,
/// meaning beside the code, and its example below is a doc test.
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

/// Aufgabe 1: Gib zu einer Punktzahl die Note als Buchstaben.
///
/// Ab 90 Punkten `'A'`, ab 80 `'B'`, ab 70 `'C'`, ab 60 `'D'`, darunter `'F'`.
/// Über 100 Punkten gibt es keine Note; dort ist `panic!` die richtige Antwort,
/// und die Meldung enthält `mehr als 100 Punkte gibt es nicht`. Der Test dazu
/// trägt `#[should_panic(expected = ...)]` und sieht nach, ob dieser Text in der
/// Meldung vorkommt.
///
/// Exercise 1: give the grade as a letter for a number of points.
///
/// From 90 points `'A'`, from 80 `'B'`, from 70 `'C'`, from 60 `'D'`, below that
/// `'F'`. Above 100 points there is no grade; there `panic!` is the right
/// answer, and the message contains `mehr als 100 Punkte gibt es nicht`. The
/// test for it carries `#[should_panic(expected = ...)]` and looks at whether
/// that text turns up in the message.
pub fn grade(punkte: u32) -> char {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Setz Punkte, Note und Ausgang zu einer Zeile zusammen.
///
/// Die Zeile lautet `85 Punkte, Note B, bestanden`, und bei einer Note, die
/// nicht besteht, endet sie auf `durchgefallen`. Note und Ausgang kommen aus
/// `grade` und `passed`, damit die Regel nur an einer Stelle steht.
///
/// Exercise 2: put points, grade and outcome together into one line.
///
/// The line reads `85 Punkte, Note B, bestanden`, and with a grade that does
/// not pass it ends on `durchgefallen`. Grade and outcome come from `grade` and
/// `passed`, so that the rule stands in one place only.
pub fn describe(punkte: u32) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sag, ob jede Punktzahl der Liste bestanden ist.
///
/// Eine leere Liste ist bestanden, weil keine Punktzahl darin durchfällt.
///
/// Exercise 3: say whether every score in the list has passed.
///
/// An empty list has passed, because no score in it fails.
pub fn all_passed(punkte: &[u32]) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}

// Deutsch: Tests neben dem Code. Sie liegen im selben Modul und sehen deshalb
// auch, was nicht `pub` ist. Dieses Modul ist grün, bevor die Aufgaben gelöst
// sind, denn es prüft die fertige Funktion.
// English: tests beside the code. They live in the same module and therefore
// see what is not `pub` as well. This module is green before the exercises are
// solved, because it checks the finished function.
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
