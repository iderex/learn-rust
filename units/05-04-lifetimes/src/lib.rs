//! 05-04 Lifetimes / Lifetimes
//!
//! Deutsch: Eine Lebensdauer ist ein Name, mit dem der Kopf einer Funktion
//! sagt, woher eine zurückgegebene Referenz stammt. In den einfachen Fällen
//! setzt der Übersetzer den Namen selbst ein.
//!
//! English: a lifetime is a name with which the head of a function says where a
//! returned reference comes from. In the simple cases the compiler fills the
//! name in by itself.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den kürzeren von zwei Texten zurück.
///
/// Diese Funktion steht fertig da und zeigt die Form: ein Name `'a` an beiden
/// Eingaben und an der Ausgabe. Damit lebt das Ergebnis so lange wie die
/// kürzere der beiden Eingaben, und länger darf es niemand halten.
///
/// Returns the shorter of two texts.
///
/// This function stands there finished and shows the shape: one name `'a` on
/// both inputs and on the output. With it the result lives as long as the
/// shorter of the two inputs, and nobody may hold it for longer.
///
/// ```
/// use unit_05_04_lifetimes::shorter;
///
/// assert_eq!(shorter("kurz", "laenger"), "kurz");
/// assert_eq!(shorter("gleich", "gleich"), "gleich");
///
/// let satz = String::from("ein ganzer Satz");
/// assert_eq!(shorter(&satz, "kurz"), "kurz");
/// ```
pub fn shorter<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if links.len() < rechts.len() {
        links
    } else {
        rechts
    }
}

/// Aufgabe 1: Gib den längeren von zwei Texten zurück.
///
/// Der Name `'a` steht schon im Kopf; ohne ihn wäre die Signatur `E0106`.
/// Sind beide gleich lang, kommt der rechte heraus.
///
/// Exercise 1: return the longer of two texts.
///
/// The name `'a` already stands in the head; without it the signature would be
/// `E0106`. If both are the same length, the right one comes out.
pub fn longest<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Gib das erste Wort eines Satzes zurück.
///
/// Hier steht keine Anmerkung, weil der Übersetzer sie einsetzt: eine einzige
/// Eingabereferenz, also bekommt die Ausgabe deren Lebensdauer. Wörter sind
/// durch ein Leerzeichen getrennt; ein Satz ohne Leerzeichen ist ein Wort.
///
/// Exercise 2: return the first word of a sentence.
///
/// No annotation stands here, because the compiler fills it in: a single input
/// reference, so the output gets its lifetime. Words are separated by a space;
/// a sentence without a space is one word.
pub fn first_word(satz: &str) -> &str {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Ein Auszug aus einem Text, der den Text nicht überleben kann.
///
/// An excerpt from a text that cannot outlive the text.
#[derive(Debug, PartialEq)]
pub struct Excerpt<'a> {
    pub teil: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Aufgabe 3: Nimm den ersten Satz des Textes als Auszug.
    ///
    /// Der Satz endet vor dem ersten Punkt. Hat der Text keinen Punkt, ist der
    /// ganze Text der erste Satz. Der Name `'a` am Kopf ist derselbe wie am
    /// struct, und genau das verbietet es, den Auszug länger zu halten als den
    /// Text.
    ///
    /// Exercise 3: take the first sentence of the text as an excerpt.
    ///
    /// The sentence ends before the first full stop. If the text has no full
    /// stop, the whole text is the first sentence. The name `'a` in the head is
    /// the same one as on the struct, and that is exactly what forbids holding
    /// the excerpt for longer than the text.
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
