//! 01-03 Funktionen / Functions
//!
//! Deutsch: Eine Funktion bekommt einen Namen, Parameter mit Typen und einen
//! Rückgabetyp. Der letzte Ausdruck im Rumpf ist der Rückgabewert, und zwar
//! genau dann, wenn kein Semikolon dahintersteht.
//!
//! English: a function gets a name, parameters with types, and a return type.
//! The last expression in the body is the return value, and it is exactly then,
//! when no semicolon stands behind it.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Gibt den Umfang eines Rechtecks zurück.
///
/// Returns the perimeter of a rectangle.
///
/// ```
/// use unit_01_03_funktionen::perimeter;
///
/// assert_eq!(perimeter(2, 3), 10);
/// ```
pub fn perimeter(breite: u32, hoehe: u32) -> u32 {
    2 * (breite + hoehe)
}

/// Aufgabe 1: Gib die Fläche eines Rechtecks zurück.
///
/// Exercise 1: return the area of a rectangle.
pub fn area(breite: u32, hoehe: u32) -> u32 {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Rechne eine Temperatur von Fahrenheit in Celsius um.
///
/// Die Rechnung ist `(fahrenheit - 32) * 5 / 9`. Schreibe die Zahlen mit
/// Nachkommastelle, sonst passen die Typen nicht zusammen.
///
/// Exercise 2: convert a temperature from Fahrenheit to Celsius.
///
/// The calculation is `(fahrenheit - 32) * 5 / 9`. Write the numbers with a
/// decimal place, otherwise the types do not fit together.
pub fn celsius_from(fahrenheit: f64) -> f64 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Gib die Fläche eines Quadrats mit der Seitenlänge `seite` zurück.
///
/// Rufe dafür `area` auf, statt noch einmal zu multiplizieren. Eine Funktion
/// darf eine andere aufrufen, und genau dafür gibt es sie.
///
/// Exercise 3: return the area of a square with side length `seite`.
///
/// Call `area` for it instead of multiplying a second time. A function may call
/// another one, and that is exactly what they are for.
pub fn square_area(seite: u32) -> u32 {
    todo!("Aufgabe 3 / Exercise 3")
}
