// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_10_05_drop_check::{
    Spur, eintraege, frueh_fallen_lassen, mit_eigenem_block, neues_buch, reihenfolge,
};

#[test]
fn reihenfolge_ist_umgekehrt_zur_vereinbarung() {
    assert_eq!(reihenfolge(), vec!["drei", "zwei", "eins"]);
}

// Deutsch: Der Unterschied zu einem `Vec` steht mit im Test, denn ein `Vec`
// raeumt von vorne nach hinten auf und gibt deshalb die Reihenfolge der
// Vereinbarung heraus. Wer die drei Spuren in einen `Vec` legt, faellt hier auf.
// English: the difference from a `Vec` is in the test as well, because a `Vec`
// cleans up front to back and therefore gives out the order of declaration.
// Whoever puts the three traces into a `Vec` is caught here.
#[test]
fn reihenfolge_ist_nicht_die_eines_vec() {
    let aus_einem_vec = {
        let buch = neues_buch();
        {
            let _spuren: Vec<Spur> = ["eins", "zwei", "drei"]
                .into_iter()
                .map(|name| Spur::neu(name, &buch))
                .collect();
        }
        eintraege(&buch)
    };

    assert_eq!(aus_einem_vec, vec!["eins", "zwei", "drei"]);
    assert_ne!(reihenfolge(), aus_einem_vec);
}

#[test]
fn ein_eigener_block_zieht_die_mittlere_spur_nach_vorn() {
    assert_eq!(mit_eigenem_block(), vec!["zwei", "drei", "eins"]);
}

#[test]
fn drop_von_hand_zieht_die_erste_spur_nach_vorn() {
    assert_eq!(frueh_fallen_lassen(), vec!["eins", "drei", "zwei"]);
}

// Deutsch: Die drei Aufgaben tragen dieselben drei Namen und unterscheiden sich
// nur in der Reihenfolge. Der Test haelt fest, dass keine zwei von ihnen
// dasselbe herausgeben, denn sonst waere eine davon abgeschrieben.
// English: the three exercises carry the same three names and differ only in
// the order. The test pins down that no two of them give out the same thing,
// because otherwise one of them was copied from another.
#[test]
fn die_drei_aufgaben_geben_drei_verschiedene_reihenfolgen() {
    let eine = reihenfolge();
    let zweite = mit_eigenem_block();
    let dritte = frueh_fallen_lassen();

    assert_ne!(eine, zweite);
    assert_ne!(eine, dritte);
    assert_ne!(zweite, dritte);

    for gefunden in [&eine, &zweite, &dritte] {
        let mut sortiert = gefunden.clone();
        sortiert.sort_unstable();
        assert_eq!(sortiert, vec!["drei", "eins", "zwei"]);
    }
}

// Deutsch: Die fertigen Teile stehen mit im Test, damit der Lauf in der Einheit
// nicht vollstaendig rot ist und man sieht, dass die Datei laeuft. Er haelt
// dazu fest, was der drop check ueberhaupt zu pruefen hat: dass `drop` laeuft
// und dabei das geliehene Buch noch lesen kann.
// English: the finished parts are in the test as well, so that the run inside
// the unit is not red all through and one sees that the file runs. It also pins
// down what the drop check has to check at all: that `drop` runs and can still
// read the borrowed book while doing so.
#[test]
fn die_fertigen_teile_stehen_schon() {
    let buch = neues_buch();

    {
        let eine = Spur::neu("eine", &buch);
        assert_eq!(eine.name(), "eine");
        assert!(eintraege(&buch).is_empty());
    }

    assert_eq!(eintraege(&buch), vec!["eine"]);
}
