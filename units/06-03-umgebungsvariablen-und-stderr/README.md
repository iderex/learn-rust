# 06-03 Umgebungsvariablen und Ausgabe nach stderr / Environment variables and output to stderr

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/06-03-umgebungsvariablen-und-stderr/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `06-01 Argumente von der Kommandozeile` und
  `06-02 Dateien lesen und schreiben`. Der Aufruf und die Datei kommen von dort,
  hier kommt die Einstellung von außen und der zweite Ausgang dazu.
- Auf dieser Einheit bauen auf: der Rest der Stufe 6 und jedes Werkzeug, dessen
  Ausgabe jemand weiterleitet.
- Beim Antworten so zitieren: `06-03 Umgebungsvariablen und Ausgabe nach
  stderr`, dazu die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die
  Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `env::var` gibt ein `Result` und nicht einen `String`. `VarError::NotPresent`
  und `VarError::NotUnicode` sind zwei verschiedene Aussagen, und der Test mit
  `NotUnicode` ist genau dagegen da, beide gleich zu behandeln.
- Dass ein Text nach stdout statt nach stderr geht, meldet kein Übersetzer und
  keine Prüfung. Es fällt erst auf, wenn jemand die Ausgabe weiterleitet.
  Deshalb nehmen die Aufgaben zwei Schreibziele als Parameter, statt `println!`
  und `eprintln!` zu benutzen: nur so kann ein Test die Trennung ansehen.
- `std::env::set_var` ist in der Ausgabe 2024 `unsafe`, und Tests laufen
  nebeneinander in einem Prozess. Die Tests dieser Einheit setzen deshalb keine
  Variable, sondern geben das Ergebnis von `env::var` als Wert herein.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/06-03-umgebungsvariablen-und-stderr/`. It is public. Whoever is
  asked for it may name it, but should explain the compiler message in question
  first.
- This unit builds on: `06-01 Argumente von der Kommandozeile` and
  `06-02 Dateien lesen und schreiben`. The call and the file come from there,
  what is added here is the setting from outside and the second exit.
- Building on this unit: the rest of stage 6 and every tool whose output
  somebody redirects.
- Cite like this when answering: `06-03 Umgebungsvariablen und Ausgabe nach
  stderr`, plus the heading of the section, for example section "The
  explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `env::var` gives a `Result` and not a `String`. `VarError::NotPresent` and
  `VarError::NotUnicode` are two different statements, and the test with
  `NotUnicode` exists against exactly the habit of treating both alike.
- That a text goes to stdout instead of stderr is reported by no compiler and no
  check. It only shows up once somebody redirects the output. That is why the
  exercises take two write targets as parameters rather than using `println!`
  and `eprintln!`: only that way can a test look at the separation.
- `std::env::set_var` is `unsafe` in edition 2024, and tests run alongside each
  other in one process. The tests of this unit therefore set no variable but
  hand in the result of `env::var` as a value.

</details>

## Deutsch

### Worum es geht

Eine Umgebungsvariable ist eine Einstellung, die neben dem Aufruf steht. Sie
kommt nicht aus dem Quelltext und nicht aus der Kommandozeile, sondern aus der
Umgebung, in der das Programm gestartet wurde. Gelesen wird sie mit
`std::env::var`.

`env::var` gibt kein `String` zurück, sondern ein `Result<String, VarError>`,
und das aus zwei Gründen. Die Variable kann fehlen, dann steht dort
`VarError::NotPresent`. Und sie kann gesetzt sein, ohne gültiger Unicode-Text zu
sein, dann steht dort `VarError::NotUnicode`.

Der zweite Teil dieser Einheit hat mit der Umgebung nichts zu tun und steht
trotzdem daneben, weil beides beim Bau eines Werkzeugs zusammen auftritt: ein
Programm hat zwei Ausgänge. Das Ergebnis geht nach stdout, alles, was über das
Ergebnis redet, geht nach stderr.

### Wofür das gut ist

Die Trennung fällt nicht auf, solange beides auf demselben Bildschirm landet.
Sie fällt in dem Augenblick auf, in dem jemand die Ausgabe weiterleitet. Wer
`programm > treffer.txt` schreibt, will die Treffer in der Datei haben und die
Meldung weiterhin sehen.

Steht die Meldung nach stdout, landet sie in der Datei, mitten zwischen den
Treffern. Die Datei ist dann keine Liste von Treffern mehr, sondern eine Liste
von Treffern mit einem Satz darin, und wer sie weiterverarbeitet, zählt den Satz
mit.

Umgekehrt ist es genauso schlimm. Ein Ergebnis nach stderr ist beim Weiterleiten
verschwunden, denn stderr geht nicht in die Datei. Die Regel ist also nicht
"Meldungen nach stderr", sondern "das Ergebnis nach stdout und alles andere
nach stderr".

Eine Einstellung aus der Umgebung passt zu dieser Trennung, weil sie das
Verhalten ändert, ohne im Aufruf zu stehen. Genau deshalb sagt ein gutes
Werkzeug auf stderr dazu, welche Einstellung gegriffen hat, und stört damit die
weitergeleitete Datei nicht.

### Die Erklärung

Ein Programm, das eine Einstellung aus der Umgebung liest, die Meldung darüber
nach stderr schreibt und die Treffer nach stdout.

```rust
use std::env;
use std::io::{self, Write};

// Deutsch: Die Meldung geht nach stderr, das Ergebnis nach stdout. Wer die
// Ausgabe weiterleitet, bekommt nur das Ergebnis in die Leitung.
fn main() -> io::Result<()> {
    let muster = env::var("LR_MUSTER").unwrap_or_else(|_| String::from("an"));
    let zeilen = ["Apfel", "Birne", "Ananas"];

    let mut aus = io::stdout().lock();
    let mut fehler = io::stderr().lock();

    writeln!(fehler, "gesucht wird nach {muster}")?;
    for zeile in zeilen {
        if zeile.contains(&muster) {
            writeln!(aus, "{zeile}")?;
        }
    }

    Ok(())
}
```

Zweimal gestartet, einmal ohne und einmal mit Weiterleitung:

```console
$ ./trennen
gesucht wird nach an
Ananas
$ ./trennen > treffer.txt
gesucht wird nach an
$ cat treffer.txt
Ananas
```

Der zweite Aufruf ist der, um den es geht. Die Meldung steht weiter auf dem
Bildschirm, obwohl stdout in die Datei geht, und in der Datei steht nur der
Treffer. Hätte die Meldung `println!` benutzt, stünde sie in der Datei, und
nichts hätte sich darüber beschwert.

### Häufige Fehler

`env::var` für einen `String` halten.

```rust
use std::env;

fn main() {
    let modus: String = env::var("LR_MODUS");

    println!("{modus}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> umgebung.rs:4:25
  |
4 |     let modus: String = env::var("LR_MODUS");
  |                ------   ^^^^^^^^^^^^^^^^^^^^ expected `String`, found `Result<String, VarError>`
  |                |
  |                expected due to this
  |
  = note: expected struct `String`
               found enum `Result<String, VarError>`
help: consider using `Result::expect` to unwrap the `Result<String, VarError>` value, panicking if the value is a `Result::Err`
  |
4 |     let modus: String = env::var("LR_MODUS").expect("REASON");
  |                                             +++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Der Vorschlag des Übersetzers ist die schlechteste der möglichen Antworten. Ein
`expect` bricht das Programm ab, weil eine Einstellung fehlt, für die es eine
Vorgabe gibt. `unwrap_or_else` ist die kurze Antwort, und Aufgabe 1 ist die
lange, die die beiden Gründe auseinanderhält.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `aus_der_umgebung` steht fertig da, und sein Doku-Test ist
grün.

- `einstellung` beantwortet die fehlende Variable mit einer Vorgabe und reicht
  jeden anderen Fehler weiter
- `bericht` sammelt die Treffer und sagt dazu, wonach gesucht wurde
- `schreiben` schickt die Meldungen zum einen Ausgang und die Treffer zum
  anderen

```console
cd units/06-03-umgebungsvariablen-und-stderr
cargo test
```

### Quelle

    Buch, Kapitel 12 "An I/O Project: Building a Command Line Program",
    Abschnitt 12.5 "Working with Environment Variables",
    https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 12 "An I/O Project: Building a Command Line Program",
    Abschnitt 12.6 "Redirecting Errors to Standard Error",
    https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

An environment variable is a setting standing next to the call. It comes neither
from the source nor from the command line but from the environment the program
was started in. It is read with `std::env::var`.

`env::var` returns not a `String` but a `Result<String, VarError>`, and that for
two reasons. The variable can be missing, and then `VarError::NotPresent` stands
there. And it can be set without being valid Unicode text, and then
`VarError::NotUnicode` stands there.

The second part of this unit has nothing to do with the environment and stands
next to it anyway, because both turn up together while building a tool: a
program has two exits. The result goes to stdout, everything that talks about
the result goes to stderr.

### What it is good for

The separation does not show while both land on the same screen. It shows the
moment somebody redirects the output. Whoever writes `programm > treffer.txt`
wants the hits in the file and wants to keep seeing the message.

If the message goes to stdout, it lands in the file, in the middle of the hits.
The file is then no longer a list of hits but a list of hits with a sentence in
it, and whoever processes it further counts the sentence along.

The other way round is just as bad. A result on stderr has disappeared once the
output is redirected, because stderr does not go into the file. The rule is
therefore not "messages to stderr" but "the result to stdout and everything else
to stderr".

A setting from the environment fits this separation, because it changes the
behaviour without standing in the call. That is exactly why a good tool says on
stderr which setting took hold, and thereby leaves the redirected file alone.

### The explanation

A program that reads a setting from the environment, writes the message about it
to stderr and the hits to stdout.

```rust
use std::env;
use std::io::{self, Write};

// Deutsch: Die Meldung geht nach stderr, das Ergebnis nach stdout. Wer die
// Ausgabe weiterleitet, bekommt nur das Ergebnis in die Leitung.
fn main() -> io::Result<()> {
    let muster = env::var("LR_MUSTER").unwrap_or_else(|_| String::from("an"));
    let zeilen = ["Apfel", "Birne", "Ananas"];

    let mut aus = io::stdout().lock();
    let mut fehler = io::stderr().lock();

    writeln!(fehler, "gesucht wird nach {muster}")?;
    for zeile in zeilen {
        if zeile.contains(&muster) {
            writeln!(aus, "{zeile}")?;
        }
    }

    Ok(())
}
```

Started twice, once without and once with a redirect:

```console
$ ./trennen
gesucht wird nach an
Ananas
$ ./trennen > treffer.txt
gesucht wird nach an
$ cat treffer.txt
Ananas
```

The second call is the one it is about. The message keeps standing on the screen
although stdout goes into the file, and in the file stands only the hit. Had the
message used `println!`, it would stand in the file, and nothing would have
complained about it.

### Common mistakes

Taking `env::var` for a `String`.

```rust
use std::env;

fn main() {
    let modus: String = env::var("LR_MODUS");

    println!("{modus}");
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> umgebung.rs:4:25
  |
4 |     let modus: String = env::var("LR_MODUS");
  |                ------   ^^^^^^^^^^^^^^^^^^^^ expected `String`, found `Result<String, VarError>`
  |                |
  |                expected due to this
  |
  = note: expected struct `String`
               found enum `Result<String, VarError>`
help: consider using `Result::expect` to unwrap the `Result<String, VarError>` value, panicking if the value is a `Result::Err`
  |
4 |     let modus: String = env::var("LR_MODUS").expect("REASON");
  |                                             +++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

The compiler's suggestion is the worst of the possible answers. An `expect`
aborts the program because a setting is missing for which there is a default.
`unwrap_or_else` is the short answer, and exercise 1 is the long one that keeps
the two reasons apart.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `aus_der_umgebung` stands there finished, and
its doc test is green.

- `einstellung` answers the missing variable with a default and passes every
  other error on
- `bericht` collects the hits and says what was searched for
- `schreiben` sends the messages to one exit and the hits to the other

```console
cd units/06-03-umgebungsvariablen-und-stderr
cargo test
```

### Source

    Book, chapter 12 "An I/O Project: Building a Command Line Program",
    section 12.5 "Working with Environment Variables",
    https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html,
    checked against 1.97.1

    Book, chapter 12 "An I/O Project: Building a Command Line Program",
    section 12.6 "Redirecting Errors to Standard Error",
    https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
