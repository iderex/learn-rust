// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_02_assoziierte_typen::{Buchstaben, Quelle, Woerter, Zaehler, einsammeln};

#[test]
fn the_finished_source_counts_up_and_then_stops() {
    let mut zaehler = Zaehler::neu(2);

    assert_eq!(zaehler.naechstes(), Some(1));
    assert_eq!(zaehler.naechstes(), Some(2));
    assert_eq!(zaehler.naechstes(), None);
    assert_eq!(zaehler.naechstes(), None);
}

#[test]
fn buchstaben_hands_out_one_character_after_another() {
    let mut buchstaben = Buchstaben::neu("los");

    assert_eq!(buchstaben.naechstes(), Some('l'));
    assert_eq!(buchstaben.naechstes(), Some('o'));
    assert_eq!(buchstaben.naechstes(), Some('s'));
    assert_eq!(buchstaben.naechstes(), None);
}

#[test]
fn buchstaben_of_an_empty_word_hands_out_nothing() {
    let mut buchstaben = Buchstaben::neu("");

    assert_eq!(buchstaben.naechstes(), None);
}

#[test]
fn buchstaben_steps_over_a_character_of_two_bytes() {
    // Deutsch: `ä` belegt zwei Bytes. Wer die Stelle um eins weiterschiebt,
    // steht neben der Zeichengrenze.
    // English: `ä` takes two bytes. Whoever moves the position on by one stands
    // beside the character boundary.
    let mut buchstaben = Buchstaben::neu("bär");

    assert_eq!(buchstaben.naechstes(), Some('b'));
    assert_eq!(buchstaben.naechstes(), Some('ä'));
    assert_eq!(buchstaben.naechstes(), Some('r'));
    assert_eq!(buchstaben.naechstes(), None);
}

#[test]
fn woerter_hands_out_one_word_after_another() {
    let mut woerter = Woerter::neu("eins zwei drei");

    assert_eq!(woerter.naechstes(), Some(String::from("eins")));
    assert_eq!(woerter.naechstes(), Some(String::from("zwei")));
    assert_eq!(woerter.naechstes(), Some(String::from("drei")));
    assert_eq!(woerter.naechstes(), None);
}

#[test]
fn woerter_does_not_make_a_word_out_of_several_spaces() {
    let mut woerter = Woerter::neu("  eins   zwei  ");

    assert_eq!(woerter.naechstes(), Some(String::from("eins")));
    assert_eq!(woerter.naechstes(), Some(String::from("zwei")));
    assert_eq!(woerter.naechstes(), None);
}

#[test]
fn woerter_of_an_empty_sentence_hands_out_nothing() {
    let mut leer = Woerter::neu("");
    let mut nur_leerzeichen = Woerter::neu("   ");

    assert_eq!(leer.naechstes(), None);
    assert_eq!(nur_leerzeichen.naechstes(), None);
}

#[test]
fn einsammeln_gathers_the_numbers_of_a_counter() {
    let mut zaehler = Zaehler::neu(4);

    assert_eq!(einsammeln(&mut zaehler), vec![1, 2, 3, 4]);
}

#[test]
fn the_same_function_gathers_characters_and_words() {
    // Deutsch: Ein Aufruf, zwei gelieferte Typen. Der Typ steht in der
    // Implementierung und nicht am Aufruf.
    // English: one function, two delivered types. The type stands in the
    // implementation and not at the call.
    let mut buchstaben = Buchstaben::neu("los");
    let mut woerter = Woerter::neu("eins zwei");

    assert_eq!(einsammeln(&mut buchstaben), vec!['l', 'o', 's']);
    assert_eq!(
        einsammeln(&mut woerter),
        vec![String::from("eins"), String::from("zwei")]
    );
}

#[test]
fn einsammeln_of_a_used_up_source_is_empty() {
    let mut zaehler = Zaehler::neu(0);
    let leer: Vec<u32> = Vec::new();

    assert_eq!(einsammeln(&mut zaehler), leer);
}
