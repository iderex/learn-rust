# 09-09 Der Webserver mit Threadpool und eigenen Tests / The web server with a thread pool and your own tests

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/09-09-webserver-mit-threadpool/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `07-05 Threads`, `07-06 Kanäle` und
  `07-07 Mutex und Arc`, woher der Kanal, das Schloss und das gemeinsame
  Eigentum kommen, und `09-08 Der Webserver, einfach und blockierend`, dessen
  Server hier einen Pool davor bekommt.
- Beim Antworten so zitieren: `09-09 Der Webserver mit Threadpool und eigenen
  Tests`, dazu die Überschrift des Abschnitts, zum Beispiel Abschnitt
  "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Kapiteltitel des Abschnitts 21.2 lautet in der gebundenen Fassung
  "From Single-Threaded to Multithreaded Server". Ältere Fassungen des Buchs
  nennen ihn anders, und die Seitenadresse führt den kürzeren Namen weiter.
- Die Stelle, an der diese Einheit hängt, ist das Loslassen des Schlosses vor
  dem Ausführen der Aufgabe. Wer es über die Ausführung hinweg festhält, baut
  einen Pool, der nacheinander arbeitet. Welche Tests das sehen und welche
  nicht, steht unter "Was diese Tests nicht beantworten".
- Die Zahlen unter "Was ein zweiter Faden bringt" sind auf einem Rechner
  gemessen. Wer sie weitergibt, gibt den Befehl mit.
- Ein Test dieser Einheit öffnet einen Anschluss auf `127.0.0.1` mit einer
  Nummer, die das Betriebssystem aussucht. Auf einem Rechner, auf dem
  Verbindungen zu sich selbst nicht durchkommen, fällt er aus einem Grund um,
  der nichts mit der Aufgabe zu tun hat.
- Das Herunterfahren in `Drop` steht fertig da und ist keine Aufgabe. Dass es
  wirkt, hängt trotzdem an Aufgabe 1: Die Schleife eines Fadens muss enden,
  wenn das Holen einen Fehler zurückgibt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-09-webserver-mit-threadpool/`.
  It is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `07-05 Threads`, `07-06 Kanäle` and
  `07-07 Mutex und Arc`, where the channel, the lock and the shared ownership
  come from, and `09-08 Der Webserver, einfach und blockierend`, whose server
  gets a pool in front of it here.
- Cite like this when answering: `09-09 Der Webserver mit Threadpool und eigenen
  Tests`, plus the heading of the section, for example section
  "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The chapter title of section 21.2 reads "From Single-Threaded to Multithreaded
  Server" in the pinned version. Older versions of the book name it differently,
  and the page address keeps the shorter name.
- The place this unit hangs on is letting go of the lock before running the
  task. Whoever holds it across the running builds a pool that works one after
  another. Which tests see that and which do not stands under "What these tests
  do not answer".
- The numbers under "What a second thread buys" are measured on one machine.
  Whoever passes them on passes the command on with them.
- One test of this unit opens a socket on `127.0.0.1` with a number the
  operating system picks. On a machine where connections to itself do not get
  through, it falls over for a reason that has nothing to do with the exercise.
- The shutdown in `Drop` stands there finished and is no exercise. That it works
  still hangs on exercise 1: the loop of a thread has to end when the fetching
  gives back an error.

</details>

## Deutsch

### Worum es geht

Der Server der Einheit davor nimmt eine Verbindung an, arbeitet sie ab und nimmt
erst danach die nächste. Ein Threadpool schiebt sich zwischen das Annehmen und
das Abarbeiten: Angenommen wird weiter an einer Stelle, gearbeitet wird auf
mehreren Fäden.

Drei Bauteile reichen dafür, und alle drei sind schon dagewesen. Ein Kanal trägt
die Arbeit vom Annehmen zu den Fäden. Ein `Arc` gibt allen Fäden dasselbe
Empfangsende. Ein `Mutex` sorgt dafür, dass immer nur einer zugleich
hineingreift.

Dazu kommt das Herunterfahren. Ein Pool, der einfach verschwindet, lässt Arbeit
liegen, die er schon angenommen hat. Ein geregeltes Herunterfahren sagt den
Fäden, dass nichts mehr kommt, und wartet dann auf sie.

### Wofür das gut ist

Weil hier zum ersten Mal ein Stück Nebenläufigkeit entsteht, das nicht aus einem
einzelnen `thread::spawn` besteht. Ein Faden pro Anfrage klingt einfacher und
ist es auch, bis jemand zehntausend Anfragen schickt; ein Pool legt die Zahl der
Fäden vorher fest.

Und weil das Herunterfahren die Stelle ist, an der `Drop` etwas tut, das nicht
Speicher freigeben ist. Der Pool nutzt aus, dass `Drop` garantiert läuft, um ein
Versprechen zu halten: Was angenommen wurde, läuft zu Ende.

Beides ist am Ende ein Server, der zwei langsame Anfragen nebeneinander bedient
statt hintereinander, und dieser Unterschied ist messbar.

### Die Erklärung

Ein Pool aus vier Fäden, vier Aufgaben, und ein Block, an dessen Ende der Pool
fallengelassen wird.

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

type Aufgabe = Box<dyn FnOnce() + Send + 'static>;

struct Threadpool {
    faeden: Vec<thread::JoinHandle<()>>,
    geber: Option<mpsc::Sender<Aufgabe>>,
}

impl Threadpool {
    fn neu(groesse: usize) -> Threadpool {
        assert!(groesse > 0, "ein Threadpool ohne Faden ist keiner");

        let (geber, empfaenger) = mpsc::channel::<Aufgabe>();
        let empfaenger = Arc::new(Mutex::new(empfaenger));
        let mut faeden = Vec::with_capacity(groesse);

        for _ in 0..groesse {
            let empfaenger = Arc::clone(&empfaenger);
            faeden.push(thread::spawn(move || {
                loop {
                    // Deutsch: Schloss nehmen, Aufgabe holen, Schloss loslassen.
                    // Das `let` beendet die Anweisung, und damit endet die
                    // Leihgabe. Erst danach laeuft die Aufgabe.
                    let geholt = empfaenger.lock().expect("das Schloss haelt").recv();

                    match geholt {
                        Ok(aufgabe) => aufgabe(),
                        Err(_) => break,
                    }
                }
            }));
        }

        Threadpool { faeden, geber: Some(geber) }
    }

    fn gib_auf<F: FnOnce() + Send + 'static>(&self, aufgabe: F) {
        self.geber
            .as_ref()
            .expect("der Pool nimmt noch an")
            .send(Box::new(aufgabe))
            .expect("ein Faden nimmt die Aufgabe");
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        // Deutsch: Erst den Sender loslassen, dann warten. Andersherum wartet
        // man auf Faeden, die noch auf Arbeit warten.
        drop(self.geber.take());

        for faden in self.faeden.drain(..) {
            let _ = faden.join();
        }
    }
}

fn main() {
    let start = Instant::now();
    let (fertig, gesammelt) = mpsc::channel();

    {
        let pool = Threadpool::neu(4);

        for nummer in 0..4 {
            let fertig = fertig.clone();
            pool.gib_auf(move || {
                thread::sleep(Duration::from_millis(200));
                fertig.send(nummer).expect("gesendet");
            });
        }
        // Deutsch: Hier endet der Block, der Pool wird fallengelassen, und das
        // Fallenlassen wartet auf die vier Aufgaben.
    }
    drop(fertig);

    let mut nummern: Vec<i32> = gesammelt.iter().collect();
    nummern.sort();

    println!("{nummern:?}");
    println!("unter 400 ms: {}", start.elapsed() < Duration::from_millis(400));
}
```

Das Programm gibt aus:

```text
[0, 1, 2, 3]
unter 400 ms: true
```

Vier Dinge daran sind kein Zufall. Der `Arc` liegt um den `Mutex` und nicht
umgekehrt: Geteilt wird das Schloss, und was es schützt, ist das Empfangsende.
Das `let` vor dem `match` ist die ganze Nebenläufigkeit dieses Pools, denn es
beendet die Anweisung und gibt das Schloss zurück, bevor die Aufgabe losläuft.
Der Sender liegt in einem `Option`, weil das Herunterfahren ihn herausnehmen
muss, ohne den Pool zu zerlegen. Und dass alle vier Zahlen ankommen, obwohl
niemand auf die Aufgaben wartet, ist die Arbeit von `Drop` am Ende des Blocks.

### Was ein zweiter Faden bringt

Gemessen und nicht behauptet. Das folgende Programm nimmt zwei Verbindungen an,
lässt jede Anfrage 300 Millisekunden dauern und schickt beide gleichzeitig los.
Die Zahl der Fäden im Pool steht als Argument dahinter, sonst ändert sich
nichts.

```rust
fn main() -> std::io::Result<()> {
    let faeden: usize = std::env::args()
        .nth(1)
        .expect("die Zahl der Faeden steht als Argument da")
        .parse()
        .expect("das Argument ist eine Zahl");

    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || {
        let pool = Threadpool::neu(faeden);
        for _ in 0..2 {
            let (strom, _) = lauscher.accept().expect("angenommen");
            pool.gib_auf(move || bediene(strom));
        }
    });

    let erste = thread::spawn(move || frage(adresse));
    let zweite = thread::spawn(move || frage(adresse));

    let a = erste.join().expect("der Faden ist durchgelaufen");
    let b = zweite.join().expect("der Faden ist durchgelaufen");
    server.join().expect("der Faden ist durchgelaufen");

    let mut zeiten = [a.as_millis(), b.as_millis()];
    zeiten.sort();

    println!("{} ms", zeiten[0]);
    println!("{} ms", zeiten[1]);
    Ok(())
}
```

```console
$ rustc --edition 2024 -O pool.rs
$ ./pool.exe 1
300 ms
601 ms
$ ./pool.exe 2
300 ms
300 ms
```

Mit einem Faden wartet die zweite Anfrage auf die erste, mit zweien nicht. Drei
Runden hintereinander ergaben dieselben vier Zahlen. Der Rest des Programms,
also `Threadpool`, `bediene` und `frage`, steht in dieser README weiter oben
oder in der Einheit davor; hier steht nur der Teil, in dem die Fadenzahl
vorkommt.

Das ist eine Messung auf einem Rechner und keine Zusage. Was sie zeigt, ist
nicht, wie schnell dieser Server ist, sondern dass ein Faden mehr die zweite
Anfrage nicht mehr warten lässt.

### Häufige Fehler

Das Empfangsende in mehrere Fäden geben, ohne es zu teilen.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (geber, empfaenger) = mpsc::channel::<Box<dyn FnOnce() + Send>>();

    for _ in 0..4 {
        thread::spawn(move || {
            while let Ok(aufgabe) = empfaenger.recv() {
                aufgabe();
            }
        });
    }

    drop(geber);
}
```

Der Übersetzer sagt dazu:

```text
error[E0382]: use of moved value: `empfaenger`
 --> geteilt.rs:8:23
  |
5 |     let (geber, empfaenger) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
  |                 ---------- move occurs because `empfaenger` has type `std::sync::mpsc::Receiver<Box<dyn FnOnce() + Send>>`, which does not implement the `Copy` trait
6 |
7 |     for _ in 0..4 {
  |     ------------- inside of this loop
8 |         thread::spawn(move || {
  |                       ^^^^^^^ value moved into closure here, in previous iteration of loop
9 |             while let Ok(aufgabe) = empfaenger.recv() {
  |             -----------------------------------------
  |             |                       |
  |             |                       use occurs due to use in closure
  |             inside of this loop

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

"in previous iteration of loop" ist die ganze Auskunft: Beim ersten Durchgang
geht es gut, beim zweiten ist der Empfänger weg. Ein Empfangsende gehört genau
einer Stelle, und wer es mehreren geben will, gibt allen dasselbe: `Arc` für das
Teilen, `Mutex` dafür, dass immer nur einer hineinsieht.

Der zweite Fehler kommt beim Warten. `join` nimmt den Faden mit, und in `Drop`
steht nur eine geliehene Referenz zur Verfügung.

```rust
use std::thread;

struct Pool {
    faeden: Vec<thread::JoinHandle<()>>,
}

impl Drop for Pool {
    fn drop(&mut self) {
        for faden in &self.faeden {
            faden.join().expect("der Faden ist durchgelaufen");
        }
    }
}

fn main() {
    let _ = Pool { faeden: Vec::new() };
}
```

Der Übersetzer sagt dazu:

```text
error[E0507]: cannot move out of `*faden` which is behind a shared reference
  --> warten.rs:10:13
   |
10 |             faden.join().expect("der Faden ist durchgelaufen");
   |             ^^^^^ ------ `*faden` moved due to this method call
   |             |
   |             move occurs because `*faden` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `*faden`
  --> <std>/std/src/thread/join_handle.rs:149:16

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0507`.
```

An `<std>` stand der Pfad zur Standardbibliothek dieses Rechners, mit der
Prüfsumme des Übersetzers darin, und die Schrägstriche standen andersherum. Das
sind die einzigen Ersetzungen, sonst steht die Meldung so da, wie sie kam.
`drain(..)` löst es, denn damit gehören die Fäden dem, der sie herausnimmt. Das
Buch nimmt an dieser Stelle `Option<JoinHandle<()>>` und `take`; beides
beantwortet dieselbe Frage, wem der Faden gehört, wenn `join` ihn haben will.

### Eigene Tests für einen Server

Ein Server hört auf einem Anschluss, und ein Test, der einen belegt, ist nicht
wie die übrigen Tests dieses Repositories: Er hängt an der Maschine, auf der er
läuft. Deshalb sind hier fast alle Tests ohne Netz gebaut, und genau einer geht
wirklich hinaus.

Die Trennung dafür liegt im Code und nicht in den Tests. `bediene_einen` nimmt
einen Leser und einen Schreiber statt einer Verbindung, also lässt sich das
ganze Verhalten von HTTP gegen ein Stück Speicher prüfen. Der Pool nimmt
Aufgaben statt Anfragen, also lässt er sich gegen Zähler und Kanäle prüfen. Was
danach übrig bleibt, ist das Zusammensetzen, und dafür reicht ein Test.

```rust
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
```

Die `0` in `127.0.0.1:0` heißt: Das Betriebssystem sucht die Nummer aus, und
`local_addr` sagt danach, welche es geworden ist. Damit stört dieser Test
weder einen zweiten Lauf neben sich noch ein Programm, das schon auf einer festen
Nummer sitzt. Und keine Zusicherung hängt daran, welche der beiden Anfragen
zuerst bedient wird: Jede Antwort wird gegen den Pfad geprüft, der sie
angefordert hat.

### Was diese Tests nicht beantworten

Dass der Pool wirklich nebeneinander arbeitet, sieht genau ein Test, und die
übrigen sehen es nicht. Gemessen und nicht vermutet: In der Lösung wurde das
`let` vor dem `match` weggenommen, sodass das Schloss über die Ausführung der
Aufgabe hinweg gehalten wird und der Pool eine Aufgabe nach der anderen
abarbeitet.

```console
$ cargo test -q -p unit-09-09-webserver-mit-threadpool --test exercise
running 6 tests
..... 5/6
exercise::die_aufgaben_laufen_nebeneinander --- FAILED

failures:

---- exercise::die_aufgaben_laufen_nebeneinander stdout ----

thread 'exercise::die_aufgaben_laufen_nebeneinander' (59584) panicked at solutions\09-09-webserver-mit-threadpool\tests\..\..\..\units\09-09-webserver-mit-threadpool\tests\exercise.rs:142:5:
eine Aufgabe hat die anderen nicht mehr angetroffen


failures:
    exercise::die_aufgaben_laufen_nebeneinander

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.02s

error: test failed, to rerun pass `-p unit-09-09-webserver-mit-threadpool --test exercise`
```

Fünf der sechs Tests bleiben grün, und darunter ist der, der über das Netz geht.
Zwei Anfragen nacheinander zu bedienen sieht von außen genauso aus wie zwei
nebeneinander, solange niemand die Zeit misst. Was den Unterschied bemerkt, ist
der Treffpunkt in `die_aufgaben_laufen_nebeneinander`, und er bemerkt ihn daran,
dass eine Aufgabe die anderen nicht mehr antrifft. Die 15 Sekunden statt einer
Fünftelsekunde sind dieselbe Auskunft, nur langsamer.

Ebenso wenig sehen die Tests, wie viele Fäden der Pool anlegt. Geprüft wird,
dass vier Aufgaben zugleich laufen können, und nicht, wie viele Fäden dafür
bereitstehen.

Und nichts hier prüft einen Server, der von außen erreichbar ist. Der eine Test
über das Netz bindet `127.0.0.1`. Auf einem Rechner, auf dem Verbindungen zu
sich selbst nicht durchkommen, fällt er um, und das sagt dann nichts über die
Lösung.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `antwort`, `antwort_fuer` und `bediene_einen` stehen fertig
da, ihre Doku-Tests sind grün, und `Drop` für den Pool steht ebenfalls fertig
da.

- `Threadpool::neu` baut den Kanal, das geteilte Empfangsende und die Fäden
- `Threadpool::gib_auf` packt eine Aufgabe ein und schickt sie los
- `bediene_mit_pool` nimmt Verbindungen an und gibt jede an den Pool

```console
cd units/09-09-webserver-mit-threadpool
cargo test
```

### Quelle

    Buch, Kapitel 21 "Final Project: Building a Multithreaded Web Server",
    Abschnitt 21.2 "From Single-Threaded to Multithreaded Server",
    https://doc.rust-lang.org/book/ch21-02-multithreaded.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 21 "Final Project: Building a Multithreaded Web Server",
    Abschnitt 21.3 "Graceful Shutdown and Cleanup",
    https://doc.rust-lang.org/book/ch21-03-graceful-shutdown-and-cleanup.html,
    geprüft gegen 1.97.1

    Die Standardbibliothek, Kapitel `std::sync::mpsc`,
    https://doc.rust-lang.org/std/sync/mpsc/index.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

The server of the unit before takes a connection, works it off and only then
takes the next one. A thread pool pushes itself between the taking and the
working off: the taking still happens at one place, the work happens on several
threads.

Three parts are enough for that, and all three have been here before. A channel
carries the work from the taking to the threads. An `Arc` gives every thread the
same receiving end. A `Mutex` makes sure only one of them reaches in at a time.

The shutdown comes on top. A pool that simply disappears leaves work lying that
it has already taken on. An orderly shutdown tells the threads that nothing more
is coming and then waits for them.

### What it is good for

Because this is where a piece of concurrency first arises that does not consist
of a single `thread::spawn`. One thread per request sounds simpler and is, until
somebody sends ten thousand requests; a pool fixes the number of threads
beforehand.

And because the shutdown is the place where `Drop` does something that is not
freeing memory. The pool uses the guarantee that `Drop` runs in order to keep a
promise: what was taken on runs to its end.

Both of them are in the end a server serving two slow requests side by side
instead of one after another, and that difference is measurable.

### The explanation

A pool of four threads, four tasks, and a block at whose end the pool is
dropped.

```rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

type Aufgabe = Box<dyn FnOnce() + Send + 'static>;

struct Threadpool {
    faeden: Vec<thread::JoinHandle<()>>,
    geber: Option<mpsc::Sender<Aufgabe>>,
}

impl Threadpool {
    fn neu(groesse: usize) -> Threadpool {
        assert!(groesse > 0, "ein Threadpool ohne Faden ist keiner");

        let (geber, empfaenger) = mpsc::channel::<Aufgabe>();
        let empfaenger = Arc::new(Mutex::new(empfaenger));
        let mut faeden = Vec::with_capacity(groesse);

        for _ in 0..groesse {
            let empfaenger = Arc::clone(&empfaenger);
            faeden.push(thread::spawn(move || {
                loop {
                    // Deutsch: Schloss nehmen, Aufgabe holen, Schloss loslassen.
                    // Das `let` beendet die Anweisung, und damit endet die
                    // Leihgabe. Erst danach laeuft die Aufgabe.
                    let geholt = empfaenger.lock().expect("das Schloss haelt").recv();

                    match geholt {
                        Ok(aufgabe) => aufgabe(),
                        Err(_) => break,
                    }
                }
            }));
        }

        Threadpool { faeden, geber: Some(geber) }
    }

    fn gib_auf<F: FnOnce() + Send + 'static>(&self, aufgabe: F) {
        self.geber
            .as_ref()
            .expect("der Pool nimmt noch an")
            .send(Box::new(aufgabe))
            .expect("ein Faden nimmt die Aufgabe");
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        // Deutsch: Erst den Sender loslassen, dann warten. Andersherum wartet
        // man auf Faeden, die noch auf Arbeit warten.
        drop(self.geber.take());

        for faden in self.faeden.drain(..) {
            let _ = faden.join();
        }
    }
}

fn main() {
    let start = Instant::now();
    let (fertig, gesammelt) = mpsc::channel();

    {
        let pool = Threadpool::neu(4);

        for nummer in 0..4 {
            let fertig = fertig.clone();
            pool.gib_auf(move || {
                thread::sleep(Duration::from_millis(200));
                fertig.send(nummer).expect("gesendet");
            });
        }
        // Deutsch: Hier endet der Block, der Pool wird fallengelassen, und das
        // Fallenlassen wartet auf die vier Aufgaben.
    }
    drop(fertig);

    let mut nummern: Vec<i32> = gesammelt.iter().collect();
    nummern.sort();

    println!("{nummern:?}");
    println!("unter 400 ms: {}", start.elapsed() < Duration::from_millis(400));
}
```

The program prints:

```text
[0, 1, 2, 3]
unter 400 ms: true
```

Four things about it are no accident. The `Arc` lies around the `Mutex` and not
the other way round: what is shared is the lock, and what it protects is the
receiving end. The `let` in front of the `match` is this pool's whole
concurrency, because it ends the statement and gives the lock back before the
task starts running. The sender lies in an `Option`, because the shutdown has to
take it out without taking the pool apart. And that all four numbers arrive
although nobody waits for the tasks is the work of `Drop` at the end of the
block.

### What a second thread buys

Measured rather than claimed. The following program takes two connections, makes
every request last 300 milliseconds and sends both off at the same time. The
number of threads in the pool stands behind it as an argument, and nothing else
changes.

```rust
fn main() -> std::io::Result<()> {
    let faeden: usize = std::env::args()
        .nth(1)
        .expect("die Zahl der Faeden steht als Argument da")
        .parse()
        .expect("das Argument ist eine Zahl");

    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || {
        let pool = Threadpool::neu(faeden);
        for _ in 0..2 {
            let (strom, _) = lauscher.accept().expect("angenommen");
            pool.gib_auf(move || bediene(strom));
        }
    });

    let erste = thread::spawn(move || frage(adresse));
    let zweite = thread::spawn(move || frage(adresse));

    let a = erste.join().expect("der Faden ist durchgelaufen");
    let b = zweite.join().expect("der Faden ist durchgelaufen");
    server.join().expect("der Faden ist durchgelaufen");

    let mut zeiten = [a.as_millis(), b.as_millis()];
    zeiten.sort();

    println!("{} ms", zeiten[0]);
    println!("{} ms", zeiten[1]);
    Ok(())
}
```

```console
$ rustc --edition 2024 -O pool.rs
$ ./pool.exe 1
300 ms
601 ms
$ ./pool.exe 2
300 ms
300 ms
```

With one thread the second request waits for the first, with two it does not.
Three rounds in a row gave the same four numbers. The rest of the program,
meaning `Threadpool`, `bediene` and `frage`, stands further up in this README or
in the unit before; what stands here is only the part the thread count appears
in.

That is a measurement on one machine and not a promise. What it shows is not how
fast this server is but that one thread more stops the second request from
waiting.

### Common mistakes

Giving the receiving end into several threads without sharing it.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (geber, empfaenger) = mpsc::channel::<Box<dyn FnOnce() + Send>>();

    for _ in 0..4 {
        thread::spawn(move || {
            while let Ok(aufgabe) = empfaenger.recv() {
                aufgabe();
            }
        });
    }

    drop(geber);
}
```

The compiler answers:

```text
error[E0382]: use of moved value: `empfaenger`
 --> geteilt.rs:8:23
  |
5 |     let (geber, empfaenger) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
  |                 ---------- move occurs because `empfaenger` has type `std::sync::mpsc::Receiver<Box<dyn FnOnce() + Send>>`, which does not implement the `Copy` trait
6 |
7 |     for _ in 0..4 {
  |     ------------- inside of this loop
8 |         thread::spawn(move || {
  |                       ^^^^^^^ value moved into closure here, in previous iteration of loop
9 |             while let Ok(aufgabe) = empfaenger.recv() {
  |             -----------------------------------------
  |             |                       |
  |             |                       use occurs due to use in closure
  |             inside of this loop

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

"in previous iteration of loop" is the whole story: the first pass goes fine, at
the second the receiver is gone. A receiving end belongs to exactly one place,
and whoever wants to give it to several gives all of them the same one: `Arc`
for the sharing, `Mutex` so that only one looks inside at a time.

The second mistake comes while waiting. `join` takes the thread with it, and in
`Drop` only a borrowed reference is available.

```rust
use std::thread;

struct Pool {
    faeden: Vec<thread::JoinHandle<()>>,
}

impl Drop for Pool {
    fn drop(&mut self) {
        for faden in &self.faeden {
            faden.join().expect("der Faden ist durchgelaufen");
        }
    }
}

fn main() {
    let _ = Pool { faeden: Vec::new() };
}
```

The compiler answers:

```text
error[E0507]: cannot move out of `*faden` which is behind a shared reference
  --> warten.rs:10:13
   |
10 |             faden.join().expect("der Faden ist durchgelaufen");
   |             ^^^^^ ------ `*faden` moved due to this method call
   |             |
   |             move occurs because `*faden` has type `JoinHandle<()>`, which does not implement the `Copy` trait
   |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `*faden`
  --> <std>/std/src/thread/join_handle.rs:149:16

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0507`.
```

Where `<std>` stands, the path to the standard library of this machine stood,
with the checksum of the compiler inside it, and the slashes leaned the other
way. Those are the only substitutions, otherwise the message stands as it came.
`drain(..)` solves it, because with it the threads belong to whoever takes them
out. The book takes `Option<JoinHandle<()>>` and `take` at this place; both
answer the same question of who owns the thread when `join` wants to have it.

### Your own tests for a server

A server listens on a socket, and a test occupying one is not like the other
tests of this repository: it hangs on the machine it runs on. That is why almost
every test here is built without a network, and exactly one really goes out.

The separation for that lies in the code and not in the tests. `bediene_einen`
takes a reader and a writer instead of a connection, so the whole behaviour of
HTTP can be checked against a piece of memory. The pool takes tasks instead of
requests, so it can be checked against counters and channels. What is left after
that is the putting together, and one test is enough for it.

```rust
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
```

The `0` in `127.0.0.1:0` means: the operating system picks the number, and
`local_addr` says afterwards which one it became. That way this test disturbs
neither a second run beside it nor a program already sitting on a fixed number.
And no assertion hangs on which of the two requests is served first: every
answer is checked against the path that asked for it.

### What these tests do not answer

That the pool really works side by side is seen by exactly one test, and the
others do not see it. Measured rather than supposed: in the solution the `let`
in front of the `match` was taken away, so that the lock is held across the
running of the task and the pool works off one task after another.

```console
$ cargo test -q -p unit-09-09-webserver-mit-threadpool --test exercise
running 6 tests
..... 5/6
exercise::die_aufgaben_laufen_nebeneinander --- FAILED

failures:

---- exercise::die_aufgaben_laufen_nebeneinander stdout ----

thread 'exercise::die_aufgaben_laufen_nebeneinander' (59584) panicked at solutions\09-09-webserver-mit-threadpool\tests\..\..\..\units\09-09-webserver-mit-threadpool\tests\exercise.rs:142:5:
eine Aufgabe hat die anderen nicht mehr angetroffen


failures:
    exercise::die_aufgaben_laufen_nebeneinander

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.02s

error: test failed, to rerun pass `-p unit-09-09-webserver-mit-threadpool --test exercise`
```

Five of the six tests stay green, and the one going over the network is among
them. Serving two requests one after another looks from the outside exactly like
serving two side by side, as long as nobody measures the time. What notices the
difference is the meeting point in `die_aufgaben_laufen_nebeneinander`, and it
notices it by one task no longer meeting the others. The 15 seconds instead of a
fifth of one are the same piece of information, only slower.

Just as little do the tests see how many threads the pool creates. What is
checked is that four tasks can run at the same time, and not how many threads
stand ready for them.

And nothing here checks a server reachable from outside. The one test over the
network binds `127.0.0.1`. On a machine where connections to itself do not get
through, it falls over, and that then says nothing about the solution.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `antwort`, `antwort_fuer` and `bediene_einen`
stand there finished, their doc tests are green, and `Drop` for the pool stands
there finished as well.

- `Threadpool::neu` builds the channel, the shared receiving end and the threads
- `Threadpool::gib_auf` boxes a task and sends it off
- `bediene_mit_pool` takes connections and hands each of them to the pool

```console
cd units/09-09-webserver-mit-threadpool
cargo test
```

### Source

    Book, chapter 21 "Final Project: Building a Multithreaded Web Server",
    section 21.2 "From Single-Threaded to Multithreaded Server",
    https://doc.rust-lang.org/book/ch21-02-multithreaded.html,
    checked against 1.97.1

    Book, chapter 21 "Final Project: Building a Multithreaded Web Server",
    section 21.3 "Graceful Shutdown and Cleanup",
    https://doc.rust-lang.org/book/ch21-03-graceful-shutdown-and-cleanup.html,
    checked against 1.97.1

    The standard library, chapter `std::sync::mpsc`,
    https://doc.rust-lang.org/std/sync/mpsc/index.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
