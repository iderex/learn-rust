//! Das Wurzelverzeichnis, zur Laufzeit gesucht / the root directory, found at
//! run time.
//!
//! Deutsch: Der Lauf muss wissen, welchen Baum er beurteilt. Diese Angabe aus
//! `env!("CARGO_MANIFEST_DIR")` zu nehmen, hieße, sie beim Übersetzen
//! festzuhalten: cargo baut ein Paket nicht neu, nur weil sein Verzeichnis
//! umgezogen ist, und die alte Zeichenkette überlebt den Umzug. Der Lauf
//! beurteilte dann ein Verzeichnis, in dem er nicht steht, und sagte nichts
//! davon. Deshalb wird die Wurzel hier vom laufenden Verzeichnis aus aufwärts
//! gesucht, und wo keine liegt, wird abgebrochen statt geraten.
//!
//! English: the run has to know which tree it is judging. Taking that from
//! `env!("CARGO_MANIFEST_DIR")` would mean fixing it at compile time: cargo
//! does not rebuild a package merely because its directory moved, and the old
//! string survives the move. The run would then judge a directory it is not
//! standing in and say nothing about it. So the root is searched upwards from
//! the running directory here, and where there is none, the run stops instead
//! of guessing.

use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Die Verzeichnisse, an denen die Wurzel dieses Repositories erkannt wird.
/// Ein Ordner unter `units/` oder `solutions/` trägt keines davon, und
/// `units/` selbst trägt zwar eine `Cargo.toml` mit `[workspace]`, aber kein
/// `units`, weshalb hier Verzeichnisse und nicht Manifeste stehen.
///
/// The directories the root of this repository is recognised by. A folder
/// under `units/` or `solutions/` carries none of them, and `units/` itself
/// does carry a `Cargo.toml` with `[workspace]` but no `units`, which is why
/// directories rather than manifests stand here.
const MARKEN: [&str; 3] = ["units", "solutions", "xtask"];

/// Warum keine Wurzel gefunden wurde.
///
/// Why no root was found.
#[derive(Debug)]
pub enum Fehler {
    /// Das laufende Verzeichnis ließ sich nicht lesen.
    Verzeichnis(io::Error),
    /// Weder das Startverzeichnis noch eines darüber trägt die Marken.
    Keine(PathBuf),
}

impl fmt::Display for Fehler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fehler::Verzeichnis(error) => write!(
                f,
                "das laufende Verzeichnis liess sich nicht lesen / the running directory could not be read: {error}"
            ),
            Fehler::Keine(start) => write!(
                f,
                "von {} aufwaerts liegt kein Wurzelverzeichnis mit {MARKEN:?} / no root directory carrying {MARKEN:?} sits at or above {}",
                start.display(),
                start.display()
            ),
        }
    }
}

impl std::error::Error for Fehler {}

/// Trägt dieses Verzeichnis alle Marken?
///
/// Does this directory carry every marker?
fn ist_wurzel(verzeichnis: &Path) -> bool {
    MARKEN.iter().all(|marke| verzeichnis.join(marke).is_dir())
}

/// Sucht von `start` aufwärts das erste Verzeichnis, das die Marken trägt.
///
/// Searches upwards from `start` for the first directory carrying the markers.
pub fn wurzel(start: &Path) -> Result<PathBuf, Fehler> {
    for verzeichnis in start.ancestors() {
        if ist_wurzel(verzeichnis) {
            return Ok(verzeichnis.to_path_buf());
        }
    }
    Err(Fehler::Keine(start.to_path_buf()))
}

/// Dasselbe, vom laufenden Verzeichnis aus. Das ist die Stelle, an der der
/// Lauf seine Wurzel bekommt.
///
/// The same, from the running directory. This is where the run gets its root.
pub fn vom_laufenden_verzeichnis() -> Result<PathBuf, Fehler> {
    let start = std::env::current_dir().map_err(Fehler::Verzeichnis)?;
    wurzel(&start)
}
