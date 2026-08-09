//! 10-06 FFI mit extern "C" / FFI with extern "C"
//!
//! Deutsch: Hinter `extern "C"` steht Code, den dieser Übersetzer nicht gesehen
//! hat. Er prüft die Deklaration gegen nichts, weil es nichts zum Prüfen gibt.
//! Was auf dieser Seite der Grenze zugesagt wird, gilt weiter. Was auf der
//! anderen Seite zugesagt wird, glaubt man oder liest es nach.
//!
//! English: behind `extern "C"` stands code this compiler has not seen. It
//! checks the declaration against nothing, because there is nothing to check
//! against. What is promised on this side of the boundary keeps holding. What is
//! promised on the other side is believed or looked up.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::ffi::{CStr, c_char, c_int};
use std::mem::offset_of;

/// Ein Punkt mit dem Speicherbild, das C zusagt.
///
/// `#[repr(C)]` ist die Zusage über die Reihenfolge und den Abstand der Felder.
/// Ohne sie ordnet der Übersetzer die Felder so an, wie es ihm passt, und die
/// Anordnung darf sich zwischen zwei Übersetzungen ändern.
///
/// A point with the memory image that C promises.
///
/// `#[repr(C)]` is the promise about the order and the distance of the fields.
/// Without it the compiler arranges the fields as it sees fit, and the
/// arrangement may change between two compilations.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Punkt {
    pub x: i32,
    pub y: i32,
}

// Deutsch: Diese beiden Funktionen liegen in der C-Bibliothek, gegen die dieses
// Programm ohnehin gebunden wird. Der Block trägt `unsafe`, weil in der Ausgabe
// 2024 jeder solche Block eines trägt: Was hier steht, ist eine Behauptung über
// fremden Code, und niemand prüft sie.
// English: these two functions live in the C library this program is linked
// against anyway. The block carries `unsafe`, because in edition 2024 every such
// block carries one: what stands here is a claim about foreign code, and nobody
// checks it.
unsafe extern "C" {
    fn abs(zahl: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

/// Der Betrag einer Zahl, gerechnet von der C-Bibliothek.
///
/// Diese Funktion steht fertig da und ist die Vorlage für die erste Aufgabe.
/// `abs` ist in C für jede Zahl erklärt, deren Betrag sich darstellen lässt.
/// Für `i32::MIN` lässt er sich nicht darstellen, und was C dann tut, ist nicht
/// festgelegt. Deshalb steht die Prüfung vor dem Aufruf und nicht dahinter, und
/// deshalb gibt diese Funktion `Option` heraus statt `i32`.
///
/// The absolute value of a number, worked out by the C library.
///
/// This function stands there finished and is the model for the first exercise.
/// `abs` is defined in C for every number whose absolute value can be
/// represented. For `i32::MIN` it cannot be represented, and what C does then is
/// not laid down. That is why the check stands in front of the call and not
/// behind it, and why this function gives out an `Option` instead of an `i32`.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::betrag;
///
/// assert_eq!(betrag(-5), Some(5));
/// assert_eq!(betrag(5), Some(5));
/// assert_eq!(betrag(0), Some(0));
///
/// // Deutsch: Der Rand, den die C-Zusage nicht deckt.
/// // English: the edge the C promise does not cover.
/// assert_eq!(betrag(i32::MIN), None);
/// ```
pub fn betrag(zahl: i32) -> Option<i32> {
    if zahl == i32::MIN {
        return None;
    }
    // Sicher, weil: `abs` ist für jedes `c_int` außer dem kleinsten erklärt,
    // und der ist eine Zeile darüber ausgeschlossen. Der Aufruf liest keinen
    // Speicher und schreibt keinen.
    //
    // Safe because: `abs` is defined for every `c_int` except the smallest one,
    // and that one is ruled out one line above. The call reads no memory and
    // writes none.
    Some(unsafe { abs(zahl) })
}

/// Die Länge eines C-Textes, gezählt von der C-Bibliothek.
///
/// Diese Funktion steht ebenfalls fertig da. Sie zeigt den angenehmen Fall: Die
/// Bedingung, die `strlen` stellt, nämlich ein mit einer Null abgeschlossener
/// Speicherbereich, ist genau das, was `CStr` zusagt. Die Begründung des
/// `unsafe`-Blocks ist deshalb ein Satz und keine Liste.
///
/// The length of a C text, counted by the C library.
///
/// This function stands there finished as well. It shows the pleasant case: the
/// condition `strlen` puts up, meaning a stretch of memory closed off by a zero,
/// is exactly what `CStr` promises. The argument of the `unsafe` block is
/// therefore one sentence and not a list.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::laenge_von_c;
///
/// assert_eq!(laenge_von_c(c"hallo"), 5);
/// ```
pub fn laenge_von_c(text: &CStr) -> usize {
    // Sicher, weil: `CStr` sagt zu, dass der Zeiger auf eine Folge zeigt, die
    // mit einer Null abgeschlossen ist, und dass sie so lange gültig bleibt wie
    // das Ausleihen.
    //
    // Safe because: `CStr` promises that the pointer points at a sequence closed
    // off by a zero, and that it stays valid for as long as the borrow.
    unsafe { strlen(text.as_ptr()) }
}

/// Der Versatz der beiden Felder von `Punkt` in Bytes.
///
/// Diese Funktion steht fertig da und misst, was `#[repr(C)]` zusagt: Das erste
/// Feld liegt am Anfang, das zweite vier Bytes dahinter. Über einen `Punkt`
/// ohne `#[repr(C)]` sagt niemand dasselbe, und deshalb prüft diese Einheit
/// darüber auch nichts.
///
/// The offset of the two fields of `Punkt` in bytes.
///
/// This function stands there finished and measures what `#[repr(C)]` promises:
/// the first field lies at the start, the second four bytes behind it. Nobody
/// says the same about a `Punkt` without `#[repr(C)]`, and that is why this unit
/// checks nothing about one.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::versatz;
///
/// assert_eq!(versatz(), (0, 4));
/// ```
pub fn versatz() -> (usize, usize) {
    (offset_of!(Punkt, x), offset_of!(Punkt, y))
}

/// Aufgabe 1: Der Abstand zweier Zahlen, gerechnet mit `abs`.
///
/// Zwei Ränder liegen davor, und beide gehören geprüft, bevor der Wert die
/// Grenze überschreitet. Die Subtraktion kann überlaufen, und ihr Ergebnis kann
/// `i32::MIN` sein. In beiden Fällen kommt `None` heraus und `abs` wird nicht
/// gerufen. `checked_sub` beantwortet den ersten Rand.
///
/// Exercise 1: the distance between two numbers, worked out with `abs`.
///
/// Two edges lie in front of it, and both belong checked before the value
/// crosses the boundary. The subtraction can overflow, and its result can be
/// `i32::MIN`. In both cases `None` comes out and `abs` is not called.
/// `checked_sub` answers the first edge.
pub fn abstand(a: i32, b: i32) -> Option<i32> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Die Länge bis zur ersten Null, gezählt von `strlen`.
///
/// `strlen` läuft, bis es eine Null findet. Enthält der Speicherbereich keine,
/// läuft es über sein Ende hinaus, und das ist undefiniertes Verhalten und kein
/// falsches Ergebnis. Die Aufgabe ist deshalb nicht, `strlen` zu rufen, sondern
/// zuerst nachzusehen, ob es aufhören kann, und `None` herauszugeben, wenn es
/// das nicht kann.
///
/// Exercise 2: the length up to the first zero, counted by `strlen`.
///
/// `strlen` runs until it finds a zero. If the stretch of memory contains none,
/// it runs past its end, and that is undefined behaviour and not a wrong result.
/// The exercise is therefore not to call `strlen` but to look first whether it
/// can stop, and to give out `None` when it cannot.
pub fn laenge_bis_null(bytes: &[u8]) -> Option<usize> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Einen `Punkt` aus acht Bytes lesen, so wie C sie hinlegt.
///
/// Die ersten vier Bytes sind `x`, die zweiten vier sind `y`, jeweils in der
/// Bytefolge dieser Maschine. `i32::from_ne_bytes` ist die Umkehrung. Sind es
/// nicht genau acht Bytes, kommt `None` heraus.
///
/// Exercise 3: read a `Punkt` out of eight bytes, the way C lays them down.
///
/// The first four bytes are `x`, the second four are `y`, each in the byte order
/// of this machine. `i32::from_ne_bytes` is the way back. If it is not exactly
/// eight bytes, `None` comes out.
pub fn punkt_aus_bytes(bytes: &[u8]) -> Option<Punkt> {
    todo!("Aufgabe 3 / Exercise 3")
}
