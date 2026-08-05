// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_02_methoden::Rectangle;

#[test]
fn new_fills_both_fields() {
    let rechteck = Rectangle::new(3, 4);

    assert_eq!(rechteck.breite, 3);
    assert_eq!(rechteck.hoehe, 4);
}

#[test]
fn new_is_called_without_a_value_in_front() {
    let quadrat = Rectangle::new(5, 5);

    assert_eq!(quadrat.area(), 25);
}

#[test]
fn perimeter_adds_both_sides_twice() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    assert_eq!(rechteck.perimeter(), 14);
}

#[test]
fn perimeter_only_reads() {
    let rechteck = Rectangle {
        breite: 2,
        hoehe: 2,
    };

    assert_eq!(rechteck.perimeter(), 8);

    // Deutsch: Nach dem Lesen steht das Rechteck unverändert da.
    // English: after the reading the rectangle stands there unchanged.
    assert_eq!(rechteck.breite, 2);
    assert_eq!(rechteck.hoehe, 2);
}

#[test]
fn double_changes_the_rectangle_in_place() {
    let mut rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    rechteck.double();

    assert_eq!(rechteck.breite, 6);
    assert_eq!(rechteck.hoehe, 8);
}

#[test]
fn double_twice_multiplies_by_four() {
    let mut rechteck = Rectangle::new(1, 2);

    rechteck.double();
    rechteck.double();

    assert_eq!(rechteck.breite, 4);
    assert_eq!(rechteck.hoehe, 8);
}
