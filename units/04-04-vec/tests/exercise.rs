// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_04_vec::{built, doubled_all, largest};

#[test]
fn built_counts_up_to_the_limit() {
    assert_eq!(built(3), vec![1, 2, 3]);
    assert_eq!(built(1), vec![1]);
}

#[test]
fn built_of_zero_is_empty() {
    assert_eq!(built(0), Vec::new());
    assert!(built(0).is_empty());
}

#[test]
fn largest_finds_the_biggest_value() {
    assert_eq!(largest(&[3, 9, 4]), Some(9));
    assert_eq!(largest(&[-3, -9]), Some(-3));
}

#[test]
fn largest_of_an_empty_list_is_none() {
    assert_eq!(largest(&[]), None);
}

#[test]
fn doubled_all_doubles_every_value() {
    assert_eq!(doubled_all(&[1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn doubled_all_of_an_empty_list_is_empty() {
    assert_eq!(doubled_all(&[]), Vec::new());
}

#[test]
fn doubled_all_leaves_the_old_list_alone() {
    let zahlen = vec![1, 2, 3];

    let neue = doubled_all(&zahlen);

    assert_eq!(neue, vec![2, 4, 6]);

    // Deutsch: Die alte Liste war nur geliehen und steht unverändert da.
    // English: the old list was only lent and stands there unchanged.
    assert_eq!(zahlen, vec![1, 2, 3]);
}
