// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::rc::Rc;

use unit_07_02_rc::{Node, chain, depth, owners, root_name};

#[test]
fn chain_of_nothing_is_nothing() {
    assert!(chain(&[]).is_none());
}

#[test]
fn chain_hands_back_the_last_node() {
    let letzter = chain(&["a", "b", "c"]).expect("drei Namen ergeben eine Kette");

    assert_eq!(letzter.name, "c");
}

#[test]
fn chain_hangs_every_name_under_the_one_before_it() {
    let letzter = chain(&["a", "b", "c"]).expect("drei Namen ergeben eine Kette");

    let mitte = letzter.eltern.as_ref().expect("c hat Eltern");
    assert_eq!(mitte.name, "b");

    let wurzel = mitte.eltern.as_ref().expect("b hat Eltern");
    assert_eq!(wurzel.name, "a");
    assert!(wurzel.eltern.is_none());
}

#[test]
fn chain_of_one_name_is_the_root_itself() {
    let einziger = chain(&["a"]).expect("ein Name ergibt eine Kette");

    assert_eq!(einziger.name, "a");
    assert!(einziger.eltern.is_none());
    assert_eq!(depth(&einziger), 1);
}

#[test]
fn depth_counts_the_node_itself() {
    let letzter = chain(&["a", "b", "c"]).expect("drei Namen ergeben eine Kette");

    assert_eq!(depth(&letzter), 3);

    let mitte = Rc::clone(letzter.eltern.as_ref().expect("c hat Eltern"));
    assert_eq!(depth(&mitte), 2);
}

#[test]
fn root_name_walks_all_the_way_up() {
    let letzter = chain(&["a", "b", "c"]).expect("drei Namen ergeben eine Kette");

    assert_eq!(root_name(&letzter), "a");
}

#[test]
fn root_name_of_the_root_is_its_own_name() {
    let einziger = chain(&["a"]).expect("ein Name ergibt eine Kette");

    assert_eq!(root_name(&einziger), "a");
}

// Deutsch: Das ist der Punkt von Rc. Zwei Kinder zeigen auf dieselbe Wurzel,
// ohne sie zu kopieren, und die Wurzel weiss, dass sie drei Besitzer hat: sich
// selbst als `wurzel` und je einen in den beiden Kindern.
// English: this is the point of Rc. Two children point at the same root without
// copying it, and the root knows it has three owners: itself as `wurzel` and one
// in each of the two children.
#[test]
fn two_children_share_one_parent_without_copying_it() {
    let wurzel = chain(&["wurzel"]).expect("ein Name ergibt eine Kette");
    assert_eq!(owners(&wurzel), 1);

    let links = Rc::new(Node {
        name: String::from("links"),
        eltern: Some(Rc::clone(&wurzel)),
    });
    let rechts = Rc::new(Node {
        name: String::from("rechts"),
        eltern: Some(Rc::clone(&wurzel)),
    });

    assert_eq!(owners(&wurzel), 3);

    let ueber_links = links.eltern.as_ref().expect("links hat Eltern");
    let ueber_rechts = rechts.eltern.as_ref().expect("rechts hat Eltern");
    assert!(Rc::ptr_eq(ueber_links, ueber_rechts));

    assert_eq!(depth(&links), 2);
    assert_eq!(depth(&rechts), 2);
}

#[test]
fn the_finished_function_counts_up_and_down_again() {
    let wert = Rc::new(String::from("Ada"));
    assert_eq!(owners(&wert), 1);

    {
        let zweiter = Rc::clone(&wert);
        assert_eq!(owners(&zweiter), 2);
    }

    assert_eq!(owners(&wert), 1);
}
