# 00-01 Was ein Programm ist und was ein Compiler tut / What a program is and what a compiler does

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/00-01-programm-und-compiler/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: nichts. Sie ist die erste Einheit des ganzen Pfades
  und setzt kein Vorwissen voraus, auch keines aus einer anderen Sprache.
- Auf dieser Einheit bauen auf: die weiteren Einheiten der Stufe 0, also die
  Kommandozeile mit rustup und cargo und das erste eigene Projekt.
- Beim Antworten so zitieren: `00-01 Was ein Programm ist und was ein Compiler
  tut`, dazu die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die
  Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Wer hier antwortet, redet mit jemandem, der noch nie programmiert hat.
  Fachbegriffe werden eingeführt, bevor sie benutzt werden.

</details>

## Deutsch

### Worum es geht

Ein Programm ist zuerst nichts als Text in einer Datei. Der Rechner kann diesen
Text nicht ausführen, denn er versteht nur eine sehr viel einfachere Sprache aus
Zahlen. Zwischen beidem steht ein Programm, das den Text liest und daraus etwas
Ausführbares macht. Dieses Programm heißt Compiler, auf Deutsch Übersetzer, und
bei Rust heißt es `rustc`.

Der Übersetzer tut dabei zwei Dinge. Er übersetzt, und er prüft. Das Prüfen ist
der Teil, der Rust ausmacht.

### Wofür das gut ist

Es gibt zwei Zeitpunkte, an denen ein Fehler auffallen kann: während des
Übersetzens oder während das Programm läuft. Der erste kostet eine
Fehlermeldung, der zweite kostet einen Absturz bei jemandem, der das Programm
benutzt.

Rust verschiebt so viel wie möglich auf den ersten Zeitpunkt. Deshalb dauert das
Übersetzen länger als bei manchen anderen Sprachen, und deshalb weist der
Übersetzer Programme zurück, die auf den ersten Blick vernünftig aussehen. Wer
das für Schikane hält, hat den Tausch noch nicht gesehen: jede Meldung hier ist
ein Absturz, der nicht stattfindet.

### Die Erklärung

Der Weg vom Text zum laufenden Programm hat drei Stationen. Erstens der
Quelltext, den ein Mensch schreibt. Zweitens der Übersetzer, der ihn liest,
prüft und in Maschinencode umsetzt. Drittens die ausführbare Datei, die der
Rechner startet.

Der kleinste vollständige Quelltext in Rust sieht so aus. `fn main` ist die
Stelle, an der ein Programm beginnt, und `println!` schreibt eine Zeile auf den
Bildschirm.

```rust
fn main() {
    println!("Hallo, Welt!");
}
```

Steht das in einer Datei `hallo.rs`, dann macht `rustc hallo.rs` daraus eine
ausführbare Datei, und der Rechner kann sie starten. Für ein einzelnes Programm
ruft man `rustc` selbst auf. Für alles Weitere nimmt man `cargo`, und das ist
die nächste Einheit.

### Häufige Fehler

Der erste Fehler, den fast jeder macht, ist ein Wert vom falschen Typ. Hier soll
in `zahl` eine Zahl stehen, und es steht Text darin.

```rust
fn main() {
    let zahl: u32 = "zwei";
    println!("{zahl}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> mismatch.rs:2:21
  |
2 |     let zahl: u32 = "zwei";
  |               ---   ^^^^^^ expected `u32`, found `&str`
  |               |
  |               expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Diese Meldung ist der Normalfall und keine Katastrophe. Sie hat eine Nummer,
`E0308`, die man nachschlagen kann. Sie hat eine Zeile und eine Spalte,
`2:21`. Sie hat einen Pfeil unter genau der Stelle, um die es geht. Und sie
sagt, was sie erwartet hat und was sie gefunden hat: erwartet `u32`, gefunden
`&str`. Die richtige Antwort darauf ist, entweder eine Zahl hinzuschreiben oder
den Typ zu ändern.

Wichtig ist der Zeitpunkt: das Programm ist nie gelaufen. Der Fehler wurde
gefunden, bevor überhaupt etwas passiert ist.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot. Die Aufgaben sind absichtlich winzig, denn
hier geht es um den Weg vom Text zum laufenden Programm und noch nicht um die
Sprache.

- `greeting` gibt `Hallo, <name>!` zurück, mit dem übergebenen Namen darin
- `doubled` gibt das Doppelte von `n` zurück

```console
cd units/00-01-programm-und-compiler
cargo test
```

### Quelle

    Buch, Kapitel 1 "Getting Started", Abschnitt 1.2 "Hello, World!",
    https://doc.rust-lang.org/book/ch01-02-hello-world.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A program is nothing but text in a file first. The machine cannot run that text,
because it understands only a far simpler language made of numbers. Between the
two stands a program that reads the text and makes something runnable out of it.
That program is called a compiler, and in Rust it is called `rustc`.

The compiler does two things while it works. It translates, and it checks. The
checking is the part that makes Rust what it is.

### What it is good for

There are two moments at which a mistake can show: while compiling, or while the
program runs. The first costs an error message, the second costs a crash in
front of somebody using the program.

Rust moves as much as it can to the first moment. That is why compiling takes
longer than in some other languages, and why the compiler turns away programs
that look reasonable at first glance. Whoever takes that for harassment has not
yet seen the trade: every message here is a crash that does not happen.

### The explanation

The way from text to a running program has three stations. First the source
text, written by a person. Second the compiler, which reads it, checks it and
turns it into machine code. Third the executable file, which the machine starts.

The smallest complete source text in Rust looks like this. `fn main` is the
place where a program begins, and `println!` writes a line to the screen.

```rust
fn main() {
    println!("Hallo, Welt!");
}
```

With that in a file `hallo.rs`, `rustc hallo.rs` makes an executable file out of
it and the machine can start it. For a single program you call `rustc` yourself.
For anything beyond that you take `cargo`, and that is the next unit.

### Common mistakes

The first mistake nearly everybody makes is a value of the wrong type. Here
`zahl` is meant to hold a number, and it holds text.

```rust
fn main() {
    let zahl: u32 = "zwei";
    println!("{zahl}");
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> mismatch.rs:2:21
  |
2 |     let zahl: u32 = "zwei";
  |               ---   ^^^^^^ expected `u32`, found `&str`
  |               |
  |               expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

This message is the normal case and not a disaster. It has a number, `E0308`,
which can be looked up. It has a line and a column, `2:21`. It has an arrow
under exactly the place in question. And it says what it expected and what it
found: expected `u32`, found `&str`. The right answer to it is either to write a
number there or to change the type.

The moment matters: the program never ran. The mistake was found before anything
happened at all.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The exercises are deliberately tiny, because
this is about the way from text to a running program and not yet about the
language.

- `greeting` returns `Hallo, <name>!` with the name that was passed in
- `doubled` returns twice `n`

```console
cd units/00-01-programm-und-compiler
cargo test
```

### Source

    Book, chapter 1 "Getting Started", section 1.2 "Hello, World!",
    https://doc.rust-lang.org/book/ch01-02-hello-world.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
