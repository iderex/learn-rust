// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_08_from_into_und_fragezeichen::{EingabeFehler, Fehler, as_fehler, divided_text};

#[test]
fn from_wraps_the_other_error() {
    let umgewandelt = Fehler::from(EingabeFehler::KeineZahl);

    assert_eq!(umgewandelt, Fehler::Eingabe(EingabeFehler::KeineZahl));
}

#[test]
fn divided_text_in_the_good_case() {
    assert_eq!(divided_text("10", "2"), Ok(5));
    assert_eq!(divided_text(" 9 ", "3"), Ok(3));
}

#[test]
fn divided_text_passes_the_reading_error_on() {
    // Deutsch: Hereingekommen ist ein `EingabeFehler`, herausgekommen ist ein
    // `Fehler`. Umgewandelt hat das `?` mit `From`.
    // English: what came in was an `EingabeFehler`, what came out is a
    // `Fehler`. The `?` converted it with `From`.
    assert_eq!(
        divided_text("zehn", "2"),
        Err(Fehler::Eingabe(EingabeFehler::KeineZahl))
    );
    assert_eq!(
        divided_text("10", "zwei"),
        Err(Fehler::Eingabe(EingabeFehler::KeineZahl))
    );
}

#[test]
fn divided_text_has_an_error_of_its_own() {
    assert_eq!(divided_text("10", "0"), Err(Fehler::DurchNull));
}

#[test]
fn as_fehler_uses_the_conversion_from_the_other_side() {
    assert_eq!(
        as_fehler(EingabeFehler::KeineZahl),
        Fehler::Eingabe(EingabeFehler::KeineZahl)
    );
}
