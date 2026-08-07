//! 06-01 Argumente von der Kommandozeile / Command line arguments
//!
//! Deutsch: `std::env::args()` gibt die Argumente eines Aufrufs, das erste ist
//! der Name des Programms. Ein Aufruf, dem etwas fehlt, wird beantwortet und
//! nicht abgebrochen.
//!
//! English: `std::env::args()` gives the arguments of a call, the first one
//! being the name of the program. A call with something missing gets an answer
//! rather than an abort.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Ein vollständiger Aufruf: ein Muster und eine Datei.
///
/// A complete call: one pattern and one file.
#[derive(Debug, PartialEq)]
pub struct Aufruf {
    pub muster: String,
    pub datei: String,
}

/// Gibt den Namen des Programms, ohne den Pfad davor.
///
/// Diese Funktion steht fertig da. Das erste Argument ist der Weg, unter dem
/// das Programm gestartet wurde, und für eine Meldung an den Aufrufer ist davon
/// nur der letzte Teil brauchbar. Eine leere Liste gibt einen leeren Namen.
///
/// Returns the name of the program, without the path in front of it.
///
/// This function stands there finished. The first argument is the way the
/// program was started under, and for a message to the caller only the last
/// part of it is of any use. An empty list gives an empty name.
///
/// ```
/// use unit_06_01_argumente_von_der_kommandozeile::program_name;
///
/// let args = vec![String::from("target/debug/suchen"), String::from("wort")];
/// assert_eq!(program_name(&args), "suchen");
///
/// assert_eq!(program_name(&[String::from("suchen")]), "suchen");
/// assert_eq!(program_name(&[]), "");
/// ```
pub fn program_name(args: &[String]) -> &str {
    let Some(erstes) = args.first() else {
        return "";
    };

    match erstes.rfind(['/', '\\']) {
        Some(stelle) => &erstes[stelle + 1..],
        None => erstes,
    }
}

/// Aufgabe 1: Lies aus den Argumenten einen vollständigen Aufruf.
///
/// `args` ist die ganze Liste, das erste Element ist der Name des Programms.
/// Danach werden genau zwei erwartet, erst das Muster, dann die Datei. Fehlt
/// etwas, sagt der Fehlertext, was fehlt, und zwar
/// `es fehlen das Muster und die Datei` oder `es fehlt die Datei`. Steht zu
/// viel da, lautet er `es ist 1 Argument zu viel` bei einem und
/// `es sind N Argumente zu viel` bei mehreren.
///
/// Exercise 1: read a complete call out of the arguments.
///
/// `args` is the whole list, its first element being the name of the program.
/// After it exactly two are expected, the pattern first and then the file. If
/// something is missing, the error text says what, namely
/// `es fehlen das Muster und die Datei` or `es fehlt die Datei`. If there is
/// too much, it reads `es ist 1 Argument zu viel` for one and
/// `es sind N Argumente zu viel` for several.
pub fn parse(args: &[String]) -> Result<Aufruf, String> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Schreib die Zeile, die den richtigen Aufruf zeigt.
///
/// Sie lautet `Aufruf: suchen <Muster> <Datei>`, wenn `programm` der Name
/// `suchen` ist. Der Name kommt von außen herein, damit die Zeile denselben
/// Namen zeigt, unter dem das Programm wirklich gestartet wurde.
///
/// Exercise 2: write the line showing the right call.
///
/// It reads `Aufruf: suchen <Muster> <Datei>` when `programm` is the name
/// `suchen`. The name comes in from outside, so that the line shows the same
/// name the program was really started under.
pub fn usage(programm: &str) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Beantworte einen Aufruf, der nicht aufgeht.
///
/// Stimmt der Aufruf, gibt es nichts zu sagen und die Antwort ist `None`.
/// Sonst stehen zwei Zeilen da: erst der Grund aus `parse`, dann die Zeile aus
/// `usage` mit dem Namen aus `program_name`. Zwischen beiden steht ein
/// Zeilenumbruch und am Ende keiner.
///
/// Exercise 3: answer a call that does not add up.
///
/// Where the call is right there is nothing to say and the answer is `None`.
/// Otherwise two lines stand there: first the reason from `parse`, then the
/// line from `usage` with the name from `program_name`. Between the two is a
/// line break, and at the end there is none.
pub fn answer(args: &[String]) -> Option<String> {
    todo!("Aufgabe 3 / Exercise 3")
}
