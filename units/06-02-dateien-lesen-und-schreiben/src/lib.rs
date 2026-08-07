//! 06-02 Dateien lesen und schreiben / Reading and writing files
//!
//! Deutsch: Jeder Zugriff auf eine Datei kann scheitern, und zwar aus Gründen,
//! die im Programm nicht stehen. Deshalb gibt hier jede Funktion ein
//! `io::Result` zurück, und eine fehlende Datei ist ein Fall und kein Absturz.
//!
//! English: every access to a file can fail, for reasons that are not written
//! in the program. That is why every function here returns an `io::Result`, and
//! a missing file is a case and not a crash.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::fs;
use std::io;
use std::path::Path;

/// Liest eine Datei ganz ein.
///
/// Diese Funktion steht fertig da. Sie zeigt die kürzeste Form: `read_to_string`
/// gibt bereits ein `io::Result<String>` zurück, also gibt es nichts zu
/// übersetzen und nichts zu verpacken.
///
/// Der Parameter ist ein `&Path` und kein `&str`, denn ein Pfad ist nicht
/// überall Text. Ein `&str` passt deshalb nicht ohne Weiteres hinein, siehe die
/// README unter "Häufige Fehler".
///
/// Reads a file in whole.
///
/// This function stands there finished. It shows the shortest form:
/// `read_to_string` already returns an `io::Result<String>`, so there is nothing
/// to translate and nothing to wrap.
///
/// The parameter is a `&Path` and not a `&str`, because a path is not text
/// everywhere. A `&str` therefore does not fit in without further ado, see the
/// README under "Common mistakes".
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

/// Aufgabe 1: Lies eine Datei und antworte auf die fehlende Datei mit `ersatz`.
///
/// Gibt es die Datei, kommt ihr Inhalt zurück. Gibt es sie nicht, kommt
/// `ersatz` zurück, und zwar als `Ok`. Jeder andere Fehler wird
/// weitergereicht und nicht verschluckt, denn "die Datei gibt es nicht" und
/// "ich darf sie nicht lesen" sind verschiedene Aussagen.
///
/// Zu unterscheiden sind die beiden an `io::Error::kind`, das ein
/// `io::ErrorKind` zurückgibt. Der gesuchte Fall heißt
/// `io::ErrorKind::NotFound`.
///
/// Exercise 1: read a file and answer the missing file with `ersatz`.
///
/// If the file is there, its content comes back. If it is not, `ersatz` comes
/// back, and as an `Ok`. Every other error is passed on and not swallowed,
/// because "the file is not there" and "I am not allowed to read it" are
/// different statements.
///
/// The two are told apart by `io::Error::kind`, which returns an
/// `io::ErrorKind`. The case wanted here is called `io::ErrorKind::NotFound`.
pub fn read_or(pfad: &Path, ersatz: &str) -> io::Result<String> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Hänge eine Zeile an eine Datei an.
///
/// Die Zeile wird mit einem `\n` dahinter angehängt. Gibt es die Datei noch
/// nicht, wird sie angelegt. Was schon darin steht, bleibt stehen.
///
/// `fs::write` kann das nicht, denn es überschreibt. Der Weg führt über
/// `fs::OpenOptions`, mit `append(true)` und `create(true)`, und über
/// `write_all` aus `std::io::Write`.
///
/// Exercise 2: append a line to a file.
///
/// The line is appended with a `\n` behind it. If the file is not there yet, it
/// gets created. Whatever already stands in it stays.
///
/// `fs::write` cannot do this, because it overwrites. The way there goes through
/// `fs::OpenOptions`, with `append(true)` and `create(true)`, and through
/// `write_all` from `std::io::Write`.
pub fn append_line(pfad: &Path, zeile: &str) -> io::Result<()> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Schreibe die passenden Zeilen aus `quelle` nach `ziel`.
///
/// Übernommen wird jede Zeile, die `enthaelt` als Teiltext hat. Jede
/// übernommene Zeile bekommt ein `\n` dahinter. Zurück kommt, wie viele Zeilen
/// es waren. Gibt es `quelle` nicht, kommt der Fehler zurück und `ziel` wird
/// nicht angefasst.
///
/// Gibt es keine passende Zeile, entsteht `ziel` trotzdem und ist leer.
///
/// Exercise 3: write the matching lines from `quelle` into `ziel`.
///
/// Every line that has `enthaelt` as a substring is taken over. Every taken line
/// gets a `\n` behind it. What comes back is how many lines there were. If
/// `quelle` is not there, the error comes back and `ziel` is not touched.
///
/// If there is no matching line, `ziel` still comes into being and is empty.
pub fn copy_lines(quelle: &Path, ziel: &Path, enthaelt: &str) -> io::Result<usize> {
    todo!("Aufgabe 3 / Exercise 3")
}
