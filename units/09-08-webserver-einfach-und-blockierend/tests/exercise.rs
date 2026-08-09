// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use unit_09_08_webserver_einfach_und_blockierend::{
    antwort, antwort_fuer, bediene_einen, bediene_einmal, kopf_lesen, pfad_aus,
};

// Deutsch: Die erste Zeile einer Antwort. Sie ist das, was diese Einheit prueft.
// English: the first line of a response. It is what this unit checks.
fn antwortzeile(text: &str) -> &str {
    text.lines().next().unwrap_or("")
}

// Deutsch: Eine Anfrage durch `bediene_einen` schicken und die Antwort als Text
// zurueckbekommen, ohne dass ein Anschluss dabei ist.
// English: send a request through `bediene_einen` and get the response back as
// text, with no socket involved.
fn durch_bediene_einen(anfrage: &[u8]) -> String {
    let mut leser = BufReader::new(anfrage);
    let mut geschrieben: Vec<u8> = Vec::new();

    bediene_einen(&mut leser, &mut geschrieben).expect("die Antwort ist geschrieben");

    String::from_utf8(geschrieben).expect("die Antwort ist Text")
}

#[test]
fn pfad_aus_reads_the_path_out_of_the_request_line() {
    assert_eq!(pfad_aus("GET / HTTP/1.1"), Some("/"));
    assert_eq!(pfad_aus("GET /rust HTTP/1.1"), Some("/rust"));
    assert_eq!(pfad_aus("GET /a/b HTTP/1.0"), Some("/a/b"));
}

#[test]
fn pfad_aus_refuses_what_this_server_cannot_do() {
    assert_eq!(pfad_aus("POST / HTTP/1.1"), None);
    assert_eq!(pfad_aus("GET / SPDY/3"), None);
    assert_eq!(pfad_aus("GET /"), None);
    assert_eq!(pfad_aus(""), None);
}

#[test]
fn antwort_fuer_knows_two_paths_and_refuses_the_rest() {
    assert_eq!(antwortzeile(&antwort_fuer("/")), "HTTP/1.1 200 OK");
    assert_eq!(antwortzeile(&antwort_fuer("/rust")), "HTTP/1.1 200 OK");
    assert_eq!(
        antwortzeile(&antwort_fuer("/gibtsnicht")),
        "HTTP/1.1 404 NOT FOUND"
    );
}

#[test]
fn antwort_fuer_carries_the_body_that_belongs_to_the_path() {
    assert!(antwort_fuer("/").ends_with("\r\n\r\nHallo"));
    assert!(antwort_fuer("/rust").ends_with("\r\n\r\nHallo, Rust"));
    assert!(antwort_fuer("/gibtsnicht").ends_with("\r\n\r\nNicht gefunden"));
}

#[test]
fn bediene_einen_answers_the_request_it_was_given() {
    let text = durch_bediene_einen(b"GET /rust HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");

    assert_eq!(antwortzeile(&text), "HTTP/1.1 200 OK");
    assert!(text.ends_with("\r\n\r\nHallo, Rust"));
}

// Deutsch: Eine erste Zeile, die keine Anfragezeile ist, und eine Verbindung,
// auf der gar nichts steht. Beide sind kein Grund abzustuerzen.
// English: a first line that is not a request line, and a connection with
// nothing on it at all. Neither is a reason to crash.
#[test]
fn bediene_einen_says_400_on_something_that_is_no_request() {
    let text = durch_bediene_einen(b"HALLO\r\n\r\n");
    assert_eq!(antwortzeile(&text), "HTTP/1.1 400 BAD REQUEST");

    let leer = durch_bediene_einen(b"");
    assert_eq!(antwortzeile(&leer), "HTTP/1.1 400 BAD REQUEST");
}

// Deutsch: Dieser Test oeffnet einen Anschluss auf 127.0.0.1 mit einer Nummer,
// die das Betriebssystem aussucht. Er ist der einzige Test der Einheit, der
// wirklich ueber das Netz geht.
// English: this test opens a socket on 127.0.0.1 with a number the operating
// system picks. It is the only test of the unit that really goes over the
// network.
#[test]
fn bediene_einmal_answers_over_a_real_connection() {
    let lauscher = TcpListener::bind("127.0.0.1:0").expect("der Anschluss steht");
    let adresse = lauscher.local_addr().expect("die Adresse steht");
    let server = thread::spawn(move || bediene_einmal(&lauscher).expect("eine Anfrage bedient"));

    let mut strom = TcpStream::connect(adresse).expect("die Verbindung steht");
    write!(strom, "GET /rust HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").expect("die Anfrage ist raus");

    let mut zurueck = String::new();
    strom
        .read_to_string(&mut zurueck)
        .expect("die Antwort ist da");

    server.join().expect("der Faden ist durchgelaufen");

    assert_eq!(antwortzeile(&zurueck), "HTTP/1.1 200 OK");
    assert!(zurueck.ends_with("\r\n\r\nHallo, Rust"));
}

#[test]
fn the_finished_functions_build_and_read_the_shape_of_http() {
    assert_eq!(
        antwort("200 OK", "Hallo"),
        "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo"
    );

    let anfrage = b"GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\nrest";
    let mut leser = BufReader::new(&anfrage[..]);

    assert_eq!(
        kopf_lesen(&mut leser).expect("der Kopf ist gelesen"),
        vec!["GET / HTTP/1.1", "Host: 127.0.0.1"]
    );
}
