// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_07_panic_und_result::{Fehler, checked_age, divided, first_line};

#[test]
fn divided_in_the_good_case() {
    assert_eq!(divided(10, 2), Ok(5));
    assert_eq!(divided(-9, 3), Ok(-3));
}

#[test]
fn divided_in_the_bad_case() {
    assert_eq!(divided(10, 0), Err(Fehler::DurchNull));
}

#[test]
fn checked_age_in_the_good_case() {
    assert_eq!(checked_age(0), Ok(0));
    assert_eq!(checked_age(42), Ok(42));
    assert_eq!(checked_age(130), Ok(130));
}

#[test]
fn checked_age_in_the_bad_case() {
    assert_eq!(checked_age(131), Err(Fehler::KeinAlter));
    assert_eq!(checked_age(1000), Err(Fehler::KeinAlter));
}

#[test]
fn first_line_in_the_good_case() {
    assert_eq!(first_line("erste\nzweite"), Ok(String::from("erste")));
    assert_eq!(first_line("nur eine"), Ok(String::from("nur eine")));
}

#[test]
fn first_line_in_the_bad_case() {
    assert_eq!(first_line(""), Err(Fehler::LeererText));
}

#[test]
fn a_result_is_treated_and_not_caught() {
    // Deutsch: Beim Aufrufer steht ein Wert, und `match` behandelt beide Fälle.
    // Nichts wird geworfen und nichts gefangen.
    // English: at the caller stands a value, and `match` treats both cases.
    // Nothing is thrown and nothing is caught.
    let gemeldet = match divided(10, 0) {
        Ok(zahl) => format!("Ergebnis {zahl}"),
        Err(fehler) => format!("Fehler {fehler:?}"),
    };

    assert_eq!(gemeldet, "Fehler DurchNull");
}
