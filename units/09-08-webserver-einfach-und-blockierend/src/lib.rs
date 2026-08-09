//! 09-08 Der Webserver, einfach und blockierend / The web server, simple and
//! blocking
//!
//! Deutsch: Ein Anschluss, eine Anfrage lesen, eine Antwort schreiben. Mehr ist
//! ein HTTP-Server nicht, solange er nur eine Anfrage zur selben Zeit bedient.
//! Warum er das tut und was das kostet, steht in der README.
//!
//! English: one socket, reading a request, writing a response. That is all an
//! HTTP server is, as long as it serves only one request at a time. Why it does
//! that and what it costs is written in the README.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::io::{self, BufRead, Write};
use std::net::TcpListener;

/// Baut eine Antwort aus Statuszeile und Rumpf.
///
/// Diese Funktion steht fertig da und zeigt, wie eine HTTP-Antwort aussieht.
/// Die Zeilen enden mit `\r\n` und nicht mit `\n`; das steht so im Protokoll
/// und ist keine Geschmacksfrage. Zwischen Kopf und Rumpf steht eine leere
/// Zeile, also zweimal `\r\n` hintereinander.
///
/// `Content-Length` sagt, wie viele Bytes danach kommen. Ohne diese Angabe
/// weiß die Gegenseite nicht, wann die Antwort zu Ende ist, und wartet.
///
/// Builds a response out of a status line and a body.
///
/// This function stands there finished and shows what an HTTP response looks
/// like. The lines end with `\r\n` and not with `\n`; that is what the protocol
/// says and it is not a matter of taste. Between the head and the body stands an
/// empty line, meaning two `\r\n` in a row.
///
/// `Content-Length` says how many bytes come afterwards. Without it the other
/// side does not know when the response is over, and waits.
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
/// Diese Funktion steht ebenfalls fertig da. Sie liest bis zur leeren Zeile und
/// nicht bis zum Ende, denn ein Ende kommt nicht: Die Gegenseite hält die
/// Verbindung offen und wartet auf die Antwort. Wer hier auf das Ende wartet,
/// wartet auf jemanden, der auf ihn wartet.
///
/// Gelesen wird trotzdem der ganze Kopf und nicht nur die erste Zeile. Was
/// ungelesen im Puffer liegt, wenn die Verbindung zugeht, kann auf der anderen
/// Seite als abgebrochene Verbindung ankommen statt als Antwort.
///
/// Reads the head of a request, meaning every line up to the first empty one.
///
/// This function stands there finished as well. It reads up to the empty line
/// and not up to the end, because no end comes: the other side keeps the
/// connection open and waits for the answer. Whoever waits for the end here
/// waits for somebody who is waiting for them.
///
/// The whole head is read all the same, and not just the first line. What lies
/// unread in the buffer when the connection closes can arrive on the other side
/// as a broken connection instead of as an answer.
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

/// Aufgabe 1: Hol den Pfad aus der ersten Zeile einer Anfrage.
///
/// Die erste Zeile sieht so aus: `GET /rust HTTP/1.1`. Drei Teile, durch je ein
/// Leerzeichen getrennt, und der mittlere ist der Pfad.
///
/// Zurück kommt `None`, wenn das Verb nicht `GET` ist, wenn der dritte Teil
/// nicht mit `HTTP/` anfängt oder wenn weniger als drei Teile dastehen. Dieser
/// Server kann genau ein Verb, und was er nicht kann, gibt er auch nicht vor zu
/// können.
///
/// Exercise 1: get the path out of the first line of a request.
///
/// The first line looks like this: `GET /rust HTTP/1.1`. Three parts, separated
/// by one space each, and the middle one is the path.
///
/// `None` comes back when the verb is not `GET`, when the third part does not
/// start with `HTTP/`, or when fewer than three parts stand there. This server
/// knows exactly one verb, and it does not pretend to know what it does not.
pub fn pfad_aus(anfragezeile: &str) -> Option<&str> {
    todo!("Aufgabe 1 / Exercise 1")
}

/// Aufgabe 2: Bau die Antwort, die zu einem Pfad gehört.
///
/// `/` beantwortet der Server mit `200 OK` und dem Rumpf `Hallo`, `/rust` mit
/// `200 OK` und dem Rumpf `Hallo, Rust`. Auf alles andere kommt
/// `404 NOT FOUND` mit dem Rumpf `Nicht gefunden`.
///
/// Gebaut wird die Antwort mit `antwort`, damit die Kopfzeilen an einer Stelle
/// stehen und nicht dreimal.
///
/// Exercise 2: build the response that belongs to a path.
///
/// `/` is answered with `200 OK` and the body `Hallo`, `/rust` with `200 OK` and
/// the body `Hallo, Rust`. Everything else gets `404 NOT FOUND` with the body
/// `Nicht gefunden`.
///
/// The response is built with `antwort`, so that the header lines stand at one
/// place and not at three.
pub fn antwort_fuer(pfad: &str) -> String {
    todo!("Aufgabe 2 / Exercise 2")
}

/// Aufgabe 3: Bediene eine Anfrage, die aus `leser` kommt, und schreib die
/// Antwort nach `schreiber`.
///
/// Der Kopf wird mit `kopf_lesen` gelesen, der Pfad kommt mit `pfad_aus` aus
/// seiner ersten Zeile, und die Antwort baut `antwort_fuer`. Steht dort keine
/// brauchbare erste Zeile, geht `400 BAD REQUEST` mit dem Rumpf
/// `Kaputte Anfrage` zurück.
///
/// Am Ende wird geschrieben und danach `flush` gerufen. Ohne das kann die
/// Antwort im Puffer stehen bleiben, während die Gegenseite auf sie wartet.
///
/// Diese Funktion nimmt zwei Ströme statt einer Verbindung, und deshalb lässt
/// sie sich ohne Anschluss prüfen. Der Anschluss kommt in Aufgabe 4 dazu.
///
/// Exercise 3: serve a request coming out of `leser` and write the response into
/// `schreiber`.
///
/// The head is read with `kopf_lesen`, the path comes out of its first line with
/// `pfad_aus`, and the response is built by `antwort_fuer`. Where no usable
/// first line stands there, `400 BAD REQUEST` with the body `Kaputte Anfrage`
/// goes back.
///
/// At the end it writes and then calls `flush`. Without that the response can
/// stay sitting in the buffer while the other side waits for it.
///
/// This function takes two streams instead of a connection, and that is why it
/// can be checked without a socket. The socket comes along in exercise 4.
pub fn bediene_einen(leser: &mut impl BufRead, schreiber: &mut impl Write) -> io::Result<()> {
    todo!("Aufgabe 3 / Exercise 3")
}

/// Aufgabe 4: Nimm genau eine Verbindung an und bediene sie.
///
/// `accept` wartet, bis jemand anklopft, und gibt dann eine Verbindung zurück.
/// Aus ihr wird gelesen und in sie wird geschrieben, und beides geht über
/// dieselbe `TcpStream`. Ein `BufReader` um eine Referenz auf sie herum liest,
/// eine zweite Referenz schreibt.
///
/// Die Arbeit selbst macht `bediene_einen`. Diese Funktion legt nur die
/// Verbindung darunter.
///
/// Exercise 4: take exactly one connection and serve it.
///
/// `accept` waits until somebody knocks and then gives back a connection. It is
/// read from and written to, and both go over the same `TcpStream`. A
/// `BufReader` around a reference to it reads, a second reference writes.
///
/// The work itself is done by `bediene_einen`. This function only puts the
/// connection underneath it.
pub fn bediene_einmal(lauscher: &TcpListener) -> io::Result<()> {
    todo!("Aufgabe 4 / Exercise 4")
}
