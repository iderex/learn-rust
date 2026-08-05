# 00-03 Das erste Projekt und eine Fehlermeldung lesen / The first project and reading an error message

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/00-03-erstes-projekt-und-fehlermeldung/`. Sie ist öffentlich. Wer
  nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht. In dieser Einheit ist das Erklären der Meldung der
  ganze Stoff, und die Lösung vorwegzunehmen nimmt ihn weg.
- Diese Einheit baut auf: `00-01 Was ein Programm ist und was ein Compiler tut`
  und `00-02 Kommandozeile, rustup und cargo`.
- Auf dieser Einheit bauen auf: alle Einheiten der Stufe 1 und danach, denn ab
  hier wird jede Fehlermeldung als lesbar vorausgesetzt.
- Beim Antworten so zitieren: `00-03 Das erste Projekt und eine Fehlermeldung
  lesen`, dazu die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die
  Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.

</details>

## Deutsch

### Worum es geht

Ein frisches Projekt von `cargo new` hat wenige Teile, und zwei Befehle reichen,
um damit zu arbeiten: `cargo run` startet das Programm, `cargo test` lässt seine
Tests laufen.

Der zweite Teil dieser Einheit ist die Antwort des Übersetzers, wenn etwas nicht
stimmt. Diese Antwort ist kein Textklumpen, sondern hat immer denselben Aufbau,
und wer den einmal kennt, liest jede weitere Meldung in Sekunden.

### Wofür das gut ist

Wer eine Meldung nicht liest, sondern nur sieht, dass sie rot ist, fängt an zu
raten. Raten kostet Stunden und lehrt nichts.

Die Meldung enthält fast immer schon die Antwort. Sie sagt, wo das Problem
steht, was sie an dieser Stelle erwartet hat, was sie stattdessen gefunden hat,
und oft schlägt sie die Änderung wörtlich vor. Das Lesen dieser vier Dinge ist
die wichtigste einzelne Fertigkeit der ersten Wochen.

### Die Erklärung

Ein Projekt aus `cargo new hallo` hat diesen Aufbau. Mehr ist am Anfang nicht
da.

```console
hallo/Cargo.toml
hallo/src/main.rs

cd hallo
cargo run
cargo test
```

`cargo run` übersetzt und startet in einem Schritt. `cargo test` übersetzt und
lässt die Tests laufen. Beide Befehle wollen im Projektordner abgeschickt
werden, sonst findet cargo die Datei `Cargo.toml` nicht.

Eine Meldung des Übersetzers hat vier Teile. Erstens die erste Zeile mit der Art
des Fehlers und seiner Nummer in eckigen Klammern. Zweitens die Zeile mit dem
Pfeil `-->`, die Datei, Zeile und Spalte nennt. Drittens der Ausschnitt aus dem
Quelltext mit Zeichen darunter, die auf die Stelle zeigen. Viertens ein Hinweis
am Ende, oft mit einem fertigen Vorschlag und immer mit dem Befehl, unter dem
sich die Nummer nachschlagen lässt.

### Häufige Fehler

Der Tippfehler in einem Namen. Hier heißt die Bindung `zahl` und benutzt wird
`zhal`.

```rust
fn main() {
    let zahl = 3;
    println!("{zhal}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0425]: cannot find value `zhal` in this scope
 --> tippfehler.rs:3:16
  |
3 |     println!("{zhal}");
  |                ^^^^
  |
help: a local variable with a similar name exists
  |
3 -     println!("{zhal}");
3 +     println!("{zahl}");
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Die vier Teile sind alle da. Die Nummer ist `E0425`. Die Stelle ist
`tippfehler.rs:3:16`, also Zeile 3, Spalte 16. Der Pfeil aus `^^^^` steht genau
unter `zhal`. Und der Hinweis am Ende ist diesmal so genau, dass er die Zeile
vorher und nachher zeigt: `-` ist, was dasteht, `+` ist, was dastehen sollte.

Wenn ein Hinweis so aussieht, ist er fast immer richtig. Wenn er fehlt, bleibt
die Nummer, und `rustc --explain E0425` schreibt eine ganze Seite dazu.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.

- `explain_url` gibt die Adresse der Seite zurück, auf der eine Fehlernummer
  erklärt wird
- `points_at` gibt Zeile und Spalte in der Schreibweise des Übersetzers zurück

```console
cd units/00-03-erstes-projekt-und-fehlermeldung
cargo test
```

### Quelle

    Buch, Kapitel 1 "Getting Started", Abschnitt 1.3 "Hello, Cargo!",
    https://doc.rust-lang.org/book/ch01-03-hello-cargo.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A fresh project from `cargo new` has few parts, and two commands are enough to
work with it: `cargo run` starts the program, `cargo test` runs its tests.

The second part of this unit is the compiler's answer when something is wrong.
That answer is not a lump of text but always has the same shape, and whoever
knows it once reads every further message in seconds.

### What it is good for

Whoever does not read a message but only sees that it is red starts guessing.
Guessing costs hours and teaches nothing.

The message nearly always holds the answer already. It says where the problem
is, what it expected at that place, what it found instead, and often it proposes
the change word for word. Reading those four things is the single most important
skill of the first weeks.

### The explanation

A project from `cargo new hallo` has this layout. There is no more than this at
the start.

```console
hallo/Cargo.toml
hallo/src/main.rs

cd hallo
cargo run
cargo test
```

`cargo run` compiles and starts in one step. `cargo test` compiles and runs the
tests. Both commands want to be sent inside the project folder, otherwise cargo
does not find the file `Cargo.toml`.

A message from the compiler has four parts. First the opening line with the kind
of error and its number in square brackets. Second the line with the arrow
`-->`, naming file, line and column. Third the excerpt from the source with
characters underneath pointing at the place. Fourth a note at the end, often
with a finished proposal and always with the command under which the number can
be looked up.

### Common mistakes

The typo in a name. Here the binding is called `zahl` and what is used is
`zhal`.

```rust
fn main() {
    let zahl = 3;
    println!("{zhal}");
}
```

The compiler answers:

```text
error[E0425]: cannot find value `zhal` in this scope
 --> tippfehler.rs:3:16
  |
3 |     println!("{zhal}");
  |                ^^^^
  |
help: a local variable with a similar name exists
  |
3 -     println!("{zhal}");
3 +     println!("{zahl}");
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

All four parts are there. The number is `E0425`. The place is
`tippfehler.rs:3:16`, meaning line 3, column 16. The arrow made of `^^^^` sits
exactly under `zhal`. And the note at the end is precise enough this time to
show the line before and after: `-` is what stands there, `+` is what should
stand there.

When a note looks like that it is nearly always right. When it is missing the
number remains, and `rustc --explain E0425` writes a whole page about it.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `explain_url` returns the address of the page explaining an error number
- `points_at` returns line and column in the compiler's spelling

```console
cd units/00-03-erstes-projekt-und-fehlermeldung
cargo test
```

### Source

    Book, chapter 1 "Getting Started", section 1.3 "Hello, Cargo!",
    https://doc.rust-lang.org/book/ch01-03-hello-cargo.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
