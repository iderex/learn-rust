// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_01_struct::{Marker, Meter, Rectangle, area_of, in_meters, new_rectangle};

#[test]
fn new_rectangle_fills_both_fields() {
    let rechteck = new_rectangle(3, 4);

    // Deutsch: Verglichen wird Feld für Feld, denn ein ganzes struct
    // vergleichen zu können ist `03-06`.
    // English: the comparison goes field by field, because comparing a whole
    // struct is `03-06`.
    assert_eq!(rechteck.breite, 3);
    assert_eq!(rechteck.hoehe, 4);
}

#[test]
fn new_rectangle_takes_zero_as_a_side() {
    let rechteck = new_rectangle(0, 7);

    assert_eq!(rechteck.breite, 0);
    assert_eq!(rechteck.hoehe, 7);
}

#[test]
fn area_of_multiplies_the_two_sides() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    assert_eq!(area_of(&rechteck), 12);

    // Deutsch: Das Rechteck ist nur geliehen und steht danach noch da.
    // English: the rectangle was only lent and is still there afterwards.
    assert_eq!(rechteck.breite, 3);
}

#[test]
fn area_of_a_rectangle_without_width() {
    let rechteck = Rectangle {
        breite: 0,
        hoehe: 4,
    };

    assert_eq!(area_of(&rechteck), 0);
}

#[test]
fn in_meters_reads_the_numbered_field() {
    let strecke = Meter(1200);

    assert_eq!(in_meters(&strecke), 1200);
    assert_eq!(in_meters(&Meter(0)), 0);
}

#[test]
fn the_struct_without_a_field_exists() {
    let marker = Marker;

    // Deutsch: Es gibt nichts zu lesen. Dass der Wert angelegt werden kann, ist
    // die ganze Aussage.
    // English: there is nothing to read. That the value can be created is the
    // whole statement.
    let _ = marker;
}
