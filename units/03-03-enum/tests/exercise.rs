// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_03_enum::{as_text, missing, range, single};

#[test]
fn missing_carries_no_data() {
    assert_eq!(as_text(&missing()), "kein Wert");
}

#[test]
fn single_carries_one_number() {
    assert_eq!(as_text(&single(17)), "17 Grad");
    assert_eq!(as_text(&single(-5)), "-5 Grad");
}

#[test]
fn range_carries_two_named_fields() {
    assert_eq!(as_text(&range(3, 9)), "von 3 bis 9 Grad");
}

#[test]
fn range_puts_the_smaller_number_first() {
    assert_eq!(as_text(&range(9, 3)), "von 3 bis 9 Grad");
}

#[test]
fn range_with_two_equal_numbers() {
    assert_eq!(as_text(&range(4, 4)), "von 4 bis 4 Grad");
}

#[test]
fn all_three_are_the_same_type() {
    // Deutsch: Drei Varianten, ein Typ, eine Funktion. Das Feld nimmt sie
    // nebeneinander auf.
    // English: three variants, one type, one function. The array takes them
    // side by side.
    let messwerte = [missing(), single(17), range(3, 9)];

    let beschreibungen: [String; 3] = [
        as_text(&messwerte[0]),
        as_text(&messwerte[1]),
        as_text(&messwerte[2]),
    ];

    assert_eq!(beschreibungen[0], "kein Wert");
    assert_eq!(beschreibungen[1], "17 Grad");
    assert_eq!(beschreibungen[2], "von 3 bis 9 Grad");
}
