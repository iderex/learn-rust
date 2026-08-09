// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::sync::atomic::{AtomicUsize, Ordering};

use unit_07_08_send_sync_und_atomare_typen::{bump, count_up, max_of, only_one_wins};

#[test]
fn count_up_ends_at_threads_times_each() {
    assert_eq!(count_up(4, 1000), 4000);
    assert_eq!(count_up(1, 5), 5);
}

#[test]
fn count_up_of_nothing_stays_at_zero() {
    assert_eq!(count_up(0, 1000), 0);
    assert_eq!(count_up(4, 0), 0);
}

// Deutsch: Viele Faeden, die oft hochzaehlen. Ein Rumpf, der vor dem Lesen nicht
// auf alle wartet, faellt hier auf, und ein Rumpf, der nicht atomar zaehlt,
// koennte hier zu wenig zaehlen.
// English: many threads counting up often. A body that does not wait for all of
// them before reading shows up here, and a body that does not count atomically
// could count too few here.
#[test]
fn count_up_loses_nothing_under_load() {
    assert_eq!(count_up(8, 10_000), 80_000);
}

#[test]
fn max_of_finds_the_largest_value() {
    assert_eq!(max_of(vec![3, 9, 4]), 9);
    assert_eq!(max_of(vec![9, 3, 4]), 9);
    assert_eq!(max_of(vec![7]), 7);
}

#[test]
fn max_of_nothing_is_zero() {
    assert_eq!(max_of(Vec::new()), 0);
}

// Deutsch: Der groesste Wert steht in der Mitte, und es sind genug Werte, dass
// die Faeden sich ueberschneiden.
// English: the largest value stands in the middle, and there are enough values
// for the threads to overlap.
#[test]
fn max_of_finds_the_largest_one_among_many() {
    let mut werte: Vec<usize> = (0..64).collect();
    werte[32] = 1000;

    assert_eq!(max_of(werte), 1000);
}

// Deutsch: Genau einer gewinnt. Dieser Test haelt das fest und beweist nicht,
// dass der Rumpf `compare_exchange` benutzt: Ein Rumpf mit `load` und danach
// `store` kam auf dem Rechner, auf dem gebaut wurde, zehnmal von zehn durch.
// Warum das so ist und was daraus folgt, steht in der README unter "Was diese
// Tests nicht beantworten".
// English: exactly one wins. This test pins that down and does not prove that
// the body uses `compare_exchange`: a body with `load` and then `store` got
// through ten times out of ten on the machine this was built on. Why that is and
// what follows from it is in the README under "What these tests do not answer".
#[test]
fn only_one_wins_no_matter_how_many_reach_for_it() {
    assert_eq!(only_one_wins(1), 1);
    assert_eq!(only_one_wins(2), 1);
    assert_eq!(only_one_wins(8), 1);
    assert_eq!(only_one_wins(64), 1);
}

#[test]
fn only_one_wins_without_a_thread_is_nobody() {
    assert_eq!(only_one_wins(0), 0);
}

#[test]
fn the_finished_function_gives_back_the_value_from_before() {
    let zaehler = AtomicUsize::new(7);

    assert_eq!(bump(&zaehler), 7);
    assert_eq!(bump(&zaehler), 8);
    assert_eq!(zaehler.load(Ordering::Relaxed), 9);
}
