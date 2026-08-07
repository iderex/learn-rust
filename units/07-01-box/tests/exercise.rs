// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_01_box::{Liste, contains, from_slice, length, sum};

/// Baut ein Glied von Hand, damit die Tests nicht von Aufgabe 1 abhängen.
///
/// Builds a link by hand, so that the tests do not depend on exercise 1.
fn glied(zahl: i64, rest: Liste) -> Liste {
    Liste::Glied(zahl, Box::new(rest))
}

#[test]
fn from_slice_builds_one_link_per_number() {
    assert_eq!(
        from_slice(&[1, 2, 3]),
        glied(1, glied(2, glied(3, Liste::Ende)))
    );
}

#[test]
fn from_slice_keeps_the_order_of_the_slice() {
    assert_eq!(from_slice(&[7, 4]), glied(7, glied(4, Liste::Ende)));
}

#[test]
fn from_slice_of_an_empty_slice_is_ende() {
    assert_eq!(from_slice(&[]), Liste::Ende);
}

#[test]
fn sum_adds_every_number() {
    assert_eq!(sum(&glied(1, glied(2, glied(3, Liste::Ende)))), 6);
    assert_eq!(sum(&glied(-5, glied(5, Liste::Ende))), 0);
}

#[test]
fn sum_of_a_list_without_links_is_zero() {
    assert_eq!(sum(&Liste::Ende), 0);
}

#[test]
fn contains_finds_a_number_in_the_middle() {
    let liste = glied(1, glied(2, glied(3, Liste::Ende)));

    assert!(contains(&liste, 2));
    assert!(contains(&liste, 1));
    assert!(contains(&liste, 3));
}

#[test]
fn contains_is_false_for_a_number_that_is_not_there() {
    assert!(!contains(&glied(1, glied(2, Liste::Ende)), 4));
}

#[test]
fn contains_of_a_list_without_links_is_false() {
    assert!(!contains(&Liste::Ende, 1));
}

#[test]
fn the_finished_function_counts_the_links() {
    assert_eq!(length(&glied(1, glied(2, Liste::Ende))), 2);
    assert_eq!(length(&Liste::Ende), 0);
}
