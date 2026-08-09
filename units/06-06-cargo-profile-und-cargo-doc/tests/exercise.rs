// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_06_06_cargo_profile_und_cargo_doc::{half_even, profile_name, sum_checked, sum_wrapping};

#[test]
fn sum_checked_adds_what_fits() {
    assert_eq!(sum_checked(&[1, 2, 3]), Some(6));
    assert_eq!(sum_checked(&[255]), Some(255));
    assert_eq!(sum_checked(&[]), Some(0));
}

// Deutsch: Dieser Test ist der Grund fuer die Aufgabe. Ein Rumpf mit `+` statt
// `checked_add` bricht hier im dev-Profil ab und gibt im release-Profil
// `Some(4)` zurueck. Beides ist nicht `None`.
// English: this test is the reason for the exercise. A body with `+` instead of
// `checked_add` aborts here in the dev profile and gives back `Some(4)` in the
// release profile. Neither of those is `None`.
#[test]
fn sum_checked_says_none_instead_of_overflowing() {
    assert_eq!(sum_checked(&[250, 10]), None);
    assert_eq!(sum_checked(&[255, 1]), None);
}

// Deutsch: Der Ueberlauf faellt erst am Ende an. Ein Rumpf, der zwischendurch
// abbricht statt weiterzurechnen, kommt hier nicht durch.
// English: the overflow only shows up at the end. A body that aborts in between
// instead of carrying on does not get through here.
#[test]
fn sum_checked_looks_at_every_step_and_not_only_at_the_result() {
    assert_eq!(sum_checked(&[200, 100, 200]), None);
}

#[test]
fn sum_wrapping_starts_from_the_front_again() {
    assert_eq!(sum_wrapping(&[250, 10]), 4);
    assert_eq!(sum_wrapping(&[255, 1]), 0);
}

#[test]
fn sum_wrapping_adds_the_ordinary_case_as_usual() {
    assert_eq!(sum_wrapping(&[1, 2, 3]), 6);
    assert_eq!(sum_wrapping(&[]), 0);
}

#[test]
fn half_even_halves() {
    assert_eq!(half_even(0), 0);
    assert_eq!(half_even(2), 1);
    assert_eq!(half_even(254), 127);
}

#[test]
#[should_panic(expected = "nur gerade Zahlen")]
fn half_even_aborts_on_an_odd_number() {
    half_even(3);
}

// Deutsch: Der Test legt sich nicht auf ein Profil fest. Er haelt fest, dass die
// Antwort zum Lauf passt, und laeuft deshalb unter `cargo test` und unter
// `cargo test --release` gleichermassen.
// English: the test does not commit to one profile. It pins down that the answer
// fits the run, and therefore runs under `cargo test` and under
// `cargo test --release` alike.
#[test]
fn the_finished_function_names_the_profile_of_this_run() {
    #[cfg(debug_assertions)]
    assert_eq!(profile_name(), "debug");

    #[cfg(not(debug_assertions))]
    assert_eq!(profile_name(), "release");
}
