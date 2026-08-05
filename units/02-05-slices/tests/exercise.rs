// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_02_05_slices::{first_word, sum_of, without_first};

#[test]
fn first_word_takes_the_part_before_the_space() {
    assert_eq!(first_word("hallo welt"), "hallo");
    assert_eq!(first_word("a b c"), "a");
}

#[test]
fn first_word_without_a_space_is_the_whole_text() {
    assert_eq!(first_word("hallo"), "hallo");
    assert_eq!(first_word(""), "");
}

#[test]
fn first_word_copies_nothing() {
    let text = String::from("hallo welt");

    let wort = first_word(&text);

    assert_eq!(wort, "hallo");

    // Deutsch: Dieselbe Adresse heißt, dass der Slice in den Text hineinzeigt
    // und keine Kopie angelegt wurde.
    // English: the same address means the slice points into the text and no
    // copy was made.
    assert_eq!(wort.as_ptr(), text.as_ptr());
}

#[test]
fn without_first_drops_the_first_number() {
    assert_eq!(without_first(&[1, 2, 3]), [2, 3]);
    assert_eq!(without_first(&[7]), []);
}

#[test]
fn without_first_copies_nothing() {
    let zahlen = [1, 2, 3];

    let rest = without_first(&zahlen);

    assert_eq!(rest, [2, 3]);
    assert_eq!(rest.as_ptr(), zahlen[1..].as_ptr());
}

#[test]
fn sum_of_adds_the_numbers() {
    assert_eq!(sum_of(&[1, 2, 3]), 6);
    assert_eq!(sum_of(&[]), 0);
}

#[test]
fn sum_of_takes_a_whole_array_and_a_part_of_one() {
    let zahlen = [1, 2, 3, 4];

    assert_eq!(sum_of(&zahlen), 10);
    assert_eq!(sum_of(&zahlen[1..3]), 5);
}
