// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_06_ffi_mit_extern_c::{
    Punkt, abstand, betrag, laenge_bis_null, laenge_von_c, punkt_aus_bytes, versatz,
};

#[test]
fn abstand_zaehlt_in_beide_richtungen() {
    assert_eq!(abstand(10, 4), Some(6));
    assert_eq!(abstand(4, 10), Some(6));
}

#[test]
fn abstand_von_einer_zahl_zu_sich_selbst_ist_null() {
    assert_eq!(abstand(7, 7), Some(0));
    assert_eq!(abstand(i32::MIN, i32::MIN), Some(0));
}

#[test]
fn abstand_weist_den_ueberlauf_der_subtraktion_zurueck() {
    // Deutsch: Die Differenz passt in kein i32, also darf sie gar nicht erst
    // gebildet werden.
    // English: the difference fits into no i32, so it may not be formed at all.
    assert_eq!(abstand(i32::MAX, -1), None);
    assert_eq!(abstand(0, i32::MIN), None);

    // Deutsch: Dieser Fall trennt das Prüfen vom Umlaufen. Eine umlaufende
    // Subtraktion liefert hier -2147483647, dessen Betrag sich darstellen lässt,
    // und käme mit einer Antwort heraus statt mit None.
    // English: this case separates checking from wrapping. A wrapping
    // subtraction delivers -2147483647 here, whose magnitude can be
    // represented, and would come out with an answer instead of None.
    assert_eq!(abstand(i32::MAX, -2), None);
    assert_eq!(abstand(-2, i32::MAX), None);
}

#[test]
fn abstand_weist_den_rand_zurueck_den_die_c_zusage_nicht_deckt() {
    // Deutsch: Die Differenz ist genau i32::MIN. Sie passt in ein i32, ihr
    // Betrag passt nicht, und deshalb bekommt abs sie nicht zu sehen.
    // English: the difference is exactly i32::MIN. It fits into an i32, its
    // magnitude does not, and that is why abs never gets to see it.
    assert_eq!(abstand(i32::MIN, 0), None);
    assert_eq!(abstand(-1, i32::MAX), None);
}

#[test]
fn laenge_bis_null_zaehlt_bis_zur_null() {
    assert_eq!(laenge_bis_null(b"hallo\0"), Some(5));
    assert_eq!(laenge_bis_null(b"\0"), Some(0));
}

#[test]
fn laenge_bis_null_zaehlt_nur_bis_zur_ersten_null() {
    assert_eq!(laenge_bis_null(b"ab\0cd\0"), Some(2));
}

#[test]
fn laenge_bis_null_ohne_null_ist_nichts() {
    // Deutsch: Ohne Null könnte strlen nicht aufhören, also wird es nicht
    // gerufen.
    // English: without a zero strlen could not stop, so it is not called.
    assert_eq!(laenge_bis_null(b"hallo"), None);
    assert_eq!(laenge_bis_null(b""), None);
}

#[test]
fn punkt_aus_bytes_liest_beide_felder() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&3i32.to_ne_bytes());
    bytes.extend_from_slice(&(-4i32).to_ne_bytes());

    assert_eq!(punkt_aus_bytes(&bytes), Some(Punkt { x: 3, y: -4 }));
}

#[test]
fn punkt_aus_bytes_liest_die_null() {
    assert_eq!(punkt_aus_bytes(&[0; 8]), Some(Punkt { x: 0, y: 0 }));
}

#[test]
fn punkt_aus_bytes_weist_die_falsche_laenge_zurueck() {
    assert_eq!(punkt_aus_bytes(&[0; 7]), None);
    assert_eq!(punkt_aus_bytes(&[0; 9]), None);
    assert_eq!(punkt_aus_bytes(&[]), None);
}

#[test]
fn die_fertigen_funktionen_zeigen_dieselbe_form() {
    assert_eq!(betrag(-5), Some(5));
    assert_eq!(betrag(i32::MIN), None);

    assert_eq!(laenge_von_c(c"hallo"), 5);

    assert_eq!(versatz(), (0, 4));
}
