//! Je Prüfung ein Fall, der genau sie auslöst, und ein Nachbarfall, der nichts
//! auslöst.
//!
//! Deutsch: Alle Fälle laufen gegen einen gebauten Baum und nie gegen dieses
//! Repository. Ein Fall, der das echte Repository beurteilt, beweist den Stand
//! des Baums an dem Tag, an dem er lief, und nicht die Prüfung.
//!
//! English: every case runs against a built tree and never against this
//! repository. A case judging the real repository proves the state of the tree
//! on the day it ran, not the check.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use xtask::{CheckId, check};

/// Ein gebauter Baum in einem eigenen Verzeichnis, der sich selbst aufräumt.
///
/// A built tree in a directory of its own that cleans itself up.
struct Tree {
    root: PathBuf,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Tree {
    /// Ein Baum, gegen den jede Prüfung schweigt.
    ///
    /// A tree against which every check stays silent.
    fn baseline() -> Tree {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("xtask-fixture-{}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let tree = Tree { root };

        tree.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        tree.write("LICENSE-MIT", "MIT License\n");
        tree.write("LICENSE-CC-BY-4.0", "Attribution 4.0 International\n");
        tree.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"solutions/*\"]\n\n[workspace.package]\nlicense = \"MIT\"\n",
        );
        tree.write(
            "units/Cargo.toml",
            "[workspace]\nmembers = [\"[0-9][0-9]-*\"]\n\n[workspace.package]\nlicense = \"MIT\"\n",
        );
        tree.write("llms.txt", LLMS);
        tree.write("units/02-01-move/README.md", &readme("02-01 Verschieben"));
        tree.write("units/02-01-move/Cargo.toml", &manifest("unit-02-01-move"));
        tree.write("units/02-01-move/src/lib.rs", "// leer / empty\n");
        tree.write("units/template/README.md", &readme("<nn-nn> Vorlage"));
        tree.write("units/template/Cargo.toml", &manifest("unit-vorlage"));
        tree.write(
            "solutions/02-01-move/Cargo.toml",
            &manifest("unit-02-01-move"),
        );
        tree.write("solutions/02-01-move/src/lib.rs", "// leer / empty\n");
        tree
    }

    fn write(&self, rel: &str, content: &str) {
        let path = self.root.join(rel);
        fs::create_dir_all(path.parent().expect("ein Pfad hat ein Elternteil")).expect("mkdir");
        fs::write(path, content).expect("write");
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel)).expect("read")
    }

    fn remove(&self, rel: &str) {
        fs::remove_file(self.root.join(rel)).expect("remove");
    }

    /// Ersetzt genau ein Vorkommen und besteht darauf, dass es eines gab.
    ///
    /// Replaces exactly one occurrence and insists there was one.
    fn edit(&self, rel: &str, from: &str, to: &str) {
        let text = self.read(rel);
        assert!(
            text.contains(from),
            "der Fall wollte {from:?} in {rel} ersetzen und fand es nicht"
        );
        self.write(rel, &text.replacen(from, to, 1));
    }

    /// Benennt eine Einheit um, samt Lösung und Eintrag in llms.txt.
    ///
    /// Renames a unit together with its solution and its entry in llms.txt.
    fn rename_unit(&self, from: &str, to: &str) {
        fs::rename(
            self.root.join("units").join(from),
            self.root.join("units").join(to),
        )
        .expect("rename unit");
        fs::rename(
            self.root.join("solutions").join(from),
            self.root.join("solutions").join(to),
        )
        .expect("rename solution");
        self.edit("llms.txt", from, to);
    }

    /// Die Kennungen der Befunde, ohne Dopplungen.
    ///
    /// The ids of the findings, without duplicates.
    fn ids(&self) -> BTreeSet<CheckId> {
        check(&self.root)
            .expect("der Lauf liest den Baum")
            .into_iter()
            .map(|finding| finding.check)
            .collect()
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn manifest(name: &str) -> String {
    format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nlicense = \"MIT\"\n")
}

const LLMS: &str = "# learn-rust\n\n- [02-01](units/02-01-move/README.md)\n";

const GERMAN_PROSE: &str = "Ein Wert hat genau einen Eigentuemer, und wer ihn weitergibt, gibt das Eigentum mit. Die alte Bindung ist danach nicht mehr benutzbar, und der Uebersetzer sagt das mit einer Nummer statt mit einem Rat. Wer den Wert behalten will, leiht ihn aus oder kopiert ihn ausdruecklich, und beides steht weiter unten an einem Beispiel.";

const ENGLISH_PROSE: &str = "A value has exactly one owner, and whoever hands it on hands the ownership with it. The old binding cannot be used afterwards, and the compiler says so with a number rather than with advice. Whoever wants to keep the value borrows it or copies it explicitly, and both are shown below in an example.";

/// Eine README, gegen die jede Prüfung schweigt.
///
/// A README against which every check stays silent.
fn readme(title: &str) -> String {
    format!(
        "# {title}\n\
         \n\
         <details>\n\
         <summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>\n\
         Deutsch: ein Hinweis. English: a note.\n\
         </details>\n\
         \n\
         ## Deutsch\n\
         \n\
         ### Die Erklaerung\n\
         \n\
         {GERMAN_PROSE}\n\
         \n\
         Der Uebersetzer meldet dazu error[E0382].\n\
         \n\
         ```rust\n\
         fn main() {{}}\n\
         ```\n\
         \n\
         ### Quelle\n\
         \n\
         Buch, Kapitel 4 \"Understanding Ownership\", geprueft gegen 1.97.1\n\
         \n\
         ## English\n\
         \n\
         ### The explanation\n\
         \n\
         {ENGLISH_PROSE}\n\
         \n\
         The compiler reports error[E0382] for it.\n\
         \n\
         ```rust\n\
         fn main() {{}}\n\
         ```\n\
         \n\
         ### Source\n\
         \n\
         Book, chapter 4 \"Understanding Ownership\", checked against 1.97.1\n"
    )
}

fn only(id: CheckId) -> BTreeSet<CheckId> {
    BTreeSet::from([id])
}

fn nothing() -> BTreeSet<CheckId> {
    BTreeSet::new()
}

#[test]
fn the_baseline_tree_triggers_nothing() {
    assert_eq!(Tree::baseline().ids(), nothing());
}

#[test]
fn sprachabschnitte_refuses_a_readme_without_an_english_section() {
    let tree = Tree::baseline();
    tree.edit("units/02-01-move/README.md", "## English", "## Englisch");
    assert_eq!(tree.ids(), only(CheckId::Sprachabschnitte));
}

#[test]
fn sprachabschnitte_passes_a_heading_with_trailing_space() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "## English\n",
        "## English \n",
    );
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn sprachbalance_refuses_an_english_one_liner_under_a_long_german_section() {
    let tree = Tree::baseline();
    tree.edit("units/02-01-move/README.md", ENGLISH_PROSE, "Short.");
    assert_eq!(tree.ids(), only(CheckId::Sprachbalance));
}

#[test]
fn sprachbalance_passes_an_english_section_a_little_shorter() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        ENGLISH_PROSE,
        "A value has exactly one owner, and whoever hands it on hands the ownership with it. The old binding cannot be used afterwards, and the compiler says so with a number rather than with advice.",
    );
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn sprachform_refuses_a_code_block_in_one_language_only() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "### Source",
        "```rust\nfn extra() {}\n```\n\n### Source",
    );
    assert_eq!(tree.ids(), only(CheckId::Sprachform));
}

#[test]
fn sprachform_passes_the_same_code_block_in_both_languages() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "### Quelle",
        "```rust\nfn extra() {}\n```\n\n### Quelle",
    );
    tree.edit(
        "units/02-01-move/README.md",
        "### Source",
        "```rust\nfn extra() {}\n```\n\n### Source",
    );
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn quelle_refuses_a_unit_naming_another_version_than_the_pinned_one() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "geprueft gegen 1.97.1",
        "geprueft gegen 1.96.0",
    );
    tree.edit(
        "units/02-01-move/README.md",
        "checked against 1.97.1",
        "checked against 1.96.0",
    );
    assert_eq!(tree.ids(), only(CheckId::Quelle));
}

#[test]
fn quelle_passes_the_version_named_with_a_word_in_front() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "geprueft gegen 1.97.1",
        "geprueft gegen Version 1.97.1",
    );
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn loesung_refuses_a_solution_carrying_another_package_name() {
    let tree = Tree::baseline();
    tree.edit(
        "solutions/02-01-move/Cargo.toml",
        "name = \"unit-02-01-move\"",
        "name = \"loesung-02-01-move\"",
    );
    assert_eq!(tree.ids(), only(CheckId::Loesung));
}

#[test]
fn loesung_refuses_a_missing_solution_folder() {
    let tree = Tree::baseline();
    fs::remove_dir_all(tree.root.join("solutions").join("02-01-move")).expect("remove");
    assert_eq!(tree.ids(), only(CheckId::Loesung));
}

#[test]
fn loesung_passes_a_solution_differing_only_in_its_version() {
    let tree = Tree::baseline();
    tree.edit("solutions/02-01-move/Cargo.toml", "0.1.0", "0.2.0");
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn nummerierung_refuses_a_stage_whose_first_unit_is_not_number_one() {
    let tree = Tree::baseline();
    tree.rename_unit("02-01-move", "02-02-move");
    assert_eq!(tree.ids(), only(CheckId::Nummerierung));
}

#[test]
fn nummerierung_passes_a_renamed_unit_keeping_its_number() {
    let tree = Tree::baseline();
    tree.rename_unit("02-01-move", "02-01-verschieben");
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn lizenz_refuses_a_manifest_without_the_field() {
    let tree = Tree::baseline();
    tree.edit("units/02-01-move/Cargo.toml", "license = \"MIT\"\n", "");
    assert_eq!(tree.ids(), only(CheckId::Lizenz));
}

#[test]
fn lizenz_refuses_a_missing_licence_file() {
    let tree = Tree::baseline();
    tree.remove("LICENSE-CC-BY-4.0");
    assert_eq!(tree.ids(), only(CheckId::Lizenz));
}

#[test]
fn lizenz_passes_the_field_with_trailing_space() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/Cargo.toml",
        "license = \"MIT\"\n",
        "license = \"MIT\"  \n",
    );
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn hinweisblock_refuses_a_unit_without_the_block() {
    let tree = Tree::baseline();
    tree.edit(
        "units/02-01-move/README.md",
        "<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>\n",
        "",
    );
    assert_eq!(tree.ids(), only(CheckId::Hinweisblock));
}

#[test]
fn hinweisblock_refuses_an_llms_txt_missing_a_unit() {
    let tree = Tree::baseline();
    tree.edit("llms.txt", "units/02-01-move/README.md", "units/README.md");
    assert_eq!(tree.ids(), only(CheckId::Hinweisblock));
}

#[test]
fn hinweisblock_passes_a_block_that_is_open_by_default() {
    let tree = Tree::baseline();
    tree.edit("units/02-01-move/README.md", "<details>", "<details open>");
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn dateinamen_refuses_a_file_name_with_an_umlaut() {
    let tree = Tree::baseline();
    tree.write("units/02-01-move/src/grüße.rs", "// leer / empty\n");
    assert_eq!(tree.ids(), only(CheckId::Dateinamen));
}

#[test]
fn dateinamen_refuses_a_folder_name_that_is_not_lowercase() {
    let tree = Tree::baseline();
    tree.write("units/02-01-move/Tests/exercise.rs", "// leer / empty\n");
    assert_eq!(tree.ids(), only(CheckId::Dateinamen));
}

#[test]
fn dateinamen_passes_the_names_cargo_writes_itself() {
    let tree = Tree::baseline();
    tree.write("units/Cargo.lock", "version = 4\n");
    assert_eq!(tree.ids(), nothing());
}

#[test]
fn dateinamen_passes_the_same_name_spelled_out() {
    let tree = Tree::baseline();
    tree.write("units/02-01-move/src/gruesse.rs", "// leer / empty\n");
    assert_eq!(tree.ids(), nothing());
}

/// Der Baum eines Falls liegt für sich, damit zwei Fälle einander nicht sehen.
///
/// The tree of a case stands on its own, so that two cases cannot see each
/// other.
#[test]
fn two_cases_do_not_share_a_tree() {
    let one = Tree::baseline();
    let two = Tree::baseline();
    assert_ne!(one.root, two.root);
    one.remove("LICENSE-MIT");
    assert_eq!(one.ids(), only(CheckId::Lizenz));
    assert_eq!(two.ids(), nothing());
}

/// Die Pfade der Fälle liegen ausserhalb dieses Repositories.
///
/// The paths of the cases lie outside this repository.
#[test]
fn a_case_never_reads_this_repository() {
    let tree = Tree::baseline();
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!tree.root.starts_with(here));
}
