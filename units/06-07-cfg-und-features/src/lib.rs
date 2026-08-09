//! 06-07 #[cfg] und Features / #[cfg] and features
//!
//! Deutsch: Ein Feature ist ein Name in `Cargo.toml`, den der Bauende an- oder
//! abschalten kann. `#[cfg]` hängt Code an diesen Namen: Ist er aus, ist der
//! Code nicht bloß tot, er ist gar nicht erst da. `cfg!` ist die andere Form,
//! ein gewöhnlicher Wahrheitswert, bei dem beide Zweige übersetzt werden.
//!
//! English: a feature is a name in `Cargo.toml` that whoever builds can switch
//! on or off. `#[cfg]` hangs code on that name: with the name off the code is
//! not merely dead, it is not there at all. `cfg!` is the other form, an
//! ordinary boolean where both branches get compiled.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Die Teile, die in dieser Übersetzung eingebaut sind, mit Feature.
///
/// Diese Funktion steht fertig da, und sie steht zweimal da. Welche der beiden
/// Fassungen im Paket landet, entscheidet das `#[cfg]` darüber, und die andere
/// erreicht den Übersetzer gar nicht erst. Beide tragen denselben Namen und
/// dieselbe Signatur, denn wer sie aufruft, soll nicht wissen müssen, welche
/// von beiden gerade da ist.
///
/// The parts built into this compilation, with the feature.
///
/// This function stands there finished, and it stands there twice. Which of the
/// two versions lands in the package is decided by the `#[cfg]` above it, and
/// the other one does not reach the compiler at all. Both carry the same name
/// and the same signature, because whoever calls them should not have to know
/// which of the two is there at the moment.
#[cfg(feature = "zusammenfassung")]
pub fn eingebaute_teile() -> Vec<&'static str> {
    vec!["kern", "zusammenfassung"]
}

/// Die Teile, die in dieser Übersetzung eingebaut sind, ohne Feature.
///
/// The parts built into this compilation, without the feature.
#[cfg(not(feature = "zusammenfassung"))]
pub fn eingebaute_teile() -> Vec<&'static str> {
    vec!["kern"]
}

/// Sagt, ob das Feature an ist.
///
/// Diese Funktion steht ebenfalls fertig da und zeigt die andere Form. `cfg!`
/// ist ein Makro, das zu `true` oder `false` wird, und beide Zweige eines `if`
/// darüber werden übersetzt. Das ist der Unterschied zu `#[cfg]`: Dort wird ein
/// Zweig entfernt, hier wird er nur nicht genommen.
///
/// Beides hat seinen Ort. `#[cfg]` braucht, wer Code fernhalten will, der ohne
/// das Feature nicht einmal übersetzt, etwa weil er eine Abhängigkeit benutzt,
/// die nur mit dem Feature da ist. `cfg!` reicht, wo beide Wege sowieso
/// übersetzen, und es kostet keinen zweiten Rumpf.
///
/// Says whether the feature is on.
///
/// This function stands there finished as well and shows the other form. `cfg!`
/// is a macro that turns into `true` or `false`, and both branches of an `if`
/// over it get compiled. That is the difference from `#[cfg]`: there a branch is
/// removed, here it is merely not taken.
///
/// Both have their place. `#[cfg]` is what you need to keep code away that would
/// not even compile without the feature, for instance because it uses a
/// dependency that is only there with the feature. `cfg!` is enough where both
/// ways compile anyway, and it costs no second body.
///
/// Dieser Doku-Test steht hier und nicht an `eingebaute_teile`, denn er soll in
/// beiden Übersetzungen laufen. Ein Doku-Test an einer Funktion mit `#[cfg]`
/// verschwindet mit ihr, und der Prüflauf aus `CONTRIBUTING.md` baut nur die
/// Fassung ohne das Feature.
///
/// This doc test stands here and not on `eingebaute_teile`, because it is meant
/// to run in both compilations. A doc test on a function with a `#[cfg]`
/// disappears along with it, and the check run from `CONTRIBUTING.md` builds
/// only the version without the feature.
///
/// ```
/// use unit_06_07_cfg_und_features::{eingebaute_teile, zusammenfassung_an};
///
/// // Deutsch: Der Kern ist immer dabei, in beiden Fassungen.
/// // English: the core is always there, in both versions.
/// assert_eq!(eingebaute_teile()[0], "kern");
///
/// // Deutsch: Der zweite Teil kommt genau dann dazu, wenn das Feature an ist.
/// // English: the second part comes along exactly when the feature is on.
/// assert_eq!(
///     eingebaute_teile().contains(&"zusammenfassung"),
///     zusammenfassung_an()
/// );
/// ```
pub fn zusammenfassung_an() -> bool {
    cfg!(feature = "zusammenfassung")
}

/// Aufgabe 1: Schreibe den Bericht, mit und ohne Zusammenfassung.
///
/// Ohne das Feature sind das die Zeilen, mit `\n` verbunden. Mit dem Feature
/// kommt eine letzte Zeile dazu, die `Zeilen: <anzahl>` heißt. Eine leere Liste
/// ergibt ohne das Feature den leeren Text und mit ihm nur die Zeile
/// `Zeilen: 0`.
///
/// Zu lösen ist das mit zwei Rümpfen und je einem `#[cfg]` darüber, so wie
/// `eingebaute_teile` es vormacht. Beide Rümpfe tragen denselben Namen und
/// dieselbe Signatur.
///
/// Exercise 1: write the report, with and without the summary.
///
/// Without the feature that is the lines joined with `\n`. With the feature a
/// last line comes along reading `Zeilen: <count>`. An empty list gives the
/// empty text without the feature and only the line `Zeilen: 0` with it.
///
/// This is to be solved with two bodies and a `#[cfg]` over each of them, the
/// way `eingebaute_teile` shows. Both bodies carry the same name and the same
/// signature.
pub fn bericht(zeilen: &[&str]) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Beschreibe die Übersetzung in einem Satz, mit `cfg!`.
///
/// Heraus kommt `"Bericht mit Zusammenfassung"`, wenn das Feature an ist, und
/// sonst `"Bericht ohne Zusammenfassung"`.
///
/// Zu lösen ist das mit einem einzigen Rumpf und einem `if cfg!(...)`, nicht mit
/// zwei Rümpfen. Der Unterschied zu Aufgabe 1 ist die ganze Aufgabe: Hier
/// übersetzen beide Zweige, und ein Tippfehler im nicht genommenen Zweig fällt
/// deshalb sofort auf.
///
/// Exercise 2: describe the compilation in one sentence, with `cfg!`.
///
/// What comes out is `"Bericht mit Zusammenfassung"` when the feature is on and
/// `"Bericht ohne Zusammenfassung"` otherwise.
///
/// This is to be solved with a single body and an `if cfg!(...)`, not with two
/// bodies. The difference from exercise 1 is the whole exercise: here both
/// branches compile, and a typo in the branch not taken therefore shows up right
/// away.
pub fn beschreibung() -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Ein Bericht als Struktur, deren Feld am Feature hängt.
///
/// Das `#[cfg]` steht hier an einem Feld und nicht an einer Funktion. Ohne das
/// Feature hat diese Struktur ein Feld, mit ihm zwei, und wer sie baut, schreibt
/// das `#[cfg]` im Struktur-Literal noch einmal hin.
///
/// A report as a struct whose field hangs on the feature.
///
/// The `#[cfg]` stands on a field here and not on a function. Without the
/// feature this struct has one field, with it two, and whoever builds it writes
/// the `#[cfg]` in the struct literal a second time.
#[derive(Debug, PartialEq, Eq)]
pub struct Bericht {
    /// Die Zeilen des Berichts.
    ///
    /// The lines of the report.
    pub zeilen: Vec<String>,

    /// Die Anzahl der Zeilen, nur mit dem Feature.
    ///
    /// The number of lines, only with the feature.
    #[cfg(feature = "zusammenfassung")]
    pub anzahl: usize,
}

/// Aufgabe 3: Baue den Bericht als Struktur.
///
/// Die Zeilen kommen als `String` in das Feld `zeilen`, in derselben
/// Reihenfolge. Mit dem Feature kommt `anzahl` dazu und trägt die Anzahl der
/// Zeilen.
///
/// Im Struktur-Literal steht dafür ein `#[cfg(feature = "zusammenfassung")]`
/// direkt vor dem Feld `anzahl`. Wer es vergisst, bekommt ohne das Feature eine
/// Meldung über ein Feld, das es nicht gibt, und mit ihm eine über ein Feld, das
/// fehlt.
///
/// Exercise 3: build the report as a struct.
///
/// The lines go into the field `zeilen` as `String`, in the same order. With the
/// feature `anzahl` comes along and carries the number of lines.
///
/// In the struct literal that means a `#[cfg(feature = "zusammenfassung")]`
/// directly in front of the field `anzahl`. Whoever forgets it gets a message
/// about a field that does not exist without the feature, and one about a field
/// that is missing with it.
pub fn neuer_bericht(zeilen: &[&str]) -> Bericht {
    todo!("Aufgabe 3 / Exercise 3")
}
