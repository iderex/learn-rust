//! 06-06 Cargo-Profile und cargo doc / Cargo profiles and cargo doc, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/06-06-cargo-profile-und-cargo-doc/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/06-06-cargo-profile-und-cargo-doc/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

/// Sagt, mit welchem Profil dieser Lauf gebaut wurde.
///
/// Says which profile this run was built with.
///
/// # Beispiele / Examples
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

/// Zählt die Werte zusammen und sagt es, wenn es nicht aufgeht.
///
/// Adds the values up and says so when it does not work out.
pub fn sum_checked(werte: &[u8]) -> Option<u8> {
    let mut summe: u8 = 0;

    for wert in werte {
        summe = summe.checked_add(*wert)?;
    }

    Some(summe)
}

/// Zählt die Werte zusammen und lässt sie überlaufen.
///
/// Adds the values up and lets them overflow.
pub fn sum_wrapping(werte: &[u8]) -> u8 {
    let mut summe: u8 = 0;

    for wert in werte {
        summe = summe.wrapping_add(*wert);
    }

    summe
}

/// Halbiert eine gerade Zahl.
///
/// Halves an even number.
///
/// # Panics
///
/// Deutsch: Bricht mit `nur gerade Zahlen` ab, wenn `wert` ungerade ist.
///
/// English: aborts with `nur gerade Zahlen` when `wert` is odd.
pub fn half_even(wert: u8) -> u8 {
    assert!(wert.is_multiple_of(2), "nur gerade Zahlen");

    wert / 2
}
