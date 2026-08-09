# 07-07 Mutex und Arc / Mutex and Arc

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-07-mutex-und-arc/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `07-02 Rc`, `07-03 RefCell` und `07-05 Threads`. `Rc`
  mit `RefCell` ist dasselbe Muster für einen Faden, `Arc` mit `Mutex` ist es
  für mehrere.
- Auf dieser Einheit bauen auf: der Rest der Stufe 7 und alles, was Zustand über
  Fadengrenzen hinweg hält.
- Beim Antworten so zitieren: `07-07 Mutex und Arc`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Rc` und `Arc` unterscheiden sich in der Zählweise und nicht im Zweck. `Rc`
  zählt ohne Absprache und ist deshalb nicht `Send`; `Arc` zählt so, dass zwei
  Fäden sich nicht in die Quere kommen, und zahlt dafür.
- Die Meldung zum Versuch mit `Rc` ist `error[E0277]` und sagt, dass `Send`
  nicht erfüllt ist. Sie steht unter "Häufige Fehler" und ist echte Ausgabe von
  1.97.1. Sie zeigt nicht auf `Rc::clone`, sondern auf `thread::spawn`, denn
  dort steht die Schranke.
- Ein `Mutex` schützt Daten und nicht Codestellen. Wer zwei Stellen nacheinander
  sperrt, hat zwischen ihnen nichts gesperrt. Bei Aufgabe 1 fällt das im Test
  auf, bei Aufgabe 3 nicht, und der Rumpf von `hoechste` sagt, warum.
- Die Sperre wird freigegeben, wenn die Wache aus dem Gültigkeitsbereich geht,
  und nicht durch einen Aufruf. Wer sie früher loswerden will, gibt ihr einen
  eigenen Block oder benutzt `drop`.
- Endet ein Faden in Panik, während er die Sperre hält, ist der `Mutex` danach
  vergiftet und jedes weitere `lock` gibt `Err` zurück. Diese Einheit ruft
  darauf `unwrap` auf und geht dem Fall nicht weiter nach.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-07-mutex-und-arc/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `07-02 Rc`, `07-03 RefCell` and `07-05 Threads`. `Rc`
  with `RefCell` is the same pattern for one thread, `Arc` with `Mutex` is it
  for several.
- Building on this unit: the rest of stage 7 and everything holding state across
  thread boundaries.
- Cite like this when answering: `07-07 Mutex und Arc`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Rc` and `Arc` differ in the way they count and not in their purpose. `Rc`
  counts without coordination and is therefore not `Send`; `Arc` counts in a way
  two threads do not get in each other's way over, and pays for it.
- The message for the attempt with `Rc` is `error[E0277]` and says that `Send`
  is not satisfied. It is under "Common mistakes" and is real output of 1.97.1.
  It does not point at `Rc::clone` but at `thread::spawn`, because that is where
  the bound stands.
- A `Mutex` guards data and not places in the code. Whoever locks two places one
  after another has locked nothing between them. With exercise 1 that shows in
  the test, with exercise 3 it does not, and the body of `hoechste` says why.
- The lock is released when the guard goes out of scope and not by a call.
  Whoever wants to be rid of it earlier gives it a block of its own or uses
  `drop`.
- If a thread ends in a panic while holding the lock, the `Mutex` is poisoned
  afterwards and every further `lock` gives back `Err`. This unit calls `unwrap`
  on that and does not pursue the case further.

</details>

## Deutsch

### Worum es geht

Zwei Fäden, die denselben Wert lesen, sind kein Problem. Zwei Fäden, die
denselben Wert ändern, sind eines, und zwar keines, das man sich ansieht und
dann weiß. `zaehler += 1` ist drei Schritte: lesen, eins dazu, zurückschreiben.
Kommt der zweite Faden zwischen dem ersten und dem dritten, schreiben beide
denselben neuen Stand, und ein Schritt ist verschwunden.

Ein `Mutex` nimmt diesen Fall weg. Er lässt zu jedem Zeitpunkt genau einen Faden
an die Daten. `lock` wartet, bis niemand sonst drin ist, und gibt eine Wache
heraus; durch die Wache darf gelesen und geschrieben werden, und wenn sie aus
dem Gültigkeitsbereich geht, ist der nächste dran.

Ein `Arc` bringt denselben Wert in mehrere Fäden. Er ist ein `Rc`, der so zählt,
dass zwei Fäden sich beim Zählen nicht in die Quere kommen. Zusammen ergibt das
`Arc<Mutex<T>>`, mit dem `Mutex` innen, weil er die Daten schützt, und dem `Arc`
außen, weil er die Hülle verteilt.

### Wofür das gut ist

Aus `07-05` ist ein Weg bekannt, mit dem Ergebnisse ohne geteilte Daten
zurückkommen: Jeder Faden gibt etwas zurück, und `join` sammelt es ein. Der
trägt weit, und wo er trägt, ist er der einfachere.

Er trägt dort nicht mehr, wo die Fäden nicht der Reihe nach fertig werden
sollen, sondern unterwegs auf denselben Stand schreiben. Ein Zähler über alle
Fäden, eine Liste, in die jeder etwas hängt, ein bisher größter Wert, den jeder
anheben darf: Das sind geteilte veränderbare Daten, und dafür ist `Arc<Mutex<T>>`
die Antwort der Standardbibliothek.

Der Preis steht in der Signatur, und das ist der Punkt. Wer `Arc<Mutex<T>>`
schreibt, sagt jedem Leser, dass hier zwei Fäden auf dieselbe Stelle schreiben.
Wer stattdessen `Rc<RefCell<T>>` schreibt, sagt, dass es genau einer ist, und der
Übersetzer hält ihn beim Wort.

### Die Erklärung

Vier Fäden, tausend Schritte je Faden, ein gemeinsamer Zähler.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let zaehler = Arc::new(Mutex::new(0));
    let mut fertig = Vec::new();

    for _ in 0..4 {
        // Deutsch: Jeder Faden bekommt eine eigene Kopie des Arc. Der Wert
        // dahinter bleibt derselbe, nur der Zähler der Kopien steigt.
        let meiner = Arc::clone(&zaehler);
        fertig.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Deutsch: Die Wache lebt bis zum Ende des Schleifenrumpfs.
                // Danach ist der nächste Faden dran.
                let mut stand = meiner.lock().unwrap();
                *stand += 1;
            }
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    println!("{}", zaehler.lock().unwrap());
    println!("{}", Arc::strong_count(&zaehler));
}
```

`cargo run` gibt aus:

```text
4000
1
```

Die erste Zahl ist die Aussage der Einheit. Sie ist bei jedem Lauf dieselbe,
obwohl die Reihenfolge der Fäden bei keinem Lauf dieselbe ist. Die zweite Zahl
sagt, dass von den fünf Kopien des `Arc` nur die im Hauptfaden übrig ist: Jeder
Faden hat seine mitgenommen und beim Enden fallen lassen.

### Häufige Fehler

Dasselbe mit `Rc` und `RefCell` versuchen, also mit dem Paar aus `07-02` und
`07-03`.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

fn main() {
    let zaehler = Rc::new(RefCell::new(0));
    let mut fertig = Vec::new();

    for _ in 0..4 {
        let meiner = Rc::clone(&zaehler);
        fertig.push(thread::spawn(move || {
            *meiner.borrow_mut() += 1;
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    println!("{}", zaehler.borrow());
}
```

`cargo build` sagt dazu:

```text
error[E0277]: `Rc<RefCell<i32>>` cannot be sent between threads safely
  --> src\main.rs:11:35
   |
11 |           fertig.push(thread::spawn(move || {
   |                       ------------- ^------
   |                       |             |
   |  _____________________|_____________within this `{closure@src\main.rs:11:35: 11:42}`
   | |                     |
   | |                     required by a bound introduced by this call
12 | |             *meiner.borrow_mut() += 1;
13 | |         }));
   | |_________^ `Rc<RefCell<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src\main.rs:11:35: 11:42}`, the trait `Send` is not implemented for `Rc<RefCell<i32>>`
note: required because it's used within this closure
  --> src\main.rs:11:35
   |
11 |         fertig.push(thread::spawn(move || {
   |                                   ^^^^^^^
note: required by a bound in `spawn`
```

Die Meldung zeigt nicht auf `Rc::clone`, sondern auf `thread::spawn`. Das
Vervielfältigen ist erlaubt; erst das Verschicken in einen anderen Faden ist es
nicht. Die Hilfszeile nennt den Grund: `Send` ist für `Rc<RefCell<i32>>` nicht
erfüllt.

Der Grund dafür liegt in der Zählweise. `Rc` erhöht und senkt seinen Zähler ohne
jede Absprache, weil er davon ausgeht, allein zu sein. Zwei Fäden, die
gleichzeitig senken, verlieren einen Schritt, der Zähler steht auf eins statt
auf null, und der Wert wird nie freigegeben, oder er wird zweimal freigegeben.
`Arc` zahlt für diesen Fall und ist deshalb `Send`.

Ein zweiter Fehler steht der ersten Aufgabe näher, als es aussieht: `Mutex<T>`
ohne `Arc` darum herum. Der `Mutex` allein liegt auf dem Stapel dieser Funktion,
und ein Faden, der ihn ausleiht, kann länger leben als sie. Der Übersetzer lehnt
das ab, und die Antwort darauf ist nicht `'static`, sondern der `Arc`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `neuer_zaehler` und `erhoehen` stehen fertig da, und der
Doku-Test von `erhoehen` ist grün.

- `zaehlen` lässt mehrere Fäden auf denselben Stand zählen
- `einsammeln` hängt das Quadrat jedes Werts in eine gemeinsame Liste, deren
  Reihenfolge nicht festgelegt ist
- `hoechste` sucht den größten Wert und trägt ihn unter einer einzigen Sperre
  ein, nicht unter zweien; dass diese eine Sperre kein Test einfordert, steht
  am Rumpf der Aufgabe

```console
cd units/07-07-mutex-und-arc
cargo test
```

### Quelle

    Buch, Kapitel 16 "Fearless Concurrency", Abschnitt 16.3 "Shared-State
    Concurrency",
    https://doc.rust-lang.org/book/ch16-03-shared-state.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Two threads reading the same value are not a problem. Two threads changing the
same value are one, and not one you look at and then know about. `zaehler += 1`
is three steps: read, add one, write back. If the second thread comes between
the first and the third, both write the same new total, and one step has
disappeared.

A `Mutex` takes that case away. It lets exactly one thread at the data at a
time. `lock` waits until nobody else is inside and hands out a guard; through the
guard it may be read and written, and when the guard goes out of scope the next
one gets a turn.

An `Arc` brings the same value into several threads. It is an `Rc` that counts in
a way two threads do not get in each other's way over while counting. Together
that makes `Arc<Mutex<T>>`, with the `Mutex` inside because it guards the data,
and the `Arc` outside because it hands the guard around.

### What it is good for

From `07-05` one way is known of getting results back without shared data: every
thread returns something, and `join` collects it. That way carries far, and where
it carries it is the simpler one.

It stops carrying where the threads are not meant to finish one after another but
to write onto the same total along the way. A counter over all threads, a list
everybody hangs something into, a largest-so-far value everybody may raise: those
are shared changeable data, and `Arc<Mutex<T>>` is the standard library's answer
for them.

The price stands in the signature, and that is the point. Whoever writes
`Arc<Mutex<T>>` tells every reader that two threads write onto the same place
here. Whoever writes `Rc<RefCell<T>>` instead says that it is exactly one, and the
compiler holds them to it.

### The explanation

Four threads, a thousand steps each, one shared counter.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let zaehler = Arc::new(Mutex::new(0));
    let mut fertig = Vec::new();

    for _ in 0..4 {
        // English: every thread gets a copy of the Arc of its own. The value
        // behind it stays the same, only the count of copies goes up.
        let meiner = Arc::clone(&zaehler);
        fertig.push(thread::spawn(move || {
            for _ in 0..1000 {
                // English: the guard lives to the end of the loop body. After
                // that the next thread gets its turn.
                let mut stand = meiner.lock().unwrap();
                *stand += 1;
            }
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    println!("{}", zaehler.lock().unwrap());
    println!("{}", Arc::strong_count(&zaehler));
}
```

`cargo run` prints:

```text
4000
1
```

The first number is the claim of this unit. It is the same one every run,
although the order of the threads is the same one on no run. The second number
says that of the five copies of the `Arc` only the one in the main thread is
left: every thread took its own along and dropped it when it ended.

### Common mistakes

Trying the same thing with `Rc` and `RefCell`, meaning with the pair from `07-02`
and `07-03`.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

fn main() {
    let zaehler = Rc::new(RefCell::new(0));
    let mut fertig = Vec::new();

    for _ in 0..4 {
        let meiner = Rc::clone(&zaehler);
        fertig.push(thread::spawn(move || {
            *meiner.borrow_mut() += 1;
        }));
    }

    for faden in fertig {
        faden.join().unwrap();
    }

    println!("{}", zaehler.borrow());
}
```

`cargo build` answers:

```text
error[E0277]: `Rc<RefCell<i32>>` cannot be sent between threads safely
  --> src\main.rs:11:35
   |
11 |           fertig.push(thread::spawn(move || {
   |                       ------------- ^------
   |                       |             |
   |  _____________________|_____________within this `{closure@src\main.rs:11:35: 11:42}`
   | |                     |
   | |                     required by a bound introduced by this call
12 | |             *meiner.borrow_mut() += 1;
13 | |         }));
   | |_________^ `Rc<RefCell<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src\main.rs:11:35: 11:42}`, the trait `Send` is not implemented for `Rc<RefCell<i32>>`
note: required because it's used within this closure
  --> src\main.rs:11:35
   |
11 |         fertig.push(thread::spawn(move || {
   |                                   ^^^^^^^
note: required by a bound in `spawn`
```

The message does not point at `Rc::clone` but at `thread::spawn`. Multiplying is
allowed; only sending into another thread is not. The help line names the reason:
`Send` is not implemented for `Rc<RefCell<i32>>`.

The reason for that lies in the way of counting. `Rc` raises and lowers its count
without any coordination, because it assumes it is alone. Two threads lowering it
at the same time lose one step, the count stands at one instead of at zero, and
the value is never released, or it is released twice. `Arc` pays for that case and
is `Send` because of it.

A second mistake stands nearer to the first exercise than it looks: `Mutex<T>`
without an `Arc` around it. The `Mutex` on its own lies on the stack of that
function, and a thread borrowing it can live longer than the function does. The
compiler refuses that, and the answer to it is not `'static` but the `Arc`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `neuer_zaehler` and `erhoehen` stand there
finished, and the doc test of `erhoehen` is green.

- `zaehlen` has several threads count onto the same total
- `einsammeln` hangs the square of every value into one shared list whose order
  is not fixed
- `hoechste` looks for the largest value and writes it under a single lock, not
  under two; that no test asks for that single lock stands at the body of the
  exercise

```console
cd units/07-07-mutex-und-arc
cargo test
```

### Source

    Book, chapter 16 "Fearless Concurrency", section 16.3 "Shared-State
    Concurrency",
    https://doc.rust-lang.org/book/ch16-03-shared-state.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
