// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_06_05_iteratoren::{Zaehler, erste_ueber, quadrate_der_geraden, verdoppelt};

#[test]
fn the_finished_function_doubles_every_number() {
    assert_eq!(verdoppelt(&[1, 2, 3]), vec![2, 4, 6]);
    assert_eq!(verdoppelt(&[]), Vec::<i32>::new());
}

#[test]
fn quadrate_der_geraden_squares_only_the_even_ones() {
    assert_eq!(quadrate_der_geraden(&[1, 2, 3, 4]), vec![4, 16]);
}

// Deutsch: Erst filtern, dann quadrieren. Wer zuerst quadriert, hat lauter
// gerade Quadrate und kommt auf [4, 16, 36, 64] statt auf [4, 16].
// English: filter first, then square. Whoever squares first has nothing but
// even squares and arrives at [4, 16, 36, 64] instead of [4, 16].
#[test]
fn quadrate_der_geraden_filters_before_it_squares() {
    assert_eq!(
        quadrate_der_geraden(&[1, 2, 3, 4, 5, 6, 7, 8]),
        vec![4, 16, 36, 64]
    );
    assert_eq!(quadrate_der_geraden(&[3, 5, 7]), Vec::<i32>::new());
}

#[test]
fn quadrate_der_geraden_keeps_the_order_of_the_list() {
    assert_eq!(quadrate_der_geraden(&[4, 2, 6]), vec![16, 4, 36]);
}

#[test]
fn quadrate_der_geraden_of_an_empty_list_is_empty() {
    assert_eq!(quadrate_der_geraden(&[]), Vec::<i32>::new());
}

#[test]
fn zaehler_hands_out_one_to_five_and_then_nothing() {
    let mut zaehler = Zaehler::neu();

    assert_eq!(zaehler.next(), Some(1));
    assert_eq!(zaehler.next(), Some(2));
    assert_eq!(zaehler.next(), Some(3));
    assert_eq!(zaehler.next(), Some(4));
    assert_eq!(zaehler.next(), Some(5));
    assert_eq!(zaehler.next(), None);
}

// Deutsch: Einmal None, immer None. Ein Zaehler, der nach dem Ende wieder
// anfaengt, kommt hier nicht durch.
// English: once None, always None. A counter starting over after the end does
// not get through here.
#[test]
fn zaehler_stays_empty_once_it_is_used_up() {
    let mut zaehler = Zaehler::neu();
    for _ in 0..5 {
        zaehler.next();
    }

    assert_eq!(zaehler.next(), None);
    assert_eq!(zaehler.next(), None);
}

// Deutsch: Zwei Zaehler stoeren einander nicht, denn der Stand gehoert zum
// Wert und nicht zum Typ.
// English: two counters do not disturb each other, because the count belongs to
// the value and not to the type.
#[test]
fn two_counters_count_on_their_own() {
    let mut einer = Zaehler::neu();
    let mut anderer = Zaehler::neu();

    assert_eq!(einer.next(), Some(1));
    assert_eq!(einer.next(), Some(2));
    assert_eq!(anderer.next(), Some(1));
}

// Deutsch: Diese Methoden hat niemand fuer Zaehler geschrieben. Sie kommen mit
// dem Trait, und dass sie hier laufen, ist der ganze Grund, es zu erfuellen.
// English: nobody wrote these methods for Zaehler. They come with the trait,
// and that they run here is the whole reason to fulfil it.
#[test]
fn the_trait_brings_its_methods_along() {
    assert_eq!(Zaehler::neu().sum::<u32>(), 15);
    assert_eq!(Zaehler::neu().count(), 5);
    assert_eq!(Zaehler::neu().filter(|zahl| zahl % 2 == 0).count(), 2);
    assert_eq!(
        Zaehler::neu().map(|zahl| zahl * 10).take(2).sum::<u32>(),
        30
    );
}

// Deutsch: Zwei Zaehler nebeneinander, um eins versetzt, ihre Paare
// multipliziert, davon die durch drei teilbaren zusammengezaehlt. Aus (1,2),
// (2,3), (3,4) und (4,5) werden 2, 6, 12 und 20, und uebrig bleiben 6 und 12.
// English: two counters side by side, offset by one, their pairs multiplied,
// and of those the ones divisible by three added up. Out of (1,2), (2,3), (3,4)
// and (4,5) come 2, 6, 12 and 20, and 6 and 12 are what is left.
#[test]
fn the_trait_methods_also_work_with_each_other() {
    let summe: u32 = Zaehler::neu()
        .zip(Zaehler::neu().skip(1))
        .map(|(links, rechts)| links * rechts)
        .filter(|produkt| produkt % 3 == 0)
        .sum();

    assert_eq!(summe, 18);
}

#[test]
fn erste_ueber_finds_the_first_one_and_not_the_largest() {
    assert_eq!(erste_ueber(&[1, 7, 3, 9], 5), Some(7));
}

#[test]
fn erste_ueber_is_none_when_no_number_is_large_enough() {
    assert_eq!(erste_ueber(&[1, 2, 3], 5), None);
    assert_eq!(erste_ueber(&[], 0), None);
}

// Deutsch: Groesser heisst groesser und nicht groesser oder gleich. Die
// Schwelle selbst kommt nicht durch.
// English: greater means greater and not greater or equal. The threshold itself
// does not get through.
#[test]
fn erste_ueber_does_not_take_the_threshold_itself() {
    assert_eq!(erste_ueber(&[5, 6], 5), Some(6));
}
