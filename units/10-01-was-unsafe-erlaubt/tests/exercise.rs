// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_01_was_unsafe_erlaubt::{adresse, erstes_und_letztes, lesen, teilen};

#[test]
fn the_finished_function_builds_a_pointer_without_unsafe() {
    let wert = 7;
    let zeiger = adresse(&wert);

    assert_eq!(unsafe { *zeiger }, 7);
}

#[test]
fn lesen_gives_back_what_the_pointer_points_at() {
    let wert = 42;

    // Deutsch: Der Zeiger zeigt auf `wert`, der hier noch lebt. Das ist die
    // Zusage, die die Funktion vom Aufrufer verlangt.
    // English: the pointer points at `wert`, which is still alive here. That is
    // the promise the function asks of the caller.
    assert_eq!(unsafe { lesen(adresse(&wert)) }, 42);
}

#[test]
fn lesen_works_for_a_pointer_into_a_slice() {
    let zahlen = [1, 2, 3];
    let zeiger = &zahlen[2] as *const i32;

    assert_eq!(unsafe { lesen(zeiger) }, 3);
}

#[test]
fn teilen_hands_out_two_halves_that_can_both_be_changed() {
    let mut zahlen = [1, 2, 3, 4, 5, 6];
    let (links, rechts) = teilen(&mut zahlen, 2);

    links[0] = 10;
    rechts[0] = 30;

    assert_eq!(links, [10, 2]);
    assert_eq!(rechts, [30, 4, 5, 6]);
    assert_eq!(zahlen, [10, 2, 30, 4, 5, 6]);
}

#[test]
fn teilen_at_the_ends_gives_one_empty_half() {
    let mut zahlen = [1, 2, 3];

    let (links, rechts) = teilen(&mut zahlen, 0);
    assert!(links.is_empty());
    assert_eq!(rechts, [1, 2, 3]);

    let (links, rechts) = teilen(&mut zahlen, 3);
    assert_eq!(links, [1, 2, 3]);
    assert!(rechts.is_empty());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn teilen_past_the_end_stops_instead_of_reading_wrongly() {
    let mut zahlen = [1, 2, 3];

    let _ = teilen(&mut zahlen, 4);
}

#[test]
fn erstes_und_letztes_hands_out_both_ends() {
    let mut zahlen = [1, 2, 3, 4];

    let (erstes, letztes) = erstes_und_letztes(&mut zahlen).unwrap();
    *erstes = 10;
    *letztes = 40;

    assert_eq!(zahlen, [10, 2, 3, 40]);
}

#[test]
fn erstes_und_letztes_of_two_elements_hands_out_both_of_them() {
    let mut zahlen = [1, 2];

    let (erstes, letztes) = erstes_und_letztes(&mut zahlen).unwrap();
    *erstes += 1;
    *letztes += 1;

    assert_eq!(zahlen, [2, 3]);
}

#[test]
fn erstes_und_letztes_needs_at_least_two_elements() {
    let mut eines = [1];
    let mut keines: [i32; 0] = [];

    assert!(erstes_und_letztes(&mut eines).is_none());
    assert!(erstes_und_letztes(&mut keines).is_none());
}
