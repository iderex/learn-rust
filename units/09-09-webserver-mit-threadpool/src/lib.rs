//! 09-09 Der Webserver mit Threadpool und eigenen Tests / The web server with a
//! thread pool and your own tests
//!
//! Deutsch: Der Server aus der Einheit davor bedient eine Anfrage nach der
//! anderen. Hier kommt ein Pool aus Fäden davor, der mehrere gleichzeitig
//! annimmt, und ein Herunterfahren, das die angenommene Arbeit noch zu Ende
//! laufen lässt. Warum das so gebaut ist, steht in der README.
//!
//! English: the server of the unit before serves one request after another.
//! Here a pool of threads goes in front of it that takes several at the same
//! time, plus a shutdown that lets the accepted work run to its end. Why it is
//! built that way is written in the README.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

use std::io::{self, BufRead, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

/// Eine Aufgabe für den Pool.
///
/// `FnOnce` heißt: Sie läuft genau einmal und darf dabei aufbrauchen, was sie
/// mitgenommen hat. `Send` heißt: Sie darf auf einen anderen Faden. `'static`
/// heißt: Sie leiht sich nichts, das vor ihr endet. Und `Box` steht davor, weil
/// jede Aufgabe anders groß ist, die Warteschlange aber einen festen Typ
/// braucht.
///
/// A task for the pool.
///
/// `FnOnce` means: it runs exactly once and may use up what it took with it.
/// `Send` means: it may go onto another thread. `'static` means: it borrows
/// nothing that ends before it does. And `Box` stands in front because every
/// task has a different size while the queue needs one fixed type.
pub type Aufgabe = Box<dyn FnOnce() + Send + 'static>;

/// Baut eine Antwort aus Statuszeile und Rumpf.
///
/// Diese Funktion steht fertig da. Die Zeilen einer HTTP-Nachricht enden mit
/// `\r\n`, zwischen Kopf und Rumpf steht eine leere Zeile, und
/// `Content-Length` sagt, wie viele Bytes danach kommen.
///
/// Builds a response out of a status line and a body.
///
/// This function stands there finished. The lines of an HTTP message end with
/// `\r\n`, between head and body stands an empty line, and `Content-Length`
/// says how many bytes come afterwards.
///
/// ```
/// use unit_09_09_webserver_mit_threadpool::antwort;
///
/// let text = antwort("200 OK", "Hallo");
///
/// assert_eq!(text, "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo");
/// ```
pub fn antwort(status: &str, rumpf: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\nContent-Length: {laenge}\r\n\r\n{rumpf}",
        laenge = rumpf.len()
    )
}

/// Baut die Antwort, die zu einem Pfad gehört.
///
/// Diese Funktion steht fertig da. Sie ist dieselbe wie in der Einheit davor
/// und liegt hier noch einmal, weil jede Einheit ein Paket für sich ist und
/// keine andere einbindet.
///
/// Builds the response that belongs to a path.
///
/// This function stands there finished. It is the same one as in the unit
/// before and lies here a second time, because every unit is a package of its
/// own and includes no other.
///
/// ```
/// use unit_09_09_webserver_mit_threadpool::antwort_fuer;
///
/// assert!(antwort_fuer("/").ends_with("\r\n\r\nHallo"));
/// assert!(antwort_fuer("/rust").ends_with("\r\n\r\nHallo, Rust"));
/// assert!(antwort_fuer("/gibtsnicht").starts_with("HTTP/1.1 404 NOT FOUND"));
/// ```
pub fn antwort_fuer(pfad: &str) -> String {
    match pfad {
        "/" => antwort("200 OK", "Hallo"),
        "/rust" => antwort("200 OK", "Hallo, Rust"),
        _ => antwort("404 NOT FOUND", "Nicht gefunden"),
    }
}

/// Liest den Kopf einer Anfrage, also alle Zeilen bis zur ersten leeren.
///
/// Gelesen wird bis zur leeren Zeile und nicht bis zum Ende, denn ein Ende
/// kommt nicht: Die Gegenseite hält die Verbindung offen und wartet auf die
/// Antwort.
///
/// Reads the head of a request, meaning every line up to the first empty one.
///
/// Reading goes up to the empty line and not up to the end, because no end
/// comes: the other side keeps the connection open and waits for the answer.
fn kopf_lesen(leser: &mut impl BufRead) -> io::Result<Vec<String>> {
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
/// `None` kommt zurück, wenn das Verb nicht `GET` ist, wenn der dritte Teil
/// nicht mit `HTTP/` anfängt oder wenn weniger als drei Teile dastehen.
///
/// Gets the path out of the first line of a request.
///
/// `None` comes back when the verb is not `GET`, when the third part does not
/// start with `HTTP/`, or when fewer than three parts stand there.
fn pfad_aus(anfragezeile: &str) -> Option<&str> {
    let mut teile = anfragezeile.split(' ');
    let verb = teile.next()?;
    let pfad = teile.next()?;
    let version = teile.next()?;

    if verb != "GET" || !version.starts_with("HTTP/") {
        return None;
    }

    Some(pfad)
}

/// Bedient eine Anfrage aus `leser` und schreibt die Antwort nach `schreiber`.
///
/// Diese Funktion steht fertig da. Sie nimmt zwei Ströme statt einer
/// Verbindung und lässt sich deshalb ohne Anschluss prüfen. Steht keine
/// brauchbare erste Zeile da, geht `400 BAD REQUEST` zurück.
///
/// Serves a request out of `leser` and writes the response into `schreiber`.
///
/// This function stands there finished. It takes two streams instead of a
/// connection and can therefore be checked without a socket. Where no usable
/// first line stands there, `400 BAD REQUEST` goes back.
///
/// ```
/// use std::io::BufReader;
/// use unit_09_09_webserver_mit_threadpool::bediene_einen;
///
/// let anfrage = b"GET /rust HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
/// let mut leser = BufReader::new(&anfrage[..]);
/// let mut geschrieben: Vec<u8> = Vec::new();
///
/// bediene_einen(&mut leser, &mut geschrieben).unwrap();
///
/// let text = String::from_utf8(geschrieben).unwrap();
/// assert!(text.ends_with("\r\n\r\nHallo, Rust"));
/// ```
pub fn bediene_einen(leser: &mut impl BufRead, schreiber: &mut impl Write) -> io::Result<()> {
    let kopf = kopf_lesen(leser)?;
    let pfad = kopf.first().and_then(|zeile| pfad_aus(zeile));

    let text = match pfad {
        Some(pfad) => antwort_fuer(pfad),
        None => antwort("400 BAD REQUEST", "Kaputte Anfrage"),
    };

    schreiber.write_all(text.as_bytes())?;
    schreiber.flush()
}

/// Ein Pool aus Fäden, die Aufgaben aus einer gemeinsamen Warteschlange nehmen.
///
/// `geber` steht in einem `Option`, damit das Herunterfahren ihn herausnehmen
/// und fallen lassen kann, ohne den Pool selbst zu zerlegen. Genau daran
/// merken die Fäden, dass nichts mehr kommt.
///
/// A pool of threads taking tasks out of one shared queue.
///
/// `geber` sits in an `Option` so that the shutdown can take it out and drop it
/// without taking the pool itself apart. That is exactly how the threads notice
/// that nothing more is coming.
pub struct Threadpool {
    faeden: Vec<thread::JoinHandle<()>>,
    geber: Option<mpsc::Sender<Aufgabe>>,
}

impl Threadpool {
    /// Aufgabe 1: Bau einen Pool mit `groesse` Fäden.
    ///
    /// Ein Kanal wird angelegt. Der Sender bleibt beim Pool, der Empfänger
    /// gehört allen Fäden gemeinsam, und "gemeinsam" heißt hier `Arc` für das
    /// Teilen und `Mutex` dafür, dass immer nur einer zugleich hineinsieht.
    ///
    /// Jeder Faden läuft in einer Schleife: Schloss nehmen, eine Aufgabe holen,
    /// **Schloss wieder loslassen**, dann die Aufgabe ausführen. Wer das
    /// Schloss über die Ausführung hinweg festhält, hat einen Pool gebaut, der
    /// eine Aufgabe nach der anderen erledigt. Die Schleife endet, wenn das
    /// Holen einen Fehler zurückgibt, denn dann ist der Sender weg.
    ///
    /// `groesse` von 0 ist kein Pool. Dann wird abgebrochen, mit einer
    /// Meldung, in der `ohne Faden` steht.
    ///
    /// Exercise 1: build a pool with `groesse` threads.
    ///
    /// A channel is created. The sender stays with the pool, the receiver
    /// belongs to all the threads together, and "together" means `Arc` for the
    /// sharing and `Mutex` so that only one of them looks inside at a time.
    ///
    /// Every thread runs in a loop: take the lock, fetch a task, **let go of
    /// the lock again**, then run the task. Whoever holds the lock across the
    /// running has built a pool that works off one task after another. The loop
    /// ends when the fetching gives back an error, because then the sender is
    /// gone.
    ///
    /// A `groesse` of 0 is no pool. Then it aborts, with a message carrying
    /// `ohne Faden` in it.
    ///
    /// # Panics
    ///
    /// Deutsch: bei `groesse == 0`. / English: on `groesse == 0`.
    pub fn neu(groesse: usize) -> Threadpool {
        todo!("Aufgabe 1 / Exercise 1")
    }

    /// Aufgabe 2: Gib eine Aufgabe an den Pool.
    ///
    /// Die Aufgabe wird eingepackt und in den Kanal geschickt. Der erste Faden,
    /// der frei ist, nimmt sie. Geht das Schicken schief, weil kein Faden mehr
    /// da ist, wird abgebrochen.
    ///
    /// Exercise 2: hand a task to the pool.
    ///
    /// The task is boxed and sent into the channel. The first thread that is
    /// free takes it. If the sending goes wrong because no thread is left, it
    /// aborts.
    ///
    /// # Panics
    ///
    /// Deutsch: wenn kein Faden mehr wartet. / English: when no thread is
    /// waiting any more.
    pub fn gib_auf<F>(&self, aufgabe: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Das geregelte Herunterfahren, und es steht fertig da.
///
/// Zuerst wird der Sender herausgenommen und fallen gelassen. Damit gibt das
/// Holen in jedem Faden einen Fehler zurück, sobald die Warteschlange leer ist,
/// und die Schleifen enden. Danach wird auf jeden Faden gewartet. Beides in
/// dieser Reihenfolge: Wer zuerst wartet und dann den Sender loslässt, wartet
/// auf Fäden, die noch auf Arbeit warten.
///
/// The orderly shutdown, and it stands there finished.
///
/// First the sender is taken out and dropped. That makes the fetching in every
/// thread give back an error as soon as the queue is empty, and the loops end.
/// Then every thread is waited for. Both in that order: whoever waits first and
/// lets go of the sender afterwards is waiting for threads that are still
/// waiting for work.
impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.geber.take());

        for faden in self.faeden.drain(..) {
            let _ = faden.join();
        }
    }
}

/// Aufgabe 3: Nimm `anzahl` Verbindungen an und gib jede an den Pool.
///
/// `accept` wartet, bis jemand anklopft, und gibt dann eine Verbindung zurück.
/// Diese Verbindung wandert in eine Aufgabe, und die Aufgabe bedient sie mit
/// `bediene_einen`. Angenommen wird also hier, gearbeitet wird auf einem Faden
/// des Pools, und deshalb kann die nächste Verbindung schon drankommen, während
/// die vorige noch bedient wird.
///
/// Ein `BufReader` um eine Referenz auf die Verbindung herum liest, eine zweite
/// Referenz schreibt. Was `bediene_einen` an Fehlern zurückgibt, hat auf einem
/// Faden des Pools niemanden mehr, der es entgegennimmt; die Verbindung geht
/// dann einfach zu.
///
/// Exercise 3: take `anzahl` connections and hand each of them to the pool.
///
/// `accept` waits until somebody knocks and then gives back a connection. That
/// connection travels into a task, and the task serves it with `bediene_einen`.
/// The taking therefore happens here, the work happens on a thread of the pool,
/// and that is why the next connection can come up while the previous one is
/// still being served.
///
/// A `BufReader` around a reference to the connection reads, a second reference
/// writes. Whatever errors `bediene_einen` gives back have nobody left on a
/// pool thread to take them; the connection then simply closes.
pub fn bediene_mit_pool(
    lauscher: &TcpListener,
    pool: &Threadpool,
    anzahl: usize,
) -> io::Result<()> {
    todo!("Aufgabe 3 / Exercise 3")
}
