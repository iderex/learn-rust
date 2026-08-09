//! 07-07 Mutex und Arc / Mutex and Arc, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-07-mutex-und-arc/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-07-mutex-und-arc/README.md`. What
//! is here is only the bodies that turn the unit's tests green.

use std::sync::{Arc, Mutex};
use std::thread;

/// Ein neuer Zähler, der bei null steht und geteilt werden kann.
///
/// A new counter standing at zero that can be shared.
pub fn neuer_zaehler() -> Arc<Mutex<usize>> {
    Arc::new(Mutex::new(0))
}

/// Erhöht den Zähler um `um`.
///
/// Raises the counter by `um`.
///
/// ```
/// use std::sync::Arc;
/// use std::thread;
/// use unit_07_07_mutex_und_arc::{erhoehen, neuer_zaehler};
///
/// let zaehler = neuer_zaehler();
/// let mut fertig = Vec::new();
///
/// for _ in 0..4 {
///     let meiner = Arc::clone(&zaehler);
///     fertig.push(thread::spawn(move || {
///         for _ in 0..1000 {
///             erhoehen(&meiner, 1);
///         }
///     }));
/// }
///
/// for faden in fertig {
///     faden.join().unwrap();
/// }
///
/// assert_eq!(*zaehler.lock().unwrap(), 4000);
/// assert_eq!(Arc::strong_count(&zaehler), 1);
/// ```
pub fn erhoehen(zaehler: &Mutex<usize>, um: usize) {
    let mut stand = zaehler.lock().unwrap();
    *stand += um;
}

/// Zählt mit mehreren Fäden auf einen gemeinsamen Stand.
///
/// Counts onto one shared total with several threads.
pub fn zaehlen(faeden: usize, pro_faden: usize) -> usize {
    let zaehler = neuer_zaehler();
    let mut fertig = Vec::new();

    for _ in 0..faeden {
        let meiner = Arc::clone(&zaehler);
        fertig.push(thread::spawn(move || {
            for _ in 0..pro_faden {
                erhoehen(&meiner, 1);
            }
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    let stand = zaehler.lock().unwrap();
    *stand
}

/// Sammelt die Quadrate ein, jedes in seinem eigenen Faden.
///
/// Collects the squares, each in a thread of its own.
pub fn einsammeln(werte: Vec<u64>) -> Vec<u64> {
    let gesammelt = Arc::new(Mutex::new(Vec::new()));
    let mut fertig = Vec::new();

    for wert in werte {
        let meine = Arc::clone(&gesammelt);
        fertig.push(thread::spawn(move || {
            let quadrat = wert * wert;
            meine.lock().unwrap().push(quadrat);
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    let gesammelt = gesammelt.lock().unwrap();
    gesammelt.clone()
}

/// Findet den größten Wert, verteilt auf mehrere Fäden.
///
/// Finds the largest value, spread over several threads.
pub fn hoechste(werte: Vec<i64>, faeden: usize) -> Option<i64> {
    if werte.is_empty() {
        return None;
    }

    let faeden = faeden.max(1);
    let werte = Arc::new(werte);
    let ergebnis: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
    let mut fertig = Vec::new();

    for start in 0..faeden {
        let meine = Arc::clone(&werte);
        let ergebnis = Arc::clone(&ergebnis);
        fertig.push(thread::spawn(move || {
            let Some(meins) = meine.iter().skip(start).step_by(faeden).copied().max() else {
                return;
            };
            // Deutsch: Vergleichen und Schreiben stehen unter derselben Wache.
            // Zwei getrennte `lock` liessen dazwischen eine Luecke.
            // English: comparing and writing stand under the same guard. Two
            // separate `lock` calls would leave a gap between them.
            let mut stand = ergebnis.lock().unwrap();
            match *stand {
                Some(bisher) if bisher >= meins => {}
                _ => *stand = Some(meins),
            }
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    let stand = ergebnis.lock().unwrap();
    *stand
}
