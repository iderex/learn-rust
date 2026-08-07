// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_06_08_build_rs::{as_line, colours, contains, longest};

#[test]
fn contains_finds_what_stands_in_the_file() {
    assert!(contains("rot"));
    assert!(contains("tuerkis"));
}

#[test]
fn contains_refuses_what_does_not_stand_there() {
    assert!(!contains("lila"));
    assert!(!contains(""));
}

// Deutsch: Genau verglichen heisst genau. Ein Rumpf, der die Schreibweise
// einebnet, kommt hier nicht durch.
// English: exactly compared means exactly. A body that levels the spelling out
// does not get through here.
#[test]
fn contains_compares_exactly() {
    assert!(!contains("Rot"));
    assert!(!contains("ROT"));
    assert!(!contains(" rot"));
}

#[test]
fn longest_takes_the_longest_name() {
    assert_eq!(longest(), "tuerkis");
}

// Deutsch: Der laengste steht in der Datei nicht vorn. Ein Rumpf, der einfach
// den ersten Eintrag nimmt, faellt hier auf.
// English: the longest one does not stand at the front of the file. A body that
// simply takes the first entry shows up here.
#[test]
fn longest_is_not_simply_the_first_entry() {
    assert_ne!(longest(), colours()[0]);
}

#[test]
fn as_line_joins_every_name_in_the_order_of_the_file() {
    assert_eq!(as_line(), "rot, gruen, blau, gelb, tuerkis");
}

// Deutsch: Kein Komma am Ende. Ein Rumpf, der hinter jeden Namen eines haengt,
// faellt hier auf und sonst nirgends.
// English: no comma at the end. A body that appends one behind every name shows
// up here and nowhere else.
#[test]
fn as_line_puts_no_separator_behind_the_last_name() {
    assert!(!as_line().ends_with(", "));
    assert!(as_line().ends_with("tuerkis"));
}

#[test]
fn the_finished_function_hands_the_generated_list_out() {
    assert_eq!(colours().len(), 5);
    assert_eq!(colours()[0], "rot");
    assert_eq!(colours()[4], "tuerkis");
}
