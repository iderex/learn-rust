// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use std::collections::HashSet;
use std::io::{BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use unit_09_09_webserver_mit_threadpool::{
    Threadpool, antwort, antwort_fuer, bediene_einen, bediene_mit_pool,
};

// Deutsch: Wie lange eine Aufgabe am Treffpunkt auf die anderen wartet, bevor
// sie aufgibt. Grosszuegig gewaehlt: Ein Pool, der wirklich nebeneinander
// arbeitet, braucht dafuer Millisekunden.
// English: how long a task waits at the meeting point for the others before it
// gives up. Chosen generously: a pool that really works side by side needs
// milliseconds for it.
const WARTEZEIT: Duration = Duration::from_secs(5);

// Deutsch: Wo sich die Aufgaben treffen. `angekommen` zaehlt, `faeden` sammelt,
// auf welchen Faeden sie liefen, und `abgelaufen` sagt, ob eine von ihnen die
// anderen nicht mehr abgewartet hat.
// English: where the tasks meet. `angekommen` counts, `faeden` collects which
// threads they ran on, and `abgelaufen` says whether one of them stopped
// waiting for the others.
#[derive(Default)]
struct Treffpunkt {
    angekommen: usize,
    faeden: HashSet<thread::ThreadId>,
    abgelaufen: bool,
}

// Deutsch: Die erste Zeile einer Antwort.
// English: the first line of a response.
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

// Deutsch: Eine Anfrage ueber das Netz stellen und die Antwort im Ganzen lesen.
// English: put a request over the network and read the answer whole.
fn frage(adresse: SocketAddr, pfad: &str) -> String {
    let mut strom = TcpStream::connect(adresse).expect("die Verbindung steht");
    write!(strom, "GET {pfad} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").expect("die Anfrage ist raus");

    let mut zurueck = String::new();
    strom
        .read_to_string(&mut zurueck)
        .expect("die Antwort ist da");

    zurueck
}

// Deutsch: `should_panic` steht hier mit Text. Ohne ihn waere der Test schon
// davon gruen, dass die Aufgabe `todo!()` ist.
// English: `should_panic` stands here with a text. Without it the test would go
// green merely from the exercise being `todo!()`.
#[test]
#[should_panic(expected = "ohne Faden")]
fn ein_pool_ohne_faden_wird_abgelehnt() {
    let _ = Threadpool::neu(0);
}

#[test]
fn jede_aufgabe_wird_ausgefuehrt() {
    let zaehler = Arc::new(AtomicUsize::new(0));
    let pool = Threadpool::neu(4);

    for _ in 0..16 {
        let zaehler = Arc::clone(&zaehler);
        pool.gib_auf(move || {
            zaehler.fetch_add(1, Ordering::SeqCst);
        });
    }

    drop(pool);

    assert_eq!(zaehler.load(Ordering::SeqCst), 16);
}

// Deutsch: Vier Aufgaben auf vier Faeden, und jede wartet am Treffpunkt auf die
// drei anderen. Ein Pool, der eine Aufgabe nach der anderen abarbeitet, kommt
// hier nicht durch: Die erste wartet dann auf drei, die nie ankommen, und gibt
// nach `WARTEZEIT` auf.
// English: four tasks on four threads, and each waits at the meeting point for
// the other three. A pool working off one task after another does not get
// through here: the first one then waits for three that never arrive and gives
// up after `WARTEZEIT`.
#[test]
fn die_aufgaben_laufen_nebeneinander() {
    const WIE_VIELE: usize = 4;

    let treffpunkt = Arc::new((Mutex::new(Treffpunkt::default()), Condvar::new()));
    let pool = Threadpool::neu(WIE_VIELE);

    for _ in 0..WIE_VIELE {
        let treffpunkt = Arc::clone(&treffpunkt);
        pool.gib_auf(move || {
            let (schloss, signal) = &*treffpunkt;
            let mut stand = schloss.lock().expect("das Schloss haelt");

            stand.angekommen += 1;
            stand.faeden.insert(thread::current().id());
            signal.notify_all();

            while stand.angekommen < WIE_VIELE {
                let (weiter, ergebnis) = signal
                    .wait_timeout(stand, WARTEZEIT)
                    .expect("das Schloss haelt");
                stand = weiter;

                if ergebnis.timed_out() {
                    stand.abgelaufen = true;
                    break;
                }
            }
        });
    }

    drop(pool);

    let (schloss, _) = &*treffpunkt;
    let stand = schloss.lock().expect("das Schloss haelt");

    assert!(
        !stand.abgelaufen,
        "eine Aufgabe hat die anderen nicht mehr angetroffen"
    );
    assert_eq!(stand.angekommen, WIE_VIELE);
    assert_eq!(stand.faeden.len(), WIE_VIELE);
}

// Deutsch: Ein Pool mit einem Faden und drei Aufgaben, die zusammen 150
// Millisekunden dauern. Nach dem Fallenlassen sind alle drei durch.
// English: a pool with one thread and three tasks taking 150 milliseconds
// together. After the dropping all three are through.
#[test]
fn beim_fallenlassen_laufen_angenommene_aufgaben_zu_ende() {
    let zaehler = Arc::new(AtomicUsize::new(0));
    let pool = Threadpool::neu(1);

    for _ in 0..3 {
        let zaehler = Arc::clone(&zaehler);
        pool.gib_auf(move || {
            thread::sleep(Duration::from_millis(50));
            zaehler.fetch_add(1, Ordering::SeqCst);
        });
    }

    drop(pool);

    assert_eq!(zaehler.load(Ordering::SeqCst), 3);
}

// Deutsch: Dieser Test oeffnet einen Anschluss auf 127.0.0.1 mit einer Nummer,
// die das Betriebssystem aussucht. Er ist der einzige Test der Einheit, der
// wirklich ueber das Netz geht. Welche der beiden Anfragen zuerst bedient wird,
// steht nirgends, und keine Zusicherung hier haengt daran.
// English: this test opens a socket on 127.0.0.1 with a number the operating
// system picks. It is the only test of the unit that really goes over the
// network. Which of the two requests is served first is written nowhere, and no
// assertion here hangs on it.
#[test]
fn zwei_anfragen_werden_nebeneinander_bedient() {
    let lauscher = TcpListener::bind("127.0.0.1:0").expect("der Anschluss steht");
    let adresse = lauscher.local_addr().expect("die Adresse steht");

    let server = thread::spawn(move || {
        let pool = Threadpool::neu(2);
        bediene_mit_pool(&lauscher, &pool, 2).expect("zwei Verbindungen angenommen");
    });

    let erste = thread::spawn(move || frage(adresse, "/"));
    let zweite = thread::spawn(move || frage(adresse, "/rust"));

    let a = erste.join().expect("der Faden ist durchgelaufen");
    let b = zweite.join().expect("der Faden ist durchgelaufen");
    server.join().expect("der Faden ist durchgelaufen");

    assert_eq!(antwortzeile(&a), "HTTP/1.1 200 OK");
    assert!(a.ends_with("\r\n\r\nHallo"));

    assert_eq!(antwortzeile(&b), "HTTP/1.1 200 OK");
    assert!(b.ends_with("\r\n\r\nHallo, Rust"));
}

#[test]
fn die_fertigen_teile_bauen_die_form_von_http() {
    assert_eq!(
        antwort("200 OK", "Hallo"),
        "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo"
    );
    assert_eq!(
        antwortzeile(&antwort_fuer("/gibtsnicht")),
        "HTTP/1.1 404 NOT FOUND"
    );

    let text = durch_bediene_einen(b"GET /rust HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
    assert_eq!(antwortzeile(&text), "HTTP/1.1 200 OK");
    assert!(text.ends_with("\r\n\r\nHallo, Rust"));

    let kaputt = durch_bediene_einen(b"HALLO\r\n\r\n");
    assert_eq!(antwortzeile(&kaputt), "HTTP/1.1 400 BAD REQUEST");
}
