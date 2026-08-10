//! 10-07 Miri / Miri, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/10-07-miri/README.md`. Hier stehen
//! nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/10-07-miri/README.md`. What is here
//! is only the bodies that turn the unit's tests green.

/// Liest ein Element hinter einem rohen Zeiger.
///
/// Reads an element behind a raw pointer.
///
/// ```
/// use unit_10_07_miri::lese;
///
/// assert_eq!(lese(&[7, 8, 9], 0), Some(7));
/// assert_eq!(lese(&[7, 8, 9], 2), Some(9));
/// assert_eq!(lese(&[7, 8, 9], 3), None);
/// assert_eq!(lese(&[], 0), None);
/// ```
pub fn lese(werte: &[i64], stelle: usize) -> Option<i64> {
    if stelle >= werte.len() {
        return None;
    }

    // SAFETY: `stelle` ist kleiner als die Länge, also liegt `add(stelle)` in
    // derselben Zuteilung wie `werte` und zeigt auf ein gültiges `i64`.
    // SAFETY: `stelle` is smaller than the length, so `add(stelle)` lies in the
    // same allocation as `werte` and points at a valid `i64`.
    unsafe { Some(*werte.as_ptr().add(stelle)) }
}

/// Zählt die Werte über einen rohen Zeiger zusammen.
///
/// Adds the values up over a raw pointer.
pub fn summe_ueber_zeiger(werte: &[i64]) -> i64 {
    let anzahl = werte.len();
    let zeiger = werte.as_ptr();
    let mut summe = 0;

    for schritt in 0..anzahl {
        // SAFETY: `schritt` bleibt unter der Länge, also zeigt `add(schritt)`
        // in dieselbe Zuteilung und auf ein gültiges `i64`. Das eine Feld
        // hinter dem letzten wird nie gelesen.
        // SAFETY: `schritt` stays below the length, so `add(schritt)` points
        // into the same allocation and at a valid `i64`. The one slot behind
        // the last is never read.
        summe += unsafe { *zeiger.add(schritt) };
    }

    summe
}

/// Tauscht zwei Werte über rohe Zeiger.
///
/// Swaps two values over raw pointers.
pub fn tauschen(links: &mut i64, rechts: &mut i64) {
    let erster: *mut i64 = links;
    let zweiter: *mut i64 = rechts;

    // SAFETY: Beide Zeiger kommen aus je einer eigenen `&mut`-Referenz. Sie
    // sind gültig, ausgerichtet und zeigen auf verschiedene Stellen, denn zwei
    // `&mut` derselben Stelle kann es nicht geben.
    // SAFETY: both pointers come out of a `&mut` reference of their own. They
    // are valid, aligned and point at different places, because two `&mut` of
    // the same place cannot exist.
    unsafe {
        std::ptr::swap(erster, zweiter);
    }
}

/// Erhöht jeden Wert um `um`, geschrieben über einen rohen Zeiger.
///
/// Raises every value by `um`, written through a raw pointer.
pub fn erhoehen_ueber_zeiger(werte: &mut [i64], um: i64) {
    // Deutsch: Die Länge steht fest, bevor der Zeiger benutzt wird. Was die
    // umgekehrte Reihenfolge unter Miri ergibt, ist gemessen und steht in der
    // README.
    // English: the length is settled before the pointer gets used. What the
    // other order gives under Miri is measured and stands in the README.
    let anzahl = werte.len();
    let zeiger = werte.as_mut_ptr();

    for schritt in 0..anzahl {
        // SAFETY: `schritt` bleibt unter der Länge, also zeigt `add(schritt)`
        // in dieselbe Zuteilung, und zwischen den Schritten greift nichts
        // anderes auf `werte` zu.
        // SAFETY: `schritt` stays below the length, so `add(schritt)` points
        // into the same allocation, and between the steps nothing else reaches
        // for `werte`.
        unsafe {
            *zeiger.add(schritt) += um;
        }
    }
}
