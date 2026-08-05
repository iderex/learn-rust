// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_10_error_und_box_dyn_error::eingabe::summe;
use unit_04_10_error_und_box_dyn_error::fehler::AppFehler;
use unit_04_10_error_und_box_dyn_error::summe_aus_texten;

#[test]
fn the_message_for_people_names_the_reason() {
    let fehler = AppFehler::from("zwei".parse::<i32>().unwrap_err());

    assert_eq!(
        fehler.to_string(),
        "keine ganze Zahl: invalid digit found in string"
    );
}

#[test]
fn the_second_conversion_reaches_the_same_type() {
    let fehler = AppFehler::from("halb".parse::<f64>().unwrap_err());

    assert_eq!(fehler.to_string(), "keine Kommazahl: invalid float literal");
}

#[test]
fn summe_in_the_good_case() {
    assert_eq!(summe("2", "0.5"), Ok(2.5));
    assert_eq!(summe(" 10 ", " 0.25 "), Ok(10.25));
}

#[test]
fn the_top_level_gives_the_number_back() {
    match summe_aus_texten("2", "0.5") {
        Ok(zahl) => assert_eq!(zahl, 2.5),
        Err(fehler) => panic!("unerwartet: {fehler}"),
    }
}

#[test]
fn the_top_level_passes_the_first_error_on() {
    let fehler = summe_aus_texten("zwei", "0.5").unwrap_err();

    assert_eq!(
        fehler.to_string(),
        "keine ganze Zahl: invalid digit found in string"
    );
}

#[test]
fn the_top_level_passes_the_second_error_on() {
    let fehler = summe_aus_texten("2", "halb").unwrap_err();

    assert_eq!(fehler.to_string(), "keine Kommazahl: invalid float literal");
}
