//! 10-06 FFI mit extern "C" / FFI with extern "C", gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/10-06-ffi-mit-extern-c/README.md`.
//! Hier stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/10-06-ffi-mit-extern-c/README.md`. What is here is only the bodies
//! that turn the unit's tests green.

use std::ffi::{CStr, c_char, c_int};
use std::mem::offset_of;

/// Ein Punkt mit dem Speicherbild, das C zusagt.
///
/// A point with the memory image that C promises.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Punkt {
    pub x: i32,
    pub y: i32,
}

unsafe extern "C" {
    fn abs(zahl: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

/// Der Betrag einer Zahl, gerechnet von der C-Bibliothek.
///
/// The absolute value of a number, worked out by the C library.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::betrag;
///
/// assert_eq!(betrag(-5), Some(5));
/// assert_eq!(betrag(i32::MIN), None);
/// ```
pub fn betrag(zahl: i32) -> Option<i32> {
    if zahl == i32::MIN {
        return None;
    }
    // Sicher, weil: `abs` ist für jedes `c_int` außer dem kleinsten erklärt,
    // und der ist eine Zeile darüber ausgeschlossen.
    //
    // Safe because: `abs` is defined for every `c_int` except the smallest one,
    // and that one is ruled out one line above.
    Some(unsafe { abs(zahl) })
}

/// Die Länge eines C-Textes, gezählt von der C-Bibliothek.
///
/// The length of a C text, counted by the C library.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::laenge_von_c;
///
/// assert_eq!(laenge_von_c(c"hallo"), 5);
/// ```
pub fn laenge_von_c(text: &CStr) -> usize {
    // Sicher, weil: `CStr` sagt zu, dass der Zeiger auf eine mit einer Null
    // abgeschlossene Folge zeigt, die so lange gültig bleibt wie das Ausleihen.
    //
    // Safe because: `CStr` promises that the pointer points at a sequence closed
    // off by a zero which stays valid for as long as the borrow.
    unsafe { strlen(text.as_ptr()) }
}

/// Der Versatz der beiden Felder von `Punkt` in Bytes.
///
/// The offset of the two fields of `Punkt` in bytes.
///
/// ```
/// use unit_10_06_ffi_mit_extern_c::versatz;
///
/// assert_eq!(versatz(), (0, 4));
/// ```
pub fn versatz() -> (usize, usize) {
    (offset_of!(Punkt, x), offset_of!(Punkt, y))
}

/// Der Abstand zweier Zahlen, gerechnet mit `abs`.
///
/// The distance between two numbers, worked out with `abs`.
pub fn abstand(a: i32, b: i32) -> Option<i32> {
    betrag(a.checked_sub(b)?)
}

/// Die Länge bis zur ersten Null, gezählt von `strlen`.
///
/// The length up to the first zero, counted by `strlen`.
pub fn laenge_bis_null(bytes: &[u8]) -> Option<usize> {
    if !bytes.contains(&0) {
        return None;
    }
    // Sicher, weil: Der Bereich enthält eine Null, also hält `strlen` innerhalb
    // von `bytes` an und liest über dessen Ende hinaus nichts. Der Bereich ist
    // für die Dauer des Ausleihens gültig, und niemand schreibt währenddessen
    // hinein.
    //
    // Safe because: the stretch contains a zero, so `strlen` stops inside
    // `bytes` and reads nothing past its end. The stretch is valid for as long
    // as the borrow, and nobody writes into it meanwhile.
    Some(unsafe { strlen(bytes.as_ptr() as *const c_char) })
}

/// Liest einen `Punkt` aus acht Bytes, so wie C sie hinlegt.
///
/// Reads a `Punkt` out of eight bytes, the way C lays them down.
pub fn punkt_aus_bytes(bytes: &[u8]) -> Option<Punkt> {
    let (erste, zweite) = bytes.split_at_checked(4)?;
    Some(Punkt {
        x: i32::from_ne_bytes(erste.try_into().ok()?),
        y: i32::from_ne_bytes(zweite.try_into().ok()?),
    })
}
