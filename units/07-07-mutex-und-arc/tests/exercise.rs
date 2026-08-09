// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::sync::Arc;
use std::thread;
use unit_07_07_mutex_und_arc::{einsammeln, erhoehen, hoechste, neuer_zaehler, zaehlen};

#[test]
fn zaehlen_verliert_keinen_schritt() {
    assert_eq!(zaehlen(4, 1000), 4000);
    assert_eq!(zaehlen(1, 1), 1);
}

#[test]
fn zaehlen_ohne_faeden_oder_ohne_schritte_bleibt_bei_null() {
    assert_eq!(zaehlen(0, 100), 0);
    assert_eq!(zaehlen(4, 0), 0);
}

// Deutsch: Derselbe Aufruf, zwanzig Mal, mit genug Schritten, dass zwei Faeden
// sich wirklich begegnen. Ein Rumpf, der zum Lesen und zum Schreiben zwei
// getrennte `lock` nimmt, gibt hier eine zu kleine Zahl heraus, und zwar nicht
// bei jedem Lauf dieselbe. Dieser Test ist die Stelle, an der die Sperre
// gebraucht wird.
// English: the same call, twenty times over, with enough steps for two threads
// to really meet. A body taking two separate `lock` calls, one to read and one
// to write, gives out too small a number here, and not the same one every run.
// This test is the place where the lock is needed.
#[test]
fn zaehlen_gibt_bei_jedem_lauf_dasselbe() {
    for _ in 0..20 {
        assert_eq!(zaehlen(8, 5000), 40_000);
    }
}

// Deutsch: Der Test sortiert, denn die Reihenfolge, in der die Faeden ihr
// Quadrat eintragen, ist nicht festgelegt.
// English: the test sorts, because the order the threads write their square in
// is not fixed.
#[test]
fn einsammeln_bringt_jedes_quadrat() {
    let mut gefunden = einsammeln(vec![1, 2, 3, 4]);
    gefunden.sort_unstable();

    assert_eq!(gefunden, vec![1, 4, 9, 16]);
}

#[test]
fn einsammeln_laesst_die_leere_liste_leer() {
    assert_eq!(einsammeln(Vec::new()), Vec::<u64>::new());
}

#[test]
fn einsammeln_behaelt_jeden_wert_einzeln() {
    let mut gefunden = einsammeln(vec![3, 3, 0]);
    gefunden.sort_unstable();

    assert_eq!(gefunden, vec![0, 9, 9]);
}

#[test]
fn hoechste_findet_den_groessten_wert() {
    assert_eq!(hoechste(vec![3, 9, 2, 7], 3), Some(9));
    assert_eq!(hoechste(vec![-5, -2, -9], 2), Some(-2));
}

#[test]
fn hoechste_von_nichts_ist_nichts() {
    assert_eq!(hoechste(Vec::new(), 4), None);
}

// Deutsch: Mehr Faeden als Werte ist erlaubt. Die uebrigen Faeden finden
// nichts und tragen deshalb auch nichts ein.
// English: more threads than values is allowed. The remaining threads find
// nothing and therefore write nothing.
#[test]
fn hoechste_vertraegt_mehr_faeden_als_werte() {
    assert_eq!(hoechste(vec![4], 6), Some(4));
}

#[test]
fn hoechste_nimmt_null_faeden_als_einen() {
    assert_eq!(hoechste(vec![1, 5, 3], 0), Some(5));
    assert_eq!(hoechste(vec![1, 5, 3], 1), Some(5));
}

// Deutsch: Wer zum Vergleichen und zum Schreiben zwei getrennte `lock` nimmt,
// laesst dazwischen eine Luecke. Zwanzig Laeufe ueber eine Liste, deren
// groesster Wert ganz hinten steht, treffen diese Luecke.
// English: whoever takes two separate `lock` calls, one to compare and one to
// write, leaves a gap between them. Twenty runs over a list whose largest value
// sits right at the end hit that gap.
#[test]
fn hoechste_gibt_bei_jedem_lauf_dasselbe() {
    let werte: Vec<i64> = (0..2000).collect();

    for _ in 0..20 {
        assert_eq!(hoechste(werte.clone(), 8), Some(1999));
    }
}

// Deutsch: Die fertigen Teile stehen mit im Test, damit der Lauf in der Einheit
// nicht vollstaendig rot ist und man sieht, dass die Datei laeuft.
// English: the finished parts are in the test as well, so that the run inside
// the unit is not red all through and one sees that the file runs.
#[test]
fn die_fertigen_funktionen_stehen_schon() {
    let zaehler = neuer_zaehler();
    assert_eq!(*zaehler.lock().unwrap(), 0);

    let mut fertig = Vec::new();
    for _ in 0..3 {
        let meiner = Arc::clone(&zaehler);
        fertig.push(thread::spawn(move || erhoehen(&meiner, 7)));
    }
    for faden in fertig {
        faden.join().unwrap();
    }

    assert_eq!(*zaehler.lock().unwrap(), 21);
    assert_eq!(Arc::strong_count(&zaehler), 1);
}
