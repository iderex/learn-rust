//! Die Befehle des Prüflaufs, aus CONTRIBUTING.md gelesen statt abgeschrieben.
//!
//! Deutsch: CONTRIBUTING.md sagt, dass der Prüflauf genau einmal im Repository
//! steht, und dieser Abschnitt ist diese eine Stelle. Ein Ablauf, der die
//! Befehle noch einmal aufschriebe, wäre die zweite Stelle, und zwei Stellen
//! laufen auseinander, sobald eine von beiden sich ändert. Dieser Baustein
//! liest stattdessen den Block unter der Überschrift und gibt seine Zeilen
//! zurück.
//!
//! English: CONTRIBUTING.md says the check run stands exactly once in the
//! repository, and that section is the one place. A route that wrote the
//! commands down again would be the second place, and two places drift apart as
//! soon as one of them changes. This part reads the block under the heading
//! instead and returns its lines.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

/// Die Überschrift, unter der der Block mit den Befehlen steht.
///
/// The heading under which the block with the commands sits.
pub const HEADING: &str = "### Der Prüflauf / The check run";

/// Der Zaun, der den Block öffnet.
///
/// The fence that opens the block.
const FENCE: &str = "```console";

/// Zeichen, die eine Befehlszeile nicht tragen darf. Der Lauf zerlegt eine
/// Zeile an Leerzeichen und startet das erste Wort als Programm, ohne eine
/// Shell dazwischen. Eine Zeile mit Anführungszeichen oder einem Rohr sähe
/// dann aus wie etwas, das sie nicht tut, und deshalb wird sie abgelehnt
/// statt anders ausgeführt, als sie dasteht.
///
/// Characters a command line may not carry. The run splits a line at spaces
/// and starts the first word as a program, with no shell in between. A line
/// with quotes or a pipe would look like something it does not do, so it is
/// refused rather than executed differently from how it is written.
const REFUSED: &str = "\"'`$&;|<>(){}\\";

/// Warum die Befehlsliste nicht gelesen werden konnte.
///
/// Why the list of commands could not be read.
#[derive(Debug)]
pub enum Fehler {
    /// CONTRIBUTING.md ließ sich nicht lesen.
    Lesen(io::Error),
    /// Die Überschrift steht nicht in der Datei.
    KeineUeberschrift,
    /// Unter der Überschrift steht kein Block.
    KeinBlock,
    /// Der Block steht da, trägt aber keine Zeile.
    LeererBlock,
    /// Eine Zeile trägt ein Zeichen, das ohne Shell etwas anderes bedeutete.
    Zeichen { zeile: String, zeichen: char },
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fehler::Lesen(error) => write!(
                f,
                "CONTRIBUTING.md liess sich nicht lesen / CONTRIBUTING.md could not be read: {error}"
            ),
            Fehler::KeineUeberschrift => write!(
                f,
                "CONTRIBUTING.md hat keine Ueberschrift {HEADING:?} / CONTRIBUTING.md has no heading {HEADING:?}"
            ),
            Fehler::KeinBlock => write!(
                f,
                "unter der Ueberschrift steht kein Block {FENCE:?} / there is no {FENCE:?} block under the heading"
            ),
            Fehler::LeererBlock => write!(
                f,
                "der Block unter der Ueberschrift traegt keine Zeile / the block under the heading carries no line"
            ),
            Fehler::Zeichen { zeile, zeichen } => write!(
                f,
                "die Zeile {zeile:?} traegt das Zeichen {zeichen:?}, das ohne Shell etwas anderes bedeutete / the line {zeile:?} carries {zeichen:?}, which would mean something else without a shell"
            ),
        }
    }
}

impl std::error::Error for Fehler {}

/// Die Befehlszeilen aus der CONTRIBUTING.md unter `root`.
///
/// The command lines from the CONTRIBUTING.md under `root`.
pub fn befehle(root: &Path) -> Result<Vec<String>, Fehler> {
    let text = fs::read_to_string(root.join("CONTRIBUTING.md")).map_err(Fehler::Lesen)?;
    aus_text(&text)
}

/// Dieselbe Arbeit an einem Text, damit ein Fall sie ohne Datei prüfen kann.
///
/// The same work on a text, so a case can check it without a file.
pub fn aus_text(text: &str) -> Result<Vec<String>, Fehler> {
    let section = abschnitt(text).ok_or(Fehler::KeineUeberschrift)?;

    let open = section.find(FENCE).ok_or(Fehler::KeinBlock)?;
    let after = &section[open + FENCE.len()..];
    let close = after.find("\n```").ok_or(Fehler::KeinBlock)?;

    let mut zeilen = Vec::new();
    for zeile in after[..close].lines() {
        let zeile = zeile.trim();
        if zeile.is_empty() {
            continue;
        }
        if let Some(zeichen) = zeile.chars().find(|c| REFUSED.contains(*c)) {
            return Err(Fehler::Zeichen {
                zeile: zeile.to_string(),
                zeichen,
            });
        }
        zeilen.push(zeile.to_string());
    }
    if zeilen.is_empty() {
        return Err(Fehler::LeererBlock);
    }
    Ok(zeilen)
}

/// Der Text zwischen der Überschrift und der nächsten Überschrift. Ohne diesen
/// Schnitt läse ein fehlender Block den der nächsten Überschrift mit.
///
/// The text between the heading and the next heading. Without that cut a
/// missing block would read the one belonging to the next heading.
fn abschnitt(text: &str) -> Option<&str> {
    let start = if text.starts_with(HEADING) {
        0
    } else {
        text.find(&format!("\n{HEADING}"))? + 1
    };
    let rest = &text[start + HEADING.len()..];
    let end = ["\n### ", "\n## ", "\n# "]
        .iter()
        .filter_map(|marker| rest.find(marker))
        .min()
        .unwrap_or(rest.len());
    Some(&rest[..end])
}
