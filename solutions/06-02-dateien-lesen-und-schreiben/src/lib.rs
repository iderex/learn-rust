//! 06-02 Dateien lesen und schreiben / Reading and writing files, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/06-02-dateien-lesen-und-schreiben/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/06-02-dateien-lesen-und-schreiben/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

/// Liest eine Datei ganz ein.
///
/// Reads a file in whole.
///
/// ```
/// use std::fs;
/// use unit_06_02_dateien_lesen_und_schreiben::read_text;
///
/// let pfad = std::env::temp_dir().join(format!("lr-06-02-doku-{}.txt", std::process::id()));
/// fs::write(&pfad, "hallo\n").unwrap();
///
/// assert_eq!(read_text(&pfad).unwrap(), "hallo\n");
///
/// fs::remove_file(&pfad).unwrap();
/// ```
pub fn read_text(pfad: &Path) -> io::Result<String> {
    fs::read_to_string(pfad)
}

/// Liest eine Datei und antwortet auf die fehlende Datei mit `ersatz`.
///
/// Reads a file and answers the missing file with `ersatz`.
pub fn read_or(pfad: &Path, ersatz: &str) -> io::Result<String> {
    match fs::read_to_string(pfad) {
        Ok(inhalt) => Ok(inhalt),
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(ersatz.to_string()),
        Err(fehler) => Err(fehler),
    }
}

/// Hängt eine Zeile an eine Datei an.
///
/// Appends a line to a file.
pub fn append_line(pfad: &Path, zeile: &str) -> io::Result<()> {
    let mut datei = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(pfad)?;

    datei.write_all(zeile.as_bytes())?;
    datei.write_all(b"\n")
}

/// Schreibt die passenden Zeilen aus `quelle` nach `ziel`.
///
/// Writes the matching lines from `quelle` into `ziel`.
pub fn copy_lines(quelle: &Path, ziel: &Path, enthaelt: &str) -> io::Result<usize> {
    let inhalt = fs::read_to_string(quelle)?;

    let mut heraus = String::new();
    let mut gezaehlt = 0;
    for zeile in inhalt.lines() {
        if zeile.contains(enthaelt) {
            heraus.push_str(zeile);
            heraus.push('\n');
            gezaehlt += 1;
        }
    }

    fs::write(ziel, heraus)?;
    Ok(gezaehlt)
}
