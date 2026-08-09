// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_05_fortgeschrittene_typen::{
    abbruch, beschreibe, erstes_und_letztes, zahl_oder_abbruch, zusammen,
};

#[test]
fn zusammen_setzt_die_teile_mit_einem_leerzeichen_zusammen() {
    assert_eq!(zusammen("Guten", "Tag"), Ok(String::from("Guten Tag")));
}

#[test]
fn zusammen_weist_einen_leeren_ersten_teil_zurueck() {
    assert_eq!(zusammen("", "Tag"), Err(String::from("ein Teil ist leer")));
}

#[test]
fn zusammen_weist_einen_leeren_zweiten_teil_zurueck() {
    assert_eq!(
        zusammen("Guten", ""),
        Err(String::from("ein Teil ist leer"))
    );
}

// Deutsch: Der Rueckgabetyp ist ueber den Alias angeschrieben. Dass die
// Zuweisung an ein ausgeschriebenes Result geht, ist der ganze Punkt eines
// Alias: Es ist derselbe Typ und nicht ein zweiter.
// English: the return type is written down through the alias. That the
// assignment to a written-out Result works is the whole point of an alias: it is
// the same type and not a second one.
#[test]
fn der_alias_ist_derselbe_typ_wie_das_ausgeschriebene_result() {
    let ergebnis: Result<String, String> = zusammen("eins", "zwei");

    assert_eq!(ergebnis, Ok(String::from("eins zwei")));
}

#[test]
#[should_panic(expected = "Abbruch: keine Zahl")]
fn abbruch_nennt_den_grund() {
    abbruch("keine Zahl");
}

// Deutsch: Diese Funktion steht fertig da, und der Weg ueber Ok ruft den
// Abbruch nicht auf.
// English: this function stands there finished, and the way through Ok does not
// call the abort.
#[test]
fn zahl_oder_abbruch_liest_eine_zahl() {
    assert_eq!(zahl_oder_abbruch("12"), 12);
    assert_eq!(zahl_oder_abbruch("0"), 0);
}

#[test]
#[should_panic(expected = "Abbruch: zwoelf")]
fn zahl_oder_abbruch_bricht_bei_text_ab() {
    zahl_oder_abbruch("zwoelf");
}

#[test]
fn erstes_und_letztes_von_nichts_ist_nichts() {
    assert_eq!(erstes_und_letztes(&[]), None);
}

#[test]
fn erstes_und_letztes_bei_einem_element_ist_zweimal_dasselbe() {
    assert_eq!(erstes_und_letztes(&[7]), Some((7, 7)));
}

// Deutsch: Ein Rumpf, der das zweite statt des letzten Elements nimmt, faellt
// hier auf, und einer, der zweimal das erste nimmt, ebenso.
// English: a body taking the second instead of the last element shows up here,
// and one taking the first twice does as well.
#[test]
fn erstes_und_letztes_nimmt_die_beiden_enden() {
    assert_eq!(erstes_und_letztes(&[10, 20, 30]), Some((10, 30)));
    assert_eq!(erstes_und_letztes(&[-1, 0, 0, 5]), Some((-1, 5)));
}

// Deutsch: Der fertige Beschreiber nimmt Typen ohne feste Groesse an und einen
// mit fester Groesse ebenso.
// English: the finished describer takes types without a fixed size and one with
// a fixed size alike.
#[test]
fn der_fertige_beschreiber_nimmt_auch_typen_ohne_feste_groesse() {
    assert_eq!(beschreibe("hallo"), "\"hallo\"");
    assert_eq!(beschreibe(&[1, 2, 3][..]), "[1, 2, 3]");
    assert_eq!(beschreibe(&7), "7");
}
