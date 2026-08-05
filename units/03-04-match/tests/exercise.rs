// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_04_match::{Reading, carried_values, highest, label};

#[test]
fn highest_of_a_single_value_is_that_value() {
    assert_eq!(highest(&Reading::Temperature(17)), 17);
    assert_eq!(highest(&Reading::Temperature(-5)), -5);
}

#[test]
fn highest_of_a_range_is_the_upper_bound() {
    assert_eq!(highest(&Reading::Range { von: 3, bis: 9 }), 9);
}

#[test]
fn highest_of_a_missing_value_is_zero() {
    assert_eq!(highest(&Reading::Missing), 0);
}

#[test]
fn label_names_every_case() {
    assert_eq!(label(&Reading::Missing), "leer");
    assert_eq!(label(&Reading::Temperature(17)), "einzeln");
    assert_eq!(label(&Reading::Range { von: 3, bis: 9 }), "bereich");
}

#[test]
fn carried_values_counts_the_numbers_of_a_case() {
    assert_eq!(carried_values(&Reading::Missing), 0);
    assert_eq!(carried_values(&Reading::Temperature(17)), 1);
    assert_eq!(carried_values(&Reading::Range { von: 3, bis: 9 }), 2);
}

#[test]
fn carried_values_does_not_look_at_the_numbers() {
    // Deutsch: Andere Zahlen, dieselbe Antwort. Gezählt wird der Fall und nicht
    // sein Inhalt.
    // English: different numbers, same answer. What is counted is the case and
    // not its content.
    assert_eq!(carried_values(&Reading::Temperature(-40)), 1);
    assert_eq!(carried_values(&Reading::Range { von: 0, bis: 0 }), 2);
}
