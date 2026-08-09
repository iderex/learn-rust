// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_09_trait_objekte::{
    Flaeche, Form, Quadrat, Rechteck, flaeche_von, gesamt_dyn, gesamt_enum, groesste_dyn,
};

// Deutsch: Dieselben drei Formen einmal hinter `dyn` und einmal als `enum`. Die
// beiden Listen sind der Vergleich, um den es in dieser Einheit geht.
// English: the same three shapes once behind `dyn` and once as an `enum`. The
// two lists are the comparison this unit is about.
fn hinter_dyn() -> Vec<Box<dyn Flaeche>> {
    vec![
        Box::new(Rechteck {
            breite: 3,
            hoehe: 4,
        }),
        Box::new(Quadrat { seite: 5 }),
        Box::new(Rechteck {
            breite: 2,
            hoehe: 2,
        }),
    ]
}

fn als_enum() -> Vec<Form> {
    vec![
        Form::Rechteck {
            breite: 3,
            hoehe: 4,
        },
        Form::Quadrat { seite: 5 },
        Form::Rechteck {
            breite: 2,
            hoehe: 2,
        },
    ]
}

#[test]
fn gesamt_dyn_adds_up_a_mixed_list() {
    assert_eq!(gesamt_dyn(&hinter_dyn()), 41);
}

#[test]
fn gesamt_dyn_of_nothing_is_zero() {
    assert_eq!(gesamt_dyn(&[]), 0);
}

#[test]
fn form_answers_for_both_of_its_cases() {
    assert_eq!(
        Form::Rechteck {
            breite: 3,
            hoehe: 4
        }
        .flaeche(),
        12
    );
    assert_eq!(Form::Quadrat { seite: 5 }.flaeche(), 25);
}

// Deutsch: Beide Wege sollen dieselbe Zahl liefern. Steht hier eine andere,
// rechnet einer von beiden anders, und die Einheit vergliche zwei Aufgaben.
// English: both ways should give the same number. A different one here means
// one of the two calculates differently, and the unit would be comparing two
// tasks.
#[test]
fn gesamt_enum_gives_the_same_number_as_gesamt_dyn() {
    assert_eq!(gesamt_enum(&als_enum()), 41);
    assert_eq!(gesamt_enum(&als_enum()), gesamt_dyn(&hinter_dyn()));
}

#[test]
fn gesamt_enum_of_nothing_is_zero() {
    assert_eq!(gesamt_enum(&[]), 0);
}

#[test]
fn groesste_dyn_names_the_largest_shape() {
    assert_eq!(groesste_dyn(&hinter_dyn()), Some("Quadrat"));
}

#[test]
fn groesste_dyn_of_nothing_is_none() {
    assert_eq!(groesste_dyn(&[]), None);
}

// Deutsch: Zwei gleich grosse Formen, das Quadrat zuerst. Bei Gleichstand
// gewinnt die erste, und ein Rumpf mit `max_by_key` faellt hier auf, weil der
// bei Gleichstand die letzte nimmt.
// English: two shapes of equal size, the square first. On a tie the first one
// wins, and a body with `max_by_key` shows up here, because that one takes the
// last on a tie.
#[test]
fn groesste_dyn_keeps_the_first_one_on_a_tie() {
    let gleich: Vec<Box<dyn Flaeche>> = vec![
        Box::new(Quadrat { seite: 6 }),
        Box::new(Rechteck {
            breite: 6,
            hoehe: 6,
        }),
    ];

    assert_eq!(groesste_dyn(&gleich), Some("Quadrat"));
}

#[test]
fn the_finished_function_takes_both_types() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };
    let quadrat = Quadrat { seite: 5 };

    assert_eq!(flaeche_von(&rechteck), 12);
    assert_eq!(flaeche_von(&quadrat), 25);
    assert_eq!(rechteck.name(), "Rechteck");
    assert_eq!(quadrat.name(), "Quadrat");
}
