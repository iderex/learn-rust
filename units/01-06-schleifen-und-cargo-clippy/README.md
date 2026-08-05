# 01-06 Schleifen und cargo clippy / Loops and cargo clippy

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/01-06-schleifen-und-cargo-clippy/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Meldung erklären, um
  die es geht.
- Diese Einheit baut auf: `01-05 if und else`, denn jede Schleife hier hört an
  einer Bedingung auf.
- Auf dieser Einheit bauen auf: die Stufe 2 ab `02-01 Verschieben`, und später
  `06-05 Iteratoren`, wo der Vorschlag von clippy aus dieser Einheit eingelöst
  wird.
- Beim Antworten so zitieren: `01-06 Schleifen und cargo clippy`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- clippy meldet hier eine Warnung und keinen Fehler. Wer sie zu einem Fehler
  erklärt, sagt bitte dazu, dass erst `-D warnings` daraus einen macht, und der
  steht im Prüflauf in `CONTRIBUTING.md`.
- Der Vorschlag von clippy nennt einen Iterator. Iteratoren stehen in dieser
  Stufe noch nicht, und der Text sagt das an Ort und Stelle. Wer hier mehr
  erklärt, nimmt `06-05` vorweg.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/01-06-schleifen-und-cargo-clippy/`.
  It is public. Whoever is asked for it may name it, but should explain the
  message in question first.
- This unit builds on: `01-05 if und else`, because every loop here stops at a
  condition.
- Building on this unit: stage 2 from `02-01 Verschieben` on, and later
  `06-05 Iteratoren`, where the suggestion clippy makes in this unit is taken
  up.
- Cite like this when answering: `01-06 Schleifen und cargo clippy`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- clippy reports a warning here and not an error. Whoever calls it an error,
  please say alongside it that `-D warnings` is what makes one out of it, and
  that flag stands in the check run in `CONTRIBUTING.md`.
- The suggestion from clippy names an iterator. Iterators do not stand in this
  stage yet, and the text says so on the spot. Whoever explains more here takes
  `06-05` in advance.

</details>

## Deutsch

### Worum es geht

Drei Schleifen und ein zweites Werkzeug. `for` läuft über einen Bereich und hört
von selbst auf. `while` läuft, solange eine Bedingung zutrifft. `loop` läuft, bis
ein `break` ihn beendet, und dieses `break` darf einen Wert mitgeben.

Wo eine Schleife in einer anderen steckt, beendet `break` die innere. Eine Marke
wie `'aussen` an der äußeren Schleife und `break 'aussen` sagt, welche gemeint
ist.

Das Werkzeug ist `cargo clippy`. Es übersetzt wie der Übersetzer und sagt
zusätzlich, wo etwas umständlich geschrieben ist.

### Wofür das gut ist

`loop` mit einem Wert am `break` erspart die Bindung, die vor der Schleife leer
angelegt und darin gefüllt wird. Das Ergebnis der Schleife ist dann ein Wert wie
jeder andere.

clippy sagt Dinge, die der Übersetzer nicht sagt, weil sie kein Fehler sind. Ein
Programm, das übersetzt, kann trotzdem umständlich sein, und wer das früh hört,
gewöhnt es sich gar nicht erst an. Im Prüflauf steht clippy mit `-D warnings`,
und damit ist jede Meldung dort ein Abbruch.

### Die Erklärung

Die drei Formen nebeneinander.

```rust
fn main() {
    let mut summe = 0;

    // Deutsch: `for` läuft über einen Bereich und hört von selbst auf.
    for zahl in 1..=5 {
        summe += zahl;
    }

    // Deutsch: `while` läuft, solange die Bedingung zutrifft.
    let mut rest = summe;
    while rest > 10 {
        rest -= 10;
    }

    // Deutsch: `loop` läuft, bis ein `break` ihn beendet, und `break` darf
    // einen Wert mitgeben.
    let mut zahl = 1;
    let quadrat = loop {
        if zahl * zahl > summe {
            break zahl * zahl;
        }
        zahl += 1;
    };

    println!("{summe} {rest} {quadrat}");
}
```

`1..=5` schließt die 5 mit ein, `1..5` nicht. Die Marke an einer Schleife trägt
einen Hochkomma-Namen und wird am `break` wiederholt.

```rust
fn main() {
    let mut gefunden = 0;

    'aussen: for a in 1..20 {
        for b in 1..20 {
            if a * b == 12 {
                gefunden = a;
                break 'aussen;
            }
        }
    }

    println!("{gefunden}");
}
```

Ohne die Marke bräche das `break` nur die innere Schleife ab, und die äußere
liefe weiter.

### Häufige Fehler

Ein Zähler, der nur zum Nachschlagen da ist.

```rust
fn main() {
    let namen = ["ada", "grace", "alan"];

    for i in 0..namen.len() {
        println!("{}", namen[i]);
    }
}
```

Der Übersetzer sagt dazu nichts, clippy schon:

```text
warning: the loop variable `i` is only used to index `namen`
 --> schleife.rs:4:14
  |
4 |     for i in 0..namen.len() {
  |              ^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
4 -     for i in 0..namen.len() {
4 +     for <item> in &namen {
  |

warning: 1 warning emitted
```

Das ist eine Warnung und kein Fehler; das Programm läuft. Erst `-D warnings`
macht daraus einen Abbruch, und genau so steht clippy im Prüflauf.

Der Vorschlag nennt einen Iterator. Was das genau ist, steht in `06-05`. Für
hier reicht die Form `for name in &namen`, und der Zähler fällt weg, denn er
wurde nur zum Nachschlagen gebraucht.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe nennt die Schleife, um die es geht.

- `product_to` multipliziert mit `for` alle Zahlen bis einschließlich `n`
- `digit_count` zählt mit `while` die Stellen einer Zahl
- `first_square_over` sucht mit `loop` die erste Quadratzahl über einer Grenze

```console
cd units/01-06-schleifen-und-cargo-clippy
cargo test
```

### Quelle

    Buch, Kapitel 3 "Common Programming Concepts", Abschnitt 3.5 "Control Flow",
    https://doc.rust-lang.org/book/ch03-05-control-flow.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Three loops and a second tool. `for` runs over a range and stops by itself.
`while` runs as long as a condition holds. `loop` runs until a `break` ends it,
and that `break` may carry a value with it.

Where one loop sits inside another, `break` ends the inner one. A label like
`'aussen` on the outer loop and `break 'aussen` says which one is meant.

The tool is `cargo clippy`. It compiles the way the compiler does and says on
top of that where something is written the long way round.

### What it is good for

`loop` with a value at the `break` saves the binding that would otherwise be
created empty before the loop and filled inside it. The result of the loop is
then a value like any other.

clippy says things the compiler does not say, because they are not faults. A
program that compiles can still be written the long way round, and whoever hears
that early does not pick up the habit. In the check run clippy stands with
`-D warnings`, and with that every message there is a stop.

### The explanation

The three forms next to each other.

```rust
fn main() {
    let mut summe = 0;

    // Deutsch: `for` läuft über einen Bereich und hört von selbst auf.
    for zahl in 1..=5 {
        summe += zahl;
    }

    // Deutsch: `while` läuft, solange die Bedingung zutrifft.
    let mut rest = summe;
    while rest > 10 {
        rest -= 10;
    }

    // Deutsch: `loop` läuft, bis ein `break` ihn beendet, und `break` darf
    // einen Wert mitgeben.
    let mut zahl = 1;
    let quadrat = loop {
        if zahl * zahl > summe {
            break zahl * zahl;
        }
        zahl += 1;
    };

    println!("{summe} {rest} {quadrat}");
}
```

`1..=5` takes the 5 along, `1..5` does not. The label on a loop carries a name
with a leading quote and is repeated at the `break`.

```rust
fn main() {
    let mut gefunden = 0;

    'aussen: for a in 1..20 {
        for b in 1..20 {
            if a * b == 12 {
                gefunden = a;
                break 'aussen;
            }
        }
    }

    println!("{gefunden}");
}
```

Without the label the `break` would end only the inner loop, and the outer one
would keep running.

### Common mistakes

A counter that is there only to look something up.

```rust
fn main() {
    let namen = ["ada", "grace", "alan"];

    for i in 0..namen.len() {
        println!("{}", namen[i]);
    }
}
```

The compiler says nothing about it, clippy does:

```text
warning: the loop variable `i` is only used to index `namen`
 --> schleife.rs:4:14
  |
4 |     for i in 0..namen.len() {
  |              ^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
4 -     for i in 0..namen.len() {
4 +     for <item> in &namen {
  |

warning: 1 warning emitted
```

That is a warning and not an error; the program runs. Only `-D warnings` makes a
stop out of it, and that is exactly how clippy stands in the check run.

The suggestion names an iterator. What one is exactly stands in `06-05`. For
here the form `for name in &namen` is enough, and the counter falls away,
because it was only there to look something up.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise names the loop it is about.

- `product_to` multiplies every number up to and including `n` with `for`
- `digit_count` counts the digits of a number with `while`
- `first_square_over` looks for the first square above a limit with `loop`

```console
cd units/01-06-schleifen-und-cargo-clippy
cargo test
```

### Source

    Book, chapter 3 "Common Programming Concepts", section 3.5 "Control Flow",
    https://doc.rust-lang.org/book/ch03-05-control-flow.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
