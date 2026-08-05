// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_03_use_und_sichtbarkeit::{boiling_in_fahrenheit, in_kelvin, round_trip};

#[test]
fn boiling_in_fahrenheit_is_two_hundred_and_twelve() {
    assert_eq!(boiling_in_fahrenheit(), 212);
}

#[test]
fn round_trip_comes_back_to_the_same_number() {
    assert_eq!(round_trip(100), 100);
    assert_eq!(round_trip(0), 0);
}

#[test]
fn round_trip_loses_something_on_the_way() {
    // Deutsch: Ganze Zahlen teilen ohne Rest, und deshalb kommt aus 37 die 36.
    // English: whole numbers divide without a remainder, and that is why 37
    // comes back as 36.
    assert_eq!(round_trip(37), 36);
}

#[test]
fn in_kelvin_adds_the_offset() {
    assert_eq!(in_kelvin(0), 273);
    assert_eq!(in_kelvin(100), 373);
    assert_eq!(in_kelvin(-273), 0);
}

#[test]
fn the_short_and_the_long_path_are_the_same_function() {
    // Deutsch: `pub use` in `src/lib.rs` gibt den Namen nach außen weiter. Der
    // lange Pfad geht weiterhin auch, und beide meinen dasselbe.
    // English: `pub use` in `src/lib.rs` hands the name on outwards. The long
    // path keeps working too, and both mean the same thing.
    let kurz = unit_04_03_use_und_sichtbarkeit::to_fahrenheit(100);
    let lang = unit_04_03_use_und_sichtbarkeit::messwerte::celsius::to_fahrenheit(100);

    assert_eq!(kurz, 212);
    assert_eq!(kurz, lang);
}
