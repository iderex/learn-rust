// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::sync::mpsc;
use std::thread;

use unit_07_06_kanaele::{drain, echo, merge_two, send_all};

#[test]
fn send_all_keeps_the_order_of_the_input() {
    assert_eq!(send_all(vec![1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(send_all(vec![7]), vec![7]);
}

#[test]
fn send_all_of_nothing_is_nothing() {
    assert_eq!(send_all(Vec::new()), Vec::<i32>::new());
}

// Deutsch: Ein Rumpf, der nur den ersten Wert entgegennimmt, kommt durch die
// Liste mit einem Eintrag und faellt hier auf.
// English: a body taking only the first value gets through the list with one
// entry and shows up here.
#[test]
fn send_all_takes_everything_and_not_only_the_first() {
    assert_eq!(send_all(vec![10, 20, 30, 40, 50]).len(), 5);
}

// Deutsch: Die Reihenfolge zweier Faeden ist nicht festgelegt, deshalb wird
// sortiert verglichen. Was festgelegt ist, ist die Menge.
// English: the order of two threads is not fixed, which is why the comparison is
// made sorted. What is fixed is the set.
#[test]
fn merge_two_brings_every_value_of_both_lists() {
    let mut heraus = merge_two(vec![1, 2, 3], vec![4, 5]);
    heraus.sort_unstable();

    assert_eq!(heraus, vec![1, 2, 3, 4, 5]);
}

#[test]
fn merge_two_works_with_an_empty_side() {
    let mut heraus = merge_two(vec![1, 2], Vec::new());
    heraus.sort_unstable();
    assert_eq!(heraus, vec![1, 2]);

    let mut andersherum = merge_two(Vec::new(), vec![3]);
    andersherum.sort_unstable();
    assert_eq!(andersherum, vec![3]);

    assert_eq!(merge_two(Vec::new(), Vec::new()), Vec::<i32>::new());
}

// Deutsch: Gleiche Werte in beiden Listen bleiben zwei Werte. Ein Rumpf, der
// nach dem Sammeln doppelte wegwirft, faellt hier auf.
// English: equal values in both lists stay two values. A body throwing duplicates
// away after collecting shows up here.
#[test]
fn merge_two_keeps_a_value_that_stands_in_both_lists_twice() {
    let mut heraus = merge_two(vec![1, 1], vec![1]);
    heraus.sort_unstable();

    assert_eq!(heraus, vec![1, 1, 1]);
}

#[test]
fn drain_takes_what_was_sent_before() {
    let (sender, empfaenger) = mpsc::channel();
    for wert in [1, 2, 3] {
        sender.send(wert).expect("der Empfaenger lebt noch");
    }
    drop(sender);

    assert_eq!(drain(empfaenger), vec![1, 2, 3]);
}

#[test]
fn drain_of_a_channel_nobody_used_is_nothing() {
    let (sender, empfaenger) = mpsc::channel::<i32>();
    drop(sender);

    assert_eq!(drain(empfaenger), Vec::<i32>::new());
}

// Deutsch: Hier faellt der Sender erst spaeter weg, in einem eigenen Faden.
// `drain` muss also warten und darf nicht sofort mit einer leeren Liste
// zurueckkommen.
// English: here the sender falls away only later, in a thread of its own.
// `drain` therefore has to wait and may not come back with an empty list right
// away.
#[test]
fn drain_waits_until_the_last_sender_is_gone() {
    let (sender, empfaenger) = mpsc::channel();

    thread::spawn(move || {
        for wert in [1, 2, 3, 4] {
            sender.send(wert).expect("der Empfaenger lebt noch");
        }
    });

    assert_eq!(drain(empfaenger), vec![1, 2, 3, 4]);
}

#[test]
fn the_finished_function_sends_and_takes_in_one_thread() {
    assert_eq!(echo(42), 42);
    assert_eq!(echo(-1), -1);
}
