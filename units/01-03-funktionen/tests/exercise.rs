// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_03_funktionen::{area, celsius_from, perimeter, square_area};

#[test]
fn area_multiplies_its_two_sides() {
    assert_eq!(area(3, 4), 12);
}

#[test]
fn area_of_nothing_is_nothing() {
    assert_eq!(area(0, 7), 0);
}

#[test]
fn celsius_from_knows_the_freezing_point() {
    assert_eq!(celsius_from(32.0), 0.0);
}

#[test]
fn celsius_from_knows_the_boiling_point() {
    assert_eq!(celsius_from(212.0), 100.0);
}

#[test]
fn square_area_is_the_area_of_two_equal_sides() {
    assert_eq!(square_area(5), area(5, 5));
    assert_eq!(square_area(5), 25);
}

#[test]
fn the_example_walks_around_the_rectangle() {
    assert_eq!(perimeter(3, 4), 14);
}
