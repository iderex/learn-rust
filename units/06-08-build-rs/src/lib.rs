//! 06-08 build.rs / build.rs
//!
//! Deutsch: Ein Build-Skript ist ein Rust-Programm, das vor dem Übersetzen
//! dieses Pakets läuft. Es kann etwas erzeugen, das der Code danach benutzt, und
//! es kann Cargo sagen, wann es wieder laufen muss.
//!
//! English: a build script is a Rust program that runs before this package is
//! compiled. It can produce something the code then uses, and it can tell Cargo
//! when it has to run again.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

// Deutsch: Hier kommt herein, was `build.rs` erzeugt hat. `OUT_DIR` ist ein
// Ordner, den Cargo für dieses Paket anlegt, und `include!` setzt den Inhalt der
// Datei an diese Stelle, als stünde er hier. Die Datei liegt unter `target/` und
// nicht im Repository, denn sie entsteht bei jedem Bau neu.
// English: what `build.rs` produced comes in here. `OUT_DIR` is a folder Cargo
// creates for this package, and `include!` puts the content of the file at this
// place, as if it stood here. The file lies under `target/` and not in the
// repository, because it comes into being anew with every build.
include!(concat!(env!("OUT_DIR"), "/farben.rs"));

/// Gibt die erzeugte Liste zurück.
///
/// Diese Funktion steht fertig da. Sie zeigt, was von dem Skript im Code
/// ankommt: eine ganz gewöhnliche Konstante, die niemand von Hand geschrieben
/// hat.
///
/// Returns the generated list.
///
/// This function stands there finished. It shows what arrives in the code from
/// the script: an entirely ordinary constant that nobody wrote by hand.
///
/// ```
/// use unit_06_08_build_rs::colours;
///
/// assert_eq!(colours().len(), 5);
/// assert_eq!(colours()[0], "rot");
/// ```
pub fn colours() -> &'static [&'static str] {
    &FARBEN
}

/// Aufgabe 1: Sag, ob `name` in der erzeugten Liste steht.
///
/// Verglichen wird genau, ohne Rücksicht auf Groß- und Kleinschreibung
/// drumherum. `"rot"` steht drin, `"Rot"` nicht.
///
/// Exercise 1: say whether `name` is in the generated list.
///
/// The comparison is exact, without any regard for upper and lower case around
/// it. `"rot"` is in there, `"Rot"` is not.
pub fn contains(name: &str) -> bool {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib den längsten Namen aus der erzeugten Liste zurück.
///
/// Gezählt wird in Bytes, also mit `len`. Sind zwei gleich lang, kommt der
/// zurück, der vorn steht.
///
/// Exercise 2: return the longest name out of the generated list.
///
/// Counting is in bytes, so with `len`. If two are equally long, the one
/// standing first comes back.
pub fn longest() -> &'static str {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Baue eine Zeile aus allen Namen, durch Komma und Leerzeichen
/// getrennt.
///
/// Die Reihenfolge ist die der Datei. Aus einer leeren Liste würde der leere
/// Text, aber diese Liste ist nicht leer.
///
/// Exercise 3: build one line out of all names, separated by a comma and a
/// space.
///
/// The order is the one of the file. An empty list would give the empty text,
/// but this list is not empty.
pub fn as_line() -> String {
    todo!("Aufgabe 3 / Exercise 3")
}
