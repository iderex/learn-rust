// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_06_01_argumente_von_der_kommandozeile::{Aufruf, answer, parse, program_name, usage};

/// Baut eine Argumentliste, wie `std::env::args().collect()` sie gibt.
///
/// Builds an argument list the way `std::env::args().collect()` gives one.
fn args(teile: &[&str]) -> Vec<String> {
    teile.iter().map(|teil| teil.to_string()).collect()
}

#[test]
fn parse_reads_the_pattern_and_the_file() {
    assert_eq!(
        parse(&args(&["suchen", "wort", "buch.txt"])),
        Ok(Aufruf {
            muster: String::from("wort"),
            datei: String::from("buch.txt"),
        })
    );
}

#[test]
fn parse_names_both_missing_arguments() {
    assert_eq!(
        parse(&args(&["suchen"])),
        Err(String::from("es fehlen das Muster und die Datei"))
    );
}

#[test]
fn parse_of_an_empty_list_names_both_as_well() {
    // Deutsch: Ohne den Namen des Programms fehlen dieselben zwei Angaben.
    // English: without the name of the program the same two are missing.
    assert_eq!(
        parse(&args(&[])),
        Err(String::from("es fehlen das Muster und die Datei"))
    );
}

#[test]
fn parse_names_the_missing_file() {
    assert_eq!(
        parse(&args(&["suchen", "wort"])),
        Err(String::from("es fehlt die Datei"))
    );
}

#[test]
fn parse_says_how_many_arguments_are_too_many() {
    assert_eq!(
        parse(&args(&["suchen", "wort", "buch.txt", "noch.txt"])),
        Err(String::from("es ist 1 Argument zu viel"))
    );
    assert_eq!(
        parse(&args(&["suchen", "wort", "buch.txt", "a", "b", "c"])),
        Err(String::from("es sind 3 Argumente zu viel"))
    );
}

#[test]
fn usage_shows_the_name_and_both_places() {
    assert_eq!(usage("suchen"), "Aufruf: suchen <Muster> <Datei>");
    assert_eq!(usage("zaehlen"), "Aufruf: zaehlen <Muster> <Datei>");
}

#[test]
fn answer_to_a_right_call_is_none() {
    assert_eq!(answer(&args(&["suchen", "wort", "buch.txt"])), None);
}

#[test]
fn answer_carries_the_reason_and_the_usage_line() {
    assert_eq!(
        answer(&args(&["target/debug/suchen", "wort"])),
        Some(String::from(
            "es fehlt die Datei\nAufruf: suchen <Muster> <Datei>"
        ))
    );
}

#[test]
fn answer_uses_the_name_the_program_was_called_under() {
    // Deutsch: Der Pfad steht im ersten Argument, in der Meldung steht er nicht.
    // English: the path stands in the first argument, in the message it does
    // not.
    let antwort = answer(&args(&["/usr/local/bin/zaehlen"])).unwrap();

    assert_eq!(
        antwort,
        "es fehlen das Muster und die Datei\nAufruf: zaehlen <Muster> <Datei>"
    );
}

#[test]
fn the_finished_function_cuts_the_path_off() {
    assert_eq!(program_name(&args(&["target/debug/suchen"])), "suchen");
    assert_eq!(program_name(&args(&["suchen"])), "suchen");
    assert_eq!(program_name(&args(&[])), "");
}
