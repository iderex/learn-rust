// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_02_04_veraenderbares_ausleihen::{add_into, append_twice, double_in_place};

#[test]
fn double_in_place_doubles_the_number() {
    let mut zahl = 21;

    double_in_place(&mut zahl);

    assert_eq!(zahl, 42);
}

#[test]
fn double_in_place_leaves_zero_alone() {
    let mut zahl = 0;

    double_in_place(&mut zahl);

    assert_eq!(zahl, 0);
}

#[test]
fn double_in_place_twice_multiplies_by_four() {
    let mut zahl = -3;

    double_in_place(&mut zahl);
    double_in_place(&mut zahl);

    assert_eq!(zahl, -12);
}

#[test]
fn append_twice_appends_the_addition_two_times() {
    let mut text = String::from("hallo");

    append_twice(&mut text, "!");

    assert_eq!(text, "hallo!!");
}

#[test]
fn append_twice_on_an_empty_text() {
    let mut text = String::new();

    append_twice(&mut text, "ab");

    assert_eq!(text, "abab");
}

#[test]
fn add_into_adds_the_borrowed_number() {
    let mut ziel = 40;
    let summand = 2;

    add_into(&mut ziel, &summand);

    assert_eq!(ziel, 42);

    // Deutsch: Der Summand ist nur geliehen und steht unverändert da.
    // English: the summand was only lent and stands there unchanged.
    assert_eq!(summand, 2);
}

#[test]
fn add_into_with_a_negative_number() {
    let mut ziel = 10;

    add_into(&mut ziel, &-4);

    assert_eq!(ziel, 6);
}
