// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_07_display_selbst_schreiben::{Percent, Reading, Sensor, for_debugging, for_people};

#[test]
fn display_of_a_reading_names_every_case() {
    assert_eq!(Reading::Missing.to_string(), "kein Wert");
    assert_eq!(Reading::Temperature(17).to_string(), "17 Grad");
    assert_eq!(
        Reading::Range { von: 3, bis: 9 }.to_string(),
        "3 bis 9 Grad"
    );
}

#[test]
fn display_of_a_reading_with_negative_numbers() {
    assert_eq!(Reading::Temperature(-5).to_string(), "-5 Grad");
    assert_eq!(
        Reading::Range { von: -9, bis: -3 }.to_string(),
        "-9 bis -3 Grad"
    );
}

#[test]
fn display_of_a_sensor_uses_the_reading() {
    let sensor = Sensor {
        name: String::from("Flur"),
        wert: Reading::Temperature(17),
    };

    assert_eq!(format!("{sensor}"), "Flur: 17 Grad");
}

#[test]
fn display_of_a_sensor_without_a_reading() {
    let sensor = Sensor {
        name: String::from("Keller"),
        wert: Reading::Missing,
    };

    assert_eq!(format!("{sensor}"), "Keller: kein Wert");
}

#[test]
fn for_people_and_for_debugging_are_two_different_outputs() {
    let sensor = Sensor {
        name: String::from("Flur"),
        wert: Reading::Temperature(17),
    };

    assert_eq!(for_people(&sensor), "Flur: 17 Grad");
    assert_eq!(
        for_debugging(&sensor),
        "Sensor { name: \"Flur\", wert: Temperature(17) }"
    );

    // Deutsch: Zwei Ausgaben aus einem Wert, und sie sind verschieden.
    // English: two outputs out of one value, and they are different.
    assert_ne!(for_people(&sensor), for_debugging(&sensor));
}

#[test]
fn the_model_in_the_unit_shows_the_shape() {
    // Deutsch: `Percent` steht fertig da, mit beiden Ausgaben.
    // English: `Percent` stands there finished, with both outputs.
    let anteil = Percent(42);

    assert_eq!(anteil.to_string(), "42 %");
    assert_eq!(format!("{anteil:?}"), "Percent(42)");
}
