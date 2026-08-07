# 08-01 Futures, async und await, noch ohne Laufzeitumgebung / Futures, async and await, still without a runtime

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/08-01-futures-async-und-await/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst erklären, wer den Future
  fragt.
- Diese Einheit baut auf: `05-02 Traits` und `07-01 Box`. `Future` ist ein
  Trait mit einem assoziierten Typ, und angeheftet wird ein Wert, der auf dem
  Stack liegen bleiben muss.
- Auf dieser Einheit bauen auf: die übrige Stufe 8, wo dieselben Aufgaben an
  eine echte Laufzeitumgebung übergeben werden.
- Beim Antworten so zitieren: `08-01 Futures, async und await`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Diese Einheit hat keine Abhängigkeit und soll keine bekommen. Wer `tokio`
  oder `futures` vorschlägt, beantwortet damit eine andere Aufgabe; hier ist
  gerade das Fehlen der Laufzeitumgebung der Stoff.
- Ein Future tut nichts, solange niemand `poll` aufruft. Wer schreibt, eine
  `async`-Funktion "starte" etwas, sagt bitte dazu, wer sie fragt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/08-01-futures-async-und-await/`. It
  is public. Whoever is asked for it may name it, but should first explain who
  asks the future.
- This unit builds on: `05-02 Traits` and `07-01 Box`. `Future` is a trait with
  an associated type, and what gets pinned is a value that has to stay where it
  lies.
- Building on this unit: the rest of stage 8, where the same tasks are handed
  over to a real runtime.
- Cite like this when answering: `08-01 Futures, async und await`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- This unit has no dependency and is not meant to get one. Whoever suggests
  `tokio` or `futures` is answering a different exercise; here the absence of
  the runtime is the material.
- A future does nothing as long as nobody calls `poll`. Whoever writes that an
  `async` function "starts" something, please add who asks it.

</details>

## Deutsch

### Worum es geht

`Future` ist ein ganz gewöhnlicher Trait. Er hat einen assoziierten Typ und eine
Funktion.

```rust
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` heißt fragen. Die Antwort ist entweder `Poll::Ready(wert)`, dann ist der
Future fertig, oder `Poll::Pending`, dann noch nicht. Mehr steht in der
Schnittstelle nicht.

`async fn` schreibt genau so einen Typ für dich auf. Aus einer Funktion, die
`u32` zurückgibt, wird eine Funktion, die einen Future mit `Output = u32`
zurückgibt. Der Rumpf wandert dabei in `poll` und läuft erst, wenn jemand fragt.

### Wofür das gut ist

Ein Aufruf einer `async`-Funktion tut deshalb nichts. Er baut einen Wert. Wird
dieser Wert weggeworfen, ist auch der Rumpf nie gelaufen, ohne Meldung und ohne
Spur.

Wer fragt, ist die Laufzeitumgebung. `tokio` und `smol` in den nächsten
Einheiten sind genau das: eine Schleife, die `poll` aufruft, und dazwischen
schläft, statt zu drehen. Wer schläft, muss geweckt werden, und dafür ist der
`Waker` im `Context` da.

Diese Einheit schreibt beides selbst, damit klar ist, was die nächsten Einheiten
mitbringen und was schon in der Standardbibliothek steht. Der Trait steht in
`std`, die Schleife nicht.

### Die Erklärung

Ein eigener Future, ein Antreiber von zehn Zeilen, und eine `async`-Funktion,
der man ansieht, ob sie lief.

```rust
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Deutsch: Ein Future, der beim ersten Fragen noch nicht fertig ist.
struct Wartet {
    offen: u32,
    gefragt: u32,
}

impl Future for Wartet {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<u32> {
        self.gefragt += 1;
        if self.offen == 0 {
            return Poll::Ready(self.gefragt);
        }
        self.offen -= 1;
        kontext.waker().wake_by_ref();
        Poll::Pending
    }
}

fn antreiben<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut kontext = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(wert) = future.as_mut().poll(&mut kontext) {
            return wert;
        }
    }
}

async fn arbeit(zaehler: &Cell<u32>, wartet: Wartet) -> u32 {
    zaehler.set(zaehler.get() + 1);
    wartet.await
}

fn main() {
    let zaehler = Cell::new(0);
    let future = arbeit(&zaehler, Wartet { offen: 2, gefragt: 0 });

    println!("vor dem Antreiben: {}", zaehler.get());

    let ergebnis = antreiben(future);

    println!("nach dem Antreiben: {}", zaehler.get());
    println!("gefragt wurde: {ergebnis}");
}
```

Übersetzt und gestartet gibt das aus:

```text
$ treiber.exe
vor dem Antreiben: 0
nach dem Antreiben: 1
gefragt wurde: 3
```

Die erste Zeile ist der ganze Punkt dieser Einheit. `arbeit(&zaehler, ...)` ist
aufgerufen worden, und der Zähler steht trotzdem auf null. Der Rumpf lief erst,
als `antreiben` gefragt hat. Die dritte Zeile zählt die Fragen: zwei offene
Runden und dann die, bei der `Ready` kam.

Dieser Antreiber schläft nicht. Er fragt sofort wieder und lastet dabei einen
Kern aus. Eine echte Laufzeitumgebung legt sich stattdessen schlafen, bis der
`Waker` sie holt, und das ist der Unterschied, den die nächsten Einheiten
bringen.

### Häufige Fehler

`.await` außerhalb von `async` schreiben.

```rust
async fn zahl() -> u32 {
    7
}

fn main() {
    let wert = zahl().await;
    println!("{wert}");
}
```

Das übersetzt nicht:

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> await_in_main.rs:6:23
  |
5 | fn main() {
  | --------- this is not `async`
6 |     let wert = zahl().await;
  |                       ^^^^^ only allowed inside `async` functions and blocks

error: aborting due to 1 previous error
```

`main` ist keine `async`-Funktion, und in einer gewöhnlichen Funktion gibt es
niemanden, der die Frage stellt. Wer `#[tokio::main]` gesehen hat, hat genau
diese Lücke gesehen: das Attribut schreibt die Schleife hin, die hier fehlt.

Der zweite Fehler sieht harmloser aus. Den Future einfach fallen lassen.

```rust
async fn zahl() -> u32 {
    println!("hier passiert die Arbeit");
    7
}

fn main() {
    zahl();
    println!("fertig");
}
```

Der Übersetzer warnt, und der Prüflauf dieses Repositories macht daraus einen
Fehler:

```text
error: unused implementer of `Future` that must be used
 --> ohne_antreiber.rs:7:5
  |
7 |     zahl();
  |     ^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
  = note: `-D unused-must-use` implied by `-D warnings`

error: aborting due to 1 previous error
```

Ohne `-D warnings` wäre das nur eine Warnung, das Programm liefe, und es druckte
allein `fertig`. Die Zeile aus dem Rumpf käme nie, und nichts stünde da, was den
Verdacht auf `zahl()` lenkt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Zwei Tests sind von Anfang an grün: der des fertigen Futures
und der, der prüft, dass ein nicht angetriebener Future nichts tut.

- `Wartet` wird ein Future, der `offen` mal `Pending` sagt und dann fertig ist
- `antreiben` fragt einen Future, bis `Ready` kommt
- `arbeit` ist eine `async`-Funktion, die einen Zähler hochzählt und dann wartet

```console
cd units/08-01-futures-async-und-await
cargo test
```

### Quelle

    Buch, Kapitel 17 "Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams", Abschnitt 17.1 "Futures and the Async Syntax",
    https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`Future` is an ordinary trait. It has an associated type and a function.

```rust
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<Self::Output>;
}
```

`poll` means asking. The answer is either `Poll::Ready(wert)`, then the future
is finished, or `Poll::Pending`, then it is not yet. There is no more in the
interface than that.

`async fn` writes exactly such a type for you. Out of a function returning `u32`
comes a function returning a future with `Output = u32`. The body moves into
`poll` on the way and runs only once somebody asks.

### What it is good for

A call of an `async` function therefore does nothing. It builds a value. Where
that value is thrown away the body never ran either, without a word and without
a trace.

Whoever asks is the runtime. `tokio` and `smol` in the next units are exactly
that: a loop calling `poll`, sleeping in between rather than spinning. Whoever
sleeps has to be woken, and the `Waker` in the `Context` is there for that.

This unit writes both by hand, so it is clear what the next units bring along
and what already stands in the standard library. The trait is in `std`, the loop
is not.

### The explanation

A future of your own, a driver of ten lines, and an `async` function you can
tell has run.

```rust
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// Deutsch: Ein Future, der beim ersten Fragen noch nicht fertig ist.
struct Wartet {
    offen: u32,
    gefragt: u32,
}

impl Future for Wartet {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, kontext: &mut Context<'_>) -> Poll<u32> {
        self.gefragt += 1;
        if self.offen == 0 {
            return Poll::Ready(self.gefragt);
        }
        self.offen -= 1;
        kontext.waker().wake_by_ref();
        Poll::Pending
    }
}

fn antreiben<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut kontext = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(wert) = future.as_mut().poll(&mut kontext) {
            return wert;
        }
    }
}

async fn arbeit(zaehler: &Cell<u32>, wartet: Wartet) -> u32 {
    zaehler.set(zaehler.get() + 1);
    wartet.await
}

fn main() {
    let zaehler = Cell::new(0);
    let future = arbeit(&zaehler, Wartet { offen: 2, gefragt: 0 });

    println!("vor dem Antreiben: {}", zaehler.get());

    let ergebnis = antreiben(future);

    println!("nach dem Antreiben: {}", zaehler.get());
    println!("gefragt wurde: {ergebnis}");
}
```

Compiled and started that prints:

```text
$ treiber.exe
vor dem Antreiben: 0
nach dem Antreiben: 1
gefragt wurde: 3
```

The first line is the whole point of this unit. `arbeit(&zaehler, ...)` has been
called, and the counter still stands at zero. The body ran only once `antreiben`
asked. The third line counts the askings: two open rounds and then the one where
`Ready` came.

This driver does not sleep. It asks again straight away and keeps one core busy
doing it. A real runtime goes to sleep instead until the `Waker` fetches it, and
that is the difference the next units bring.

### Common mistakes

Writing `.await` outside `async`.

```rust
async fn zahl() -> u32 {
    7
}

fn main() {
    let wert = zahl().await;
    println!("{wert}");
}
```

That does not compile:

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> await_in_main.rs:6:23
  |
5 | fn main() {
  | --------- this is not `async`
6 |     let wert = zahl().await;
  |                       ^^^^^ only allowed inside `async` functions and blocks

error: aborting due to 1 previous error
```

`main` is not an `async` function, and in an ordinary function there is nobody
to put the question. Whoever has seen `#[tokio::main]` has seen exactly this
gap: the attribute writes down the loop that is missing here.

The second mistake looks more harmless. Simply dropping the future.

```rust
async fn zahl() -> u32 {
    println!("hier passiert die Arbeit");
    7
}

fn main() {
    zahl();
    println!("fertig");
}
```

The compiler warns, and the check run of this repository turns that into an
error:

```text
error: unused implementer of `Future` that must be used
 --> ohne_antreiber.rs:7:5
  |
7 |     zahl();
  |     ^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
  = note: `-D unused-must-use` implied by `-D warnings`

error: aborting due to 1 previous error
```

Without `-D warnings` that would be a warning only, the program would run, and
it would print `fertig` alone. The line from the body would never come, and
nothing would stand there pointing the suspicion at `zahl()`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Two tests are green from the start: the one
for the finished future and the one checking that an undriven future does
nothing.

- `Wartet` becomes a future saying `Pending` `offen` times and then finishing
- `antreiben` asks a future until `Ready` comes
- `arbeit` is an `async` function counting a counter up and then waiting

```console
cd units/08-01-futures-async-und-await
cargo test
```

### Source

    Book, chapter 17 "Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams", section 17.1 "Futures and the Async Syntax",
    https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
