//! 04-01 Pakete und Crates / Packages and crates
//!
//! Deutsch: Ein Paket beschreibt eine `Cargo.toml`, eine Crate übersetzt der
//! Übersetzer in einem Lauf. Die Wurzel einer Bibliothek ist `src/lib.rs`, die
//! eines Programms `src/main.rs`, und weitere Programme liegen unter `src/bin/`.
//!
//! English: a package is what a `Cargo.toml` describes, a crate is what the
//! compiler compiles in one run. The root of a library is `src/lib.rs`, that of
//! a program `src/main.rs`, and further programs live under `src/bin/`.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Sagt zu einer Wurzel, welche Art von Crate dort anfängt.
///
/// `src/lib.rs` gehört zu einer Bibliothek, `src/main.rs` zu einem Programm,
/// und alles unter `src/bin/` ebenfalls zu einem Programm.
///
/// Says for a root which kind of crate begins there.
///
/// `src/lib.rs` belongs to a library, `src/main.rs` to a program, and
/// everything under `src/bin/` to a program as well.
///
/// ```
/// use unit_04_01_pakete_und_crates::crate_kind;
///
/// assert_eq!(crate_kind("src/lib.rs"), Some("bibliothek"));
/// assert_eq!(crate_kind("src/main.rs"), Some("programm"));
/// assert_eq!(crate_kind("src/bin/zweites.rs"), Some("programm"));
/// assert_eq!(crate_kind("Cargo.toml"), None);
/// ```
pub fn crate_kind(wurzel: &str) -> Option<&'static str> {
    if wurzel == "src/lib.rs" {
        Some("bibliothek")
    } else if wurzel == "src/main.rs" || wurzel.starts_with("src/bin/") {
        Some("programm")
    } else {
        None
    }
}

/// Aufgabe 1: Gib zu einer Art von Crate ihre Wurzel zurück.
///
/// "bibliothek" hat ihre Wurzel in `src/lib.rs`, "programm" in `src/main.rs`.
/// Für alles andere gibt es keine Wurzel, und die Antwort ist `None`.
///
/// Exercise 1: return the root for a kind of crate.
///
/// "bibliothek" has its root in `src/lib.rs`, "programm" in `src/main.rs`. For
/// anything else there is no root, and the answer is `None`.
pub fn crate_root(art: &str) -> Option<&'static str> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Zähle die Crates eines Pakets.
///
/// Ein Paket trägt höchstens eine Bibliothek und beliebig viele Programme.
/// `hat_bibliothek` sagt, ob die Bibliothek da ist, `programme` zählt die
/// Programme.
///
/// Exercise 2: count the crates of a package.
///
/// A package carries at most one library and any number of programs.
/// `hat_bibliothek` says whether the library is there, `programme` counts the
/// programs.
pub fn crate_count(hat_bibliothek: bool, programme: u32) -> u32 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib den Pfad eines weiteren Programms zurück.
///
/// Es liegt unter `src/bin/` und heißt wie seine Datei, also wird aus
/// "zweites" der Pfad `src/bin/zweites.rs`.
///
/// Exercise 3: return the path of a further program.
///
/// It lives under `src/bin/` and is named after its file, so out of "zweites"
/// comes the path `src/bin/zweites.rs`.
pub fn binary_root(name: &str) -> String {
    todo!("Aufgabe 3 / Exercise 3")
}
