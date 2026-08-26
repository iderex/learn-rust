//! Die Wurzel folgt dem Startverzeichnis, nicht dem Verzeichnis, in dem dieser
//! Lauf übersetzt wurde.
//!
//! Deutsch: Jeder Fall läuft gegen einen gebauten Baum und nie gegen dieses
//! Repository. Ein Fall, der die echte Wurzel wiedererkennt, ginge auch dann
//! grün aus, wenn die Antwort beim Übersetzen festgehalten wäre, und genau das
//! ist der Fehler, den diese Datei ausschließen soll.
//!
//! English: every case runs against a built tree and never against this
//! repository. A case recognising the real root would come out green even if
//! the answer were fixed at compile time, and that is exactly the defect this
//! file exists to rule out.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use xtask::wurzel::{Fehler, wurzel};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Ein gebautes Verzeichnis, das sich selbst aufräumt.
///
/// A built directory that cleans itself up.
struct Baum {
    root: PathBuf,
}

impl Baum {
    /// Ein leeres Verzeichnis ohne jede Marke.
    ///
    /// An empty directory carrying no marker at all.
    fn leer() -> Baum {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("xtask-wurzel-{}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("mkdir");
        Baum { root }
    }

    /// Legt unterhalb dieses Verzeichnisses die genannten Ordner an.
    ///
    /// Creates the named folders below this directory.
    fn ordner(&self, rel: &[&str]) -> &Baum {
        for one in rel {
            fs::create_dir_all(self.root.join(one)).expect("mkdir");
        }
        self
    }

    fn pfad(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }
}

impl Drop for Baum {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Die drei Marken einer Wurzel, unterhalb von `rel`.
///
/// The three markers of a root, below `rel`.
fn marken(baum: &Baum, rel: &str) {
    let unter = |name: &str| format!("{rel}/{name}");
    baum.ordner(&[&unter("units"), &unter("solutions"), &unter("xtask")]);
}

/// Vergleicht zwei Pfade über ihre aufgelöste Form, weil das Temp-Verzeichnis
/// unter Windows als kurzer Name daherkommen kann.
///
/// Compares two paths through their resolved form, because the temp directory
/// can arrive as a short name on Windows.
fn gleich(links: &Path, rechts: &Path) {
    let links = fs::canonicalize(links).expect("canonicalize links");
    let rechts = fs::canonicalize(rechts).expect("canonicalize rechts");
    assert_eq!(links, rechts);
}

#[test]
fn die_wurzel_ist_das_startverzeichnis_selbst() {
    let baum = Baum::leer();
    marken(&baum, ".");
    gleich(&wurzel(&baum.root).expect("eine Wurzel"), &baum.root);
}

#[test]
fn die_wurzel_wird_von_tief_darunter_gefunden() {
    let baum = Baum::leer();
    marken(&baum, ".");
    baum.ordner(&["units/02-01-move/src"]);
    let tief = baum.pfad("units/02-01-move/src");
    gleich(&wurzel(&tief).expect("eine Wurzel"), &baum.root);
}

/// Der Fall, der beim festgehaltenen Pfad grün gewesen wäre: zwei Bäume, und
/// die Antwort muss dem Start folgen statt einmal zu lauten.
///
/// The case that would have been green under the fixed path: two trees, and
/// the answer has to follow the start rather than reading the same once.
#[test]
fn zwei_baeume_bekommen_zwei_antworten() {
    let einer = Baum::leer();
    let anderer = Baum::leer();
    marken(&einer, ".");
    marken(&anderer, ".");
    gleich(
        &wurzel(&einer.pfad("units")).expect("eine Wurzel"),
        &einer.root,
    );
    gleich(
        &wurzel(&anderer.pfad("units")).expect("eine Wurzel"),
        &anderer.root,
    );
}

/// Der zweite Fall, der beim festgehaltenen Pfad grün gewesen wäre: dort kam
/// immer ein Pfad zurück, auch wo gar kein Repository liegt.
///
/// The second case that would have been green under the fixed path: there a
/// path always came back, including where no repository sits at all.
#[test]
fn ohne_marken_kommt_ein_fehler_statt_eines_pfades() {
    let baum = Baum::leer();
    baum.ordner(&["irgendwo/tief"]);
    match wurzel(&baum.pfad("irgendwo/tief")) {
        Err(Fehler::Keine(start)) => assert_eq!(start, baum.pfad("irgendwo/tief")),
        other => panic!("erwartet war Fehler::Keine, bekommen: {other:?}"),
    }
}

/// Der Nachbarfall: eine Marke fehlt, also ist das Verzeichnis keine Wurzel
/// und die Suche geht darüber hinaus weiter.
///
/// The neighbour case: one marker is missing, so the directory is not a root
/// and the search carries on past it.
#[test]
fn eine_fehlende_marke_macht_noch_keine_wurzel() {
    let baum = Baum::leer();
    marken(&baum, ".");
    baum.ordner(&["fast/units", "fast/xtask", "fast/units/tief"]);
    gleich(
        &wurzel(&baum.pfad("fast/units/tief")).expect("eine Wurzel"),
        &baum.root,
    );
}
