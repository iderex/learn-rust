// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_01_04_kommentare_und_cargo_fmt::{discounted, rounded_up_to_full_euro, vat_of};

#[test]
fn vat_of_takes_nineteen_percent() {
    assert_eq!(vat_of(100), 19);
    assert_eq!(vat_of(0), 0);
}

#[test]
fn vat_of_cuts_off_the_remainder() {
    assert_eq!(vat_of(10), 1);
}

#[test]
fn discounted_takes_ten_percent_off() {
    assert_eq!(discounted(100), 90);
    assert_eq!(discounted(0), 0);
}

#[test]
fn rounded_up_to_full_euro_leaves_a_full_euro_alone() {
    assert_eq!(rounded_up_to_full_euro(200), 200);
}

#[test]
fn rounded_up_to_full_euro_climbs_to_the_next_one() {
    assert_eq!(rounded_up_to_full_euro(201), 300);
    assert_eq!(rounded_up_to_full_euro(299), 300);
}
