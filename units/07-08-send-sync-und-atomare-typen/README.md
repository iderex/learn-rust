# 07-08 Send, Sync und die atomaren Typen / Send, Sync and the atomic types

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/07-08-send-sync-und-atomare-typen/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `07-02 Rc` und `07-06 Kanäle`. `Arc` ist das `Rc` für
  mehrere Fäden, und die Fäden kommen aus der Einheit davor.
- Auf dieser Einheit bauen auf: alles, was einen Wert zwischen Fäden teilt,
  statt ihn zu verschicken.
- Beim Antworten so zitieren: `07-08 Send, Sync und die atomaren Typen`, dazu
  die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `fetch_add` gibt den Wert von vorher zurück und nicht den neuen. Wer das
  Gegenteil behauptet, sagt bitte, an welchem Doku-Test.
- Ob ein Rumpf `compare_exchange` benutzt oder `load` und danach `store`, sagen
  die Tests dieser Einheit nicht. Das steht unter "Was diese Tests nicht
  beantworten" mit der Zahl, die dabei herauskam, und diese Aussage bleibt
  negativ.
- `Ordering` ist in dieser Einheit immer `Relaxed`, und das ist eine
  Entscheidung und keine Vollständigkeit. Was die anderen Ordnungen zusagen,
  steht hier nicht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/07-08-send-sync-und-atomare-typen/`. It is public. Whoever is asked
  for it may name it, but should explain the compiler message in question first.
- This unit builds on: `07-02 Rc` and `07-06 Kanäle`. `Arc` is the `Rc` for
  several threads, and the threads come from the unit before.
- Building on this unit: everything that shares a value between threads instead
  of sending it.
- Cite like this when answering: `07-08 Send, Sync und die atomaren Typen`, plus
  the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `fetch_add` returns the value from before and not the new one. Whoever claims
  the opposite, please say on which doc test.
- Whether a body uses `compare_exchange` or `load` followed by `store` is not
  answered by the tests of this unit. That stands under "What these tests do not
  answer" with the number that came out of it, and that statement stays
  negative.
- `Ordering` is always `Relaxed` in this unit, and that is a decision and not
  completeness. What the other orderings promise does not stand here.

</details>

## Deutsch

### Worum es geht

`Send` heißt: Dieser Wert darf in einen anderen Faden bewegt werden. `Sync`
heißt: Auf diesen Wert dürfen mehrere Fäden gleichzeitig durch eine gemeinsame
Referenz zugreifen.

Geschrieben werden beide fast nie. Der Übersetzer setzt sie selbst, sobald alle
Teile eines Typs sie tragen, und bemerkt werden sie deshalb erst an der Stelle,
an der einer fehlt. `Rc` ist so eine Stelle: Sein Zähler ist nicht atomar, also
ist `Rc` nicht `Send`.

`Arc` ist dasselbe mit einem atomaren Zähler. Und die Typen aus
`std::sync::atomic`, etwa `AtomicUsize`, sind Werte, die sich durch eine
gemeinsame Referenz verändern lassen, weil jede Änderung ein einziger Schritt
ist.

### Wofür das gut ist

Ein Zähler, den mehrere Fäden hochzählen, ist die kleinste Aufgabe, an der
gewöhnlicher Code scheitert. Lesen, eins dazurechnen und zurückschreiben sind
drei Schritte, und zwischen ihnen kann ein anderer Faden dasselbe tun. Am Ende
fehlen Zahlen, und niemand sieht, wo.

`fetch_add` ist ein Schritt. Zwischen Lesen und Schreiben kommt nichts, weil es
kein Dazwischen gibt. Dasselbe gilt für `fetch_max` und für `compare_exchange`,
und dafür braucht es kein `mut` und keine Sperre.

Der eigentliche Gewinn liegt aber beim Übersetzer. Ohne `Send` und `Sync` wäre
"diesen Wert darf man nicht teilen" eine Sache der Aufmerksamkeit. Mit ihnen ist
es eine Sache, die der Bau zurückweist, und die nächste Überschrift zeigt, wie
das aussieht.

### Die Erklärung

Vier Fäden zählen denselben Zähler hoch, und danach steht da, was dastehen soll.

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // Deutsch: `Arc` ist das `Rc` für mehrere Fäden. Der Zähler darin darf auch
    // durch eine gemeinsame Referenz verändert werden, denn er ist atomar.
    let zaehler = Arc::new(AtomicUsize::new(0));

    let mut faeden = Vec::new();
    for _ in 0..4 {
        let meiner = Arc::clone(&zaehler);
        faeden.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Deutsch: `fetch_add` gibt den Wert von vorher zurück, nicht
                // den neuen.
                meiner.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for faden in faeden {
        faden.join().expect("der Faden ist durchgelaufen");
    }

    println!("{}", zaehler.load(Ordering::Relaxed));

    let einzeln = AtomicUsize::new(7);
    println!("{}", einzeln.fetch_add(1, Ordering::Relaxed));
    println!("{}", einzeln.load(Ordering::Relaxed));
}
```

Das Programm gibt aus:

```text
4000
7
8
```

Die 4000 ist der Punkt. Mit einem gewöhnlichen `usize` stünde dort eine kleinere
Zahl, und zwar bei jedem Lauf eine andere.

Die 7 und die 8 darunter sind derselbe Zähler nach einem einzigen `fetch_add`.
Zurück kam 7, der Wert von vorher, und drinsteht 8. Wer 8 erwartet hat, hat
einen Fehler eingebaut, den kein Test dieser Einheit ihm abnimmt.

### Häufige Fehler

Ein `Rc` in einen Faden schieben.

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let geteilt = Rc::new(1);

    let faden = thread::spawn(move || {
        println!("{geteilt}");
    });

    faden.join().expect("der Faden ist durchgelaufen");
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `Rc<i32>` cannot be sent between threads safely
 --> rc-im-faden.rs:7:31
  |
7 |       let faden = thread::spawn(move || {
  |                   ------------- ^------
  |                   |             |
  |  _________________|_____________within this `{closure@rc-im-faden.rs:7:31: 7:38}`
  | |                 |
  | |                 required by a bound introduced by this call
8 | |         println!("{geteilt}");
9 | |     });
  | |_____^ `Rc<i32>` cannot be sent between threads safely
  |
  = help: within `{closure@rc-im-faden.rs:7:31: 7:38}`, the trait `Send` is not implemented for `Rc<i32>`
note: required because it's used within this closure
 --> rc-im-faden.rs:7:31
  |
7 |     let faden = thread::spawn(move || {
  |                               ^^^^^^^
note: required by a bound in `spawn`
 --> <std>/thread/functions.rs:125:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

An `<std>` stand der Pfad zur Standardbibliothek dieses Rechners, mit der
Prüfsumme des Übersetzers darin. Das ist die einzige Ersetzung, sonst steht die
Meldung so da, wie sie kam.

`Rc` ist nicht `Send`, und deshalb ist auch die Closure, die es festhält, nicht
`Send`. Der Weg ist `Arc` statt `Rc`. Das ist kein Umweg um eine Regel, sondern
der Typ, der den Zähler atomar führt und dafür ein wenig langsamer ist.

### Was diese Tests nicht beantworten

Aufgabe 3 verlangt `compare_exchange`. Ob ein Rumpf das benutzt oder stattdessen
`load` und danach `store`, sagt kein Test dieser Einheit.

Nachgemessen und nicht vermutet: Die Lösung wurde auf `load` und danach `store`
umgestellt, und die Testdatei lief danach zehnmal.

```console
$ gruen=0; rot=0
$ for i in $(seq 1 10); do
>   if cargo test -q -p unit-07-08-send-sync-und-atomare-typen --test exercise >/dev/null 2>&1
>   then gruen=$((gruen+1)); else rot=$((rot+1)); fi
> done
$ echo "gruen=$gruen rot=$rot"
gruen=10 rot=0
```

Der Grund liegt an der Sache und nicht an den Tests. Zwischen `load` und `store`
liegt ein Fenster von wenigen Anweisungen, und zwei Fäden müssen genau dort
aufeinandertreffen. Auf diesem Rechner starten sie nacheinander und sind vorbei,
bevor der nächste ankommt. Ein Wettlauf ist keine Eigenschaft, die ein Test
zurückweisen kann; er ist eine Eigenschaft, die ein Test manchmal sieht.

Daraus folgt nichts über die Richtigkeit der beiden Fassungen. `load` und danach
`store` ist falsch, auch wenn zehn Läufe grün sind, und `compare_exchange` ist
richtig, ohne dass ein Lauf das zeigt. Wer hier grün als Beleg nimmt, nimmt
einen Beleg für etwas anderes.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `bump` steht fertig da, und sein Doku-Test ist grün.

- `count_up` lässt mehrere Fäden denselben Zähler hochzählen
- `max_of` sucht das Größte, mit `fetch_max`
- `only_one_wins` lässt genau einen gewinnen, mit `compare_exchange`

```console
cd units/07-08-send-sync-und-atomare-typen
cargo test
```

### Quelle

    Buch, Kapitel 16 "Fearless Concurrency", Abschnitt 16.4
    "Extensible Concurrency with Send and Sync",
    https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`Send` means: this value may be moved into another thread. `Sync` means: several
threads may reach this value at the same time through a shared reference.

Both are almost never written. The compiler puts them in itself as soon as every
part of a type carries them, and they are therefore only noticed at the place
where one is missing. `Rc` is such a place: its counter is not atomic, so `Rc` is
not `Send`.

`Arc` is the same thing with an atomic counter. And the types out of
`std::sync::atomic`, for example `AtomicUsize`, are values that can be changed
through a shared reference, because every change is a single step.

### What it is good for

A counter that several threads count up is the smallest task ordinary code fails
at. Reading, adding one and writing back are three steps, and between them
another thread can do the same. At the end numbers are missing, and nobody sees
where.

`fetch_add` is one step. Nothing comes between reading and writing, because there
is no in between. The same holds for `fetch_max` and for `compare_exchange`, and
none of it needs a `mut` or a lock.

The actual gain, however, lies with the compiler. Without `Send` and `Sync`,
"this value may not be shared" would be a matter of attention. With them it is a
matter the build refuses, and the next heading shows what that looks like.

### The explanation

Four threads count the same counter up, and afterwards what should stand there
stands there.

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    // Deutsch: `Arc` ist das `Rc` für mehrere Fäden. Der Zähler darin darf auch
    // durch eine gemeinsame Referenz verändert werden, denn er ist atomar.
    let zaehler = Arc::new(AtomicUsize::new(0));

    let mut faeden = Vec::new();
    for _ in 0..4 {
        let meiner = Arc::clone(&zaehler);
        faeden.push(thread::spawn(move || {
            for _ in 0..1000 {
                // Deutsch: `fetch_add` gibt den Wert von vorher zurück, nicht
                // den neuen.
                meiner.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for faden in faeden {
        faden.join().expect("der Faden ist durchgelaufen");
    }

    println!("{}", zaehler.load(Ordering::Relaxed));

    let einzeln = AtomicUsize::new(7);
    println!("{}", einzeln.fetch_add(1, Ordering::Relaxed));
    println!("{}", einzeln.load(Ordering::Relaxed));
}
```

The program prints:

```text
4000
7
8
```

The 4000 is the point. With an ordinary `usize` a smaller number would stand
there, and a different one on every run.

The 7 and the 8 below it are the same counter after a single `fetch_add`. What
came back was 7, the value from before, and what stands inside is 8. Whoever
expected 8 has built in a mistake that no test of this unit takes off their
hands.

### Common mistakes

Pushing an `Rc` into a thread.

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let geteilt = Rc::new(1);

    let faden = thread::spawn(move || {
        println!("{geteilt}");
    });

    faden.join().expect("der Faden ist durchgelaufen");
}
```

The compiler answers:

```text
error[E0277]: `Rc<i32>` cannot be sent between threads safely
 --> rc-im-faden.rs:7:31
  |
7 |       let faden = thread::spawn(move || {
  |                   ------------- ^------
  |                   |             |
  |  _________________|_____________within this `{closure@rc-im-faden.rs:7:31: 7:38}`
  | |                 |
  | |                 required by a bound introduced by this call
8 | |         println!("{geteilt}");
9 | |     });
  | |_____^ `Rc<i32>` cannot be sent between threads safely
  |
  = help: within `{closure@rc-im-faden.rs:7:31: 7:38}`, the trait `Send` is not implemented for `Rc<i32>`
note: required because it's used within this closure
 --> rc-im-faden.rs:7:31
  |
7 |     let faden = thread::spawn(move || {
  |                               ^^^^^^^
note: required by a bound in `spawn`
 --> <std>/thread/functions.rs:125:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Where `<std>` stands, the path to the standard library of this machine stood,
with the checksum of the compiler inside it. That is the only substitution,
otherwise the message stands as it came.

`Rc` is not `Send`, and therefore the closure holding it is not `Send` either.
The way out is `Arc` instead of `Rc`. That is not a detour around a rule but the
type that keeps the counter atomic and is a little slower for it.

### What these tests do not answer

Exercise 3 asks for `compare_exchange`. Whether a body uses that or `load`
followed by `store` instead is not said by any test of this unit.

Measured rather than supposed: the solution was switched over to `load` followed
by `store`, and the test file ran ten times afterwards.

```console
$ gruen=0; rot=0
$ for i in $(seq 1 10); do
>   if cargo test -q -p unit-07-08-send-sync-und-atomare-typen --test exercise >/dev/null 2>&1
>   then gruen=$((gruen+1)); else rot=$((rot+1)); fi
> done
$ echo "gruen=$gruen rot=$rot"
gruen=10 rot=0
```

The reason lies with the matter and not with the tests. Between `load` and
`store` lies a window of a few instructions, and two threads have to meet exactly
there. On this machine they start one after another and are over before the next
one arrives. A race is not a property a test can refuse; it is a property a test
sometimes sees.

Nothing about the correctness of the two versions follows from that. `load`
followed by `store` is wrong even when ten runs are green, and `compare_exchange`
is right without any run showing it. Whoever takes green here as evidence takes
evidence for something else.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `bump` stands there finished, and its doc test
is green.

- `count_up` lets several threads count the same counter up
- `max_of` looks for the largest one, with `fetch_max`
- `only_one_wins` lets exactly one win, with `compare_exchange`

```console
cd units/07-08-send-sync-und-atomare-typen
cargo test
```

### Source

    Book, chapter 16 "Fearless Concurrency", section 16.4
    "Extensible Concurrency with Send and Sync",
    https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
