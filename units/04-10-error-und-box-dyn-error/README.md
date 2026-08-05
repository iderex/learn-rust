# 04-10 std::error::Error und Box&lt;dyn Error&gt; / std::error::Error and Box&lt;dyn Error&gt;

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/04-10-error-und-box-dyn-error/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `04-02 Module`, `04-08 From, Into und der Operator ?`
  und `04-09 TryFrom und ein eigener Fehlertyp`. Mit ihr ist die Stufe 4 zu
  Ende.
- Auf dieser Einheit bauen auf: `06-01 Argumente von der Kommandozeile` und
  alles, was mehrere Fehlerarten zusammenführt.
- Beim Antworten so zitieren: `04-10 std::error::Error und Box<dyn Error>`, dazu
  die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Box<dyn Error>` gehört an die oberste Ebene und nicht in jede Funktion. Wer
  es überall vorschlägt, nimmt dem Aufrufer die Möglichkeit, die Fälle zu
  unterscheiden.
- Die Einheit liegt in mehreren Dateien, so wie `04-02`. Wer eine Aufgabe löst,
  schreibt in die Datei des Moduls.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-10-error-und-box-dyn-error/`. It
  is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `04-02 Module`, `04-08 From, Into und der Operator ?` and
  `04-09 TryFrom und ein eigener Fehlertyp`. With it stage 4 ends.
- Building on this unit: `06-01 Argumente von der Kommandozeile` and everything
  that brings several error kinds together.
- Cite like this when answering: `04-10 std::error::Error und Box<dyn Error>`,
  plus the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Box<dyn Error>` belongs at the top level and not in every function. Whoever
  suggests it everywhere takes the caller's chance to tell the cases apart.
- The unit lies in several files, the way `04-02` does. Whoever solves an
  exercise writes into the file of the module.

</details>

## Deutsch

### Worum es geht

`std::error::Error` ist der Trait, an dem sich Fehler erkennen. Er verlangt
`Debug` und `Display` und sonst nichts; der `impl`-Block bleibt leer.

`Box<dyn Error>` ist ein Kasten, in den jeder Fehler passt, der diesen Trait
kann. Was für ein Fehler darin liegt, weiß der Typ nicht mehr, nur noch, dass es
einer ist.

Beides zusammen ergibt die übliche Aufteilung. Innen hat jede Ebene ihren
eigenen Fehlertyp, und fremde Fehler kommen über `From` hinein, so wie in
`04-08`. An der obersten Ebene, wo alles zusammenläuft, steht dann
`Box<dyn Error>`.

### Wofür das gut ist

Der eigene Fehlertyp trägt die Unterscheidung. Wer ihn bekommt, kann mit `match`
sagen, was zu tun ist, und `04-09` hat gezeigt, wie er dabei die Meldung für
Menschen gleich mitbringt.

Ganz oben ist diese Unterscheidung nichts mehr wert. Dort wird der Fehler
gemeldet und das Programm beendet, und dafür reicht es, dass er eine Meldung
hat. `Box<dyn Error>` spart an dieser einen Stelle die Aufzählung aller Fälle,
die dort niemand mehr liest.

Der Preis ist genau diese Unterscheidung. Wer `Box<dyn Error>` schon in der
Mitte benutzt, kann später nicht mehr fragen, welcher Fall es war, ohne den
Kasten wieder aufzumachen.

### Die Erklärung

Ein eigener Fehlertyp, zwei fremde Fehlerarten, und eine oberste Ebene.

```rust
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

// Deutsch: Ein eigener Fehlertyp, den zwei fremde Fehlerarten erreichen.
#[derive(Debug)]
enum AppFehler {
    KeineZahl(ParseIntError),
    KeineKommazahl(ParseFloatError),
}

impl fmt::Display for AppFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppFehler::KeineZahl(fehler) => write!(f, "keine ganze Zahl: {fehler}"),
            AppFehler::KeineKommazahl(fehler) => write!(f, "keine Kommazahl: {fehler}"),
        }
    }
}

// Deutsch: `Error` verlangt `Debug` und `Display` und braucht selbst keinen
// Rumpf. Damit passt der Typ in ein `Box<dyn Error>`.
impl Error for AppFehler {}

impl From<ParseIntError> for AppFehler {
    fn from(fehler: ParseIntError) -> Self {
        AppFehler::KeineZahl(fehler)
    }
}

impl From<ParseFloatError> for AppFehler {
    fn from(fehler: ParseFloatError) -> Self {
        AppFehler::KeineKommazahl(fehler)
    }
}

fn summe(ganz: &str, komma: &str) -> Result<f64, AppFehler> {
    // Deutsch: Zwei fremde Fehlerarten, ein eigener Fehlertyp, zweimal `?`.
    let a: i32 = ganz.trim().parse()?;
    let b: f64 = komma.trim().parse()?;

    Ok(f64::from(a) + b)
}

fn oberste_ebene(ganz: &str, komma: &str) -> Result<f64, Box<dyn Error>> {
    // Deutsch: `Box<dyn Error>` nimmt jeden Fehler an, der `Error` kann.
    let ergebnis = summe(ganz, komma)?;

    Ok(ergebnis)
}

fn main() {
    println!("{:?}", oberste_ebene("2", "0.5"));

    match oberste_ebene("zwei", "0.5") {
        Ok(zahl) => println!("{zahl}"),
        Err(fehler) => println!("{fehler}"),
    }

    match oberste_ebene("2", "halb") {
        Ok(zahl) => println!("{zahl}"),
        Err(fehler) => println!("{fehler}"),
    }
}
```

Das Programm gibt aus:

```text
Ok(2.5)
keine ganze Zahl: invalid digit found in string
keine Kommazahl: invalid float literal
```

Hinter dem Doppelpunkt steht die Meldung des fremden Fehlers. Sie steht dort,
weil die Variante ihn mitträgt statt ihn wegzuwerfen, und weil `Display` ihn
mit ausgibt.

### Häufige Fehler

`Error` ohne `Display`.

```rust
use std::error::Error;

#[derive(Debug)]
struct MeinFehler;

impl Error for MeinFehler {}

fn main() {
    println!("{:?}", MeinFehler);
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `MeinFehler` doesn't implement `std::fmt::Display`
 --> ohnedisplay.rs:6:16
  |
6 | impl Error for MeinFehler {}
  |                ^^^^^^^^^^ unsatisfied trait bound
  |
help: the trait `std::fmt::Display` is not implemented for `MeinFehler`
 --> ohnedisplay.rs:4:1
  |
4 | struct MeinFehler;
  | ^^^^^^^^^^^^^^^^^
note: required by a bound in `std::error::Error`
 --> .../library/core/src/error.rs:59:0

error: aborting due to 1 previous error
```

In der vorletzten Zeile steht im Lauf ein Pfad in die Quellen des Übersetzers;
hier steht dafür `...`, weil er auf jedem Rechner anders lautet.

Der leere `impl`-Block ist also nicht ganz leer: er verlangt zwei andere
Implementierungen. `Debug` kommt mit `derive`, `Display` schreibt man von Hand,
und beides zusammen ist die Bedingung dafür, dass ein Fehler in einen
`Box<dyn Error>` passt.

### Die Aufgaben

Die Rümpfe sind `todo!()`, und die Tests in `tests/exercise.rs` sind so lange
rot. Die Einheit liegt in drei Dateien: `src/lib.rs` ist die oberste Ebene,
`src/fehler.rs` trägt den Fehlertyp, `src/eingabe.rs` das Lesen.

- `Display for AppFehler` in `src/fehler.rs`
- `From<ParseFloatError> for AppFehler` in `src/fehler.rs`
- `summe_aus_texten` in `src/lib.rs`, mit `Box<dyn Error>`

```console
cd units/04-10-error-und-box-dyn-error
cargo test
```

### Quelle

    Buch, Kapitel 9 "Error Handling", Abschnitt 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    geprüft gegen 1.97.1

    Standardbibliothek, "Error in std::error",
    https://doc.rust-lang.org/std/error/trait.Error.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`std::error::Error` is the trait by which errors are recognised. It demands
`Debug` and `Display` and nothing else; the `impl` block stays empty.

`Box<dyn Error>` is a box every error fits into that can do this trait. What
kind of error lies in it is nothing the type knows any more, only that it is
one.

Both together give the usual layout. Inside, every level has an error type of
its own, and foreign errors come in through `From`, the way they did in `04-08`.
At the top level, where everything comes together, `Box<dyn Error>` then stands.

### What it is good for

The error type of its own carries the distinction. Whoever receives it can say
with `match` what to do, and `04-09` showed how it brings the message for people
along at the same time.

At the very top that distinction is worth nothing any more. There the error gets
reported and the program ends, and for that it is enough that it has a message.
`Box<dyn Error>` saves listing all the cases at that one place, where nobody
reads them any more.

The price is exactly that distinction. Whoever uses `Box<dyn Error>` in the
middle already cannot ask later which case it was without opening the box again.

### The explanation

An error type of its own, two foreign error kinds, and a top level.

```rust
use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

// Deutsch: Ein eigener Fehlertyp, den zwei fremde Fehlerarten erreichen.
#[derive(Debug)]
enum AppFehler {
    KeineZahl(ParseIntError),
    KeineKommazahl(ParseFloatError),
}

impl fmt::Display for AppFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppFehler::KeineZahl(fehler) => write!(f, "keine ganze Zahl: {fehler}"),
            AppFehler::KeineKommazahl(fehler) => write!(f, "keine Kommazahl: {fehler}"),
        }
    }
}

// Deutsch: `Error` verlangt `Debug` und `Display` und braucht selbst keinen
// Rumpf. Damit passt der Typ in ein `Box<dyn Error>`.
impl Error for AppFehler {}

impl From<ParseIntError> for AppFehler {
    fn from(fehler: ParseIntError) -> Self {
        AppFehler::KeineZahl(fehler)
    }
}

impl From<ParseFloatError> for AppFehler {
    fn from(fehler: ParseFloatError) -> Self {
        AppFehler::KeineKommazahl(fehler)
    }
}

fn summe(ganz: &str, komma: &str) -> Result<f64, AppFehler> {
    // Deutsch: Zwei fremde Fehlerarten, ein eigener Fehlertyp, zweimal `?`.
    let a: i32 = ganz.trim().parse()?;
    let b: f64 = komma.trim().parse()?;

    Ok(f64::from(a) + b)
}

fn oberste_ebene(ganz: &str, komma: &str) -> Result<f64, Box<dyn Error>> {
    // Deutsch: `Box<dyn Error>` nimmt jeden Fehler an, der `Error` kann.
    let ergebnis = summe(ganz, komma)?;

    Ok(ergebnis)
}

fn main() {
    println!("{:?}", oberste_ebene("2", "0.5"));

    match oberste_ebene("zwei", "0.5") {
        Ok(zahl) => println!("{zahl}"),
        Err(fehler) => println!("{fehler}"),
    }

    match oberste_ebene("2", "halb") {
        Ok(zahl) => println!("{zahl}"),
        Err(fehler) => println!("{fehler}"),
    }
}
```

The program prints:

```text
Ok(2.5)
keine ganze Zahl: invalid digit found in string
keine Kommazahl: invalid float literal
```

Behind the colon stands the message of the foreign error. It stands there
because the variant carries it along instead of throwing it away, and because
`Display` prints it with the rest.

### Common mistakes

`Error` without `Display`.

```rust
use std::error::Error;

#[derive(Debug)]
struct MeinFehler;

impl Error for MeinFehler {}

fn main() {
    println!("{:?}", MeinFehler);
}
```

The compiler answers:

```text
error[E0277]: `MeinFehler` doesn't implement `std::fmt::Display`
 --> ohnedisplay.rs:6:16
  |
6 | impl Error for MeinFehler {}
  |                ^^^^^^^^^^ unsatisfied trait bound
  |
help: the trait `std::fmt::Display` is not implemented for `MeinFehler`
 --> ohnedisplay.rs:4:1
  |
4 | struct MeinFehler;
  | ^^^^^^^^^^^^^^^^^
note: required by a bound in `std::error::Error`
 --> .../library/core/src/error.rs:59:0

error: aborting due to 1 previous error
```

In the second to last line a path into the compiler's sources stands during the
run; a `...` stands here instead, because it reads differently on every machine.

The empty `impl` block is therefore not quite empty: it demands two other
implementations. `Debug` comes with `derive`, `Display` is written by hand, and
both together are the condition for an error to fit into a `Box<dyn Error>`.

### The exercises

The bodies are `todo!()`, and the tests in `tests/exercise.rs` stay red for as
long as they are. The unit lies in three files: `src/lib.rs` is the top level,
`src/fehler.rs` carries the error type, `src/eingabe.rs` the reading.

- `Display for AppFehler` in `src/fehler.rs`
- `From<ParseFloatError> for AppFehler` in `src/fehler.rs`
- `summe_aus_texten` in `src/lib.rs`, with `Box<dyn Error>`

```console
cd units/04-10-error-und-box-dyn-error
cargo test
```

### Source

    Book, chapter 9 "Error Handling", section 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    checked against 1.97.1

    Standard library, "Error in std::error",
    https://doc.rust-lang.org/std/error/trait.Error.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
