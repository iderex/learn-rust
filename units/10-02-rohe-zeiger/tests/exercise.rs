// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_02_rohe_zeiger::{adresse_von, ersetzen, lies, zeigen_auf_dasselbe};

#[test]
fn adresse_von_zeigt_auf_den_wert() {
    let zahl = 5;
    let zeiger = adresse_von(&zahl);

    assert!(!zeiger.is_null());

    // Sicher, weil: `zeiger` kommt aus einer Referenz auf `zahl`, und `zahl`
    // lebt bis zum Ende dieses Tests.
    // Safe because: `zeiger` comes from a reference to `zahl`, and `zahl` lives
    // until the end of this test.
    assert_eq!(unsafe { *zeiger }, 5);
}

#[test]
fn lies_gibt_den_wert_hinter_dem_zeiger_heraus() {
    let zahl = 42;

    // Sicher, weil: Der Zeiger kommt aus einer Referenz auf `zahl`, ist damit
    // nicht null, ausgerichtet und gueltig, und `zahl` lebt bis zum Ende.
    // Safe because: the pointer comes from a reference to `zahl`, is therefore
    // not null, aligned and valid, and `zahl` lives until the end.
    assert_eq!(unsafe { lies(adresse_von(&zahl)) }, 42);
}

// Deutsch: Ein Rumpf, der eine feste Zahl zurueckgibt, faellt hier auf, denn
// zwei verschiedene Werte koennen nicht dieselbe Antwort haben.
// English: a body returning a fixed number shows up here, because two different
// values cannot have the same answer.
#[test]
fn lies_haengt_am_zeiger_und_nicht_an_der_funktion() {
    let eins = 1;
    let zwei = 2;

    let gelesen_eins = unsafe { lies(adresse_von(&eins)) };
    let gelesen_zwei = unsafe { lies(adresse_von(&zwei)) };

    assert_eq!(gelesen_eins, 1);
    assert_eq!(gelesen_zwei, 2);
}

#[test]
fn ersetzen_schreibt_den_neuen_wert() {
    let mut wert = 1;

    // Sicher, weil: Der Zeiger kommt aus einer veraenderbaren Referenz auf
    // `wert`, und waehrend des Aufrufs greift nichts sonst darauf zu.
    // Safe because: the pointer comes from a mutable reference to `wert`, and
    // during the call nothing else reaches it.
    unsafe {
        ersetzen(&mut wert, 2);
    }

    assert_eq!(wert, 2);
}

// Deutsch: Ein Rumpf, der den neuen Wert zurueckgibt, faellt hier auf. Der alte
// muss geholt werden, bevor geschrieben wird.
// English: a body returning the new value shows up here. The old one has to be
// fetched before the write happens.
#[test]
fn ersetzen_gibt_den_alten_wert_heraus() {
    let mut wert = 7;

    let alt = unsafe { ersetzen(&mut wert, 9) };

    assert_eq!(alt, 7);
    assert_eq!(wert, 9);
}

#[test]
fn ersetzen_geht_zweimal_hintereinander() {
    let mut wert = 1;
    let zeiger: *mut i32 = &mut wert;

    // Sicher, weil: Der Zeiger kommt aus einer veraenderbaren Referenz auf
    // `wert`, und waehrend der Aufrufe greift nichts sonst darauf zu.
    // Safe because: the pointer comes from a mutable reference to `wert`, and
    // during the calls nothing else reaches it.
    let (erst, dann) = unsafe { (ersetzen(zeiger, 2), ersetzen(zeiger, 3)) };

    assert_eq!(erst, 1);
    assert_eq!(dann, 2);
    assert_eq!(wert, 3);
}

#[test]
fn zeigen_auf_dasselbe_erkennt_dieselbe_stelle() {
    let zahl = 5;
    let zeiger = adresse_von(&zahl);

    assert!(zeigen_auf_dasselbe(zeiger, zeiger));
    assert!(zeigen_auf_dasselbe(adresse_von(&zahl), adresse_von(&zahl)));
}

// Deutsch: Ein Rumpf, der immer true sagt, faellt hier auf. Zwei Werte mit
// derselben Zahl liegen an zwei Stellen.
// English: a body always saying true shows up here. Two values carrying the
// same number lie at two places.
#[test]
fn zeigen_auf_dasselbe_erkennt_zwei_stellen() {
    let eine = 5;
    let andere = 5;

    assert!(!zeigen_auf_dasselbe(
        adresse_von(&eine),
        adresse_von(&andere)
    ));
}
