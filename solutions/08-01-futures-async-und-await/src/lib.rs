//! 08-01 Futures, async und await, noch ohne Laufzeitumgebung / Futures, async
//! and await, still without a runtime, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/08-01-futures-async-und-await/README.md`. Hier stehen nur die Rümpfe,
//! die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/08-01-futures-async-und-await/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// Ein Future, der beim ersten Fragen schon fertig ist.
///
/// A future that is finished at the first asking.
pub struct Sofort {
    pub wert: u32,
}

impl Sofort {
    /// Ein Future, der `wert` herausgibt, sobald er gefragt wird.
    ///
    /// A future handing out `wert` as soon as it is asked.
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

    fn poll(self: Pin<&mut Self>, _kontext: &mut Context<'_>) -> Poll<u32> {
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

impl Future for Wartet {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<u32> {
        self.gefragt += 1;
        if self.offen == 0 {
            return Poll::Ready(self.gefragt);
        }
        self.offen -= 1;
        kontext.waker().wake_by_ref();
        Poll::Pending
    }
}

/// Treibt einen Future an, bis er fertig ist.
///
/// Drives a future until it is finished.
pub fn antreiben<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut kontext = Context::from_waker(Waker::noop());

    loop {
        if let Poll::Ready(wert) = future.as_mut().poll(&mut kontext) {
            return wert;
        }
    }
}

/// Zählt einen Zähler hoch und wartet danach auf einen Future.
///
/// Counts a counter up and then waits for a future.
pub async fn arbeit(zaehler: &Cell<u32>, wartet: Wartet) -> u32 {
    zaehler.set(zaehler.get() + 1);
    wartet.await
}
