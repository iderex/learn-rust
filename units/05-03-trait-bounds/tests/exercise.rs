// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_03_trait_bounds::{largest, largest_reported, reported, smallest};

#[test]
fn largest_finds_the_biggest_number() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(largest(&[-3, -9]), Some(&-3));
}

#[test]
fn largest_works_for_texts_as_well() {
    // Deutsch: Texte lassen sich vergleichen, also erfüllen sie die Schranke.
    // English: texts can be compared, so they fulfil the bound.
    assert_eq!(largest(&["drei", "neun"]), Some(&"neun"));
}

#[test]
fn largest_of_an_empty_list_is_none() {
    assert_eq!(largest::<i32>(&[]), None);
}

#[test]
fn reported_writes_the_value_out() {
    assert_eq!(reported(42), "Wert 42");
    assert_eq!(reported("Text"), "Wert Text");
    assert_eq!(reported(1.5), "Wert 1.5");
}

#[test]
fn largest_reported_joins_both_bounds() {
    assert_eq!(largest_reported(&[3, 9, 4]), "groesster Wert 9");
    assert_eq!(largest_reported(&["drei", "neun"]), "groesster Wert neun");
}

#[test]
fn largest_reported_of_an_empty_list() {
    assert_eq!(largest_reported::<i32>(&[]), "keine Werte");
}

#[test]
fn the_finished_function_shows_the_same_shape() {
    assert_eq!(smallest(&[3, 9, 4]), Some(&3));
    assert_eq!(smallest::<i32>(&[]), None);
}
