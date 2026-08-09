// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_04_varianz::{Notiz, ersetzen, kuerzer, kuerzeste, laenge_unter, laengere};

#[test]
fn laengere_nimmt_den_laengeren() {
    assert_eq!(laengere("kurz", "viel laenger"), "viel laenger");
    assert_eq!(laengere("viel laenger", "kurz"), "viel laenger");
}

#[test]
fn laengere_nimmt_bei_gleicher_laenge_den_ersten() {
    assert_eq!(laengere("eins", "zwei"), "eins");
}

#[test]
fn laengere_bringt_zwei_lebenszeiten_auf_eine() {
    // Deutsch: Der erste Text lebt ewig, der zweite nur bis zum Ende dieses
    // Tests. Dass beide in dieselbe Signatur passen, ist die Kovarianz.
    // English: the first text lives forever, the second only to the end of this
    // test. That both fit into the same signature is covariance.
    let ewig: &'static str = "eine ewig lebende Zeile";
    let kurz = String::from("kurz");

    assert_eq!(laengere(ewig, &kurz), "eine ewig lebende Zeile");
    assert_eq!(laengere(&kurz, ewig), "eine ewig lebende Zeile");
}

#[test]
fn ersetzen_gibt_den_alten_text_heraus() {
    let mut notiz = Notiz { text: "alt" };

    assert_eq!(ersetzen(&mut notiz, "neu"), "alt");
}

#[test]
fn ersetzen_schreibt_den_neuen_text() {
    let mut notiz = Notiz { text: "alt" };
    ersetzen(&mut notiz, "neu");

    assert_eq!(notiz, Notiz { text: "neu" });
}

#[test]
fn ersetzen_geht_zweimal_hintereinander() {
    let mut notiz = Notiz { text: "eins" };

    assert_eq!(ersetzen(&mut notiz, "zwei"), "eins");
    assert_eq!(ersetzen(&mut notiz, "drei"), "zwei");
    assert_eq!(notiz.text, "drei");
}

#[test]
fn kuerzeste_nimmt_die_kuerzeste() {
    let notizen = [
        Notiz { text: "mittellang" },
        Notiz { text: "kurz" },
        Notiz {
            text: "eine ganz lange",
        },
    ];

    assert_eq!(kuerzeste(&notizen), Some("kurz"));
}

#[test]
fn kuerzeste_nimmt_bei_gleicher_laenge_die_erste() {
    let notizen = [Notiz { text: "eins" }, Notiz { text: "zwei" }];

    assert_eq!(kuerzeste(&notizen), Some("eins"));
}

#[test]
fn kuerzeste_von_nichts_ist_nichts() {
    assert_eq!(kuerzeste(&[]), None);
}

#[test]
fn kuerzeste_ueberlebt_das_ausleihen_der_liste() {
    // Deutsch: Die Liste wird fallen gelassen, die herausgegebene Referenz gilt
    // weiter. Sie kommt aus der Notiz und nicht aus dem Ausleihen der Liste.
    // English: the list is dropped, the reference given out stays valid. It
    // comes out of the note and not out of borrowing the list.
    let gefunden = {
        let notizen = vec![Notiz { text: "kurz" }, Notiz { text: "laenger" }];
        kuerzeste(&notizen)
    };

    assert_eq!(gefunden, Some("kurz"));
}

#[test]
fn die_fertigen_funktionen_zeigen_dieselbe_form() {
    fn zusammen<'a>(eine: Notiz<'a>, andere: Notiz<'a>) -> usize {
        eine.text.len() + andere.text.len()
    }

    let text = String::from("kurz");

    assert_eq!(
        zusammen(kuerzer(Notiz { text: "ewig" }), Notiz { text: &text }),
        8
    );
    assert_eq!(laenge_unter(str::len), 4);
}
