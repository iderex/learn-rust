# 09-08 Der Webserver, einfach und blockierend / The web server, simple and blocking

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/09-08-webserver-einfach-und-blockierend/`. Sie ist öffentlich. Wer
  nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `06-02 Dateien lesen und schreiben`, woher `BufReader`
  und die Ströme kommen, und `07-05 Threads`, denn der Server läuft in den
  Beispielen in einem eigenen Faden.
- Auf dieser Einheit bauen auf: alles, was denselben Server mehr als eine
  Anfrage zur selben Zeit bedienen lässt.
- Beim Antworten so zitieren: `09-08 Der Webserver, einfach und blockierend`,
  dazu die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Zeilen einer HTTP-Nachricht enden mit `\r\n`. Steht in `antwort` ein `\n`,
  fallen vier der acht Tests um. Ob der ganze Kopf gelesen wird oder nur die
  erste Zeile, sehen dieselben Tests dagegen nicht.
- Die Zahlen unter "Warum nur eine Anfrage zur selben Zeit" sind auf einem
  Rechner gemessen. Wer sie weitergibt, gibt den Befehl mit.
- Dass dieser Server nur eine Anfrage zur selben Zeit bedient, zeigt kein Test
  dieser Einheit. Das steht unter "Was diese Tests nicht beantworten", und diese
  Aussage bleibt negativ.
- Ein Test dieser Einheit öffnet einen Anschluss auf `127.0.0.1` mit einer
  Nummer, die das Betriebssystem aussucht. Auf einem Rechner, auf dem
  Verbindungen zu sich selbst nicht durchkommen, fällt er aus einem Grund um,
  der nichts mit der Aufgabe zu tun hat.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/09-08-webserver-einfach-und-blockierend/`. It is public. Whoever is
  asked for it may name it, but should explain the compiler message in question
  first.
- This unit builds on: `06-02 Dateien lesen und schreiben`, where `BufReader`
  and the streams come from, and `07-05 Threads`, because the server runs in a
  thread of its own in the examples.
- Building on this unit: everything that lets the same server serve more than
  one request at a time.
- Cite like this when answering: `09-08 Der Webserver, einfach und blockierend`,
  plus the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The lines of an HTTP message end with `\r\n`. With a `\n` in `antwort`, four of
  the eight tests fall over. Whether the whole head is read or only the first
  line is not seen by those same tests.
- The numbers under "Why only one request at a time" are measured on one
  machine. Whoever passes them on passes the command on with them.
- That this server serves only one request at a time is shown by no test of this
  unit. That stands under "What these tests do not answer", and that statement
  stays negative.
- One test of this unit opens a socket on `127.0.0.1` with a number the
  operating system picks. On a machine where connections to itself do not get
  through, it falls over for a reason that has nothing to do with the exercise.

</details>

## Deutsch

### Worum es geht

Ein Webserver ist ein Programm, das an einem Anschluss wartet, Text liest und
Text zurückschreibt. HTTP ist dabei nichts Geheimnisvolles: Die Anfrage ist eine
Zeile mit Verb, Pfad und Version, danach kommen Kopfzeilen, dann eine leere
Zeile. Die Antwort sieht genauso aus, nur dass vorn eine Statuszeile steht.

`TcpListener` wartet, `accept` gibt eine Verbindung zurück, und die Verbindung
ist ein `TcpStream`, aus dem gelesen und in den geschrieben wird. Mehr Bauteile
kommen in dieser Einheit nicht vor.

Der Server, der hier entsteht, bedient eine Anfrage nach der anderen. Warum das
so ist und was es kostet, steht weiter unten, und die Zahlen dazu sind gemessen.

### Wofür das gut ist

Weil es die kleinste Fassung von etwas ist, das sonst hinter einem Rahmenwerk
verschwindet. Wer einmal gesehen hat, dass eine Antwort schlicht Text mit `\r\n`
darin ist, liest die Fehlermeldungen größerer Werkzeuge anders.

Und weil hier zum ersten Mal sichtbar wird, was Blockieren wirklich heißt.
Solange der Server an einer Verbindung liest, tut er nichts anderes, auch dann
nicht, wenn schon jemand anders anklopft. Das ist keine Meinung über
Nebenläufigkeit, sondern eine Eigenschaft dieses Aufbaus, und sie lässt sich
messen.

Die nächste Einheit dieser Stufe nimmt genau diesen Punkt auf.

### Die Erklärung

Ein Server und ein Aufrufer im selben Programm, damit man beide Seiten sieht.

```rust
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn bediene_einmal(lauscher: &TcpListener) -> std::io::Result<()> {
    // Deutsch: `accept` wartet, bis jemand anklopft, und gibt dann genau eine
    // Verbindung zurueck.
    let (strom, _) = lauscher.accept()?;
    let mut leser = BufReader::new(&strom);

    let mut kopf = Vec::new();
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
        kopf.push(ohne_ende.to_string());
    }

    println!("gelesen: {:?}", kopf.first());

    let rumpf = "Hallo";
    let text = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{rumpf}",
        rumpf.len()
    );

    let mut schreiber = &strom;
    schreiber.write_all(text.as_bytes())?;
    schreiber.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || bediene_einmal(&lauscher));

    let mut strom = TcpStream::connect(adresse)?;
    write!(strom, "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")?;

    let mut zurueck = String::new();
    strom.read_to_string(&mut zurueck)?;
    server.join().expect("der Faden ist durchgelaufen")?;

    println!("zurueck: {zurueck:?}");
    Ok(())
}
```

Das Programm gibt aus:

```text
gelesen: Some("GET / HTTP/1.1")
zurueck: "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo"
```

Drei Dinge daran sind kein Zufall. Die `0` in `127.0.0.1:0` heißt: Das
Betriebssystem sucht die Nummer aus, und `local_addr` sagt danach, welche es
geworden ist; so kollidiert der Lauf mit nichts. Gelesen wird bis zur leeren
Zeile und nicht bis zum Ende der Verbindung, denn dieses Ende kommt nicht,
solange die Gegenseite auf die Antwort wartet. Und die Antwort trägt
`Content-Length`, weil die Gegenseite sonst nicht weiß, wann sie zu Ende ist.

### Warum nur eine Anfrage zur selben Zeit

Der Rumpf oben nimmt eine Verbindung an, arbeitet sie ab und ist fertig. Wer
mehrere bedienen will, ruft ihn mehrmals auf, und das heißt: nacheinander. Solange
an der ersten Verbindung gearbeitet wird, liegt die zweite in der Warteschlange
des Betriebssystems.

Gemessen und nicht behauptet. Das folgende Programm lässt jede Anfrage 300
Millisekunden dauern und schickt zwei davon gleichzeitig los.

```rust
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn bediene_einmal(lauscher: &TcpListener) -> std::io::Result<()> {
    let (strom, _) = lauscher.accept()?;
    let mut leser = BufReader::new(&strom);
    let mut zeile = String::new();

    loop {
        zeile.clear();
        if leser.read_line(&mut zeile)? == 0 {
            break;
        }
        if zeile.trim_end_matches(['\r', '\n']).is_empty() {
            break;
        }
    }

    // Deutsch: Hier steht die Arbeit, die eine Anfrage macht.
    thread::sleep(Duration::from_millis(300));

    let mut schreiber = &strom;
    schreiber.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo")?;
    schreiber.flush()?;
    Ok(())
}

fn frage(adresse: SocketAddr) -> Duration {
    let start = Instant::now();
    let mut strom = TcpStream::connect(adresse).expect("die Verbindung steht");
    write!(strom, "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").expect("die Anfrage ist raus");
    let mut zurueck = String::new();
    strom.read_to_string(&mut zurueck).expect("die Antwort ist da");
    start.elapsed()
}

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || {
        bediene_einmal(&lauscher).expect("erste Anfrage");
        bediene_einmal(&lauscher).expect("zweite Anfrage");
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
$ rustc --edition 2024 -O nacheinander.rs
$ ./nacheinander.exe
300 ms
601 ms
```

Beide Aufrufer starten zur selben Zeit. Der eine ist nach 300 Millisekunden
fertig, der andere nach 601. Bei einem Server, der beide gleichzeitig bedient,
stünde zweimal ungefähr 300 da. Drei Läufe hintereinander ergaben 300 und 601,
300 und 601, dann 300 und 600.

Das ist eine Messung auf einem Rechner und keine Zusage. Was sie zeigt, ist
nicht, wie schnell dieser Server ist, sondern dass die zweite Anfrage auf die
erste wartet.

### Häufige Fehler

In eine Verbindung schreiben, die nicht als veränderbar dasteht.

```rust
use std::io::Write;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let (strom, _) = lauscher.accept()?;

    strom.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;

    Ok(())
}
```

Der Übersetzer sagt dazu:

```text
error[E0596]: cannot borrow `strom` as mutable, as it is not declared as mutable
 --> schreiben.rs:8:5
  |
8 |     strom.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let (mut strom, _) = lauscher.accept()?;
  |          +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

Der Vorschlag des Übersetzers löst es, und er ist nicht der einzige Weg. `Write`
ist auch für `&TcpStream` da, also reicht eine gemeinsame Referenz zum
Schreiben. Das ist der Grund, warum in der Lösung `let mut schreiber = &strom;`
neben einem `BufReader` auf dieselbe Verbindung stehen kann.

Der zweite Fehler kommt beim Lesen. `lines` gibt keine Zeichenketten zurück,
sondern Ergebnisse.

```rust
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let (strom, _) = lauscher.accept()?;
    let leser = BufReader::new(&strom);

    for zeile in leser.lines() {
        if zeile.is_empty() {
            break;
        }
    }

    Ok(())
}
```

Der Übersetzer sagt dazu:

```text
error[E0599]: no method named `is_empty` found for enum `Result<T, E>` in the current scope
  --> zeilen.rs:10:18
   |
10 |         if zeile.is_empty() {
   |                  ^^^^^^^^ method not found in `Result<String, std::io::Error>`
   |
note: the method `is_empty` exists on the type `String`
  --> <std>/alloc/src/string.rs:1889:4
help: use the `?` operator to extract the `String` value, propagating a `Result::Err` value to the caller
   |
10 |         if zeile?.is_empty() {
   |                 +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
```

An `<std>` stand der Pfad zur Standardbibliothek dieses Rechners, mit der
Prüfsumme des Übersetzers darin. Das ist die einzige Ersetzung, sonst steht die
Meldung so da, wie sie kam. Lesen kann schiefgehen, und deshalb kommt aus
`lines` ein `Result` heraus, das ausgepackt werden will.

### Was diese Tests nicht beantworten

Dass dieser Server nur eine Anfrage zur selben Zeit bedient, zeigt kein Test
dieser Einheit. Die Tests schicken eine Anfrage oder eine nach der anderen, und
ein Server, der beide gleichzeitig bediente, käme durch dieselben Tests. Der
Beleg für das Nacheinander steht weiter oben und ist ein eigenes Programm mit
einer Zeitmessung, kein Test.

Ebenso wenig sehen die Tests, ob `bediene_einen` den ganzen Kopf liest oder nur
die erste Zeile. Nachgemessen und nicht vermutet: In der Lösung wurde
`kopf_lesen` durch ein einzelnes `read_line` ersetzt, sodass der Rest der
Anfrage ungelesen liegen bleibt, und die Testdatei danach dreimal ausgeführt.

```console
$ cargo test -q -p unit-09-08-webserver-einfach-und-blockierend --test exercise
running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Dreimal dieselbe Zeile, dreimal grün. Der Grund, den ganzen Kopf zu lesen, ist
eine Eigenschaft der Gegenseite und keine dieser Tests: Was ungelesen
liegenbleibt, wenn die Verbindung zugeht, kann drüben als abgebrochene
Verbindung ankommen statt als Antwort. Auf diesem Rechner kam es nicht so weit,
und daraus folgt nichts über andere Rechner.

Was die Tests dagegen sehr wohl sehen, ist das Zeilenende. Wird in `antwort`
jedes `\r\n` durch `\n` ersetzt, fallen vier der acht Tests um, weil sie die
Antwort im Ganzen vergleichen statt sie zu zerlegen.

Ein Test dieser Einheit geht wirklich über das Netz. Er bindet `127.0.0.1` mit
einer Nummer, die das Betriebssystem aussucht. Auf einem Rechner, auf dem
Verbindungen zu sich selbst nicht durchkommen, fällt er um, und das sagt dann
nichts über die Lösung.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `antwort` und `kopf_lesen` stehen fertig da, und ihre
Doku-Tests sind grün.

- `pfad_aus` holt den Pfad aus der ersten Zeile einer Anfrage
- `antwort_fuer` baut die Antwort, die zu einem Pfad gehört
- `bediene_einen` liest aus einem Strom und schreibt in einen zweiten
- `bediene_einmal` nimmt eine Verbindung an und gibt sie an `bediene_einen`

```console
cd units/09-08-webserver-einfach-und-blockierend
cargo test
```

### Quelle

    Buch, Kapitel 21 "Final Project: Building a Multithreaded Web Server",
    Abschnitt 21.1 "Building a Single-Threaded Web Server",
    https://doc.rust-lang.org/book/ch21-01-single-threaded.html,
    geprüft gegen 1.97.1

    Die Standardbibliothek, Kapitel `std::net`,
    https://doc.rust-lang.org/std/net/index.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A web server is a program that waits at a socket, reads text and writes text
back. HTTP is nothing mysterious about it: the request is a line with a verb, a
path and a version, then come header lines, then an empty line. The response
looks the same, except that a status line stands at the front.

`TcpListener` waits, `accept` gives back a connection, and the connection is a
`TcpStream` that is read from and written to. No further parts appear in this
unit.

The server built here serves one request after another. Why that is and what it
costs stands further down, and the numbers for it are measured.

### What it is good for

Because it is the smallest version of something that otherwise disappears behind
a framework. Whoever has once seen that a response is plainly text with `\r\n` in
it reads the error messages of larger tools differently.

And because this is where it first becomes visible what blocking really means.
As long as the server is reading on one connection it does nothing else, not even
when somebody else is already knocking. That is not an opinion about concurrency
but a property of this arrangement, and it can be measured.

The next unit of this stage picks up exactly that point.

### The explanation

A server and a caller in the same program, so that both sides are visible.

```rust
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn bediene_einmal(lauscher: &TcpListener) -> std::io::Result<()> {
    // Deutsch: `accept` wartet, bis jemand anklopft, und gibt dann genau eine
    // Verbindung zurueck.
    let (strom, _) = lauscher.accept()?;
    let mut leser = BufReader::new(&strom);

    let mut kopf = Vec::new();
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
        kopf.push(ohne_ende.to_string());
    }

    println!("gelesen: {:?}", kopf.first());

    let rumpf = "Hallo";
    let text = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{rumpf}",
        rumpf.len()
    );

    let mut schreiber = &strom;
    schreiber.write_all(text.as_bytes())?;
    schreiber.flush()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || bediene_einmal(&lauscher));

    let mut strom = TcpStream::connect(adresse)?;
    write!(strom, "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n")?;

    let mut zurueck = String::new();
    strom.read_to_string(&mut zurueck)?;
    server.join().expect("der Faden ist durchgelaufen")?;

    println!("zurueck: {zurueck:?}");
    Ok(())
}
```

The program prints:

```text
gelesen: Some("GET / HTTP/1.1")
zurueck: "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo"
```

Three things about it are no accident. The `0` in `127.0.0.1:0` means: the
operating system picks the number, and `local_addr` says afterwards which one it
became; that way the run collides with nothing. Reading goes up to the empty line
and not up to the end of the connection, because that end does not come while the
other side is waiting for the answer. And the response carries `Content-Length`,
because otherwise the other side does not know when it is over.

### Why only one request at a time

The body above takes one connection, works it off and is done. Whoever wants to
serve several calls it several times, and that means: one after another. As long
as work is going on at the first connection, the second one lies in the queue of
the operating system.

Measured rather than claimed. The following program makes every request take 300
milliseconds and sends two of them off at the same time.

```rust
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn bediene_einmal(lauscher: &TcpListener) -> std::io::Result<()> {
    let (strom, _) = lauscher.accept()?;
    let mut leser = BufReader::new(&strom);
    let mut zeile = String::new();

    loop {
        zeile.clear();
        if leser.read_line(&mut zeile)? == 0 {
            break;
        }
        if zeile.trim_end_matches(['\r', '\n']).is_empty() {
            break;
        }
    }

    // Deutsch: Hier steht die Arbeit, die eine Anfrage macht.
    thread::sleep(Duration::from_millis(300));

    let mut schreiber = &strom;
    schreiber.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHallo")?;
    schreiber.flush()?;
    Ok(())
}

fn frage(adresse: SocketAddr) -> Duration {
    let start = Instant::now();
    let mut strom = TcpStream::connect(adresse).expect("die Verbindung steht");
    write!(strom, "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").expect("die Anfrage ist raus");
    let mut zurueck = String::new();
    strom.read_to_string(&mut zurueck).expect("die Antwort ist da");
    start.elapsed()
}

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let adresse = lauscher.local_addr()?;

    let server = thread::spawn(move || {
        bediene_einmal(&lauscher).expect("erste Anfrage");
        bediene_einmal(&lauscher).expect("zweite Anfrage");
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
$ rustc --edition 2024 -O nacheinander.rs
$ ./nacheinander.exe
300 ms
601 ms
```

Both callers start at the same time. One is done after 300 milliseconds, the
other after 601. With a server serving both at the same time, roughly 300 would
stand there twice. Three runs in a row gave 300 and 601, 300 and 601, then 300
and 600.

That is a measurement on one machine and not a promise. What it shows is not how
fast this server is but that the second request waits for the first.

### Common mistakes

Writing into a connection that does not stand there as mutable.

```rust
use std::io::Write;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let (strom, _) = lauscher.accept()?;

    strom.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;

    Ok(())
}
```

The compiler answers:

```text
error[E0596]: cannot borrow `strom` as mutable, as it is not declared as mutable
 --> schreiben.rs:8:5
  |
8 |     strom.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let (mut strom, _) = lauscher.accept()?;
  |          +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

The compiler's suggestion solves it, and it is not the only way. `Write` is there
for `&TcpStream` as well, so a shared reference is enough for writing. That is
the reason why `let mut schreiber = &strom;` can stand next to a `BufReader` on
the same connection in the solution.

The second mistake comes while reading. `lines` gives back no strings but
results.

```rust
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let lauscher = TcpListener::bind("127.0.0.1:0")?;
    let (strom, _) = lauscher.accept()?;
    let leser = BufReader::new(&strom);

    for zeile in leser.lines() {
        if zeile.is_empty() {
            break;
        }
    }

    Ok(())
}
```

The compiler answers:

```text
error[E0599]: no method named `is_empty` found for enum `Result<T, E>` in the current scope
  --> zeilen.rs:10:18
   |
10 |         if zeile.is_empty() {
   |                  ^^^^^^^^ method not found in `Result<String, std::io::Error>`
   |
note: the method `is_empty` exists on the type `String`
  --> <std>/alloc/src/string.rs:1889:4
help: use the `?` operator to extract the `String` value, propagating a `Result::Err` value to the caller
   |
10 |         if zeile?.is_empty() {
   |                 +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0599`.
```

Where `<std>` stands, the path to the standard library of this machine stood,
with the checksum of the compiler inside it. That is the only substitution,
otherwise the message stands as it came. Reading can go wrong, and that is why a
`Result` comes out of `lines` that wants unpacking.

### What these tests do not answer

That this server serves only one request at a time is shown by no test of this
unit. The tests send one request, or one after another, and a server serving both
at the same time would get through the same tests. The evidence for the one after
another stands further up and is a program of its own with a time measurement,
not a test.

Just as little do the tests see whether `bediene_einen` reads the whole head or
only the first line. Measured rather than supposed: in the solution `kopf_lesen`
was replaced by a single `read_line`, so that the rest of the request stays
unread, and the test file was run three times afterwards.

```console
$ cargo test -q -p unit-09-08-webserver-einfach-und-blockierend --test exercise
running 8 tests
........
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The same line three times, green three times. The reason for reading the whole
head is a property of the other side and none of these tests: what stays unread
when the connection closes can arrive over there as a broken connection instead
of as an answer. On this machine it did not come to that, and nothing about other
machines follows from it.

What the tests do see is the line ending. With every `\r\n` inside `antwort`
replaced by `\n`, four of the eight tests fall over, because they compare the
response as a whole instead of taking it apart.

One test of this unit really goes over the network. It binds `127.0.0.1` with a
number the operating system picks. On a machine where connections to itself do
not get through, it falls over, and that then says nothing about the solution.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `antwort` and `kopf_lesen` stand there
finished, and their doc tests are green.

- `pfad_aus` gets the path out of the first line of a request
- `antwort_fuer` builds the response that belongs to a path
- `bediene_einen` reads out of one stream and writes into a second
- `bediene_einmal` takes a connection and hands it to `bediene_einen`

```console
cd units/09-08-webserver-einfach-und-blockierend
cargo test
```

### Source

    Book, chapter 21 "Final Project: Building a Multithreaded Web Server",
    section 21.1 "Building a Single-Threaded Web Server",
    https://doc.rust-lang.org/book/ch21-01-single-threaded.html,
    checked against 1.97.1

    The standard library, chapter `std::net`,
    https://doc.rust-lang.org/std/net/index.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
