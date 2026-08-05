// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_04_02_module::texte::shouted;
use unit_04_02_module::zahlen::intern::rounded_down;
use unit_04_02_module::zahlen::{doubled, summed};

#[test]
fn doubled_lives_in_the_module_zahlen() {
    assert_eq!(doubled(21), 42);
    assert_eq!(doubled(-3), -6);
}

#[test]
fn rounded_down_lives_one_level_deeper() {
    assert_eq!(rounded_down(47), 40);
    assert_eq!(rounded_down(40), 40);
    assert_eq!(rounded_down(7), 0);
}

#[test]
fn shouted_lives_in_the_other_branch() {
    assert_eq!(shouted("hallo"), "HALLO!");
    assert_eq!(shouted(""), "!");
}

#[test]
fn the_path_through_the_tree_is_what_the_test_names() {
    // Deutsch: Der Test nennt Pfade und keine Dateien. Dass `doubled` in
    // `src/zahlen.rs` steht und `rounded_down` in `src/zahlen/intern.rs`, sieht
    // er nicht.
    // English: the test names paths and not files. That `doubled` stands in
    // `src/zahlen.rs` and `rounded_down` in `src/zahlen/intern.rs` is nothing it
    // sees.
    assert_eq!(unit_04_02_module::zahlen::doubled(2), 4);
    assert_eq!(unit_04_02_module::zahlen::intern::rounded_down(19), 10);
}

#[test]
fn the_finished_function_reaches_upwards() {
    // Deutsch: `summed` steht in `zahlen`, `summed_twice` in `zahlen::intern`
    // und ruft es über `super` auf. Beide stehen fertig da.
    // English: `summed` stands in `zahlen`, `summed_twice` in `zahlen::intern`
    // and calls it through `super`. Both stand there finished.
    assert_eq!(summed(20, 22), 42);
    assert_eq!(unit_04_02_module::zahlen::intern::summed_twice(20, 22), 84);
}
