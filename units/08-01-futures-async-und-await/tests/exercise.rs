// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::cell::Cell;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use unit_08_01_futures_async_und_await::{Sofort, Wartet, antreiben, arbeit};

#[test]
fn the_finished_future_is_ready_at_the_first_asking() {
    let mut future = pin!(Sofort::neu(7));
    let mut kontext = Context::from_waker(Waker::noop());

    assert_eq!(future.as_mut().poll(&mut kontext), Poll::Ready(7));
}

#[test]
fn wartet_says_pending_until_it_has_been_asked_often_enough() {
    let mut future = pin!(Wartet::neu(2));
    let mut kontext = Context::from_waker(Waker::noop());

    assert_eq!(future.as_mut().poll(&mut kontext), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut kontext), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut kontext), Poll::Ready(3));
}

#[test]
fn wartet_with_nothing_open_is_ready_at_the_first_asking() {
    let mut future = pin!(Wartet::neu(0));
    let mut kontext = Context::from_waker(Waker::noop());

    assert_eq!(future.as_mut().poll(&mut kontext), Poll::Ready(1));
}

#[test]
fn antreiben_gives_the_value_of_a_future_that_is_ready_at_once() {
    assert_eq!(antreiben(Sofort::neu(41)), 41);
}

#[test]
fn antreiben_keeps_asking_until_the_future_is_finished() {
    // Deutsch: Zwei offene Runden, also drei Fragen bis zum Ergebnis.
    // English: two open rounds, meaning three askings until the result.
    assert_eq!(antreiben(Wartet::neu(2)), 3);
    assert_eq!(antreiben(Wartet::neu(5)), 6);
}

#[test]
fn a_future_that_nobody_asks_does_nothing() {
    let zaehler = Cell::new(0);
    let future = arbeit(&zaehler, Wartet::neu(1));

    // Deutsch: Der Rumpf von `arbeit` ist noch nicht gelaufen.
    // English: the body of `arbeit` has not run yet.
    assert_eq!(zaehler.get(), 0);

    let ergebnis = antreiben(future);

    assert_eq!(zaehler.get(), 1);
    assert_eq!(ergebnis, 2);
}

#[test]
fn arbeit_hands_the_result_of_the_awaited_future_on() {
    let zaehler = Cell::new(0);

    assert_eq!(antreiben(arbeit(&zaehler, Wartet::neu(0))), 1);
    assert_eq!(antreiben(arbeit(&zaehler, Wartet::neu(3))), 4);
    assert_eq!(zaehler.get(), 2);
}

#[test]
fn dropping_a_future_undriven_leaves_the_counter_alone() {
    // Deutsch: Der Future wird gebaut und fallen gelassen. Nichts davon läuft.
    // English: the future is built and dropped. None of it runs.
    let zaehler = Cell::new(0);

    drop(arbeit(&zaehler, Wartet::neu(1)));

    assert_eq!(zaehler.get(), 0);
}
