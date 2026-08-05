// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_02_03_ausleihen::{doubled_through, total_length, vowel_count};

#[test]
fn vowel_count_counts_the_five_vowels() {
    assert_eq!(vowel_count(&String::from("hallo")), 2);
    assert_eq!(vowel_count(&String::from("aeiou")), 5);
}

#[test]
fn vowel_count_of_a_text_without_vowels() {
    assert_eq!(vowel_count(&String::from("rhythm")), 0);
    assert_eq!(vowel_count(&String::new()), 0);
}

#[test]
fn vowel_count_leaves_the_text_with_the_caller() {
    let text = String::from("umlaute");

    assert_eq!(vowel_count(&text), 4);

    // Deutsch: `text` steht noch da, denn übergeben war nur eine Ausleihe.
    // English: `text` is still there, because only a loan was handed over.
    assert_eq!(text, "umlaute");
}

#[test]
fn doubled_through_reads_through_the_reference() {
    assert_eq!(doubled_through(&21), 42);
    assert_eq!(doubled_through(&0), 0);
}

#[test]
fn doubled_through_leaves_the_number_with_the_caller() {
    let zahl = -3;

    assert_eq!(doubled_through(&zahl), -6);
    assert_eq!(zahl, -3);
}

#[test]
fn total_length_adds_two_borrowed_texts() {
    let a = String::from("hallo");
    let b = String::from("welt");

    assert_eq!(total_length(&a, &b), 9);
    assert_eq!(a, "hallo");
    assert_eq!(b, "welt");
}

#[test]
fn total_length_takes_the_same_text_twice() {
    let text = String::from("hallo");

    // Deutsch: Zwei geteilte Ausleihen desselben Werts zur selben Zeit sind
    // erlaubt.
    // English: two shared loans of the same value at the same time are allowed.
    assert_eq!(total_length(&text, &text), 10);
}
