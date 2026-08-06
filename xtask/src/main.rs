//! Der Prüflauf über das Repository / the check run over the repository.
//!
//! Deutsch: `cargo run -p xtask -- check` sagt, was es angesehen hat, was es
//! nicht ansieht, und was es gefunden hat. `cargo run -p xtask -- ci` schickt
//! den ganzen Prüflauf ab, also auch die Befehle von cargo, und liest, welche
//! das sind, aus CONTRIBUTING.md.
//!
//! English: `cargo run -p xtask -- check` says what it examined, what it does
//! not examine, and what it found. `cargo run -p xtask -- ci` sends the whole
//! check run, meaning the cargo commands as well, and reads which those are
//! from CONTRIBUTING.md.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use xtask::befehle;
use xtask::{CHECKS, UNCHECKED, check};

fn main() -> ExitCode {
    let verb = std::env::args().nth(1);
    match verb.as_deref() {
        Some("check") => run(),
        Some("ci") => ci(),
        _ => {
            eprintln!("Nutzung / usage: cargo run -p xtask -- check");
            eprintln!("Nutzung / usage: cargo run -p xtask -- ci");
            ExitCode::from(2)
        }
    }
}

/// Schickt jeden Befehl des Prüflaufs ab und hält beim ersten roten an.
///
/// Sends every command of the check run and stops at the first red one.
fn ci() -> ExitCode {
    let root = root();
    println!("xtask ci, Wurzel / root: {}", root.display());
    println!(
        "Die Befehle stehen in CONTRIBUTING.md unter {:?} und werden von dort gelesen.",
        befehle::HEADING
    );
    println!(
        "The commands are in CONTRIBUTING.md under {:?} and are read from there.",
        befehle::HEADING
    );
    println!();

    let befehle = match befehle::befehle(&root) {
        Ok(befehle) => befehle,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(2);
        }
    };

    println!("Gelesen / read");
    for befehl in &befehle {
        println!("  {befehl}");
    }
    println!();

    for befehl in &befehle {
        match schicke(&root, befehl) {
            Ok(true) => println!("gruen / green: {befehl}\n"),
            Ok(false) => {
                eprintln!("rot / red: {befehl}");
                eprintln!(
                    "Der Lauf haelt beim ersten roten Befehl an / the run stops at the first red command."
                );
                return ExitCode::FAILURE;
            }
            Err(error) => {
                eprintln!("nicht startbar / could not start: {befehl}: {error}");
                return ExitCode::from(2);
            }
        }
    }
    println!("Jeder Befehl gruen / every command green.");
    ExitCode::SUCCESS
}

/// Schickt einen Befehl aus dem Wurzelverzeichnis ab, ohne Shell dazwischen.
///
/// Sends one command from the root directory, with no shell in between.
fn schicke(root: &Path, befehl: &str) -> std::io::Result<bool> {
    println!("### {befehl}");
    let mut worte = befehl.split_whitespace();
    let programm = worte
        .next()
        .expect("eine nicht leere Zeile hat ein erstes Wort / a non-empty line has a first word");
    let status = Command::new(programm)
        .args(worte)
        .current_dir(root)
        .status()?;
    Ok(status.success())
}

fn run() -> ExitCode {
    let root = root();
    println!("xtask check, Wurzel / root: {}", root.display());
    println!();

    println!("Angesehen / examined");
    for id in CHECKS {
        let (de, en) = id.description();
        println!("  {:<18} {de}", id.as_str());
        println!("  {:<18} {en}", "");
    }
    println!();

    println!("Nicht angesehen / not examined");
    for (de, en) in UNCHECKED {
        println!("  {de}");
        println!("  {en}");
    }
    println!();

    let findings = match check(&root) {
        Ok(findings) => findings,
        Err(error) => {
            eprintln!(
                "Der Lauf konnte den Baum nicht lesen / the run could not read the tree: {error}"
            );
            return ExitCode::from(2);
        }
    };

    if findings.is_empty() {
        println!("Befunde / findings: keine / none");
        return ExitCode::SUCCESS;
    }
    println!("Befunde / findings: {}", findings.len());
    for finding in &findings {
        println!(
            "  {:<18} {}: {}",
            finding.check.as_str(),
            finding.subject,
            finding.detail
        );
    }
    ExitCode::FAILURE
}

/// Das Wurzelverzeichnis des Repositories, aus der Lage dieses Pakets.
///
/// The root of the repository, from where this package sits.
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask liegt unterhalb der Wurzel / xtask sits below the root")
        .to_path_buf()
}
