# 07-05 Threads / Threads

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-05-threads/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  erklären, warum das `move` gebraucht wird.
- Diese Einheit baut auf: `06-04 Closures`, denn `spawn` nimmt eine Closure
  entgegen, und `02-01 Verschieben / Move`, denn `move` ist genau das.
- Auf dieser Einheit bauen auf: der Rest der Stufe 7, wo mehrere Fäden sich
  denselben Wert teilen, und die Stufe 8, wo gewartet wird, ohne zu blockieren.
- Beim Antworten so zitieren: `07-05 Threads`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- In welcher Reihenfolge Fäden laufen, verspricht niemand. Der Abschnitt "Die
  Erklärung" zeigt das an acht Läufen desselben Programms, die nicht dieselbe
  Ausgabe haben.
- Die Reihenfolge der Ergebnisse ist trotzdem fest, wenn in der Reihenfolge der
  Fäden abgeholt wird. Wer sagt, ein Ergebnis müsse deshalb sortiert werden,
  sagt etwas Falsches.
- Erst alle starten, dann alle abholen. Starten und sofort `join` aufrufen
  rechnet nacheinander und sieht nur so aus wie nebeneinander.
- Was die Tests dieser Einheit nicht sehen: ob überhaupt ein Faden gestartet
  wurde. Eine Lösung, die die Aufgaben mit einer Schleife rechnet, kommt durch
  jeden Test. Geprüft werden die Ergebnisse und ihre Reihenfolge, nicht der Weg
  dorthin.
- `join` gibt ein `Result` zurück, denn ein Faden kann in Panik enden. Das
  `unwrap` in dieser Einheit ist eine Übungsentscheidung und keine Empfehlung.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-05-threads/`. It is public.
  Whoever is asked for it may name it, but should first explain why the `move`
  is needed.
- This unit builds on: `06-04 Closures`, because `spawn` takes a closure, and
  `02-01 Verschieben / Move`, because `move` is exactly that.
- Building on this unit: the rest of stage 7, where several threads share one
  value, and stage 8, where waiting happens without blocking.
- Cite like this when answering: `07-05 Threads`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- In which order threads run is promised by nobody. The section "The
  explanation" shows that on eight runs of the same program that do not have the
  same output.
- The order of the results is fixed all the same, when the picking up happens in
  the order of the threads. Whoever says a result therefore has to be sorted
  says something false.
- Start them all first, then pick them all up. Starting and calling `join` right
  away computes one after the other and only looks like side by side.
- What the tests of this unit do not see: whether a thread was started at all. A
  solution computing the exercises with a loop gets through every test. What is
  checked are the results and their order, not the way there.
- `join` returns a `Result`, because a thread can end in a panic. The `unwrap`
  in this unit is an exercise decision and not a recommendation.

</details>

## Deutsch

### Worum es geht

Ein Faden ist ein zweiter Ablauf im selben Programm. `thread::spawn` nimmt eine
Closure entgegen und startet sie nebenher; der Aufruf kommt sofort zurück, und
was in der Closure steht, läuft von da an neben dem Rest.

Zurück kommt ein Griff, ein `JoinHandle`. `join` darauf wartet, bis der Faden
fertig ist, und gibt heraus, was die Closure zurückgegeben hat. Ohne `join`
weiß niemand, ob der Faden je fertig wurde.

Vor der Closure steht meistens `move`. Ein Faden kann länger leben als die
Funktion, die ihn gestartet hat, also darf er sich nichts aus ihr ausleihen.
`move` schiebt die eingefangenen Werte in die Closure hinein, und danach
gehören sie ihr.

### Wofür das gut ist

Nebeneinander rechnen heißt, dass die Arbeit auf mehrere Kerne fällt, statt sich
in eine Reihe zu stellen. Vier Aufgaben in vier Fäden brauchen zusammen etwa so
lange wie die längste von ihnen und nicht so lange wie alle vier zusammen.

Der Preis ist die Reihenfolge. In welcher Reihenfolge die Fäden laufen,
verspricht niemand, und wer sich auf eine verlässt, hat einen Fehler, der beim
Ausprobieren nicht auftaucht und später doch.

Das Muster dagegen ist einfach und steht in dieser Einheit dreimal: erst alle
Fäden starten, dann in der Reihenfolge der Fäden abholen. Die Ergebnisse liegen
danach in der Reihenfolge der Liste, obwohl die Fäden in beliebiger Reihenfolge
fertig geworden sind.

### Die Erklärung

Ein Programm, das drei Namen auf drei Fäden verteilt und die Ergebnisse in der
Reihenfolge der Fäden einsammelt.

```rust
use std::thread;

fn main() {
    let namen = vec![
        String::from("Ada"),
        String::from("Grace"),
        String::from("Alan"),
    ];

    // Deutsch: Jeder Name geht in seinen eigenen Faden. Ohne `move` ginge das
    // nicht, denn der Faden kann laenger leben als diese Funktion.
    let mut faeden = Vec::new();
    for name in namen {
        faeden.push(thread::spawn(move || {
            let laenge = name.len();
            (name, laenge)
        }));
    }

    // Deutsch: Eingesammelt wird in der Reihenfolge der Faeden und nicht in der
    // Reihenfolge, in der sie fertig werden.
    let mut ergebnisse = Vec::new();
    for faden in faeden {
        ergebnisse.push(faden.join().unwrap());
    }

    for (name, laenge) in &ergebnisse {
        println!("{name} {laenge}");
    }
    println!("{}", ergebnisse.len());
}
```

Das Programm gibt aus:

```text
Ada 3
Grace 5
Alan 4
3
```

Diese Ausgabe steht fest, und dreimal hintereinander gestartet war sie dreimal
dieselbe. Das liegt nicht daran, dass die Fäden in dieser Reihenfolge fertig
werden, sondern daran, dass in der Reihenfolge der Fäden abgeholt wird.

Wie wenig fest die andere Reihenfolge ist, zeigt ein zweites Programm. Es
schreibt aus dem Faden heraus statt am Ende.

```rust
use std::thread;

fn main() {
    let mut faeden = Vec::new();
    for nummer in 1..=4 {
        faeden.push(thread::spawn(move || {
            println!("{nummer}");
        }));
    }
    for faden in faeden {
        faden.join().unwrap();
    }
}
```

Achtmal hintereinander gestartet, jede Zeile ein Lauf:

```console
1 2 3 4
1 3 2 4
1 3 2 4
1 2 3 4
1 2 4 3
1 2 4 3
1 3 2 4
1 2 3 4
```

Dasselbe Programm, dieselbe Maschine, vier verschiedene Ausgaben. Wer aus der
ersten Zeile schließt, die Fäden liefen der Reihe nach, hat das Programm einmal
zu wenig gestartet.

### Häufige Fehler

Das `move` weglassen und einen Wert aus der Funktion ausleihen wollen.

```rust
use std::thread;

fn main() {
    let name = String::from("Ada");

    let faden = thread::spawn(|| {
        println!("{name}");
    });

    faden.join().unwrap();
}
```

Der Übersetzer sagt dazu:

```text
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> ohne-move.rs:6:31
  |
6 |     let faden = thread::spawn(|| {
  |                               ^^ may outlive borrowed value `name`
7 |         println!("{name}");
  |                    ---- `name` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> ohne-move.rs:6:17
  |
6 |       let faden = thread::spawn(|| {
  |  _________________^
7 | |         println!("{name}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
6 |     let faden = thread::spawn(move || {
  |                               ++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0373`.
```

Die Notiz sagt den Grund genauer als der erste Satz. `spawn` verlangt, dass die
Closure `'static` überlebt, also alles mitbringt, was sie braucht. Dass in
diesem Beispiel unten `join` steht und der Faden gar nicht länger leben kann,
ändert nichts: geprüft wird der Typ und nicht, was das Programm hinterher tut.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `in_einem_faden` steht fertig da, und seine beiden Doku-Tests
sind grün.

- `summe_in_faeden` zählt zusammen, eine Zahl je Faden
- `quadrate_in_faeden` behält die Reihenfolge der Liste
- `zeichen_in_faeden` schiebt einen `String` in den Faden und nicht eine Zahl

```console
cd units/07-05-threads
cargo test
```

### Quelle

    Buch, Kapitel 16 "Fearless Concurrency",
    Abschnitt 16.1 "Using Threads to Run Code Simultaneously",
    https://doc.rust-lang.org/book/ch16-01-threads.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A thread is a second course of events inside the same program. `thread::spawn`
takes a closure and starts it alongside; the call comes back at once, and what
stands in the closure runs next to the rest from then on.

What comes back is a handle, a `JoinHandle`. `join` on it waits until the thread
is done and hands out what the closure returned. Without `join` nobody knows
whether the thread ever finished.

In front of the closure there mostly stands `move`. A thread can live longer
than the function that started it, so it may borrow nothing from it. `move`
pushes the captured values into the closure, and afterwards they belong to it.

### What it is good for

Computing side by side means the work falls onto several cores instead of
queueing up. Four pieces of work in four threads together take about as long as
the longest of them and not as long as all four together.

The price is the ordering. In which order the threads run is promised by nobody,
and whoever relies on one has a fault that does not turn up while trying it out
and does turn up later.

The pattern against it is simple and stands in this unit three times: start all
the threads first, then pick them up in the order of the threads. The results
lie in the order of the list afterwards, although the threads finished in any
order.

### The explanation

A program spreading three names over three threads and collecting the results in
the order of the threads.

```rust
use std::thread;

fn main() {
    let namen = vec![
        String::from("Ada"),
        String::from("Grace"),
        String::from("Alan"),
    ];

    // Deutsch: Jeder Name geht in seinen eigenen Faden. Ohne `move` ginge das
    // nicht, denn der Faden kann laenger leben als diese Funktion.
    let mut faeden = Vec::new();
    for name in namen {
        faeden.push(thread::spawn(move || {
            let laenge = name.len();
            (name, laenge)
        }));
    }

    // Deutsch: Eingesammelt wird in der Reihenfolge der Faeden und nicht in der
    // Reihenfolge, in der sie fertig werden.
    let mut ergebnisse = Vec::new();
    for faden in faeden {
        ergebnisse.push(faden.join().unwrap());
    }

    for (name, laenge) in &ergebnisse {
        println!("{name} {laenge}");
    }
    println!("{}", ergebnisse.len());
}
```

The program prints:

```text
Ada 3
Grace 5
Alan 4
3
```

This output is fixed, and started three times in a row it was the same three
times. That is not because the threads finish in this order but because the
picking up happens in the order of the threads.

How little fixed the other order is, a second program shows. It writes out of
the thread instead of at the end.

```rust
use std::thread;

fn main() {
    let mut faeden = Vec::new();
    for nummer in 1..=4 {
        faeden.push(thread::spawn(move || {
            println!("{nummer}");
        }));
    }
    for faden in faeden {
        faden.join().unwrap();
    }
}
```

Started eight times in a row, one run per line:

```console
1 2 3 4
1 3 2 4
1 3 2 4
1 2 3 4
1 2 4 3
1 2 4 3
1 3 2 4
1 2 3 4
```

The same program, the same machine, four different outputs. Whoever concludes
from the first line that the threads ran in order has started the program once
too few.

### Common mistakes

Leaving the `move` out and wanting to borrow a value from the function.

```rust
use std::thread;

fn main() {
    let name = String::from("Ada");

    let faden = thread::spawn(|| {
        println!("{name}");
    });

    faden.join().unwrap();
}
```

The compiler answers:

```text
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> ohne-move.rs:6:31
  |
6 |     let faden = thread::spawn(|| {
  |                               ^^ may outlive borrowed value `name`
7 |         println!("{name}");
  |                    ---- `name` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> ohne-move.rs:6:17
  |
6 |       let faden = thread::spawn(|| {
  |  _________________^
7 | |         println!("{name}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
6 |     let faden = thread::spawn(move || {
  |                               ++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0373`.
```

The note says the reason more precisely than the first sentence. `spawn`
requires the closure to outlive `'static`, meaning to bring along everything it
needs. That `join` stands below in this example and the thread cannot live
longer at all changes nothing: what is checked is the type and not what the
program does afterwards.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `in_einem_faden` stands there finished, and
both of its doc tests are green.

- `summe_in_faeden` adds up, one number per thread
- `quadrate_in_faeden` keeps the order of the list
- `zeichen_in_faeden` pushes a `String` into the thread and not a number

```console
cd units/07-05-threads
cargo test
```

### Source

    Book, chapter 16 "Fearless Concurrency",
    section 16.1 "Using Threads to Run Code Simultaneously",
    https://doc.rust-lang.org/book/ch16-01-threads.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
