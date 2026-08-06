// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_07_deklarative_makros::{groesster, quadrat, ueber_hundert, ueber_zehn, vec_von};

#[test]
fn quadrat_rechnet_mit_dem_ganzen_ausdruck() {
    assert_eq!(quadrat!(4), 16);
    assert_eq!(quadrat!(1 + 2), 9);
}

#[test]
fn groesster_bei_einem_wert_ist_dieser_wert() {
    assert_eq!(groesster!(3), 3);
}

#[test]
fn groesster_findet_den_groessten() {
    assert_eq!(groesster!(3, 9, 4), 9);
    assert_eq!(groesster!(9, 3, 4), 9);
    assert_eq!(groesster!(3, 4, 9), 9);
}

// Deutsch: Verglichen wird mit >, also geht das Makro auch mit Texten. Eine
// Funktion mit fester Signatur koennte das so nicht.
// English: comparing happens with >, so the macro goes with texts as well. A
// function with a fixed signature could not do that this way.
#[test]
fn groesster_nimmt_auch_texte() {
    assert_eq!(groesster!("aal", "zebra", "kuh"), "zebra");
}

#[test]
fn groesster_nimmt_ein_abschliessendes_komma() {
    assert_eq!(groesster!(3, 9, 4,), 9);
}

#[test]
fn vec_von_ohne_werte_ist_leer() {
    let leer: Vec<i32> = vec_von![];

    assert!(leer.is_empty());
}

// Deutsch: Ein Rumpf, der die Werte umdreht, faellt hier auf.
// English: a body turning the values around shows up here.
#[test]
fn vec_von_haelt_die_reihenfolge() {
    assert_eq!(vec_von![1, 2, 3], vec![1, 2, 3]);
    assert_eq!(vec_von!["a", "b"], vec!["a", "b"]);
}

#[test]
fn vec_von_nimmt_ein_abschliessendes_komma() {
    assert_eq!(vec_von![1, 2, 3,], vec![1, 2, 3]);
}

#[test]
fn der_erzeugte_pruefer_liegt_ueber_seiner_grenze() {
    assert!(ueber_zehn(11));
    assert!(ueber_hundert(101));
}

// Deutsch: Genau an der Grenze ist die Antwort false. Ein >= statt eines >
// faellt hier auf und sonst nirgends.
// English: at exactly the limit the answer is false. A >= instead of a > shows
// up here and nowhere else.
#[test]
fn der_erzeugte_pruefer_liegt_nicht_auf_seiner_grenze() {
    assert!(!ueber_zehn(10));
    assert!(!ueber_hundert(100));
}

// Deutsch: Zwei Aufrufe desselben Makros ergeben zwei verschiedene Funktionen.
// Der zweite kennt die Grenze des ersten nicht.
// English: two calls of the same macro give two different functions. The second
// does not know the limit of the first.
#[test]
fn die_beiden_pruefer_tragen_verschiedene_grenzen() {
    assert!(ueber_zehn(50));
    assert!(!ueber_hundert(50));
}
