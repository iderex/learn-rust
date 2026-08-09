// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_07_miri::{erhoehen_ueber_zeiger, lese, summe_ueber_zeiger, tauschen};

#[test]
fn summe_ueber_zeiger_adds_every_value() {
    assert_eq!(summe_ueber_zeiger(&[1, 2, 3]), 6);
    assert_eq!(summe_ueber_zeiger(&[-4, 4]), 0);
    assert_eq!(summe_ueber_zeiger(&[7]), 7);
}

// Deutsch: Aus einer leeren Liste kommt 0, und der Zeiger darf dabei kein
// einziges Mal gelesen werden. Unter Miri faellt ein Rumpf auf, der es doch
// tut.
// English: out of an empty list comes 0, and the pointer may not be read a
// single time while doing it. Under Miri a body doing it anyway shows up.
#[test]
fn summe_ueber_zeiger_of_nothing_is_zero() {
    assert_eq!(summe_ueber_zeiger(&[]), 0);
}

#[test]
fn tauschen_swaps_the_two_values() {
    let mut links = 3;
    let mut rechts = 8;

    tauschen(&mut links, &mut rechts);

    assert_eq!(links, 8);
    assert_eq!(rechts, 3);
}

#[test]
fn tauschen_twice_is_where_it_started() {
    let mut links = 3;
    let mut rechts = 8;

    tauschen(&mut links, &mut rechts);
    tauschen(&mut links, &mut rechts);

    assert_eq!(links, 3);
    assert_eq!(rechts, 8);
}

#[test]
fn erhoehen_ueber_zeiger_raises_every_value() {
    let mut werte = [1, 2, 3];

    erhoehen_ueber_zeiger(&mut werte, 10);

    assert_eq!(werte, [11, 12, 13]);
}

#[test]
fn erhoehen_ueber_zeiger_of_nothing_does_nothing() {
    let mut leer: [i64; 0] = [];

    erhoehen_ueber_zeiger(&mut leer, 10);

    assert_eq!(leer, []);
}

// Deutsch: Die letzte Stelle ist noch drin, die naechste nicht mehr. Genau an
// dieser Grenze haengt der ganze `unsafe`-Block dieser Einheit.
// English: the last place is still inside, the next one is not. The whole
// `unsafe` block of this unit hangs on exactly that boundary.
#[test]
fn the_finished_function_stops_at_the_bound() {
    assert_eq!(lese(&[7, 8, 9], 0), Some(7));
    assert_eq!(lese(&[7, 8, 9], 2), Some(9));
    assert_eq!(lese(&[7, 8, 9], 3), None);
    assert_eq!(lese(&[], 0), None);
}
