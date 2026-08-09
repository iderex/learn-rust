// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_04_newtype_muster::{Kilometer, Liste, Zentimeter, addiere, summe};

// Deutsch: Aus einer Liste von Zahlen die Liste von Laengen, die die Funktionen
// erwarten.
// English: turns a list of numbers into the list of lengths the functions
// expect.
fn laengen(werte: &[u32]) -> Vec<Zentimeter> {
    werte.iter().map(|wert| Zentimeter(*wert)).collect()
}

#[test]
fn the_finished_function_adds_two_lengths() {
    assert_eq!(addiere(Zentimeter(80), Zentimeter(120)), Zentimeter(200));
}

#[test]
fn summe_adds_every_length() {
    assert_eq!(summe(&laengen(&[80, 120, 5])), Zentimeter(205));
}

#[test]
fn summe_of_an_empty_list_is_zero() {
    assert_eq!(summe(&[]), Zentimeter(0));
}

// Deutsch: Der Rueckgabewert ist wieder eingepackt. Ein Rumpf, der ein u32
// zurueckgibt, uebersetzt gar nicht erst, und das ist der Punkt des Musters.
// English: the returned value is wrapped again. A body returning a u32 does not
// even compile, and that is the point of the pattern.
#[test]
fn summe_gives_a_length_and_not_a_number() {
    let ergebnis = summe(&laengen(&[1, 2]));

    assert_eq!(ergebnis.0, 3);
    assert_eq!(ergebnis, Zentimeter(3));
}

#[test]
fn display_writes_the_entries_in_brackets() {
    let liste = Liste(vec![String::from("a"), String::from("b")]);

    assert_eq!(format!("{liste}"), "[a, b]");
}

#[test]
fn display_of_an_empty_list_is_two_brackets() {
    assert_eq!(format!("{}", Liste(Vec::new())), "[]");
}

#[test]
fn display_of_one_entry_has_no_comma() {
    assert_eq!(format!("{}", Liste(vec![String::from("a")])), "[a]");
}

#[test]
fn from_turns_kilometres_into_centimetres() {
    assert_eq!(Zentimeter::from(Kilometer(2)), Zentimeter(200_000));
}

#[test]
fn from_of_zero_stays_zero() {
    assert_eq!(Zentimeter::from(Kilometer(0)), Zentimeter(0));
}

// Deutsch: Wer From schreibt, bekommt into dazu. Der Test steht hier, weil das
// der Grund ist, From zu nehmen und keine eigene Funktion.
// English: whoever writes From gets into along with it. The test stands here
// because that is the reason to take From and not a function of your own.
#[test]
fn into_works_because_from_is_written() {
    let laenge: Zentimeter = Kilometer(3).into();

    assert_eq!(laenge, Zentimeter(300_000));
}
