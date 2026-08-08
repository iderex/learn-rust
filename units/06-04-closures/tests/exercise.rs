// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_06_04_closures::{apply, apply_twice, for_each_even, make_adder};

// Deutsch: `mal zwei plus eins` ist mit Absicht nicht vertauschbar. Ein Rumpf,
// der nur einmal anwendet oder die Reihenfolge dreht, kommt hier nicht durch.
// English: `times two plus one` is deliberately not interchangeable. A body that
// applies only once, or turns the order around, does not get through here.
fn mal_zwei_plus_eins(zahl: i32) -> i32 {
    zahl * 2 + 1
}

#[test]
fn apply_twice_applies_twice_and_not_once() {
    assert_eq!(apply_twice(mal_zwei_plus_eins, 1), 7);
    assert_eq!(apply_twice(mal_zwei_plus_eins, 0), 3);
}

#[test]
fn apply_twice_feeds_the_first_result_into_the_second_call() {
    // Deutsch: Zweimal anwenden ist nicht dasselbe wie zweimal ausrechnen und
    // zusammenzaehlen. 1 ergibt 7, nicht 6.
    // English: applying twice is not the same as computing twice and adding up.
    // 1 gives 7, not 6.
    assert_ne!(
        apply_twice(mal_zwei_plus_eins, 1),
        mal_zwei_plus_eins(1) + mal_zwei_plus_eins(1)
    );
}

#[test]
fn apply_twice_takes_a_closure_that_captures() {
    let summand = 10;

    assert_eq!(apply_twice(|zahl| zahl + summand, 1), 21);
}

#[test]
fn for_each_even_reports_the_even_ones_in_order() {
    let mut gesehen = Vec::new();

    for_each_even(&[1, 2, 3, 4, 6, 7], |zahl| gesehen.push(zahl));

    assert_eq!(gesehen, vec![2, 4, 6]);
}

#[test]
fn for_each_even_counts_zero_and_the_negative_ones_as_even() {
    let mut gesehen = Vec::new();

    for_each_even(&[-4, -3, 0, 5], |zahl| gesehen.push(zahl));

    assert_eq!(gesehen, vec![-4, 0]);
}

#[test]
fn for_each_even_reports_nothing_without_an_even_number() {
    let mut aufrufe = 0;

    for_each_even(&[1, 3, 5], |_| aufrufe += 1);
    for_each_even(&[], |_| aufrufe += 1);

    assert_eq!(aufrufe, 0);
}

#[test]
fn make_adder_adds_its_own_summand() {
    let plus_drei = make_adder(3);
    let plus_null = make_adder(0);

    assert_eq!(plus_drei(4), 7);
    assert_eq!(plus_null(4), 4);
}

// Deutsch: Zwei Closures aus derselben Funktion halten verschiedene Werte fest.
// Ein Rumpf, der den Summanden irgendwo teilt, faellt hier auf.
// English: two closures out of the same function hold different values. A body
// that shares the summand somewhere shows up here.
#[test]
fn two_adders_do_not_share_their_summand() {
    let plus_eins = make_adder(1);
    let plus_hundert = make_adder(100);

    assert_eq!(plus_eins(0), 1);
    assert_eq!(plus_hundert(0), 100);
    assert_eq!(plus_eins(0), 1);
}

// Deutsch: Der Rueckgabetyp ist `Fn` und nicht `FnOnce`, also darf dieselbe
// Closure mehrmals aufgerufen werden.
// English: the return type is `Fn` and not `FnOnce`, so the same closure may be
// called more than once.
#[test]
fn an_adder_can_be_called_more_than_once() {
    let plus_zwei = make_adder(2);

    assert_eq!(plus_zwei(1), 3);
    assert_eq!(plus_zwei(1), 3);
    assert_eq!(plus_zwei(plus_zwei(1)), 5);
}

#[test]
fn the_finished_function_applies_once() {
    assert_eq!(apply(|zahl| zahl + 1, 41), 42);
    assert_eq!(apply(mal_zwei_plus_eins, 1), 3);
}
