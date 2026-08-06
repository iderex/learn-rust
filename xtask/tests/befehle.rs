//! Fälle für den Leser der Befehlsliste.
//!
//! Deutsch: Jeder Fall läuft gegen einen gebauten Text und nie gegen die
//! CONTRIBUTING.md dieses Repositories. Ein Fall, der die echte Datei liest,
//! beweist ihren Stand an dem Tag, an dem er lief, und nicht den Leser. Ob die
//! echte Datei sich lesen lässt, beantwortet der Lauf selbst, jedes Mal wenn er
//! abgeschickt wird, und er hält an, wenn nicht.
//!
//! English: every case runs against a built text and never against the
//! CONTRIBUTING.md of this repository. A case reading the real file proves its
//! state on the day it ran, not the reader. Whether the real file can be read is
//! answered by the run itself, every time it is sent, and it stops when it
//! cannot.

use xtask::befehle::{Fehler, HEADING, aus_text};

/// Ein Text mit der Überschrift, einem Block davor und einer Überschrift danach.
///
/// A text with the heading, a block before it and a heading after it.
fn dokument(block: &str) -> String {
    format!(
        "# Titel\n\n### Eine andere Ueberschrift\n\n```console\nnicht dieser befehl\n```\n\n{HEADING}\n\nEtwas Fliesstext davor.\n\n```console\n{block}\n```\n\nEtwas Fliesstext danach.\n\n### Die naechste Ueberschrift\n\n```console\nauch nicht dieser befehl\n```\n"
    )
}

#[test]
fn liest_die_zeilen_des_blocks() {
    let text = dokument("cargo fmt --all --check\n\ncargo test --workspace");
    let gelesen = aus_text(&text).expect("der Block steht da");
    assert_eq!(
        gelesen,
        vec![
            "cargo fmt --all --check".to_string(),
            "cargo test --workspace".to_string()
        ],
        "der Leser gibt die Zeilen des Blocks zurueck, leere uebersprungen"
    );
}

#[test]
fn nimmt_nur_den_block_dieser_ueberschrift() {
    let text = dokument("cargo run -p xtask -- check");
    let gelesen = aus_text(&text).expect("der Block steht da");
    assert_eq!(
        gelesen,
        vec!["cargo run -p xtask -- check".to_string()],
        "weder der Block davor noch der danach gehoert dazu"
    );
}

#[test]
fn ohne_ueberschrift_kein_ergebnis() {
    let text = dokument("cargo test --workspace").replace(HEADING, "### Etwas ganz anderes");
    assert!(
        matches!(aus_text(&text), Err(Fehler::KeineUeberschrift)),
        "ohne die Ueberschrift weiss der Leser nicht, wo er suchen soll"
    );
}

#[test]
fn ohne_block_kein_ergebnis() {
    let text = format!(
        "# Titel\n\n{HEADING}\n\nNur Fliesstext.\n\n### Die naechste\n\n```console\nnicht dieser befehl\n```\n"
    );
    assert!(
        matches!(aus_text(&text), Err(Fehler::KeinBlock)),
        "der Block der naechsten Ueberschrift zaehlt nicht als der eigene"
    );
}

#[test]
fn leerer_block_kein_ergebnis() {
    let text = dokument("");
    assert!(
        matches!(aus_text(&text), Err(Fehler::LeererBlock)),
        "ein Block ohne Zeile ist kein Prueflauf"
    );
}

#[test]
fn ein_anfuehrungszeichen_wird_abgelehnt() {
    let text = dokument("cargo test --workspace -- --test-threads \"1\"");
    match aus_text(&text) {
        Err(Fehler::Zeichen { zeichen, .. }) => assert_eq!(zeichen, '"'),
        other => panic!("eine Zeile mit Anfuehrungszeichen darf nicht durchgehen: {other:?}"),
    }
}

#[test]
fn ein_rohr_wird_abgelehnt() {
    let text = dokument("cargo test --workspace | tee lauf.txt");
    match aus_text(&text) {
        Err(Fehler::Zeichen { zeichen, .. }) => assert_eq!(zeichen, '|'),
        other => panic!("eine Zeile mit einem Rohr darf nicht durchgehen: {other:?}"),
    }
}

#[test]
fn ein_nachbar_ohne_sonderzeichen_geht_durch() {
    let text = dokument("cargo test --workspace -- --test-threads 1");
    let gelesen = aus_text(&text).expect("diese Zeile traegt kein abgelehntes Zeichen");
    assert_eq!(
        gelesen,
        vec!["cargo test --workspace -- --test-threads 1".to_string()],
        "die Ablehnung trifft das Zeichen und nicht die Laenge der Zeile"
    );
}
