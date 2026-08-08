// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_03_refcell::Protokoll;

// Deutsch: Ein Protokoll mit den genannten Zeilen darin.
// English: a log with the named lines in it.
fn protokoll_mit(zeilen: &[&str]) -> Protokoll {
    let protokoll = Protokoll::neu();
    for zeile in zeilen {
        protokoll.notieren(zeile);
    }
    protokoll
}

#[test]
fn notieren_fills_the_log_behind_a_shared_reference() {
    let protokoll = protokoll_mit(&["erste", "zweite"]);

    assert_eq!(*protokoll.zeilen_geliehen(), vec!["erste", "zweite"]);
}

#[test]
fn anzahl_counts_the_lines() {
    assert_eq!(protokoll_mit(&["erste", "zweite", "dritte"]).anzahl(), 3);
}

#[test]
fn anzahl_of_an_empty_log_is_zero() {
    assert_eq!(Protokoll::neu().anzahl(), 0);
}

#[test]
fn letzte_gives_the_line_written_last() {
    assert_eq!(
        protokoll_mit(&["erste", "zweite"]).letzte(),
        Some(String::from("zweite"))
    );
}

#[test]
fn letzte_of_an_empty_log_is_none() {
    assert_eq!(Protokoll::neu().letzte(), None);
}

// Deutsch: Eine Ausleihe, die die Methode ueberlebt, faellt erst hier auf. Der
// Rueckgabewert ist eine Kopie, also ist die Zelle danach wieder frei und die
// naechste Zeile geht hinein.
// English: a borrow outliving the method only shows up here. The returned value
// is a copy, so the cell is free again afterwards and the next line goes in.
#[test]
fn letzte_leaves_the_cell_free_afterwards() {
    let protokoll = protokoll_mit(&["erste"]);

    let gelesen = protokoll.letzte();
    protokoll.notieren("zweite");

    assert_eq!(gelesen, Some(String::from("erste")));
    assert_eq!(protokoll.anzahl(), 2);
}

#[test]
fn notieren_und_zaehlen_returns_the_new_count() {
    let protokoll = protokoll_mit(&["erste"]);

    assert_eq!(protokoll.notieren_und_zaehlen("zweite"), 2);
}

// Deutsch: Das ist der Abbruch, um den es geht. Ein Rumpf, der die Ausleihe aus
// borrow_mut noch haelt und dann anzahl aufruft, leiht dieselbe Zelle zweimal
// und bricht schon beim ersten Aufruf ab. Der zweite Aufruf steht hier, damit
// auch ein Rumpf auffaellt, der die Ausleihe erst spaeter freigibt.
// English: this is the abort it is about. A body still holding the borrow from
// borrow_mut and then calling anzahl borrows the same cell twice and aborts on
// the very first call. The second call stands here so that a body freeing the
// borrow only later shows up as well.
#[test]
fn notieren_und_zaehlen_can_be_called_twice_in_a_row() {
    let protokoll = Protokoll::neu();

    assert_eq!(protokoll.notieren_und_zaehlen("erste"), 1);
    assert_eq!(protokoll.notieren_und_zaehlen("zweite"), 2);
    assert_eq!(protokoll.letzte(), Some(String::from("zweite")));
}

// Deutsch: Der Uebersetzer nimmt zwei borrow_mut an, weil beide nur ein &self
// brauchen. Erst die Zelle zaehlt mit und bricht ab. Genau das ist der
// Unterschied zwischen der Regel beim Uebersetzen und derselben Regel zur
// Laufzeit.
// English: the compiler accepts two borrow_mut, because both need only a &self.
// Only the cell counts along and aborts. That is exactly the difference between
// the rule at compile time and the same rule at run time.
#[test]
#[should_panic(expected = "already borrowed")]
fn two_mutable_borrows_at_runtime_end_in_a_panic() {
    let protokoll = protokoll_mit(&["erste"]);

    protokoll.zwei_veraenderbare_ausleihen();
}

// Deutsch: Dieselbe Regel von der anderen Seite. Eine Ausleihe zum Lesen, die
// noch lebt, verbietet das Schreiben, und auch das faellt erst zur Laufzeit
// auf.
// English: the same rule from the other side. A borrow for reading that is
// still alive forbids writing, and that too only shows up at run time.
#[test]
#[should_panic(expected = "already borrowed")]
fn writing_while_a_read_borrow_is_still_alive_ends_in_a_panic() {
    let protokoll = protokoll_mit(&["erste"]);

    let geliehen = protokoll.zeilen_geliehen();
    protokoll.notieren("zweite");

    drop(geliehen);
}
