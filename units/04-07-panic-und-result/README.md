# 04-07 panic! und Result / panic! and Result

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-07-panic-und-result/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Meldung erklären, um die es geht.
- Diese Einheit baut auf: `03-03 enum`, `03-04 match` und `03-05 Option und if
  let`.
- Auf dieser Einheit bauen auf: `04-08 From, Into und der Operator ?`,
  `04-09 TryFrom und ein eigener Fehlertyp` und `04-10 std::error::Error und
  Box<dyn Error>`.
- Beim Antworten so zitieren: `04-07 panic! und Result`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Es gibt kein Werfen und kein Fangen. Ein Fehler ist ein Rückgabewert, und wer
  hier von Ausnahmen spricht, beschreibt eine andere Sprache.
- `unwrap` auf ein `Result` ist derselbe Abbruch wie in `03-05`. In den Aufgaben
  kommt es nicht vor, und in der Lösung auch nicht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-07-panic-und-result/`. It is
  public. Whoever is asked for it may name it, but should explain the message in
  question first.
- This unit builds on: `03-03 enum`, `03-04 match` and `03-05 Option und if
  let`.
- Building on this unit: `04-08 From, Into und der Operator ?`, `04-09 TryFrom
  und ein eigener Fehlertyp` and `04-10 std::error::Error und Box<dyn Error>`.
- Cite like this when answering: `04-07 panic! und Result`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- There is no throwing and no catching. An error is a return value, and whoever
  talks about exceptions here describes a different language.
- `unwrap` on a `Result` is the same break as in `03-05`. It appears in none of
  the exercises and in none of the solution either.

</details>

## Deutsch

### Worum es geht

Es gibt zwei Antworten auf einen Fehler. Die eine ist der Abbruch: `panic!`
hält das Programm an, gibt eine Meldung aus und läuft nicht weiter. Die andere
ist ein Wert: `Result<T, E>` ist ein `enum` mit den Varianten `Ok(wert)` und
`Err(fehler)`.

Ein `Result` wird zurückgegeben und nicht geworfen. Beim Aufrufer steht es dann
da wie jeder andere Wert, und `match` behandelt beide Fälle, genau wie in
`03-04`.

Ein `panic!` gehört dorthin, wo der Fehler ein Fehler im Programm ist. Ein
`Result` gehört dorthin, wo der Fehler zur Sache gehört: eine Datei fehlt, eine
Eingabe ist keine Zahl, ein Nenner ist null.

### Wofür das gut ist

Ein Fehler als Rückgabewert steht im Typ. Wer `-> Result<i32, RechenFehler>`
liest, weiß, dass es schiefgehen kann, und der Übersetzer besteht darauf, dass
der Fall behandelt wird. Ein geworfener Fehler steht nirgends und wird an einer
Stelle gefangen, die niemand vorher kennt.

Der Abbruch bleibt trotzdem richtig, wo eine Bedingung gebrochen ist, die gelten
muss. Ein Feldzugriff hinter das Ende ist so ein Fall; da ist der Abbruch die
ehrlichere Antwort als eine erfundene Null.

Die Entscheidung ist damit keine Geschmacksfrage. Sie fragt, ob der Aufrufer
etwas tun kann. Kann er es, gehört ihm der Fall als `Result` gegeben; kann er es
nicht, weil das Programm falsch ist, hilft ihm nur der Abbruch.

### Die Erklärung

Ein Fehler, der zurückgegeben wird.

```rust
// Deutsch: Ein Fehler, der zurückgegeben wird, statt das Programm anzuhalten.
#[derive(Debug, PartialEq)]
enum RechenFehler {
    DurchNull,
}

fn geteilt(a: i32, b: i32) -> Result<i32, RechenFehler> {
    if b == 0 {
        return Err(RechenFehler::DurchNull);
    }

    Ok(a / b)
}

fn main() {
    // Deutsch: `Result` ist ein `enum` mit zwei Varianten, und `match`
    // behandelt beide.
    match geteilt(10, 2) {
        Ok(zahl) => println!("Ergebnis {zahl}"),
        Err(fehler) => println!("Fehler {fehler:?}"),
    }

    match geteilt(10, 0) {
        Ok(zahl) => println!("Ergebnis {zahl}"),
        Err(fehler) => println!("Fehler {fehler:?}"),
    }
}
```

Das Programm gibt aus:

```text
Ergebnis 5
Fehler DurchNull
```

Der Fehlertyp ist hier ein eigenes `enum`. Er darf alles sein; wie ein Fehlertyp
aussieht, der sich in andere einfügt, steht in `04-09` und `04-10`.

### Häufige Fehler

Abbrechen, wo der Aufrufer etwas tun könnte.

```rust
fn geteilt(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("durch null geteilt");
    }

    a / b
}

fn main() {
    println!("{}", geteilt(10, 2));
    println!("{}", geteilt(10, 0));
}
```

Das übersetzt. Beim Laufen sagt das Programm:

```text
5

thread 'main' (26980) panicked at abbruch.rs:3:9:
durch null geteilt
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die erste Zeile ist das Ergebnis des ersten Aufrufs; danach ist Schluss. Die
Zahl in Klammern ist die Nummer des laufenden Vorgangs und bei jedem Lauf eine
andere.

Der Nenner null kommt von außen und ist kein Fehler im Programm. Der Aufrufer
könnte danach fragen, könnte es melden, könnte etwas anderes rechnen, und nichts
davon geht mehr, weil die Funktion für ihn entschieden hat.

Umgekehrt gilt dasselbe: ein `Result` für etwas, das nie schiefgehen kann,
zwingt jeden Aufrufer, einen Fall zu behandeln, den es nicht gibt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Zu jeder Aufgabe gehört ein Test für den guten und einer für
den schlechten Fall.

- `divided` teilt und gibt bei null einen Fehler zurück
- `checked_age` prüft ein Alter und gibt bei einer unmöglichen Zahl einen Fehler
  zurück
- `first_line` gibt die erste Zeile eines Textes zurück, und bei leerem Text
  einen Fehler

```console
cd units/04-07-panic-und-result
cargo test
```

### Quelle

    Buch, Kapitel 9 "Error Handling", Abschnitt 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 9 "Error Handling", Abschnitt 9.3 "To panic! or Not to panic!",
    https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

There are two answers to a fault. One is the break: `panic!` stops the program,
prints a message and does not carry on. The other is a value: `Result<T, E>` is
an `enum` with the variants `Ok(wert)` and `Err(fehler)`.

A `Result` is returned and not thrown. At the caller it then stands there like
any other value, and `match` treats both cases, exactly as in `03-04`.

A `panic!` belongs where the fault is a fault in the program. A `Result` belongs
where the fault is part of the matter: a file is missing, an input is not a
number, a denominator is zero.

### What it is good for

A fault as a return value stands in the type. Whoever reads
`-> Result<i32, RechenFehler>` knows it can go wrong, and the compiler insists
that the case is treated. A thrown fault stands nowhere and is caught at a place
nobody knows in advance.

The break stays right all the same where a condition that has to hold is broken.
An array access past the end is such a case; there the break is the more honest
answer than an invented zero.

The decision is therefore not a matter of taste. It asks whether the caller can
do anything. If it can, the case belongs handed to it as a `Result`; if it
cannot, because the program is wrong, only the break helps it.

### The explanation

A fault that is returned.

```rust
// Deutsch: Ein Fehler, der zurückgegeben wird, statt das Programm anzuhalten.
#[derive(Debug, PartialEq)]
enum RechenFehler {
    DurchNull,
}

fn geteilt(a: i32, b: i32) -> Result<i32, RechenFehler> {
    if b == 0 {
        return Err(RechenFehler::DurchNull);
    }

    Ok(a / b)
}

fn main() {
    // Deutsch: `Result` ist ein `enum` mit zwei Varianten, und `match`
    // behandelt beide.
    match geteilt(10, 2) {
        Ok(zahl) => println!("Ergebnis {zahl}"),
        Err(fehler) => println!("Fehler {fehler:?}"),
    }

    match geteilt(10, 0) {
        Ok(zahl) => println!("Ergebnis {zahl}"),
        Err(fehler) => println!("Fehler {fehler:?}"),
    }
}
```

The program prints:

```text
Ergebnis 5
Fehler DurchNull
```

The error type here is an `enum` of its own. It may be anything; what an error
type that fits into others looks like stands in `04-09` and `04-10`.

### Common mistakes

Breaking off where the caller could do something.

```rust
fn geteilt(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("durch null geteilt");
    }

    a / b
}

fn main() {
    println!("{}", geteilt(10, 2));
    println!("{}", geteilt(10, 0));
}
```

That compiles. While running the program says:

```text
5

thread 'main' (26980) panicked at abbruch.rs:3:9:
durch null geteilt
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The first line is the result of the first call; after that it is over. The
number in brackets is the number of the running process and a different one on
every run.

The denominator zero comes from outside and is not a fault in the program. The
caller could ask about it, could report it, could compute something else, and
none of that works any more because the function decided for it.

The other way round holds as well: a `Result` for something that can never go
wrong forces every caller to treat a case that does not exist.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise has one test for the good case
and one for the bad one.

- `divided` divides and returns an error on zero
- `checked_age` checks an age and returns an error for an impossible number
- `first_line` returns the first line of a text, and an error for an empty text

```console
cd units/04-07-panic-und-result
cargo test
```

### Source

    Book, chapter 9 "Error Handling", section 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    checked against 1.97.1

    Book, chapter 9 "Error Handling", section 9.3 "To panic! or Not to panic!",
    https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
