# 07-06 Kanäle / Channels

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-06-kanaele/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-01 Verschieben / Move` und `06-04 Closures`. Ein
  verschickter Wert wird verschoben, und der Faden bekommt eine Closure.
- Auf dieser Einheit bauen auf: der Rest der Stufe 7 und alles, was Arbeit auf
  mehrere Fäden verteilt.
- Beim Antworten so zitieren: `07-06 Kanäle`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Reihenfolge zwischen zwei sendenden Fäden ist nicht festgelegt. Ein Test,
  der eine bestimmte Reihenfolge behauptet, behauptet etwas, das nur heute und
  nur hier stimmt. Der Test dieser Einheit sortiert deshalb.
- Die Schleife über den Empfänger endet, wenn der letzte Sender weggefallen ist,
  und nicht, wenn gerade nichts kommt. Ein vergessener Sender im Hauptfaden
  lässt sie nie enden.
- `mpsc` heißt viele Sender und ein Empfänger. Der Sender wird mit `clone`
  vervielfältigt, der Empfänger nicht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-06-kanaele/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `02-01 Verschieben / Move` and `06-04 Closures`. A sent
  value is moved, and the thread gets a closure.
- Building on this unit: the rest of stage 7 and everything that spreads work
  over several threads.
- Cite like this when answering: `07-06 Kanäle`, plus the heading of the section,
  for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The order between two sending threads is not fixed. A test claiming a
  particular order claims something that holds only today and only here. The test
  of this unit therefore sorts.
- The loop over the receiver ends when the last sender has fallen away, and not
  when nothing is coming at the moment. A forgotten sender in the main thread
  lets it never end.
- `mpsc` means many senders and one receiver. The sender is multiplied with
  `clone`, the receiver is not.

</details>

## Deutsch

### Worum es geht

Ein Kanal hat zwei Enden. `mpsc::channel()` gibt beide zurück: einen Sender und
einen Empfänger. Was der Sender hineingibt, kommt am Empfänger heraus.

Der Wert wird dabei verschoben. Wer ihn geschickt hat, hat ihn danach nicht
mehr, und das ist der Grund, warum ein Kanal zwischen Fäden ohne weitere
Absprache funktioniert: Es gibt zu keinem Zeitpunkt zwei Stellen, die ihn
ändern könnten.

`mpsc` steht für "multiple producer, single consumer". Viele dürfen schicken,
und dafür wird der Sender mit `clone` vervielfältigt. Entgegennehmen darf genau
einer.

### Wofür das gut ist

Arbeit auf mehrere Fäden zu verteilen ist einfach. Das Ergebnis wieder
einzusammeln ist die Stelle, an der es schiefgeht, wenn alle in dieselbe Liste
schreiben. Ein Kanal nimmt diese Stelle weg: Jeder schickt, einer sammelt.

Die Schleife über den Empfänger endet von selbst, und zwar am richtigen Punkt.
Sie endet nicht, wenn gerade nichts kommt, sondern wenn der letzte Sender
weggefallen ist, denn erst dann steht fest, dass nichts mehr kommen kann.

Das ist zugleich die häufigste Falle. Bleibt im Hauptfaden ein Sender liegen,
der nichts mehr schickt, endet die Schleife nie, und das Programm bleibt ohne
Fehlermeldung stehen.

### Die Erklärung

Ein Faden schickt, der Hauptfaden sammelt ein.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Deutsch: Ein Kanal hat zwei Enden. `sender` schickt, `empfaenger` nimmt
    // entgegen.
    let (sender, empfaenger) = mpsc::channel();

    // Deutsch: `move` schiebt den Sender in den Faden hinein. Ohne das würde er
    // nur ausgeliehen, und der Faden könnte ihn überleben.
    let zweiter = sender.clone();
    thread::spawn(move || {
        for wert in [1, 2, 3] {
            zweiter.send(wert).expect("der Empfaenger lebt noch");
        }
    });

    // Deutsch: Der ursprüngliche Sender wird hier nicht mehr gebraucht. Fällt er
    // nicht weg, endet die Schleife unten nie.
    drop(sender);

    // Deutsch: Die Schleife endet, wenn der letzte Sender weggefallen ist.
    let mut gesammelt = Vec::new();
    for wert in empfaenger {
        gesammelt.push(wert);
    }

    println!("{gesammelt:?}");
    println!("{}", gesammelt.len());
}
```

Das Programm gibt aus:

```text
[1, 2, 3]
3
```

Die Reihenfolge steht hier fest, weil nur ein Faden schickt. Mit zwei sendenden
Fäden stünde sie nicht fest, und dann wäre eine Ausgabe wie diese kein Beleg
mehr, sondern ein Zufall, der zweimal gleich ausgegangen ist.

Die Zeile `drop(sender)` sieht überflüssig aus und ist der Kern. Der Faden hat
seine eigene Kopie, und der hier bleibt sonst bis zum Ende von `main` liegen.
Die Schleife darüber wartet dann auf etwas, das nie kommt.

### Häufige Fehler

Das `move` vor der Closure vergessen.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, empfaenger) = mpsc::channel();

    thread::spawn(|| {
        sender.send(1).expect("der Empfaenger lebt noch");
    });

    println!("{}", empfaenger.recv().expect("es kommt eine Zahl"));
}
```

Der Übersetzer sagt dazu:

```text
error[E0373]: closure may outlive the current function, but it borrows `sender`, which is owned by the current function
 --> ohne-move.rs:7:19
  |
7 |     thread::spawn(|| {
  |                   ^^ may outlive borrowed value `sender`
8 |         sender.send(1).expect("der Empfaenger lebt noch");
  |         ------ `sender` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> ohne-move.rs:7:5
  |
7 | /     thread::spawn(|| {
8 | |         sender.send(1).expect("der Empfaenger lebt noch");
9 | |     });
  | |______^
help: to force the closure to take ownership of `sender` (and any other referenced variables), use the `move` keyword
  |
7 |     thread::spawn(move || {
  |                   ++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0373`.
```

Die Anmerkung nennt den eigentlichen Grund: `thread::spawn` verlangt `'static`,
also etwas, das so lange leben darf wie das Programm. Eine ausgeliehene
Referenz auf eine Stelle in `main` kann das nicht zusagen, denn `main` kann
vorher zu Ende sein. `move` löst es, weil der Faden den Sender dann besitzt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `echo` steht fertig da, und sein Doku-Test ist grün.

- `send_all` lässt einen Faden alle Werte schicken und sammelt sie ein
- `merge_two` lässt zwei Fäden durch denselben Kanal schicken
- `drain` nimmt mit `recv` entgegen, bis der letzte Sender weg ist

```console
cd units/07-06-kanaele
cargo test
```

### Quelle

    Buch, Kapitel 16 "Fearless Concurrency", Abschnitt 16.2
    "Transfer Data Between Threads with Message Passing",
    https://doc.rust-lang.org/book/ch16-02-message-passing.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A channel has two ends. `mpsc::channel()` gives back both: a sender and a
receiver. What the sender puts in comes out at the receiver.

The value is moved while doing so. Whoever sent it does not have it afterwards,
and that is the reason a channel works between threads without any further
agreement: at no point in time are there two places that could change it.

`mpsc` stands for "multiple producer, single consumer". Many are allowed to send,
and for that the sender is multiplied with `clone`. Exactly one is allowed to
take.

### What it is good for

Spreading work over several threads is easy. Collecting the result again is the
place where it goes wrong if everybody writes into the same list. A channel takes
that place away: everybody sends, one collects.

The loop over the receiver ends by itself, and at the right point. It does not
end when nothing is coming at the moment but when the last sender has fallen
away, because only then is it settled that nothing more can come.

That is at the same time the most common trap. If a sender is left lying in the
main thread that sends nothing more, the loop never ends, and the program stops
without an error message.

### The explanation

One thread sends, the main thread collects.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    // Deutsch: Ein Kanal hat zwei Enden. `sender` schickt, `empfaenger` nimmt
    // entgegen.
    let (sender, empfaenger) = mpsc::channel();

    // Deutsch: `move` schiebt den Sender in den Faden hinein. Ohne das würde er
    // nur ausgeliehen, und der Faden könnte ihn überleben.
    let zweiter = sender.clone();
    thread::spawn(move || {
        for wert in [1, 2, 3] {
            zweiter.send(wert).expect("der Empfaenger lebt noch");
        }
    });

    // Deutsch: Der ursprüngliche Sender wird hier nicht mehr gebraucht. Fällt er
    // nicht weg, endet die Schleife unten nie.
    drop(sender);

    // Deutsch: Die Schleife endet, wenn der letzte Sender weggefallen ist.
    let mut gesammelt = Vec::new();
    for wert in empfaenger {
        gesammelt.push(wert);
    }

    println!("{gesammelt:?}");
    println!("{}", gesammelt.len());
}
```

The program prints:

```text
[1, 2, 3]
3
```

The order is fixed here because only one thread sends. With two sending threads
it would not be fixed, and then an output like this one would no longer be
evidence but a coincidence that came out the same twice.

The line `drop(sender)` looks superfluous and is the core. The thread has a copy
of its own, and the one here would otherwise lie around until the end of `main`.
The loop above it would then wait for something that never comes.

### Common mistakes

Forgetting the `move` in front of the closure.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, empfaenger) = mpsc::channel();

    thread::spawn(|| {
        sender.send(1).expect("der Empfaenger lebt noch");
    });

    println!("{}", empfaenger.recv().expect("es kommt eine Zahl"));
}
```

The compiler answers:

```text
error[E0373]: closure may outlive the current function, but it borrows `sender`, which is owned by the current function
 --> ohne-move.rs:7:19
  |
7 |     thread::spawn(|| {
  |                   ^^ may outlive borrowed value `sender`
8 |         sender.send(1).expect("der Empfaenger lebt noch");
  |         ------ `sender` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> ohne-move.rs:7:5
  |
7 | /     thread::spawn(|| {
8 | |         sender.send(1).expect("der Empfaenger lebt noch");
9 | |     });
  | |______^
help: to force the closure to take ownership of `sender` (and any other referenced variables), use the `move` keyword
  |
7 |     thread::spawn(move || {
  |                   ++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0373`.
```

The note names the actual reason: `thread::spawn` asks for `'static`, meaning
something that may live as long as the program. A borrowed reference to a place
in `main` cannot promise that, because `main` can be over earlier. `move` settles
it, because the thread then owns the sender.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `echo` stands there finished, and its doc test
is green.

- `send_all` lets one thread send all values and collects them
- `merge_two` lets two threads send through the same channel
- `drain` takes with `recv` until the last sender is gone

```console
cd units/07-06-kanaele
cargo test
```

### Source

    Book, chapter 16 "Fearless Concurrency", section 16.2
    "Transfer Data Between Threads with Message Passing",
    https://doc.rust-lang.org/book/ch16-02-message-passing.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
