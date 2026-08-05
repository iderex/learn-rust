// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_03_06_derive_mit_debug::{Reading, Rectangle, debug_block, debug_line, same};

#[test]
fn debug_line_prints_the_type_and_its_fields() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    assert_eq!(debug_line(&rechteck), "Rectangle { breite: 3, hoehe: 4 }");
}

#[test]
fn debug_block_prints_one_line_per_field() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    assert_eq!(
        debug_block(&rechteck),
        "Rectangle {\n    breite: 3,\n    hoehe: 4,\n}"
    );
}

#[test]
fn same_says_yes_for_two_equal_rectangles() {
    let a = Rectangle {
        breite: 3,
        hoehe: 4,
    };
    let b = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    assert!(same(&a, &b));
}

#[test]
fn same_says_no_when_one_field_differs() {
    let a = Rectangle {
        breite: 3,
        hoehe: 4,
    };
    let b = Rectangle {
        breite: 3,
        hoehe: 5,
    };

    assert!(!same(&a, &b));
}

#[test]
fn the_derived_implementations_are_ordinary_code() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    // Deutsch: `Copy` macht aus der Zuweisung eine Kopie, `PartialEq` erlaubt
    // den Vergleich, und `assert_eq!` gibt beide Seiten mit `Debug` aus.
    // English: `Copy` makes a copy out of the assignment, `PartialEq` allows the
    // comparison, and `assert_eq!` prints both sides with `Debug`.
    let zweites = rechteck;

    assert_eq!(rechteck, zweites);
    assert_eq!(Reading::Temperature(17), Reading::Temperature(17));
    assert_ne!(Reading::Missing, Reading::Temperature(0));
}
