//! 05-06 Doku-Tests / Doc tests
//!
//! Deutsch: Ein Beispiel in einem Doku-Kommentar ist kein Zitat, sondern ein
//! Programm. `cargo test` baut es und führt es aus, und deshalb kann es nicht
//! still veralten, während der Code darunter sich ändert.
//!
//! English: an example in a doc comment is not a quotation, it is a program.
//! `cargo test` builds it and runs it, and that is why it cannot go stale in
//! silence while the code below it changes.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Zählt die Wörter in einem Text.
///
/// Diese Funktion steht fertig da und zeigt die einfachste Form eines
/// Doku-Tests: ein Beispiel zwischen Rückstrichen, ganz ohne Zusatz. Es läuft
/// als eigenes Programm gegen die öffentliche Schnittstelle dieses Pakets, und
/// deshalb steht dort ein `use` wie in jedem anderen Programm auch.
///
/// Counts the words in a text.
///
/// This function stands there finished and shows the simplest form of a doc
/// test: an example between backticks, without any addition. It runs as a
/// program of its own against the public interface of this package, and that is
/// why a `use` stands there like in any other program.
///
/// ```
/// use unit_05_06_doku_tests::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// assert_eq!(word_count("   "), 0);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Gibt das Wort an `stelle` zurück und bricht ab, wenn dort keines steht.
///
/// Auch diese Funktion steht fertig da. Sie trägt zwei Beispiele, und das
/// zweite ist mit `should_panic` überschrieben: es gilt als bestanden, wenn es
/// abbricht.
///
/// `should_panic` an einem Doku-Test nimmt keine erwartete Meldung entgegen. Es
/// nimmt also jeden Abbruch an, auch einen aus einem ganz anderen Grund. Wer
/// den Grund festhalten will, schreibt den Test in `tests/exercise.rs`, wo
/// `#[should_panic(expected = "...")]` zur Verfügung steht.
///
/// Returns the word at `stelle` and aborts when there is none there.
///
/// This function stands there finished as well. It carries two examples, and
/// the second one is headed `should_panic`: it counts as passed when it aborts.
///
/// `should_panic` on a doc test takes no expected message. It therefore accepts
/// every abort, including one for an entirely different reason. Whoever wants to
/// pin the reason down writes the test in `tests/exercise.rs`, where
/// `#[should_panic(expected = "...")]` is available.
///
/// ```
/// use unit_05_06_doku_tests::word_at;
///
/// assert_eq!(word_at("ein zwei drei", 1), "zwei");
/// ```
///
/// ```should_panic
/// use unit_05_06_doku_tests::word_at;
///
/// word_at("ein", 5);
/// ```
pub fn word_at(text: &str, stelle: usize) -> &str {
    text.split_whitespace()
        .nth(stelle)
        .expect("an dieser Stelle steht kein Wort")
}

/// Aufgabe 1: Baue die Initialen eines Namens.
///
/// Aus jedem durch Leerraum getrennten Wort wird sein erstes Zeichen groß
/// geschrieben, und hinter jedes kommt ein Punkt. Aus einem Namen ohne Wörter
/// wird der leere Text.
///
/// Das Beispiel unten ist ein Doku-Test und deshalb rot, solange der Rumpf
/// `todo!()` ist. Grün wird er von derselben Zeile, die auch die Tests in
/// `tests/exercise.rs` grün macht.
///
/// Exercise 1: build the initials of a name.
///
/// Every word separated by whitespace contributes its first character in upper
/// case, and a dot follows each one. A name without words becomes the empty
/// text.
///
/// The example below is a doc test and therefore red for as long as the body is
/// `todo!()`. It goes green from the same line that turns the tests in
/// `tests/exercise.rs` green.
///
/// ```
/// use unit_05_06_doku_tests::initials;
///
/// assert_eq!(initials("Ada Lovelace"), "A.L.");
/// assert_eq!(initials("grace hopper"), "G.H.");
/// assert_eq!(initials("   "), "");
/// ```
pub fn initials(name: &str) -> String {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Lies eine Prozentzahl aus einem Text.
///
/// Der Leerraum am Rand wird abgeschnitten, der Rest wird als `u8` gelesen.
/// Alles, was kein `u8` ist, kommt als `Err` zurück, und dazu gehört auch eine
/// Zahl über 255.
///
/// Im Beispiel unten stehen Zeilen mit `#` am Anfang. Sie laufen mit, werden
/// aber nicht abgedruckt. So darf das Beispiel `?` benutzen, ohne dass der
/// Leser das `main` drumherum sehen muss.
///
/// Exercise 2: read a percentage out of a text.
///
/// The whitespace at the edges is cut away, the rest is read as a `u8`.
/// Everything that is not a `u8` comes back as `Err`, and a number above 255
/// belongs to that.
///
/// In the example below there are lines starting with `#`. They run along but
/// are not printed. That way the example may use `?` without the reader having
/// to see the `main` around it.
///
/// ```
/// # use unit_05_06_doku_tests::percent;
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// assert_eq!(percent(" 42 ")?, 42);
/// assert!(percent("dreiundvierzig").is_err());
/// assert!(percent("300").is_err());
/// # Ok(())
/// # }
/// ```
pub fn percent(text: &str) -> Result<u8, std::num::ParseIntError> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib den längeren der beiden Texte zurück.
///
/// Sind beide gleich lang, kommt der erste zurück. Gezählt wird in Bytes, also
/// mit `len`.
///
/// Beide Parameter und der Rückgabewert tragen dieselbe Lebensdauer `'a`, denn
/// der Rückgabewert kommt aus einem der beiden. Das ist der Fall aus
/// `05-04 Lifetimes`, hier mit einem Beispiel, das mitläuft.
///
/// Exercise 3: return the longer of the two texts.
///
/// When both are equally long, the first one comes back. Counting is in bytes,
/// so with `len`.
///
/// Both parameters and the return value carry the same lifetime `'a`, because
/// the return value comes out of one of the two. That is the case from
/// `05-04 Lifetimes`, here with an example that runs along.
///
/// ```
/// use unit_05_06_doku_tests::longest;
///
/// assert_eq!(longest("kurz", "laenger"), "laenger");
/// assert_eq!(longest("gleich", "gleich"), "gleich");
/// ```
pub fn longest<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    todo!("Aufgabe 3 / Exercise 3")
}
