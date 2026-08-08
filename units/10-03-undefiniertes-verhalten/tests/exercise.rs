// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_03_undefiniertes_verhalten::{
    Herkunft, Schritt, UEBERLAUF, UNGUELTIGER_WERT, VERKLEMMUNG, WETTLAUF, ZUGRIFF,
    erster_undefinierter, herkunft, ist_undefiniert, stelle_und_abschnitt,
};

#[test]
fn the_finished_function_knows_the_four_undefined_steps() {
    assert!(ist_undefiniert(Schritt::HaengendenZeigerLesen));
    assert!(ist_undefiniert(Schritt::FalschAusgerichtetLesen));
    assert!(ist_undefiniert(Schritt::Wettlauf));
    assert!(ist_undefiniert(Schritt::UngueltigerWert));
}

#[test]
fn the_finished_function_knows_that_a_fault_is_not_the_same_thing() {
    assert!(!ist_undefiniert(Schritt::UeberlaufBeimAddieren));
    assert!(!ist_undefiniert(Schritt::Verklemmung));
    assert!(!ist_undefiniert(Schritt::IndexUeberDenRand));
    assert!(!ist_undefiniert(Schritt::ZeigerVergleichen));
}

// Deutsch: Dieselbe Zeile der Reference nennt den haengenden und den falsch
// ausgerichteten Zugriff, also steht bei beiden derselbe Punkt.
// English: the same line of the Reference names the dangling and the misaligned
// access, so the same item stands at both.
#[test]
fn herkunft_names_the_item_for_both_pointer_steps() {
    assert_eq!(
        herkunft(Schritt::HaengendenZeigerLesen),
        Herkunft::Undefiniert(ZUGRIFF)
    );
    assert_eq!(
        herkunft(Schritt::FalschAusgerichtetLesen),
        Herkunft::Undefiniert(ZUGRIFF)
    );
}

#[test]
fn herkunft_names_the_item_for_the_race_and_the_invalid_value() {
    assert_eq!(herkunft(Schritt::Wettlauf), Herkunft::Undefiniert(WETTLAUF));
    assert_eq!(
        herkunft(Schritt::UngueltigerWert),
        Herkunft::Undefiniert(UNGUELTIGER_WERT)
    );
}

// Deutsch: Der Ueberlauf und die Verklemmung stehen in der anderen Liste. Wer
// sie unter Undefiniert einsortiert, kommt hier nicht durch, und genau das ist
// der Punkt dieser Einheit.
// English: the overflow and the deadlock stand in the other list. Whoever files
// them under undefined does not get through here, and that is exactly the point
// of this unit.
#[test]
fn herkunft_puts_the_fault_into_the_other_list() {
    assert_eq!(
        herkunft(Schritt::UeberlaufBeimAddieren),
        Herkunft::NichtUnsicher(UEBERLAUF)
    );
    assert_eq!(
        herkunft(Schritt::Verklemmung),
        Herkunft::NichtUnsicher(VERKLEMMUNG)
    );
}

#[test]
fn herkunft_says_neither_list_for_an_ordinary_step() {
    assert_eq!(
        herkunft(Schritt::ZeigerVergleichen),
        Herkunft::InKeinerListe
    );
    assert_eq!(
        herkunft(Schritt::IndexUeberDenRand),
        Herkunft::InKeinerListe
    );
}

// Deutsch: Was die Kurzform sagt und was das Nachschlagen sagt, muss fuer alle
// acht Schritte dasselbe sein. Ein Schritt, der undefiniert ist, aber keinen
// Punkt der ersten Liste nennt, waere ein Widerspruch in dieser Einheit selbst.
// English: what the short form says and what the look-up says has to be the
// same for all eight steps. A step that is undefined but names no item of the
// first list would be a contradiction inside this unit itself.
#[test]
fn the_two_answers_agree_on_every_step() {
    let alle = [
        Schritt::ZeigerVergleichen,
        Schritt::IndexUeberDenRand,
        Schritt::UeberlaufBeimAddieren,
        Schritt::Verklemmung,
        Schritt::HaengendenZeigerLesen,
        Schritt::FalschAusgerichtetLesen,
        Schritt::Wettlauf,
        Schritt::UngueltigerWert,
    ];

    for schritt in alle {
        let aus_der_liste = matches!(herkunft(schritt), Herkunft::Undefiniert(_));

        assert_eq!(ist_undefiniert(schritt), aus_der_liste, "{schritt:?}");
    }
}

#[test]
fn erster_undefinierter_names_the_index() {
    let programm = [
        Schritt::ZeigerVergleichen,
        Schritt::UeberlaufBeimAddieren,
        Schritt::Wettlauf,
        Schritt::HaengendenZeigerLesen,
    ];

    assert_eq!(erster_undefinierter(&programm), Some(2));
}

// Deutsch: Der erste zaehlt und nicht der letzte. Ein Rumpf, der die Liste
// rueckwaerts durchgeht, kaeme hier auf 3.
// English: the first one counts and not the last one. A body walking the list
// backwards would arrive at 3 here.
#[test]
fn erster_undefinierter_takes_the_first_and_not_the_last() {
    let programm = [
        Schritt::Wettlauf,
        Schritt::ZeigerVergleichen,
        Schritt::UngueltigerWert,
    ];

    assert_eq!(erster_undefinierter(&programm), Some(0));
}

#[test]
fn erster_undefinierter_is_none_for_a_defined_program() {
    let programm = [
        Schritt::ZeigerVergleichen,
        Schritt::IndexUeberDenRand,
        Schritt::UeberlaufBeimAddieren,
        Schritt::Verklemmung,
    ];

    assert_eq!(erster_undefinierter(&programm), None);
    assert_eq!(erster_undefinierter(&[]), None);
}

#[test]
fn stelle_und_abschnitt_names_both() {
    let programm = [
        Schritt::ZeigerVergleichen,
        Schritt::HaengendenZeigerLesen,
        Schritt::Wettlauf,
    ];

    assert_eq!(stelle_und_abschnitt(&programm), Some((1, ZUGRIFF)));
}

// Deutsch: Der Punkt gehoert zu der Stelle, die genannt wird, und nicht zu
// irgendeinem undefinierten Schritt im Programm.
// English: the item belongs to the point that is named and not to just any
// undefined step in the program.
#[test]
fn stelle_und_abschnitt_takes_the_item_of_that_very_step() {
    let programm = [
        Schritt::Verklemmung,
        Schritt::Wettlauf,
        Schritt::HaengendenZeigerLesen,
    ];

    assert_eq!(stelle_und_abschnitt(&programm), Some((1, WETTLAUF)));
}

#[test]
fn stelle_und_abschnitt_is_none_for_a_defined_program() {
    assert_eq!(
        stelle_und_abschnitt(&[Schritt::IndexUeberDenRand, Schritt::Verklemmung]),
        None
    );
    assert_eq!(stelle_und_abschnitt(&[]), None);
}
