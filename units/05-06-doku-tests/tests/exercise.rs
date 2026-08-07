// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_05_06_doku_tests::{initials, longest, percent, word_at, word_count};

#[test]
fn initials_take_the_first_letter_of_every_word() {
    assert_eq!(initials("Ada Lovelace"), "A.L.");
    assert_eq!(initials("grace hopper"), "G.H.");
    assert_eq!(initials("Alan Mathison Turing"), "A.M.T.");
}

#[test]
fn initials_of_nothing_are_nothing() {
    assert_eq!(initials(""), "");
    assert_eq!(initials("   "), "");
}

#[test]
fn percent_reads_a_number_with_whitespace_around_it() {
    assert_eq!(percent(" 42 "), Ok(42));
    assert_eq!(percent("0"), Ok(0));
    assert_eq!(percent("255"), Ok(255));
}

#[test]
fn percent_refuses_what_is_no_u8() {
    assert!(percent("dreiundvierzig").is_err());
    assert!(percent("").is_err());
    // Deutsch: 300 ist eine Zahl, passt aber nicht in ein u8.
    // English: 300 is a number, but it does not fit into a u8.
    assert!(percent("300").is_err());
}

#[test]
fn longest_takes_the_longer_one() {
    assert_eq!(longest("kurz", "laenger"), "laenger");
    assert_eq!(longest("laenger", "kurz"), "laenger");
}

#[test]
fn longest_takes_the_first_one_on_a_tie() {
    let links = String::from("gleich");
    let rechts = String::from("gleich");

    assert!(std::ptr::eq(longest(&links, &rechts), links.as_str()));
}

#[test]
fn the_finished_functions_show_the_same_shape() {
    assert_eq!(word_count("ein zwei drei"), 3);
    assert_eq!(word_count("   "), 0);
    assert_eq!(word_at("ein zwei drei", 1), "zwei");
}

// Deutsch: Dieser Test haelt fest, woran der Abbruch liegt. Ein Doku-Test mit
// `should_panic` kann das nicht, denn dort gibt es kein `expected`.
// English: this test pins down what the abort is about. A doc test with
// `should_panic` cannot, because there is no `expected` there.
#[test]
#[should_panic(expected = "an dieser Stelle steht kein Wort")]
fn word_at_aborts_beyond_the_last_word() {
    word_at("ein", 5);
}
