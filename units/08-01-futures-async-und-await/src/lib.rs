//! 08-01 Futures, async und await, noch ohne Laufzeitumgebung / Futures, async
//! and await, still without a runtime
//!
//! Deutsch: Ein Future ist ein Wert, den jemand fragen muss. Wird er nicht
//! gefragt, geschieht nichts, und `async` allein ändert daran nichts.
//!
//! English: a future is a value somebody has to ask. Where it is not asked,
//! nothing happens, and `async` on its own changes none of that.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Ein Future, der beim ersten Fragen schon fertig ist.
///
/// A future that is finished at the first asking.
pub struct Sofort {
    pub wert: u32,
}

impl Sofort {
    /// Ein Future, der `wert` herausgibt, sobald er gefragt wird.
    ///
    /// Dieser Future steht fertig da und ist das Muster für die erste Aufgabe.
    /// Der Doku-Test fragt ihn von Hand, also ohne alles, was sonst eine
    /// Laufzeitumgebung tut: einmal anheften, einen Kontext bauen, `poll`
    /// aufrufen. Mehr ist an der Schnittstelle nicht dran.
    ///
    /// A future handing out `wert` as soon as it is asked.
    ///
    /// This future stands there finished and is the model for the first
    /// exercise. The doc test asks it by hand, meaning without everything a
    /// runtime otherwise does: pin it once, build a context, call `poll`. There
    /// is no more to the interface than that.
    ///
    /// ```
    /// use std::future::Future;
    /// use std::pin::pin;
    /// use std::task::{Context, Poll, Waker};
    /// use unit_08_01_futures_async_und_await::Sofort;
    ///
    /// let mut future = pin!(Sofort::neu(7));
    /// let mut kontext = Context::from_waker(Waker::noop());
    ///
    /// assert_eq!(future.as_mut().poll(&mut kontext), Poll::Ready(7));
    /// ```
    pub fn neu(wert: u32) -> Self {
        Sofort { wert }
    }
}

impl Future for Sofort {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<u32> {
        Poll::Ready(self.wert)
    }
}

/// Ein Future, der erst nach mehreren Fragen fertig ist.
///
/// A future that is finished only after several askings.
pub struct Wartet {
    pub offen: u32,
    pub gefragt: u32,
}

impl Wartet {
    /// Ein Future, der `offen` mal noch nicht fertig ist.
    ///
    /// A future that is not finished `offen` times.
    pub fn neu(offen: u32) -> Self {
        Wartet { offen, gefragt: 0 }
    }
}

/// Aufgabe 1: Mach aus `Wartet` einen Future, der sich Zeit lässt.
///
/// Jeder Aufruf von `poll` zählt `gefragt` um eins hoch. Ist `offen` bei null,
/// ist der Future fertig und gibt `Poll::Ready` mit der Zahl der Fragen. Sonst
/// wird `offen` um eins kleiner, und die Antwort ist `Poll::Pending`.
///
/// Vor dem `Pending` wird der Wecker geweckt, also
/// `kontext.waker().wake_by_ref()`. Ohne das darf ein Antreiber, der zwischen
/// zwei Fragen schläft, für immer schlafen: er hat gesagt bekommen, dass später
/// etwas kommt, und niemand hat ihn geweckt. Der Antreiber der zweiten Aufgabe
/// schläft nicht und merkt den Unterschied deshalb nicht, was den Fehler
/// gefährlich macht und nicht harmlos.
///
/// Exercise 1: make `Wartet` into a future that takes its time.
///
/// Every call of `poll` counts `gefragt` up by one. Where `offen` is at zero
/// the future is finished and gives `Poll::Ready` with the number of askings.
/// Otherwise `offen` gets smaller by one and the answer is `Poll::Pending`.
///
/// Before the `Pending` the waker is woken, meaning
/// `kontext.waker().wake_by_ref()`. Without it a driver that sleeps between two
/// askings may sleep forever: it was told something comes later, and nobody
/// woke it. The driver of the second exercise does not sleep and therefore does
/// not notice the difference, which makes the mistake dangerous rather than
/// harmless.
impl Future for Wartet {
    type Output = u32;

    // Deutsch: Das `mut` steht schon da, weil der gelöste Rumpf `gefragt` und
    // `offen` ändert. Solange der Rumpf `todo!()` ist, ändert er nichts, und
    // ohne diese Zeile wäre der Prüflauf über `units/` deshalb rot.
    // English: the `mut` already stands there because the solved body changes
    // `gefragt` and `offen`. While the body is `todo!()` it changes nothing, and
    // without this line the check run over `units/` would therefore be red.
    #[allow(unused_mut)]
    fn poll(mut self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<u32> {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

/// Aufgabe 2: Treib einen Future an, bis er fertig ist.
///
/// Das ist die kleinste Laufzeitumgebung, die es gibt. Der Future wird mit
/// `std::pin::pin!` angeheftet, ein `Context` wird aus `Waker::noop()` gebaut,
/// und dann wird gefragt, bis `Poll::Ready` kommt. Der Wert daraus ist das
/// Ergebnis.
///
/// Zwischen zwei Fragen wird hier nicht geschlafen, es wird sofort wieder
/// gefragt. Eine echte Laufzeitumgebung legt sich schlafen und lässt sich vom
/// Wecker holen; das ist der Unterschied zwischen dieser Funktion und `tokio`,
/// und er ist der Grund, warum diese Funktion einen Kern voll auslastet.
///
/// Exercise 2: drive a future until it is finished.
///
/// This is the smallest runtime there is. The future is pinned with
/// `std::pin::pin!`, a `Context` is built from `Waker::noop()`, and then it is
/// asked until `Poll::Ready` comes. The value out of it is the result.
///
/// Between two askings nothing sleeps here, it is asked again straight away. A
/// real runtime goes to sleep and lets the waker fetch it; that is the
/// difference between this function and `tokio`, and it is why this function
/// keeps one core busy.
pub fn antreiben<F: Future>(future: F) -> F::Output {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Schreib eine `async`-Funktion, die man ansieht, ob sie lief.
///
/// Sie zählt `zaehler` um eins hoch und wartet danach mit `.await` auf
/// `wartet`. Ihr Ergebnis ist das Ergebnis von `wartet`.
///
/// Der Zähler ist da, damit die Behauptung dieser Einheit prüfbar wird. Wer die
/// Funktion aufruft, bekommt einen Future und sonst nichts: der Zähler steht
/// danach immer noch auf demselben Wert. Erst `antreiben` bringt den Rumpf zum
/// Laufen.
///
/// Exercise 3: write an `async` function you can tell has run.
///
/// It counts `zaehler` up by one and then waits for `wartet` with `.await`. Its
/// result is the result of `wartet`.
///
/// The counter is there so the claim of this unit can be checked. Whoever calls
/// the function gets a future and nothing else: the counter still stands at the
/// same value afterwards. Only `antreiben` gets the body running.
pub async fn arbeit(zaehler: &Cell<u32>, wartet: Wartet) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
