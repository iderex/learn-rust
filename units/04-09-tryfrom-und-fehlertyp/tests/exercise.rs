// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_09_tryfrom_und_fehlertyp::{Alter, AlterFehler, age_from, years_of};

#[test]
fn try_from_takes_a_number_that_is_an_age() {
    assert_eq!(Alter::try_from(42), Ok(Alter(42)));
    assert_eq!(Alter::try_from(0), Ok(Alter(0)));
    assert_eq!(Alter::try_from(130), Ok(Alter(130)));
}

#[test]
fn try_from_refuses_a_number_that_is_too_big() {
    assert_eq!(Alter::try_from(131), Err(AlterFehler::ZuGross));
}

#[test]
fn try_from_refuses_a_negative_number() {
    assert_eq!(Alter::try_from(-1), Err(AlterFehler::Negativ));
}

#[test]
fn the_error_carries_a_message_for_people() {
    assert_eq!(
        AlterFehler::Negativ.to_string(),
        "ein Alter ist nicht negativ"
    );
    assert_eq!(
        AlterFehler::ZuGross.to_string(),
        "ein Alter über 130 gibt es nicht"
    );
}

#[test]
fn the_message_for_people_differs_from_the_one_for_debugging() {
    let fehler = AlterFehler::ZuGross;

    assert_eq!(format!("{fehler:?}"), "ZuGross");
    assert_ne!(format!("{fehler}"), format!("{fehler:?}"));
}

#[test]
fn age_from_uses_the_other_side_of_the_conversion() {
    assert_eq!(age_from(42), Ok(Alter(42)));
    assert_eq!(age_from(200), Err(AlterFehler::ZuGross));
}

#[test]
fn a_checked_age_needs_no_second_check() {
    let alter = age_from(42);

    // Deutsch: Wer ein `Alter` in der Hand hat, hat eine geprüfte Zahl darin.
    // English: whoever holds an `Alter` holds a checked number in it.
    match alter {
        Ok(geprueft) => assert_eq!(years_of(&geprueft), 42),
        Err(fehler) => panic!("unerwartet: {fehler}"),
    }
}
