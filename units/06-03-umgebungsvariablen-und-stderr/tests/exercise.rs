// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::env::VarError;
use std::ffi::OsString;

use unit_06_03_umgebungsvariablen_und_stderr::{
    Bericht, aus_der_umgebung, bericht, einstellung, schreiben,
};

// Deutsch: Aus einer Liste von &str wird die Liste von String, die die
// Funktionen erwarten.
// English: turns a list of &str into the list of String the functions expect.
fn zeilen(teile: &[&str]) -> Vec<String> {
    teile.iter().map(|teil| teil.to_string()).collect()
}

// Deutsch: Ein Lauf mit zwei Puffern statt der beiden echten Ausgaenge. Zurueck
// kommen die Bytes, die jeder der beiden Ausgaenge bekommen hat, als Text.
// English: one run with two buffers instead of the two real exits. What comes
// back is the bytes each of the two exits got, as text.
fn lauf(bericht: &Bericht) -> (String, String) {
    let mut aus: Vec<u8> = Vec::new();
    let mut fehler: Vec<u8> = Vec::new();

    schreiben(bericht, &mut aus, &mut fehler).expect("ein Puffer laesst sich beschreiben");

    (
        String::from_utf8(aus).expect("stdout ist Text"),
        String::from_utf8(fehler).expect("stderr ist Text"),
    )
}

#[test]
fn einstellung_gives_the_value_that_is_set() {
    assert_eq!(
        einstellung(Ok(String::from("Birne")), "an"),
        Ok(String::from("Birne"))
    );
}

#[test]
fn einstellung_gives_the_default_when_the_variable_is_missing() {
    assert_eq!(
        einstellung(Err(VarError::NotPresent), "an"),
        Ok(String::from("an"))
    );
}

// Deutsch: Dieser Test haelt den Zweig eng. Ein Rumpf, der jeden Fehler mit der
// Vorgabe beantwortet, kommt durch die anderen Tests und faellt nur hier auf.
// Eine gesetzte Variable mit kaputtem Wert ist keine fehlende Variable.
// English: this test keeps the branch narrow. A body that answers every error
// with the default gets through the other tests and only shows up here. A
// variable that is set with a broken value is not a missing variable.
#[test]
fn einstellung_passes_on_an_error_that_is_not_a_missing_variable() {
    let kaputt = VarError::NotUnicode(OsString::from("kein gueltiger Text"));

    assert_eq!(
        einstellung(Err(kaputt.clone()), "an"),
        Err(kaputt),
        "NotUnicode ist keine fehlende Variable und wird weitergereicht"
    );
}

#[test]
fn bericht_collects_the_matching_lines_in_order() {
    let gefunden = bericht(&zeilen(&["Apfel", "Birne", "Ananas"]), "an");

    assert_eq!(gefunden.treffer, zeilen(&["Ananas"]));
}

#[test]
fn bericht_takes_every_line_that_matches() {
    let gefunden = bericht(&zeilen(&["Ananas", "Banane", "Birne"]), "an");

    assert_eq!(gefunden.treffer, zeilen(&["Ananas", "Banane"]));
}

#[test]
fn bericht_always_says_what_was_searched_for() {
    let gefunden = bericht(&zeilen(&["Ananas"]), "an");

    assert_eq!(gefunden.meldungen, zeilen(&["gesucht wird nach an"]));
}

#[test]
fn bericht_says_so_when_nothing_matched() {
    let gefunden = bericht(&zeilen(&["Apfel", "Birne"]), "Zwetschge");

    assert_eq!(gefunden.treffer, Vec::<String>::new());
    assert_eq!(
        gefunden.meldungen,
        zeilen(&["gesucht wird nach Zwetschge", "kein Treffer"])
    );
}

#[test]
fn schreiben_puts_the_hits_on_the_result_exit() {
    let (aus, _) = lauf(&Bericht {
        treffer: zeilen(&["Ananas", "Banane"]),
        meldungen: zeilen(&["gesucht wird nach an"]),
    });

    assert_eq!(aus, "Ananas\nBanane\n");
}

#[test]
fn schreiben_puts_the_messages_on_the_message_exit() {
    let (_, fehler) = lauf(&Bericht {
        treffer: zeilen(&["Ananas"]),
        meldungen: zeilen(&["gesucht wird nach an", "kein Treffer"]),
    });

    assert_eq!(fehler, "gesucht wird nach an\nkein Treffer\n");
}

// Deutsch: Das ist die Trennung selbst. Ein Rumpf, der beides nach `aus`
// schreibt, besteht die beiden Tests darueber halb und faellt hier auf. Genau
// diese Zeile wuerde sonst in der weitergeleiteten Datei landen.
// English: this is the separation itself. A body writing both to `aus` half
// passes the two tests above and shows up here. Exactly this line would
// otherwise end up in the redirected file.
#[test]
fn schreiben_keeps_the_message_out_of_the_result_exit() {
    let (aus, fehler) = lauf(&Bericht {
        treffer: zeilen(&["Ananas"]),
        meldungen: zeilen(&["gesucht wird nach an"]),
    });

    assert!(
        !aus.contains("gesucht wird nach"),
        "die Meldung gehoert nicht nach stdout, dort stand: {aus:?}"
    );
    assert!(fehler.contains("gesucht wird nach"));
}

#[test]
fn schreiben_writes_nothing_at_all_for_an_empty_report() {
    let (aus, fehler) = lauf(&Bericht {
        treffer: Vec::new(),
        meldungen: Vec::new(),
    });

    assert_eq!(aus, "");
    assert_eq!(fehler, "");
}

#[test]
fn the_finished_function_falls_back_to_the_default() {
    assert_eq!(aus_der_umgebung("LR_06_03_NICHT_GESETZT", "an"), "an");
}
