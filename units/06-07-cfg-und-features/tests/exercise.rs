// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
//
// Sie hat drei Sorten von Tests. Die ersten gelten in beiden Uebersetzungen.
// Danach kommen die, die nur ohne das Feature uebersetzt werden, und zuletzt
// die, die nur mit ihm uebersetzt werden. Welche Sorte gerade laeuft, sagt der
// erste Test.
//
// English: this file is the unit's only test file. The solution of the same name
// includes exactly this file and runs against the same tests.
//
// It has three kinds of test. The first ones hold in both compilations. After
// them come the ones compiled only without the feature, and last the ones
// compiled only with it. Which kind is running is said by the first test.
use unit_06_07_cfg_und_features::{
    Bericht, bericht, beschreibung, eingebaute_teile, neuer_bericht, zusammenfassung_an,
};

#[test]
fn zusammenfassung_an_sagt_dasselbe_wie_das_makro() {
    assert_eq!(zusammenfassung_an(), cfg!(feature = "zusammenfassung"));
}

#[test]
fn der_kern_ist_immer_eingebaut() {
    assert_eq!(eingebaute_teile()[0], "kern");
}

#[test]
fn eingebaute_teile_haengt_am_feature() {
    let teile = eingebaute_teile();

    assert_eq!(
        teile.contains(&"zusammenfassung"),
        zusammenfassung_an(),
        "die Liste der Teile passt nicht zum Feature"
    );
}

// Deutsch: Der Anfang des Berichts haengt nicht am Feature. Diese Zusicherung
// gilt in beiden Uebersetzungen und faengt einen Rumpf ab, der nur eine der
// beiden Fassungen richtig hinschreibt.
// English: the start of the report does not hang on the feature. This assertion
// holds in both compilations and catches a body that gets only one of the two
// versions right.
#[test]
fn der_bericht_faengt_immer_mit_den_zeilen_an() {
    assert!(bericht(&["eins", "zwei"]).starts_with("eins\nzwei"));
    assert!(bericht(&["nur eine"]).starts_with("nur eine"));
}

#[test]
fn die_beschreibung_haengt_am_feature() {
    let erwartet = if zusammenfassung_an() {
        "Bericht mit Zusammenfassung"
    } else {
        "Bericht ohne Zusammenfassung"
    };

    assert_eq!(beschreibung(), erwartet);
}

#[test]
fn der_bericht_als_struktur_behaelt_die_zeilen_und_ihre_reihenfolge() {
    let gebaut = neuer_bericht(&["drei", "eins", "zwei"]);

    assert_eq!(gebaut.zeilen, vec!["drei", "eins", "zwei"]);
}

#[test]
fn der_bericht_als_struktur_vertraegt_die_leere_liste() {
    assert!(neuer_bericht(&[]).zeilen.is_empty());
}

// Deutsch: Ab hier die Tests, die nur ohne das Feature uebersetzt werden.
// English: from here the tests that are compiled only without the feature.

#[cfg(not(feature = "zusammenfassung"))]
#[test]
fn ohne_feature_hat_der_bericht_keine_letzte_zeile() {
    assert_eq!(bericht(&["eins", "zwei"]), "eins\nzwei");
    assert_eq!(bericht(&[]), "");
}

#[cfg(not(feature = "zusammenfassung"))]
#[test]
fn ohne_feature_gibt_es_nur_den_kern() {
    assert_eq!(eingebaute_teile(), vec!["kern"]);
}

#[cfg(not(feature = "zusammenfassung"))]
#[test]
fn ohne_feature_hat_die_struktur_nur_die_zeilen() {
    assert_eq!(
        neuer_bericht(&["eins"]),
        Bericht {
            zeilen: vec![String::from("eins")],
        }
    );
}

// Deutsch: Ab hier die Tests, die nur mit dem Feature uebersetzt werden. Der
// Prueflauf aus CONTRIBUTING.md baut die Fassung ohne das Feature und laesst
// diese drei deshalb aus. Wie sie laufen, steht in der README.
// English: from here the tests compiled only with the feature. The check run
// from CONTRIBUTING.md builds the version without the feature and therefore
// leaves these three out. How they run stands in the README.

#[cfg(feature = "zusammenfassung")]
#[test]
fn mit_feature_endet_der_bericht_auf_die_anzahl() {
    assert_eq!(bericht(&["eins", "zwei"]), "eins\nzwei\nZeilen: 2");
    assert_eq!(bericht(&[]), "Zeilen: 0");
}

#[cfg(feature = "zusammenfassung")]
#[test]
fn mit_feature_kommt_die_zusammenfassung_dazu() {
    assert_eq!(eingebaute_teile(), vec!["kern", "zusammenfassung"]);
}

#[cfg(feature = "zusammenfassung")]
#[test]
fn mit_feature_traegt_die_struktur_die_anzahl() {
    assert_eq!(
        neuer_bericht(&["eins", "zwei"]),
        Bericht {
            zeilen: vec![String::from("eins"), String::from("zwei")],
            anzahl: 2,
        }
    );
}
