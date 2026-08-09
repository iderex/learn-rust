//! 10-04 Varianz / Variance
//!
//! Deutsch: Eine Lebenszeit lässt sich verkürzen, wenn der Typ darum herum es
//! zulässt. Ob er es zulässt, heißt seine Varianz. Beim Lesen ist sie kovariant,
//! bei einem Parameter kontravariant, und wo gelesen und geschrieben wird, ist
//! sie invariant und lässt gar nichts zu.
//!
//! English: a lifetime can be shortened when the type around it allows it.
//! Whether it allows it is called its variance. Reading is covariant, a
//! parameter is contravariant, and where reading and writing meet it is
//! invariant and allows nothing at all.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Eine Notiz, die eine Referenz nur zum Lesen hält.
///
/// A note holding a reference for reading only.
#[derive(Debug, PartialEq)]
pub struct Notiz<'a> {
    pub text: &'a str,
}

/// Liest eine ewig lebende Notiz als eine, die nur kurz lebt.
///
/// Diese Funktion steht fertig da. Ihr Rumpf gibt zurück, was hereinkam, und
/// trotzdem ist sie die ganze Aussage dieser Einheit: `Notiz<'static>` und
/// `Notiz<'kurz>` sind zwei verschiedene Typen, und nur weil `Notiz` kovariant
/// in `'a` ist, darf der eine als der andere gelesen werden. Hielte `Notiz`
/// ihren Text hinter `&mut`, stünde hier eine Ablehnung statt einer Zeile.
///
/// Reads a note that lives forever as one that lives only briefly.
///
/// This function stands there finished. Its body gives back what came in, and
/// it is still the whole claim of this unit: `Notiz<'static>` and `Notiz<'kurz>`
/// are two different types, and only because `Notiz` is covariant in `'a` may
/// the one be read as the other. If `Notiz` held its text behind `&mut`, a
/// refusal would stand here instead of a line.
///
/// ```
/// use unit_10_04_varianz::{Notiz, kuerzer};
///
/// // Deutsch: Diese Funktion verlangt zwei Notizen mit derselben Lebenszeit.
/// // English: this function asks for two notes with the same lifetime.
/// fn zusammen<'a>(eine: Notiz<'a>, andere: Notiz<'a>) -> usize {
///     eine.text.len() + andere.text.len()
/// }
///
/// let ewig: Notiz<'static> = Notiz { text: "ewig" };
/// let text = String::from("kurz");
/// let kurze = Notiz { text: &text };
///
/// // Deutsch: Erst das Verkuerzen bringt die ewige Notiz auf die Lebenszeit
/// // der anderen. Ohne Kovarianz gaebe es diese Zeile nicht.
/// // English: only the shortening brings the everlasting note onto the
/// // lifetime of the other. Without covariance this line would not exist.
/// assert_eq!(zusammen(kuerzer(ewig), kurze), 8);
/// ```
pub fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

/// Wendet einen Zeiger auf eine Funktion auf einen ewig lebenden Text an.
///
/// Diese Funktion steht ebenfalls fertig da und zeigt die andere Richtung. Der
/// Parameter kommt als `fn(&'kurz str) -> usize` herein und wird einer Variablen
/// vom Typ `fn(&'static str) -> usize` zugewiesen, ohne dass etwas umgewandelt
/// wird. Erlaubt ist das, weil der Parameter einer Funktion kontravariant ist:
/// Wer weniger verlangt, passt dorthin, wo mehr geboten wird. Die umgekehrte
/// Zuweisung wird abgelehnt.
///
/// Applies a function pointer to a text that lives forever.
///
/// This function stands there finished as well and shows the other direction.
/// The parameter comes in as `fn(&'kurz str) -> usize` and is assigned to a
/// variable of type `fn(&'static str) -> usize` with nothing converted. That is
/// allowed because the parameter of a function is contravariant: whoever asks
/// for less fits where more is offered. The assignment the other way round is
/// refused.
///
/// ```
/// use unit_10_04_varianz::laenge_unter;
///
/// assert_eq!(laenge_unter(str::len), 4);
/// ```
pub fn laenge_unter<'kurz>(f: fn(&'kurz str) -> usize) -> usize {
    let fuer_ewig: fn(&'static str) -> usize = f;
    fuer_ewig("ewig")
}

/// Aufgabe 1: Gib den längeren der beiden Texte heraus.
///
/// Sind beide gleich lang, gewinnt der erste. Die Signatur ist die Stelle, an
/// der Kovarianz ihre tägliche Arbeit tut: Der Aufrufer darf einen ewig
/// lebenden Text und einen kurzlebigen übergeben, denn beide dürfen auf die
/// gemeinsame Lebenszeit `'a` verkürzt werden. Ohne Kovarianz müssten beide
/// genau dieselbe Lebenszeit haben.
///
/// Exercise 1: give out the longer of the two texts.
///
/// If both are equally long, the first one wins. The signature is where
/// covariance does its daily work: the caller may pass a text that lives
/// forever and a short lived one, because both may be shortened to the shared
/// lifetime `'a`. Without covariance both would have to carry exactly the same
/// lifetime.
pub fn laengere<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Setze einen neuen Text in eine Notiz und gib den alten heraus.
///
/// Hier steht die Invarianz. `&mut Notiz<'a>` ist invariant über `'a`, und
/// deshalb muss `neu` genau `'a` tragen und nicht bloß länger leben. Wer
/// versucht, die Signatur zu `neu: &'lang str` mit `'lang: 'a` zu lockern, wird
/// abgelehnt, und die Meldung nennt die Invarianz beim Namen.
///
/// Exercise 2: put a new text into a note and give out the old one.
///
/// This is where invariance stands. `&mut Notiz<'a>` is invariant over `'a`, and
/// that is why `neu` has to carry exactly `'a` and not merely outlive it.
/// Whoever tries to loosen the signature to `neu: &'lang str` with `'lang: 'a`
/// is refused, and the message names the invariance.
pub fn ersetzen<'a>(notiz: &mut Notiz<'a>, neu: &'a str) -> &'a str {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Finde den kürzesten Text unter den Notizen.
///
/// Ist die Liste leer, kommt `None` heraus. Sind zwei gleich kurz, gewinnt die
/// erste. Der Rückgabetyp ist `Option<&'a str>` und nicht `Option<&str>` mit der
/// Lebenszeit der Liste: Die Referenz kommt aus der Notiz und nicht aus dem
/// Ausleihen der Liste, und weil `&[Notiz<'a>]` kovariant ist, überlebt sie das
/// Ende dieses Ausleihens.
///
/// Exercise 3: find the shortest text among the notes.
///
/// If the list is empty, `None` comes out. If two are equally short, the first
/// one wins. The return type is `Option<&'a str>` and not `Option<&str>` with
/// the lifetime of the list: the reference comes out of the note and not out of
/// borrowing the list, and because `&[Notiz<'a>]` is covariant it outlives the
/// end of that borrow.
pub fn kuerzeste<'a>(notizen: &[Notiz<'a>]) -> Option<&'a str> {
    todo!("Aufgabe 3 / Exercise 3")
}
