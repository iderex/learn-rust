// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::fs;
use std::path::{Path, PathBuf};

use unit_06_02_dateien_lesen_und_schreiben::{append_line, copy_lines, read_or, read_text};

// Deutsch: Ein eigener Ordner je Test, unterhalb des Temp-Verzeichnisses. Die
// Nummer des Prozesses steht mit im Namen, damit der Lauf ueber die Einheit und
// der ueber die Loesung sich nicht in die Quere kommen.
// English: a folder of its own per test, below the temp directory. The number of
// the process is part of the name, so that the run over the unit and the one
// over the solution do not get in each other's way.
fn ordner(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("lr-06-02-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("der Testordner laesst sich anlegen");
    dir
}

#[test]
fn read_or_gives_the_content_when_the_file_is_there() {
    let dir = ordner("read-or-da");
    let pfad = dir.join("notizen.txt");
    fs::write(&pfad, "erste Zeile\nzweite Zeile\n").unwrap();

    assert_eq!(
        read_or(&pfad, "nichts da").unwrap(),
        "erste Zeile\nzweite Zeile\n"
    );
}

#[test]
fn read_or_gives_the_substitute_when_the_file_is_missing() {
    let dir = ordner("read-or-fehlt");

    assert_eq!(
        read_or(&dir.join("gibt-es-nicht.txt"), "nichts da").unwrap(),
        "nichts da"
    );
}

// Deutsch: Dieser Test haelt den Zweig eng. Ein Rumpf, der jeden Fehler mit dem
// Ersatz beantwortet, kommt hier durch alle anderen Tests und faellt nur hier
// auf. Der Pfad traegt ein Nullbyte, was kein Betriebssystem als Namen annimmt,
// und ergibt deshalb `InvalidInput` statt `NotFound`.
// English: this test keeps the branch narrow. A body that answers every error
// with the substitute gets through all the other tests and only shows up here.
// The path carries a NUL byte, which no operating system accepts as a name, and
// therefore gives `InvalidInput` rather than `NotFound`.
#[test]
fn read_or_passes_on_an_error_that_is_not_a_missing_file() {
    let pfad = Path::new("mit\0nullbyte.txt");

    let fehler = read_or(pfad, "nichts da").expect_err("ein Nullbyte im Pfad ist ein Fehler");

    assert_eq!(fehler.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn append_line_creates_the_file_that_is_not_there_yet() {
    let dir = ordner("append-neu");
    let pfad = dir.join("liste.txt");

    append_line(&pfad, "erste").unwrap();

    assert_eq!(fs::read_to_string(&pfad).unwrap(), "erste\n");
}

#[test]
fn append_line_keeps_what_already_stands_there() {
    let dir = ordner("append-dazu");
    let pfad = dir.join("liste.txt");
    fs::write(&pfad, "erste\n").unwrap();

    append_line(&pfad, "zweite").unwrap();
    append_line(&pfad, "dritte").unwrap();

    assert_eq!(
        fs::read_to_string(&pfad).unwrap(),
        "erste\nzweite\ndritte\n"
    );
}

#[test]
fn copy_lines_takes_only_the_matching_ones() {
    let dir = ordner("copy-passend");
    let quelle = dir.join("quelle.txt");
    let ziel = dir.join("ziel.txt");
    fs::write(&quelle, "Apfel\nBirne\nApfelsaft\nKirsche\n").unwrap();

    assert_eq!(copy_lines(&quelle, &ziel, "Apfel").unwrap(), 2);
    assert_eq!(fs::read_to_string(&ziel).unwrap(), "Apfel\nApfelsaft\n");
}

#[test]
fn copy_lines_writes_an_empty_file_when_nothing_matches() {
    let dir = ordner("copy-leer");
    let quelle = dir.join("quelle.txt");
    let ziel = dir.join("ziel.txt");
    fs::write(&quelle, "Apfel\nBirne\n").unwrap();

    assert_eq!(copy_lines(&quelle, &ziel, "Zwetschge").unwrap(), 0);
    assert_eq!(fs::read_to_string(&ziel).unwrap(), "");
}

#[test]
fn copy_lines_reports_a_missing_source_and_leaves_the_target_alone() {
    let dir = ordner("copy-ohne-quelle");
    let quelle = dir.join("gibt-es-nicht.txt");
    let ziel = dir.join("ziel.txt");

    let fehler = copy_lines(&quelle, &ziel, "egal").expect_err("die Quelle gibt es nicht");

    assert_eq!(fehler.kind(), std::io::ErrorKind::NotFound);
    assert!(!ziel.exists());
}

#[test]
fn the_finished_function_reads_the_whole_file() {
    let dir = ordner("read-text");
    let pfad = dir.join("notizen.txt");
    fs::write(&pfad, "eine Zeile\n").unwrap();

    assert_eq!(read_text(&pfad).unwrap(), "eine Zeile\n");
}
