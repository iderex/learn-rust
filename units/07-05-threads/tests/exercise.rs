// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_05_threads::{in_einem_faden, quadrate_in_faeden, summe_in_faeden, zeichen_in_faeden};

#[test]
fn the_finished_function_doubles_in_a_thread() {
    assert_eq!(in_einem_faden(21), 42);
    assert_eq!(in_einem_faden(0), 0);
}

#[test]
fn summe_in_faeden_adds_every_number() {
    assert_eq!(summe_in_faeden(vec![1, 2, 3, 4]), 10);
}

#[test]
fn summe_in_faeden_of_an_empty_list_is_zero() {
    assert_eq!(summe_in_faeden(Vec::new()), 0);
}

// Deutsch: Eine Summe haengt nicht davon ab, in welcher Reihenfolge die Faeden
// fertig werden. Die Zahlen sind mit Absicht verschieden, damit ein Rumpf, der
// nur einen Faden abholt, hier nicht durchkommt.
// English: a sum does not depend on the order in which the threads finish. The
// numbers are deliberately different, so that a body picking up only one thread
// does not get through here.
#[test]
fn summe_in_faeden_is_the_same_whatever_the_order() {
    assert_eq!(summe_in_faeden(vec![10, 20, 30, 40, 50]), 150);
    assert_eq!(summe_in_faeden(vec![50, 40, 30, 20, 10]), 150);
}

#[test]
fn quadrate_in_faeden_squares_every_number() {
    assert_eq!(quadrate_in_faeden(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

// Deutsch: Die Reihenfolge ist die der Liste und nicht die, in der die Faeden
// fertig werden. Die Quadrate sind hier absteigend, ein sortiertes Ergebnis
// waere also ein anderes.
// English: the order is the one of the list and not the one in which the
// threads finish. The squares here go downwards, so a sorted result would be a
// different one.
#[test]
fn quadrate_in_faeden_keeps_the_order_of_the_list() {
    assert_eq!(quadrate_in_faeden(vec![4, 3, 2, 1]), vec![16, 9, 4, 1]);
}

#[test]
fn quadrate_in_faeden_of_an_empty_list_is_empty() {
    assert_eq!(quadrate_in_faeden(Vec::new()), Vec::<i32>::new());
}

// Deutsch: Mit mehr Faeden als Kernen bleibt die Reihenfolge dieselbe, denn sie
// entsteht beim Starten und nicht beim Fertigwerden.
// English: with more threads than cores the order stays the same, because it
// comes about while starting them and not while they finish.
#[test]
fn quadrate_in_faeden_keeps_the_order_with_many_threads() {
    let werte: Vec<i32> = (1..=64).rev().collect();
    let erwartet: Vec<i32> = werte.iter().map(|wert| wert * wert).collect();

    assert_eq!(quadrate_in_faeden(werte), erwartet);
}

#[test]
fn zeichen_in_faeden_adds_the_lengths() {
    let texte = vec![
        String::from("Ada"),
        String::from("Grace"),
        String::from("Alan"),
    ];

    assert_eq!(zeichen_in_faeden(texte), 12);
}

#[test]
fn zeichen_in_faeden_of_an_empty_list_is_zero() {
    assert_eq!(zeichen_in_faeden(Vec::new()), 0);
}

// Deutsch: Ein leerer Text zaehlt null und faellt nicht heraus.
// English: an empty text counts zero and does not drop out.
#[test]
fn zeichen_in_faeden_counts_an_empty_text_as_zero() {
    let texte = vec![String::new(), String::from("ab")];

    assert_eq!(zeichen_in_faeden(texte), 2);
}
