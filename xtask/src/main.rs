//! Der Prüflauf über das Repository / the check run over the repository.
//!
//! Deutsch: `cargo run -p xtask -- check` sagt, was es angesehen hat, was es
//! nicht ansieht, und was es gefunden hat.
//!
//! English: `cargo run -p xtask -- check` says what it examined, what it does
//! not examine, and what it found.

use std::path::PathBuf;
use std::process::ExitCode;

use xtask::{CHECKS, UNCHECKED, check};

fn main() -> ExitCode {
    let verb = std::env::args().nth(1);
    match verb.as_deref() {
        Some("check") => run(),
        _ => {
            eprintln!("Nutzung / usage: cargo run -p xtask -- check");
            ExitCode::from(2)
        }
    }
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
