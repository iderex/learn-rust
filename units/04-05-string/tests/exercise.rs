// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_05_string::{byte_count, char_count, joined, shortened};

#[test]
fn char_count_counts_characters_and_not_bytes() {
    // Deutsch: Dasselbe Wort, zwei Zahlen. Das ist die Aussage der Einheit.
    // English: the same word, two numbers. That is the point of the unit.
    assert_eq!(char_count("Grüße"), 5);
    assert_eq!(byte_count("Grüße"), 7);
}

#[test]
fn char_count_without_umlauts_matches_the_bytes() {
    assert_eq!(char_count("Gruss"), 5);
    assert_eq!(byte_count("Gruss"), 5);
}

#[test]
fn char_count_of_an_empty_text() {
    assert_eq!(char_count(""), 0);
}

#[test]
fn joined_puts_a_space_in_between() {
    assert_eq!(joined("Hallo", "Welt"), "Hallo Welt");
    assert_eq!(joined("Grüße", "Sie"), "Grüße Sie");
}

#[test]
fn joined_with_an_empty_part() {
    assert_eq!(joined("", "Welt"), " Welt");
}

#[test]
fn shortened_counts_characters() {
    // Deutsch: Nach drei Zeichen steht "Grü" da, und das sind vier Bytes. Ein
    // Schnitt nach drei Bytes wäre mitten im Umlaut gelandet.
    // English: after three characters "Grü" stands there, and those are four
    // bytes. A cut after three bytes would have landed inside the umlaut.
    assert_eq!(shortened("Grüße", 3), "Grü");
    assert_eq!(byte_count(&shortened("Grüße", 3)), 4);
}

#[test]
fn shortened_returns_the_whole_text_when_it_is_short() {
    assert_eq!(shortened("Gruß", 10), "Gruß");
    assert_eq!(shortened("Grüße", 0), "");
}
