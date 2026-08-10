//! 09-09 Der Webserver mit Threadpool und eigenen Tests / The web server with a
//! thread pool and your own tests, gelöst.
//!
//! Deutsch: Die Erklärung steht in
//! `units/09-09-webserver-mit-threadpool/README.md`. Hier stehen nur die
//! Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in
//! `units/09-09-webserver-mit-threadpool/README.md`. What is here is only the
//! bodies that turn the unit's tests green.

use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Eine Aufgabe für den Pool.
///
/// A task for the pool.
pub type Aufgabe = Box<dyn FnOnce() + Send + 'static>;

/// Baut eine Antwort aus Statuszeile und Rumpf.
///
/// Builds a response out of a status line and a body.
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
/// Builds the response that belongs to a path.
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
/// Reads the head of a request, meaning every line up to the first empty one.
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
/// Gets the path out of the first line of a request.
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
/// Serves a request out of `leser` and writes the response into `schreiber`.
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
/// A pool of threads taking tasks out of one shared queue.
pub struct Threadpool {
    faeden: Vec<thread::JoinHandle<()>>,
    geber: Option<mpsc::Sender<Aufgabe>>,
}

impl Threadpool {
    /// Baut einen Pool mit `groesse` Fäden.
    ///
    /// Builds a pool with `groesse` threads.
    ///
    /// # Panics
    ///
    /// Deutsch: bei `groesse == 0`. / English: on `groesse == 0`.
    pub fn neu(groesse: usize) -> Threadpool {
        assert!(groesse > 0, "ein Threadpool ohne Faden ist keiner");

        let (geber, empfaenger) = mpsc::channel::<Aufgabe>();
        let empfaenger = Arc::new(Mutex::new(empfaenger));
        let mut faeden = Vec::with_capacity(groesse);

        for _ in 0..groesse {
            let empfaenger = Arc::clone(&empfaenger);
            faeden.push(thread::spawn(move || {
                loop {
                    // Deutsch: Das Schloss wird genommen, eine Aufgabe geholt
                    // und das Schloss danach wieder losgelassen. Das `let`
                    // beendet die Anweisung, und damit endet auch die
                    // Leihgabe. Erst danach laeuft die Aufgabe.
                    // English: the lock is taken, a task fetched, and the lock
                    // let go of afterwards. The `let` ends the statement, and
                    // with it the borrow ends. Only after that the task runs.
                    let geholt = empfaenger.lock().expect("das Schloss haelt").recv();

                    match geholt {
                        Ok(aufgabe) => aufgabe(),
                        Err(_) => break,
                    }
                }
            }));
        }

        Threadpool {
            faeden,
            geber: Some(geber),
        }
    }

    /// Gibt eine Aufgabe an den Pool.
    ///
    /// Hands a task to the pool.
    ///
    /// # Panics
    ///
    /// Deutsch: wenn kein Faden mehr wartet. / English: when no thread is
    /// waiting any more.
    pub fn gib_auf<F>(&self, aufgabe: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let geber = self.geber.as_ref().expect("der Pool nimmt noch an");

        geber
            .send(Box::new(aufgabe))
            .expect("ein Faden nimmt die Aufgabe");
    }
}

/// Das geregelte Herunterfahren.
///
/// The orderly shutdown.
impl Drop for Threadpool {
    fn drop(&mut self) {
        drop(self.geber.take());

        for faden in self.faeden.drain(..) {
            let _ = faden.join();
        }
    }
}

/// Nimmt `anzahl` Verbindungen an und gibt jede an den Pool.
///
/// Takes `anzahl` connections and hands each of them to the pool.
pub fn bediene_mit_pool(
    lauscher: &TcpListener,
    pool: &Threadpool,
    anzahl: usize,
) -> io::Result<()> {
    for _ in 0..anzahl {
        let (strom, _) = lauscher.accept()?;

        pool.gib_auf(move || {
            let mut leser = BufReader::new(&strom);
            let mut schreiber = &strom;

            let _ = bediene_einen(&mut leser, &mut schreiber);
        });
    }

    Ok(())
}
