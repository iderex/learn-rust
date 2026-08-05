// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_06_hashmap::{count_of, counted, from_pairs, increment};

#[test]
fn counted_counts_every_word() {
    let karte = counted(&["hallo", "welt", "hallo"]);

    assert_eq!(karte.get("hallo").copied(), Some(2));
    assert_eq!(karte.get("welt").copied(), Some(1));
    assert_eq!(karte.len(), 2);
}

#[test]
fn counted_of_nothing_is_empty() {
    let karte = counted(&[]);

    assert!(karte.is_empty());
}

#[test]
fn count_of_finds_a_word_that_is_there() {
    let karte = from_pairs(&[("hallo", 3)]);

    assert_eq!(count_of(&karte, "hallo"), 3);
}

#[test]
fn count_of_a_missing_word_is_zero() {
    let karte = from_pairs(&[("hallo", 3)]);

    assert_eq!(count_of(&karte, "welt"), 0);

    // Deutsch: Auch in einer leeren Karte fehlt der Schlüssel, ohne Abbruch.
    // English: in an empty map the key is missing as well, without a break.
    assert_eq!(count_of(&from_pairs(&[]), "hallo"), 0);
}

#[test]
fn increment_raises_a_word_that_is_there() {
    let mut karte = from_pairs(&[("hallo", 3)]);

    increment(&mut karte, "hallo");

    assert_eq!(karte.get("hallo").copied(), Some(4));
}

#[test]
fn increment_creates_a_word_that_is_missing() {
    let mut karte = from_pairs(&[("hallo", 3)]);

    increment(&mut karte, "welt");

    assert_eq!(karte.get("welt").copied(), Some(1));
    assert_eq!(karte.len(), 2);
}
