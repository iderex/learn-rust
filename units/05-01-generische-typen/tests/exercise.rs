// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_01_generische_typen::{Paar, last_of, swapped};

#[test]
fn last_of_works_for_numbers() {
    assert_eq!(last_of(&[3, 9, 4]), Some(&4));
    assert_eq!(last_of(&[7]), Some(&7));
}

#[test]
fn last_of_works_for_texts_as_well() {
    // Deutsch: Dieselbe Funktion, ein anderer Typ.
    // English: the same function, a different type.
    assert_eq!(last_of(&["drei", "neun"]), Some(&"neun"));
}

#[test]
fn last_of_an_empty_list_is_none() {
    assert_eq!(last_of::<i32>(&[]), None);
    assert_eq!(last_of::<String>(&[]), None);
}

#[test]
fn new_builds_a_pair_of_numbers() {
    let paar = Paar::new(3, 9);

    assert_eq!(paar.links, 3);
    assert_eq!(paar.rechts, 9);
}

#[test]
fn new_builds_a_pair_of_texts() {
    let paar = Paar::new(String::from("links"), String::from("rechts"));

    assert_eq!(paar.links, "links");
    assert_eq!(paar.rechts, "rechts");
}

#[test]
fn swapped_turns_the_pair_around() {
    let paar = swapped(Paar::new(3, 9));

    assert_eq!(
        paar,
        Paar {
            links: 9,
            rechts: 3
        }
    );
}

#[test]
fn swapped_works_for_texts_too() {
    let paar = swapped(Paar::new(String::from("links"), String::from("rechts")));

    assert_eq!(paar.links, "rechts");
    assert_eq!(paar.rechts, "links");
}
