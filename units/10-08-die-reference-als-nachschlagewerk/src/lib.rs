//! 10-08 Die Reference als Nachschlagewerk / The Reference as a lookup
//!
//! Deutsch: Die Reference wird nicht gelesen, sie wird gefragt. Jede Regel
//! darin trägt eine Marke wie `[expr.as.numeric.float-as-int]`, und eine
//! Antwort ohne diese Stelle ist geraten. Die vier Funktionen hier sind vier
//! Fragen, deren Antwort dort steht und sonst nirgends.
//!
//! English: the Reference is not read, it is asked. Every rule in it carries a
//! tag such as `[expr.as.numeric.float-as-int]`, and an answer without that
//! place is a guess. The four functions here are four questions whose answer
//! stands there and nowhere else.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Wandelt eine Kommazahl mit `as` in ein `u8`.
///
/// Diese Funktion steht fertig da und ist die Vorlage für die Aufgaben. Sie
/// beantwortet keine Frage über Rust, sie gibt die Antwort weiter, die in der
/// Reference unter 8.2.4 "Operator expressions" steht: zur Null hin gerundet,
/// `NaN` wird 0, und was nicht hineinpasst, sättigt an der Grenze statt
/// umzulaufen. Das ist der Unterschied zu C, wo derselbe Fall undefiniert ist.
///
/// Converts a floating point number into a `u8` with `as`.
///
/// This function stands there finished and is the model for the exercises. It
/// answers no question about Rust, it passes on the answer that stands in the
/// Reference under 8.2.4 "Operator expressions": rounded towards zero, `NaN`
/// becomes 0, and what does not fit saturates at the bound instead of wrapping
/// around. That is the difference to C, where the same case is undefined.
///
/// ```
/// use unit_10_08_die_reference_als_nachschlagewerk::saturating_to_u8;
///
/// assert_eq!(saturating_to_u8(42.9), 42);
/// assert_eq!(saturating_to_u8(300.0), 255);
/// assert_eq!(saturating_to_u8(-1.5), 0);
/// assert_eq!(saturating_to_u8(f64::NAN), 0);
/// ```
pub fn saturating_to_u8(zahl: f64) -> u8 {
    zahl as u8
}

/// Aufgabe 1: Wandle eine ganze Zahl mit `as` in ein `u8`.
///
/// Die Frage lautet: was macht `as` zwischen zwei ganzen Zahlen, wenn der Wert
/// nicht hineinpasst? Sie sättigt hier gerade nicht, und das ist der Grund, die
/// Regel nachzuschlagen statt sie von der Funktion darüber abzuleiten. Die
/// Antwort steht unter 8.2.4 "Operator expressions" bei den numerischen Casts.
///
/// Exercise 1: convert an integer into a `u8` with `as`.
///
/// The question is: what does `as` do between two integers when the value does
/// not fit? It does not saturate here, and that is the reason to look the rule
/// up rather than carry it over from the function above. The answer stands
/// under 8.2.4 "Operator expressions" with the numeric casts.
pub fn truncating_to_u8(zahl: i32) -> u8 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Addiere zwei `u8`, ohne dass ein Überlauf das Programm anhält.
///
/// Passt die Summe, kommt `Some` heraus, sonst `None`. Die Reference sagt unter
/// 8.2.4 "Operator expressions" im Abschnitt "Overflow", dass die Rechenzeichen
/// im Debug-Bau bei einem Überlauf anhalten. Gesucht ist deshalb die Frage nach
/// der Summe und nicht die Summe selbst.
///
/// Exercise 2: add two `u8` without an overflow stopping the program.
///
/// If the sum fits, `Some` comes out, otherwise `None`. The Reference says
/// under 8.2.4 "Operator expressions" in the section "Overflow" that the
/// arithmetic operators panic on overflow in a debug build. What is wanted here
/// is therefore the question about the sum and not the sum itself.
pub fn sum_without_panic(a: u8, b: u8) -> Option<u8> {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Sage, ob an dieser Adresse ein `u32` liegen dürfte.
///
/// Die Reference führt unter 17.2 "Behavior considered undefined" den Zugriff
/// über einen falsch ausgerichteten Zeiger auf. Undefiniert heißt, dass die
/// Frage vor dem Zugriff beantwortet sein muss, denn hinterher gibt es keine
/// Meldung, an der man es merkt. Was `u32` verlangt, sagt `align_of`.
///
/// Exercise 3: say whether a `u32` would be allowed to sit at this address.
///
/// The Reference lists an access through a misaligned pointer under 17.2
/// "Behavior considered undefined". Undefined means the question has to be
/// answered before the access, because afterwards there is no message by which
/// to notice. What `u32` demands is said by `align_of`.
pub fn is_aligned_for_u32(adresse: usize) -> bool {
    todo!("Aufgabe 3 / Exercise 3")
}
