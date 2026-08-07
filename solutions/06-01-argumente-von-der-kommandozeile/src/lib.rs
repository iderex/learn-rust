//! 06-01 Argumente von der Kommandozeile / Command line arguments, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/06-01-argumente-von-der-kommandozeile/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/06-01-argumente-von-der-kommandozeile/README.md`. What is here is
//! only the bodies that turn the unit's tests green.

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
/// Returns the name of the program, without the path in front of it.
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

/// Liest aus den Argumenten einen vollständigen Aufruf.
///
/// Reads a complete call out of the arguments.
pub fn parse(args: &[String]) -> Result<Aufruf, String> {
    match args.len() {
        0 | 1 => Err(String::from("es fehlen das Muster und die Datei")),
        2 => Err(String::from("es fehlt die Datei")),
        3 => Ok(Aufruf {
            muster: args[1].clone(),
            datei: args[2].clone(),
        }),
        4 => Err(String::from("es ist 1 Argument zu viel")),
        laenge => Err(format!("es sind {} Argumente zu viel", laenge - 3)),
    }
}

/// Schreibt die Zeile, die den richtigen Aufruf zeigt.
///
/// Writes the line showing the right call.
pub fn usage(programm: &str) -> String {
    format!("Aufruf: {programm} <Muster> <Datei>")
}

/// Beantwortet einen Aufruf, der nicht aufgeht.
///
/// Answers a call that does not add up.
pub fn answer(args: &[String]) -> Option<String> {
    match parse(args) {
        Ok(_) => None,
        Err(grund) => Some(format!("{grund}\n{}", usage(program_name(args)))),
    }
}
