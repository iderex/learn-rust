//! 06-06 Cargo-Profile und cargo doc / Cargo profiles and cargo doc
//!
//! Deutsch: Derselbe Quelltext wird von Cargo auf zwei Arten gebaut. Was sich
//! dabei ändert, ist nicht nur die Geschwindigkeit, sondern auch, ob eine
//! Rechnung mit Überlauf abbricht oder weiterläuft. Wer sich darauf verlässt,
//! verlässt sich auf eine Einstellung.
//!
//! English: the same source is built by Cargo in two ways. What changes with it
//! is not only the speed but also whether a calculation with an overflow aborts
//! or carries on. Whoever relies on that relies on a setting.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Sagt, mit welchem Profil dieser Lauf gebaut wurde.
///
/// Diese Funktion steht fertig da. `cfg!(debug_assertions)` ist zur Bauzeit
/// entweder `true` oder `false` und steht danach als feste Zahl im Programm.
/// Der Name der Einstellung nennt die Zusicherungen, sie schaltet aber mehr als
/// die: Im `dev`-Profil ist sie an, und mit ihr ist auch die Überlaufprüfung an.
///
/// Says which profile this run was built with.
///
/// This function stands there finished. `cfg!(debug_assertions)` is either
/// `true` or `false` at build time and stands in the program as a fixed value
/// afterwards. The name of the setting mentions the assertions but it switches
/// more than those: in the `dev` profile it is on, and with it the overflow
/// check is on as well.
///
/// # Beispiele / Examples
///
/// Deutsch: Das Beispiel prüft nur, dass eine der beiden Antworten kommt. Es
/// darf `cfg!(debug_assertions)` nicht gegen diese Funktion halten, denn ein
/// Doku-Test wird nicht mit dem Profil des Laufs gebaut. Nachgemessen ist das
/// in der README unter "Die Erklärung".
///
/// English: the example only checks that one of the two answers comes. It may
/// not hold `cfg!(debug_assertions)` against this function, because a doc test
/// is not built with the profile of the run. That is measured in the README
/// under "The explanation".
///
/// ```
/// use unit_06_06_cargo_profile_und_cargo_doc::profile_name;
///
/// let name = profile_name();
///
/// assert!(name == "debug" || name == "release");
/// ```
pub fn profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

/// Aufgabe 1: Zähle die Werte zusammen und sage es, wenn es nicht aufgeht.
///
/// Zurück kommt `Some(summe)`, solange alles in ein `u8` passt, und `None`,
/// sobald es das nicht tut. Eine leere Liste ergibt `Some(0)`.
///
/// Der Punkt der Aufgabe ist, dass die Antwort in beiden Profilen dieselbe ist.
/// Ein einfaches `+` bricht im `dev`-Profil ab und rechnet im `release`-Profil
/// weiter, `u8::checked_add` tut in beiden dasselbe.
///
/// Exercise 1: add the values up and say so when it does not work out.
///
/// What comes back is `Some(summe)` for as long as everything fits into a `u8`,
/// and `None` as soon as it does not. An empty list gives `Some(0)`.
///
/// The point of the exercise is that the answer is the same in both profiles. A
/// plain `+` aborts in the `dev` profile and carries on in the `release`
/// profile, `u8::checked_add` does the same in both.
pub fn sum_checked(werte: &[u8]) -> Option<u8> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Zähle die Werte zusammen und lass sie absichtlich überlaufen.
///
/// Zurück kommt die Summe, die beim Überlauf vorn wieder anfängt. Eine leere
/// Liste ergibt 0.
///
/// Das ist die zweite ausdrückliche Antwort auf dieselbe Frage. Sie steht neben
/// Aufgabe 1, damit sichtbar ist, dass es zwei Antworten gibt und dass keine
/// davon das Profil ist. Zu benutzen ist `u8::wrapping_add`.
///
/// Exercise 2: add the values up and let them overflow on purpose.
///
/// What comes back is the sum that starts from the front again on an overflow.
/// An empty list gives 0.
///
/// This is the second explicit answer to the same question. It stands next to
/// exercise 1 so that it is visible that there are two answers and that neither
/// of them is the profile. What to use is `u8::wrapping_add`.
pub fn sum_wrapping(werte: &[u8]) -> u8 {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Halbiere eine gerade Zahl.
///
/// Ist `wert` gerade, kommt die Hälfte zurück. Ist er ungerade, bricht die
/// Funktion mit der Meldung `nur gerade Zahlen` ab.
///
/// Diese Aufgabe gehört zur zweiten Hälfte der Einheit. Ein Abbruch gehört in
/// den Doku-Kommentar, unter eine eigene Überschrift `# Panics`, denn `cargo
/// doc` setzt diese Überschriften ab und ein Leser sucht dort danach. Ob die
/// Überschrift da ist, prüft nichts. Die Meldung selbst prüft ein Test.
///
/// Exercise 3: halve an even number.
///
/// If `wert` is even, half of it comes back. If it is odd, the function aborts
/// with the message `nur gerade Zahlen`.
///
/// This exercise belongs to the second half of the unit. An abort belongs in the
/// doc comment, under a heading of its own, `# Panics`, because `cargo doc` sets
/// those headings apart and a reader looks for them there. Whether the heading is
/// there is checked by nothing. The message itself is checked by a test.
///
/// Deutsch: Für die Frage "ist die Zahl gerade" weist clippy an einem
/// vorzeichenlosen Typ `wert % 2 == 0` zurück und verlangt
/// `wert.is_multiple_of(2)`. Der Prüflauf dieses Repositories fährt clippy mit
/// `-D warnings`, also ist das hier kein Vorschlag.
///
/// English: for the question "is the number even", clippy refuses
/// `wert % 2 == 0` on an unsigned type and asks for `wert.is_multiple_of(2)`.
/// The check run of this repository drives clippy with `-D warnings`, so this is
/// not a suggestion here.
pub fn half_even(wert: u8) -> u8 {
    todo!("Aufgabe 3 / Exercise 3")
}
