// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_02_traits::{Flaeche, Quadrat, Rechteck};

#[test]
fn a_square_answers_the_demanded_method() {
    let quadrat = Quadrat { seite: 3 };

    assert_eq!(quadrat.flaeche(), 9);
    assert_eq!(Quadrat { seite: 0 }.flaeche(), 0);
}

#[test]
fn a_square_overrides_the_default_version() {
    let quadrat = Quadrat { seite: 3 };

    assert_eq!(quadrat.beschreibung(), "Quadrat mit Seite 3");
}

#[test]
fn a_rectangle_keeps_the_default_version() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };

    // Deutsch: Hier steht kein eigenes `beschreibung`, also gilt das aus dem
    // Trait.
    // English: no `beschreibung` of its own stands here, so the one from the
    // trait holds.
    assert_eq!(rechteck.beschreibung(), "Flaeche 12");
}

#[test]
fn the_trait_holds_for_a_foreign_type_too() {
    assert_eq!(7u32.flaeche(), 7);
    assert_eq!(0u32.flaeche(), 0);
}

#[test]
fn the_foreign_type_gets_the_default_version_as_well() {
    assert_eq!(7u32.beschreibung(), "Flaeche 7");
}

#[test]
fn two_types_answer_the_same_question() {
    // Deutsch: Verschiedene Felder, dieselbe Frage. Wer sie stellt, muss die
    // Felder nicht kennen.
    // English: different fields, the same question. Whoever asks it does not
    // have to know the fields.
    let rechteck = Rechteck {
        breite: 2,
        hoehe: 8,
    };
    let quadrat = Quadrat { seite: 4 };

    assert_eq!(rechteck.flaeche(), quadrat.flaeche());
}
