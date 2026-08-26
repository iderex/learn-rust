//! Die Prüfungen über das Repository / the checks over the repository.
//!
//! Deutsch: Jede Prüfung hat eine Kennung, eine Beschreibung dessen, was sie
//! ansieht, und eine Funktion, die Befunde zurückgibt. Der Lauf gibt außerdem
//! aus, was er nicht ansieht, damit er nicht für mehr gehalten wird, als er
//! leistet.
//!
//! English: every check has an id, a description of what it looks at, and a
//! function returning findings. The run also prints what it does not look at,
//! so it is not taken for more than it does.

pub mod befehle;
pub mod wurzel;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Die Kennung einer Prüfung / the id of a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckId {
    Sprachabschnitte,
    Sprachbalance,
    Sprachform,
    Quelle,
    Loesung,
    Nummerierung,
    Lizenz,
    Hinweisblock,
    Dateinamen,
}

impl CheckId {
    /// Die Kennung als Text, so wie der Lauf sie ausgibt.
    ///
    /// The id as text, the way the run prints it.
    pub fn as_str(self) -> &'static str {
        match self {
            CheckId::Sprachabschnitte => "sprachabschnitte",
            CheckId::Sprachbalance => "sprachbalance",
            CheckId::Sprachform => "sprachform",
            CheckId::Quelle => "quelle",
            CheckId::Loesung => "loesung",
            CheckId::Nummerierung => "nummerierung",
            CheckId::Lizenz => "lizenz",
            CheckId::Hinweisblock => "hinweisblock",
            CheckId::Dateinamen => "dateinamen",
        }
    }

    /// Was diese Prüfung ansieht, auf Deutsch und auf Englisch.
    ///
    /// What this check looks at, in German and in English.
    pub fn description(self) -> (&'static str, &'static str) {
        match self {
            CheckId::Sprachabschnitte => (
                "jede README unter units/ hat ## Deutsch und ## English mit Text unter beiden, Deutsch zuerst",
                "every README under units/ has ## Deutsch and ## English with text under both, German first",
            ),
            CheckId::Sprachbalance => (
                "kein Sprachabschnitt ist kuerzer als die Haelfte des anderen",
                "neither language section is shorter than half the other",
            ),
            CheckId::Sprachform => (
                "beide Sprachabschnitte haben gleich viele Codebloecke, gleich viele Ueberschriften und dieselben Fehlernummern",
                "both language sections have the same number of code blocks, the same number of headings and the same error numbers",
            ),
            CheckId::Quelle => (
                "jede Einheit nennt eine Quelle mit Kapiteltitel und der gebundenen Version aus rust-toolchain.toml",
                "every unit names a source with a chapter title and the pinned version from rust-toolchain.toml",
            ),
            CheckId::Loesung => (
                "zu jeder Einheit gibt es einen gleichnamigen Ordner unter solutions/, und beide Pakete tragen denselben Paketnamen",
                "every unit has a folder of the same name under solutions/, and both packages carry the same package name",
            ),
            CheckId::Nummerierung => (
                "die Nummerierung der Einheiten hat je Stufe keine Luecken und keine Dopplungen",
                "the numbering of the units has no gaps and no duplicates within a stage",
            ),
            CheckId::Lizenz => (
                "beide Lizenzdateien liegen im Wurzelverzeichnis, und jede Cargo.toml nennt license = \"MIT\"",
                "both licence files are at the root, and every Cargo.toml names license = \"MIT\"",
            ),
            CheckId::Hinweisblock => (
                "jede Einheit hat den Hinweisblock, und llms.txt nennt jede Einheit",
                "every unit has the note block, and llms.txt names every unit",
            ),
            CheckId::Dateinamen => (
                "Ordner- und Dateinamen unter units/ und solutions/ sind kleingeschrieben und ohne Umlaute",
                "folder and file names under units/ and solutions/ are lowercase and free of umlauts",
            ),
        }
    }
}

/// Alle Prüfungen, in der Reihenfolge des Laufs.
///
/// Every check, in the order of the run.
pub const CHECKS: [CheckId; 9] = [
    CheckId::Sprachabschnitte,
    CheckId::Sprachbalance,
    CheckId::Sprachform,
    CheckId::Quelle,
    CheckId::Loesung,
    CheckId::Nummerierung,
    CheckId::Lizenz,
    CheckId::Hinweisblock,
    CheckId::Dateinamen,
];

/// Was der Lauf nicht ansieht, auf Deutsch und auf Englisch.
///
/// What the run does not look at, in German and in English.
pub const UNCHECKED: [(&str, &str); 4] = [
    (
        "ob der deutsche und der englische Abschnitt wirklich dasselbe sagen",
        "whether the German and the English section really say the same thing",
    ),
    (
        "ob eine genannte Quelle die Behauptung traegt, die sie belegen soll",
        "whether a named source carries the claim it is meant to back",
    ),
    (
        "ob ein Commit die Zeile Signed-off-by traegt",
        "whether a commit carries the Signed-off-by line",
    ),
    (
        "ob ein Assistent den Hinweisen aus llms.txt folgt",
        "whether an assistant follows the guidance in llms.txt",
    ),
];

/// Ein Befund: welche Prüfung, an welcher Stelle, und was dort nicht stimmt.
///
/// A finding: which check, at which place, and what is wrong there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub check: CheckId,
    pub subject: String,
    pub detail: String,
}

/// Eine gefundene Einheit unter `units/`.
///
/// A unit found under `units/`.
#[derive(Debug, Clone)]
struct Unit {
    name: String,
    readme: PathBuf,
}

/// Läuft alle Prüfungen über `root` und gibt die Befunde zurück.
///
/// Runs every check over `root` and returns the findings.
pub fn check(root: &Path) -> io::Result<Vec<Finding>> {
    let mut findings = Vec::new();
    let units = units(root)?;
    let pinned = pinned_version(root)?;

    for unit in &units {
        let text = fs::read_to_string(&unit.readme)?;
        check_language_sections(&unit.name, &text, &mut findings);
        check_source(&unit.name, &text, pinned.as_deref(), &mut findings);
    }
    check_template_language(root, &mut findings)?;
    check_solutions(root, &units, &mut findings)?;
    check_numbering(&units, &mut findings);
    check_licence(root, &mut findings)?;
    check_note_block(root, &units, &mut findings)?;
    check_names(root, &mut findings)?;

    findings.sort_by(|a, b| (a.check, &a.subject).cmp(&(b.check, &b.subject)));
    Ok(findings)
}

/// Die Sprachabschnitte der Vorlage, die `units` nicht mitzählt.
///
/// Deutsch: Die Vorlage ist die Datei, aus der jede neue Einheit kopiert wird.
/// Ein Sprachfehler in ihr wandert in die nächste Einheit weiter, und dort wird
/// er gemeldet statt an seiner Herkunft. `quelle`, `loesung`, `hinweisblock`
/// und `nummerierung` bleiben von ihr weg: sie trägt Platzhalter statt einer
/// Quellenangabe und hat weder einen Ordner unter `solutions/` noch eine Zeile
/// in `llms.txt`.
///
/// English: the template is the file every new unit is copied from. A language
/// fault in it travels into the next unit, where it gets reported instead of at
/// its origin. `quelle`, `loesung`, `hinweisblock` and `nummerierung` stay off
/// it: it carries placeholders rather than a source reference, and it has
/// neither a folder under `solutions/` nor a line in `llms.txt`.
fn check_template_language(root: &Path, findings: &mut Vec<Finding>) -> io::Result<()> {
    let readme = root.join("units").join("template").join("README.md");
    if !readme.is_file() {
        return Ok(());
    }
    let text = fs::read_to_string(&readme)?;
    check_language_sections("template", &text, findings);
    Ok(())
}

/// Die Einheiten unter `units/`, ohne die Vorlage.
///
/// The units under `units/`, without the template.
fn units(root: &Path) -> io::Result<Vec<Unit>> {
    let dir = root.join("units");
    let mut found = Vec::new();
    if !dir.is_dir() {
        return Ok(found);
    }
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if !is_unit_name(&name) {
            continue;
        }
        found.push(Unit {
            readme: entry.path().join("README.md"),
            name,
        });
    }
    found.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(found)
}

/// Ein Einheitenname beginnt mit `<zwei Ziffern>-<zwei Ziffern>-`.
///
/// A unit name starts with `<two digits>-<two digits>-`.
fn is_unit_name(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() > 6
        && b[0].is_ascii_digit()
        && b[1].is_ascii_digit()
        && b[2] == b'-'
        && b[3].is_ascii_digit()
        && b[4].is_ascii_digit()
        && b[5] == b'-'
}

/// Die gebundene Version aus `rust-toolchain.toml`.
///
/// The pinned version from `rust-toolchain.toml`.
fn pinned_version(root: &Path) -> io::Result<Option<String>> {
    let path = root.join("rust-toolchain.toml");
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("channel")
            && let Some(value) = rest.split('"').nth(1)
        {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

/// Der Text unter `## Deutsch` und der unter `## English`.
///
/// The text under `## Deutsch` and the one under `## English`.
fn split_sections(text: &str) -> Option<(String, String)> {
    let de = text.find("\n## Deutsch")?;
    let en = text.find("\n## English")?;
    if en < de {
        return None;
    }
    let german = text[de + "\n## Deutsch".len()..en].to_string();
    let rest = &text[en + "\n## English".len()..];
    let end = rest.find("\n## ").unwrap_or(rest.len());
    Some((german, rest[..end].to_string()))
}

fn check_language_sections(name: &str, text: &str, findings: &mut Vec<Finding>) {
    let Some((german, english)) = split_sections(text) else {
        findings.push(Finding {
            check: CheckId::Sprachabschnitte,
            subject: format!("units/{name}/README.md"),
            detail: "## Deutsch und ## English fehlen, oder English steht vor Deutsch".into(),
        });
        return;
    };
    if german.trim().is_empty() || english.trim().is_empty() {
        findings.push(Finding {
            check: CheckId::Sprachabschnitte,
            subject: format!("units/{name}/README.md"),
            detail: "ein Sprachabschnitt ist leer".into(),
        });
        return;
    }

    let (dl, el) = (
        german.trim().chars().count(),
        english.trim().chars().count(),
    );
    if dl * 2 < el || el * 2 < dl {
        findings.push(Finding {
            check: CheckId::Sprachbalance,
            subject: format!("units/{name}/README.md"),
            detail: format!("Deutsch {dl} Zeichen, English {el} Zeichen"),
        });
    }

    let (dfences, dheads, derrors) = shape(&german);
    let (efences, eheads, eerrors) = shape(&english);
    if dfences != efences {
        findings.push(Finding {
            check: CheckId::Sprachform,
            subject: format!("units/{name}/README.md"),
            detail: format!("Codebloecke: Deutsch {dfences}, English {efences}"),
        });
    }
    if dheads != eheads {
        findings.push(Finding {
            check: CheckId::Sprachform,
            subject: format!("units/{name}/README.md"),
            detail: format!("Ueberschriften: Deutsch {dheads}, English {eheads}"),
        });
    }
    if derrors != eerrors {
        findings.push(Finding {
            check: CheckId::Sprachform,
            subject: format!("units/{name}/README.md"),
            detail: format!("Fehlernummern: Deutsch {derrors:?}, English {eerrors:?}"),
        });
    }
}

/// Codeblöcke, Überschriften und Fehlernummern eines Abschnitts.
///
/// Code blocks, headings and error numbers of one section.
fn shape(section: &str) -> (usize, usize, BTreeSet<String>) {
    let mut fences = 0usize;
    let mut headings = 0usize;
    for line in section.lines() {
        if line.trim_start().starts_with("```") {
            fences += 1;
        }
        if line.starts_with("##") {
            headings += 1;
        }
    }
    (fences / 2, headings, error_numbers(section))
}

/// Alle Vorkommen der Form `error[E0382]`.
///
/// Every occurrence of the form `error[E0382]`.
fn error_numbers(section: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let bytes = section.as_bytes();
    let mut i = 0;
    while let Some(pos) = section[i..].find("error[") {
        let start = i + pos;
        let after = start + "error[".len();
        if let Some(close) = section[after..].find(']') {
            let inner = &section[after..after + close];
            if inner.len() > 1
                && inner.starts_with('E')
                && inner[1..].chars().all(|c| c.is_ascii_digit())
            {
                found.insert(inner.to_string());
            }
            i = after + close;
        } else {
            i = after;
        }
        if i >= bytes.len() {
            break;
        }
    }
    found
}

fn check_source(name: &str, text: &str, pinned: Option<&str>, findings: &mut Vec<Finding>) {
    let has_chapter_title = text.contains("Kapitel") && text.contains('"');
    let has_version = match pinned {
        Some(version) => text.contains(version),
        None => false,
    };
    if !has_chapter_title || !has_version {
        findings.push(Finding {
            check: CheckId::Quelle,
            subject: format!("units/{name}/README.md"),
            detail: match pinned {
                Some(version) => format!(
                    "Kapiteltitel vorhanden: {has_chapter_title}, gebundene Version {version} genannt: {has_version}"
                ),
                None => "rust-toolchain.toml nennt keine gebundene Version".into(),
            },
        });
    }
}

/// Der Paketname aus einer `Cargo.toml`.
///
/// The package name from a `Cargo.toml`.
fn package_name(path: &Path) -> io::Result<Option<String>> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("name")
            && let Some(value) = rest.split('"').nth(1)
        {
            return Ok(Some(value.to_string()));
        }
    }
    Ok(None)
}

fn check_solutions(root: &Path, units: &[Unit], findings: &mut Vec<Finding>) -> io::Result<()> {
    for unit in units {
        let solution = root.join("solutions").join(&unit.name);
        if !solution.is_dir() {
            findings.push(Finding {
                check: CheckId::Loesung,
                subject: format!("solutions/{}", unit.name),
                detail: "der Ordner fehlt".into(),
            });
            continue;
        }
        let unit_name = package_name(&root.join("units").join(&unit.name).join("Cargo.toml"))?;
        let solution_name = package_name(&solution.join("Cargo.toml"))?;
        if unit_name != solution_name {
            findings.push(Finding {
                check: CheckId::Loesung,
                subject: format!("solutions/{}", unit.name),
                detail: format!("Paketname {solution_name:?} statt {unit_name:?}"),
            });
        }
    }
    Ok(())
}

fn check_numbering(units: &[Unit], findings: &mut Vec<Finding>) {
    let mut by_stage: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for unit in units {
        by_stage
            .entry(&unit.name[0..2])
            .or_default()
            .push(&unit.name[3..5]);
    }
    for (stage, mut numbers) in by_stage {
        numbers.sort_unstable();
        for window in numbers.windows(2) {
            if window[0] == window[1] {
                findings.push(Finding {
                    check: CheckId::Nummerierung,
                    subject: format!("units/{stage}-*"),
                    detail: format!("die Nummer {} kommt zweimal vor", window[0]),
                });
            }
        }
        for (index, number) in numbers.iter().enumerate() {
            let expected = format!("{:02}", index + 1);
            if *number != expected {
                findings.push(Finding {
                    check: CheckId::Nummerierung,
                    subject: format!("units/{stage}-*"),
                    detail: format!("an Stelle {expected} steht {number}"),
                });
                break;
            }
        }
    }
}

/// Jede `Cargo.toml` unterhalb von `root`, ohne die Bauordner.
///
/// Every `Cargo.toml` below `root`, without the build folders.
fn manifests(root: &Path, into: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type()?.is_dir() {
            if name == "target" || name == ".git" {
                continue;
            }
            manifests(&path, into)?;
        } else if name == "Cargo.toml" {
            into.push(path);
        }
    }
    Ok(())
}

fn check_licence(root: &Path, findings: &mut Vec<Finding>) -> io::Result<()> {
    for file in ["LICENSE-CC-BY-4.0", "LICENSE-MIT"] {
        if !root.join(file).is_file() {
            findings.push(Finding {
                check: CheckId::Lizenz,
                subject: file.into(),
                detail: "die Datei fehlt im Wurzelverzeichnis".into(),
            });
        }
    }
    let mut found = Vec::new();
    manifests(root, &mut found)?;
    found.sort();
    for path in found {
        let text = fs::read_to_string(&path)?;
        if !text.lines().any(|line| line.trim() == "license = \"MIT\"") {
            findings.push(Finding {
                check: CheckId::Lizenz,
                subject: relative(root, &path),
                detail: "license = \"MIT\" fehlt".into(),
            });
        }
    }
    Ok(())
}

/// Die Überschrift, an der der Hinweisblock erkannt wird.
///
/// The heading by which the note block is recognised.
pub const NOTE_BLOCK_SUMMARY: &str =
    "<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>";

fn check_note_block(root: &Path, units: &[Unit], findings: &mut Vec<Finding>) -> io::Result<()> {
    let template = root.join("units").join("template").join("README.md");
    let mut readmes: Vec<(String, PathBuf)> = units
        .iter()
        .map(|unit| (unit.name.clone(), unit.readme.clone()))
        .collect();
    if template.is_file() {
        readmes.push(("template".into(), template));
    }
    for (name, path) in &readmes {
        let text = fs::read_to_string(path)?;
        if !text.contains(NOTE_BLOCK_SUMMARY) {
            findings.push(Finding {
                check: CheckId::Hinweisblock,
                subject: format!("units/{name}/README.md"),
                detail: "der eingeklappte Hinweisblock fehlt".into(),
            });
        }
    }

    let llms = root.join("llms.txt");
    if !llms.is_file() {
        findings.push(Finding {
            check: CheckId::Hinweisblock,
            subject: "llms.txt".into(),
            detail: "die Datei fehlt im Wurzelverzeichnis".into(),
        });
        return Ok(());
    }
    let text = fs::read_to_string(llms)?;
    for unit in units {
        if !text.contains(&format!("units/{}/README.md", unit.name)) {
            findings.push(Finding {
                check: CheckId::Hinweisblock,
                subject: "llms.txt".into(),
                detail: format!("die Einheit {} wird nicht genannt", unit.name),
            });
        }
    }
    Ok(())
}

/// Namen, die groß geschrieben sein dürfen, weil das Werkzeug sie so erwartet.
///
/// Names that may be uppercase, because the tooling expects them that way.
const ALLOWED_UPPERCASE: [&str; 3] = ["README.md", "Cargo.toml", "Cargo.lock"];

fn check_names(root: &Path, findings: &mut Vec<Finding>) -> io::Result<()> {
    for top in ["units", "solutions"] {
        let dir = root.join(top);
        if dir.is_dir() {
            walk_names(root, &dir, findings)?;
        }
    }
    Ok(())
}

fn walk_names(root: &Path, dir: &Path, findings: &mut Vec<Finding>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        let is_dir = entry.file_type()?.is_dir();
        if is_dir && (name == "target" || name == ".git") {
            continue;
        }
        if name.chars().any(|c| "äöüÄÖÜß".contains(c)) {
            findings.push(Finding {
                check: CheckId::Dateinamen,
                subject: relative(root, &path),
                detail: "der Name enthaelt einen Umlaut".into(),
            });
        } else if name.chars().any(|c| c.is_uppercase()) && !ALLOWED_UPPERCASE.contains(&&*name) {
            findings.push(Finding {
                check: CheckId::Dateinamen,
                subject: relative(root, &path),
                detail: "der Name ist nicht kleingeschrieben".into(),
            });
        }
        if is_dir {
            walk_names(root, &path, findings)?;
        }
    }
    Ok(())
}

/// Ein Pfad relativ zur Wurzel, mit Schrägstrichen.
///
/// A path relative to the root, with forward slashes.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
