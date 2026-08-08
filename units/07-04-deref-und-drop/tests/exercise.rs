// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_04_deref_und_drop::{Etikett, Karton, Wachhund, length};

#[test]
fn a_karton_hands_its_value_out_through_the_star() {
    let karton = Karton(String::from("Ada"));

    assert_eq!(*karton, "Ada");
}

// Deutsch: Ohne Stern. Der Uebersetzer geht durch `Deref` hindurch, um die
// Methode zu finden, und genau das ist der Sinn von Aufgabe 1.
// English: without a star. The compiler goes through `Deref` to find the method,
// and that is exactly the point of exercise 1.
#[test]
fn a_karton_finds_the_methods_of_its_value() {
    let karton = Karton(String::from("Ada"));

    assert_eq!(karton.len(), 3);
    assert_eq!(karton.to_uppercase(), "ADA");
}

#[test]
fn a_karton_holds_other_types_as_well() {
    let zahl = Karton(41);

    assert_eq!(*zahl + 1, 42);

    let liste = Karton(vec![1, 2, 3]);

    assert_eq!(liste.len(), 3);
    assert_eq!(liste.first(), Some(&1));
}

#[test]
fn length_counts_the_bytes_of_the_text() {
    assert_eq!(length(&Karton(String::from("Ada"))), 3);
    assert_eq!(length(&Karton(String::new())), 0);
}

// Deutsch: Umlaute sind mehr als ein Byte. Ein Rumpf, der Zeichen statt Bytes
// zaehlt, faellt hier auf und sonst nirgends.
// English: umlauts are more than one byte. A body counting characters instead of
// bytes shows up here and nowhere else.
#[test]
fn length_counts_bytes_and_not_characters() {
    assert_eq!(length(&Karton(String::from("grün"))), 5);
}

#[test]
fn a_wachhund_ticks_its_box_at_the_end_of_the_scope() {
    let mut gefallen = false;

    {
        let _hund = Wachhund {
            gefallen: &mut gefallen,
        };
    }

    assert!(gefallen);
}

// Deutsch: Solange der Waechter lebt, ist nichts geschehen. Ein Rumpf, der das
// Kreuz schon beim Anlegen macht, kaeme durch den Test darueber und faellt
// hier auf.
// English: while the guard lives, nothing has happened. A body ticking the box
// already at creation would get through the test above and shows up here.
#[test]
fn a_wachhund_ticks_nothing_while_it_lives() {
    let mut gefallen = false;

    {
        let hund = Wachhund {
            gefallen: &mut gefallen,
        };

        assert!(!*hund.gefallen);
    }

    assert!(gefallen);
}

#[test]
fn a_wachhund_ticks_its_box_on_drop_by_hand_too() {
    let mut gefallen = false;

    let hund = Wachhund {
        gefallen: &mut gefallen,
    };
    drop(hund);

    assert!(gefallen);
}

#[test]
fn the_finished_type_reaches_the_methods_of_str() {
    let etikett = Etikett(String::from("Ada"));

    assert_eq!(etikett.to_uppercase(), "ADA");
    assert_eq!(etikett.len(), 3);
    assert_eq!(&*etikett, "Ada");
}
