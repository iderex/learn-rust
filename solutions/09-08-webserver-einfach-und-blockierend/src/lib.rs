//! 09-08 Der Webserver, einfach und blockierend / The web server, simple and
//! blocking, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/09-08-webserver-einfach-und-blockierend/README.md`. Hier stehen nur
//! die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-08-webserver-einfach-und-blockierend/README.md`. What is here is
//! only the bodies that turn the unit's tests green.

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpListener;

/// Baut eine Antwort aus Statuszeile und Rumpf.
///
/// Builds a response out of a status line and a body.
///
/// ```
/// use unit_09_08_webserver_einfach_und_blockierend::antwort;
///
/// let text = antwort("200 OK", "Hallo");
///
/// assert_eq!(text, "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo");
/// assert!(text.ends_with("\r\n\r\nHallo"));
/// ```
pub fn antwort(status: &str, rumpf: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\nContent-Length: {laenge}\r\n\r\n{rumpf}",
        laenge = rumpf.len()
    )
}

/// Liest den Kopf einer Anfrage, also alle Zeilen bis zur ersten leeren.
///
/// Reads the head of a request, meaning every line up to the first empty one.
///
/// ```
/// use std::io::BufReader;
/// use unit_09_08_webserver_einfach_und_blockierend::kopf_lesen;
///
/// let anfrage = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
/// let mut leser = BufReader::new(&anfrage[..]);
///
/// assert_eq!(
///     kopf_lesen(&mut leser).unwrap(),
///     vec!["GET / HTTP/1.1", "Host: 127.0.0.1"]
/// );
/// ```
pub fn kopf_lesen(leser: &mut impl BufRead) -> io::Result<Vec<String>> {
    let mut zeilen = Vec::new();
    let mut zeile = String::new();

    loop {
        zeile.clear();
        if leser.read_line(&mut zeile)? == 0 {
            break;
        }

        let ohne_ende = zeile.trim_end_matches(['\r', '\n']);
        if ohne_ende.is_empty() {
            break;
        }

        zeilen.push(ohne_ende.to_string());
    }

    Ok(zeilen)
}

/// Holt den Pfad aus der ersten Zeile einer Anfrage.
///
/// Gets the path out of the first line of a request.
pub fn pfad_aus(anfragezeile: &str) -> Option<&str> {
    let mut teile = anfragezeile.split(' ');

    let verb = teile.next()?;
    let pfad = teile.next()?;
    let version = teile.next()?;

    if verb != "GET" || !version.starts_with("HTTP/") {
        return None;
    }

    Some(pfad)
}

/// Baut die Antwort, die zu einem Pfad gehört.
///
/// Builds the response that belongs to a path.
pub fn antwort_fuer(pfad: &str) -> String {
    match pfad {
        "/" => antwort("200 OK", "Hallo"),
        "/rust" => antwort("200 OK", "Hallo, Rust"),
        _ => antwort("404 NOT FOUND", "Nicht gefunden"),
    }
}

/// Bedient eine Anfrage aus `leser` und schreibt die Antwort nach `schreiber`.
///
/// Serves a request out of `leser` and writes the response into `schreiber`.
pub fn bediene_einen(leser: &mut impl BufRead, schreiber: &mut impl Write) -> io::Result<()> {
    let kopf = kopf_lesen(leser)?;

    let text = match kopf.first().map(String::as_str).and_then(pfad_aus) {
        Some(pfad) => antwort_fuer(pfad),
        None => antwort("400 BAD REQUEST", "Kaputte Anfrage"),
    };

    schreiber.write_all(text.as_bytes())?;
    schreiber.flush()
}

/// Nimmt genau eine Verbindung an und bedient sie.
///
/// Takes exactly one connection and serves it.
pub fn bediene_einmal(lauscher: &TcpListener) -> io::Result<()> {
    let (strom, _) = lauscher.accept()?;

    // Deutsch: Gelesen und geschrieben wird über dieselbe Verbindung. Eine
    // Referenz auf sie kann beides, deshalb kommen hier zwei davon vor.
    // English: reading and writing go over the same connection. A reference to
    // it can do both, which is why two of them appear here.
    let mut leser = BufReader::new(&strom);
    let mut schreiber = &strom;

    bediene_einen(&mut leser, &mut schreiber)
}
