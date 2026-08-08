//! 06-03 Umgebungsvariablen und Ausgabe nach stderr / Environment variables and
//! output to stderr
//!
//! Deutsch: Eine Umgebungsvariable ist eine Einstellung, die von außen kommt
//! und die es nicht geben muss. Und ein Programm hat zwei Ausgänge, nicht
//! einen: das Ergebnis geht nach stdout, die Meldung darüber nach stderr.
//!
//! English: an environment variable is a setting that comes from outside and
//! does not have to be there. And a program has two exits rather than one: the
//! result goes to stdout, the message about it goes to stderr.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::env;
use std::io::{self, Write};

/// Ein Lauf des Werkzeugs: was gefunden wurde und was dazu zu sagen ist.
///
/// Die beiden Listen stehen getrennt, weil sie durch verschiedene Ausgänge
/// gehen. Solange sie in einem Feld zusammenlägen, wäre die Trennung eine
/// Absicht und keine Eigenschaft.
///
/// One run of the tool: what was found and what there is to say about it.
///
/// The two lists stand apart because they leave through different exits. As
/// long as they lay in one field, the separation would be an intention and not
/// a property.
#[derive(Debug, PartialEq)]
pub struct Bericht {
    pub treffer: Vec<String>,
    pub meldungen: Vec<String>,
}

/// Liest eine Einstellung aus der Umgebung, mit einer Vorgabe dahinter.
///
/// Diese Funktion steht fertig da. Sie ist die kurze Form für den Fall, dass
/// jeder Fehler dieselbe Antwort bekommen soll: gibt es die Variable nicht,
/// gilt die Vorgabe. Wer die Gründe auseinanderhalten will, nimmt Aufgabe 1.
///
/// Reads a setting from the environment, with a default behind it.
///
/// This function stands there finished. It is the short form for the case where
/// every error should get the same answer: if the variable is not there, the
/// default holds. Whoever wants to keep the reasons apart takes exercise 1.
///
/// ```
/// use unit_06_03_umgebungsvariablen_und_stderr::aus_der_umgebung;
///
/// // Deutsch: Diesen Namen setzt niemand, also greift die Vorgabe.
/// // English: nobody sets this name, so the default takes hold.
/// assert_eq!(aus_der_umgebung("LR_06_03_NICHT_GESETZT", "an"), "an");
/// ```
pub fn aus_der_umgebung(name: &str, vorgabe: &str) -> String {
    match env::var(name) {
        Ok(wert) => wert,
        Err(_) => vorgabe.to_string(),
    }
}

/// Aufgabe 1: Beantworte die fehlende Variable mit `vorgabe`, sonst nichts.
///
/// `gelesen` ist, was `std::env::var` zurückgegeben hat. Steht ein Wert da,
/// kommt er zurück. Fehlt die Variable, kommt `vorgabe` zurück, und zwar als
/// `Ok`. Jeder andere Fehler wird weitergereicht.
///
/// Die Fälle stehen in `env::VarError`. `NotPresent` heißt, dass niemand die
/// Variable gesetzt hat. `NotUnicode` heißt, dass sie gesetzt ist und ihr Wert
/// kein gültiger Unicode-Text ist, und das ist keine fehlende Einstellung,
/// sondern eine kaputte.
///
/// Exercise 1: answer the missing variable with `vorgabe`, and nothing else.
///
/// `gelesen` is what `std::env::var` returned. If a value stands there, it
/// comes back. If the variable is missing, `vorgabe` comes back, and as an
/// `Ok`. Every other error is passed on.
///
/// The cases live in `env::VarError`. `NotPresent` means nobody set the
/// variable. `NotUnicode` means it is set and its value is not valid Unicode
/// text, and that is not a missing setting but a broken one.
pub fn einstellung(
    gelesen: Result<String, env::VarError>,
    vorgabe: &str,
) -> Result<String, env::VarError> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Sammle die passenden Zeilen und sag dazu, wonach gesucht wurde.
///
/// In `treffer` kommt jede Zeile, die `muster` als Teiltext hat, in der
/// Reihenfolge, in der sie hereinkam. In `meldungen` steht immer zuerst die
/// Zeile `gesucht wird nach <muster>`. Ist kein Treffer dabei, folgt darauf
/// eine zweite Zeile `kein Treffer`.
///
/// Die Meldung gehört nicht zum Ergebnis. Sie steht hier nur deshalb im selben
/// Wert, weil Aufgabe 3 sie gleich wieder auseinandernimmt.
///
/// Exercise 2: collect the matching lines and say what was searched for.
///
/// Into `treffer` goes every line that has `muster` as a substring, in the
/// order it came in. In `meldungen` the line `gesucht wird nach <muster>`
/// always stands first. If there is no hit among them, a second line
/// `kein Treffer` follows it.
///
/// The message is not part of the result. It only sits in the same value here
/// because exercise 3 takes it apart again right away.
pub fn bericht(zeilen: &[String], muster: &str) -> Bericht {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Schreib die Meldungen nach `fehler` und die Treffer nach `aus`.
///
/// Jede Meldung geht mit einem `\n` dahinter nach `fehler`, jeder Treffer mit
/// einem `\n` dahinter nach `aus`. Erst die Meldungen, dann die Treffer. Was
/// nach `aus` geht, enthält keine Meldung, denn das ist der Ausgang, den
/// jemand weiterleitet.
///
/// Geschrieben wird mit `writeln!`, das auf jedem `std::io::Write` arbeitet.
/// Deshalb stehen hier zwei Parameter statt `println!` und `eprintln!`: ein
/// Test kann zwei Puffer hineingeben und danach beide einzeln ansehen.
///
/// Exercise 3: write the messages to `fehler` and the hits to `aus`.
///
/// Every message goes to `fehler` with a `\n` behind it, every hit goes to
/// `aus` with a `\n` behind it. The messages first, then the hits. What goes to
/// `aus` carries no message, because that is the exit somebody redirects.
///
/// Writing goes through `writeln!`, which works on every `std::io::Write`. That
/// is why two parameters stand here rather than `println!` and `eprintln!`: a
/// test can hand in two buffers and look at each of them afterwards.
pub fn schreiben(
    bericht: &Bericht,
    aus: &mut impl Write,
    fehler: &mut impl Write,
) -> io::Result<()> {
    todo!("Aufgabe 3 / Exercise 3")
}
