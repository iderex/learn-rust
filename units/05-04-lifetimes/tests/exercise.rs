// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_04_lifetimes::{Excerpt, first_word, longest, shorter};

#[test]
fn longest_returns_the_longer_text() {
    assert_eq!(longest("kurz", "laenger"), "laenger");
    assert_eq!(longest("ein ganzer Satz", "kurz"), "ein ganzer Satz");
}

#[test]
fn longest_returns_the_right_one_when_both_are_equal() {
    assert_eq!(longest("gleich", "sowohl"), "sowohl");
}

#[test]
fn longest_works_with_a_borrowed_string() {
    // Deutsch: Beide Eingaben tragen denselben Namen `'a`, also darf hier ein
    // geliehener String neben einem festen Text stehen.
    // English: both inputs carry the same name `'a`, so a borrowed String may
    // stand next to a fixed text here.
    let satz = String::from("ein ganzer Satz");
    assert_eq!(longest(&satz, "kurz"), "ein ganzer Satz");
}

#[test]
fn first_word_cuts_at_the_first_space() {
    assert_eq!(first_word("Lebensdauern sind Namen"), "Lebensdauern");
    assert_eq!(first_word("eins zwei drei"), "eins");
}

#[test]
fn first_word_of_a_single_word_is_that_word() {
    assert_eq!(first_word("Lebensdauern"), "Lebensdauern");
    assert_eq!(first_word(""), "");
}

#[test]
fn first_sentence_cuts_before_the_full_stop() {
    let text = String::from("Lebensdauern sind Namen. Mehr sind sie nicht.");
    let auszug = Excerpt::first_sentence(&text);

    assert_eq!(auszug.teil, "Lebensdauern sind Namen");
    assert_eq!(
        auszug,
        Excerpt {
            teil: "Lebensdauern sind Namen"
        }
    );
}

#[test]
fn first_sentence_without_a_full_stop_is_the_whole_text() {
    assert_eq!(Excerpt::first_sentence("ohne Punkt").teil, "ohne Punkt");
}

#[test]
fn the_finished_function_shows_the_same_shape() {
    assert_eq!(shorter("kurz", "laenger"), "kurz");
    assert_eq!(shorter("gleich", "gleich"), "gleich");
}
