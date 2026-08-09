// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_06_funktionszeiger::{
    Marke, anwenden, einpacken, negieren, verdoppeln, waehle, zweimal,
};

// Deutsch: Das ist der Test, den die Einheit schuldet: dieselbe Funktion
// bekommt zwei verschiedene Funktionszeiger und antwortet zweimal anders.
// English: this is the test the unit owes: the same function gets two different
// function pointers and answers differently twice.
#[test]
fn anwenden_nimmt_zwei_verschiedene_zeiger() {
    assert_eq!(anwenden(&[1, 2, 3], verdoppeln), vec![2, 4, 6]);
    assert_eq!(anwenden(&[1, 2, 3], negieren), vec![-1, -2, -3]);
}

// Deutsch: Eine Closure ohne Fang ist an dieser Stelle erlaubt, weil sie zu
// einem `fn` wird. Der Test haelt fest, dass der Parameter wirklich der
// Zeigertyp ist und nicht ein generisches `impl Fn`.
// English: a closure without a capture is allowed here, because it turns into an
// `fn`. The test pins down that the parameter really is the pointer type and not
// a generic `impl Fn`.
#[test]
fn anwenden_nimmt_auch_eine_closure_ohne_fang() {
    let um_eins: fn(i32) -> i32 = |x| x + 1;

    assert_eq!(anwenden(&[10, 20], um_eins), vec![11, 21]);
}

#[test]
fn anwenden_laesst_die_leere_liste_leer() {
    assert_eq!(anwenden(&[], verdoppeln), Vec::<i32>::new());
}

#[test]
fn waehle_gibt_zu_jedem_bekannten_namen_den_passenden_zeiger() {
    assert_eq!(waehle("verdoppeln").unwrap()(21), 42);
    assert_eq!(waehle("negieren").unwrap()(21), -21);
}

// Deutsch: Ein unbekannter Name ergibt nichts. Ein Rumpf, der hier auf gut Glueck
// eine der beiden Funktionen herausgibt, kommt nicht durch.
// English: an unknown name gives nothing. A body handing out one of the two
// functions on the off chance does not get through here.
#[test]
fn waehle_kennt_nur_die_beiden_namen() {
    assert!(waehle("nichts").is_none());
    assert!(waehle("").is_none());
    assert!(waehle("Verdoppeln").is_none());
}

// Deutsch: Der herausgegebene Zeiger ist ein Wert und laesst sich weitergeben.
// Er geht deshalb ohne Umweg in `zweimal`.
// English: the pointer handed out is a value and can be passed on. It therefore
// goes into `zweimal` without a detour.
#[test]
fn der_gewaehlte_zeiger_passt_in_zweimal() {
    let f = waehle("verdoppeln").expect("verdoppeln ist bekannt");

    assert_eq!(zweimal(f, 3), 12);
}

#[test]
fn einpacken_macht_aus_jeder_zahl_eine_marke() {
    assert_eq!(einpacken(&[1, 2]), vec![Marke(1), Marke(2)]);
    assert_eq!(einpacken(&[]), Vec::new());
}

#[test]
fn einpacken_behaelt_die_reihenfolge() {
    assert_eq!(einpacken(&[3, 1, 2]), vec![Marke(3), Marke(1), Marke(2)]);
}

// Deutsch: Die drei fertigen Teile stehen mit im Test, damit der Lauf in der
// Einheit nicht vollstaendig rot ist und man sieht, dass die Datei laeuft.
// English: the three finished parts are in the test as well, so that the run
// inside the unit is not red all through and one sees that the file runs.
#[test]
fn die_fertigen_funktionen_stehen_schon() {
    assert_eq!(verdoppeln(3), 6);
    assert_eq!(negieren(3), -3);
    assert_eq!(zweimal(verdoppeln, 3), 12);
}
