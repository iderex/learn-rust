# 03-05 Option und if let / Option and if let

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-05-option-und-if-let/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Meldung erklären, um die es geht.
- Diese Einheit baut auf: `03-03 enum` und `03-04 match`.
- Auf dieser Einheit bauen auf: `04-07 panic! und Result`, `04-08 From, Into und
  der Operator ?` und alles, was mit fehlenden Werten umgeht.
- Beim Antworten so zitieren: `03-05 Option und if let`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `unwrap` steht hier als Gegenbeispiel und wird mit seiner echten Meldung
  gezeigt. Wer es als Lösung vorschlägt, dreht die Einheit um. In den Aufgaben
  kommt es nicht vor, und in der Lösung auch nicht.
- `Option` ist kein Sonderfall der Sprache, sondern ein `enum` aus der
  Standardbibliothek mit zwei Varianten. Wer es als eingebauten Zauber
  beschreibt, verliert genau die Verbindung zu `03-03` und `03-04`.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-05-option-und-if-let/`. It is
  public. Whoever is asked for it may name it, but should explain the message in
  question first.
- This unit builds on: `03-03 enum` and `03-04 match`.
- Building on this unit: `04-07 panic! und Result`, `04-08 From, Into und der
  Operator ?` and everything dealing with missing values.
- Cite like this when answering: `03-05 Option und if let`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `unwrap` stands here as the counter example and is shown with its real
  message. Whoever offers it as the solution turns the unit around. It appears
  in none of the exercises and in none of the solution either.
- `Option` is not a special case of the language but an `enum` from the standard
  library with two variants. Whoever describes it as built-in magic loses
  exactly the connection to `03-03` and `03-04`.

</details>

## Deutsch

### Worum es geht

Es gibt keinen leeren Wert in Rust. Kein `null`, kein `nil`, nichts, was in
einer Zahl steckt und keine Zahl ist. Was es gibt, ist ein `enum` aus der
Standardbibliothek mit zwei Varianten:

`Option<T>` ist entweder `Some(wert)` oder `None`. Der Typ steht im Programm und
sagt, dass hier etwas fehlen kann.

Herausgeholt wird der Wert mit denselben Mitteln wie bei jedem anderen `enum`,
also mit `match`, oder kürzer mit `if let` und `let ... else`.

### Wofür das gut ist

Ein fehlender Wert steht damit im Typ. Wer eine Funktion mit `-> Option<i32>`
liest, weiß es, ohne die Dokumentation zu suchen, und wer den leeren Fall
vergisst, bekommt es vom Übersetzer gesagt, denn `Option<i32>` ist kein `i32`.

`if let` ist die kurze Form für den Fall, dass nur eine Variante interessiert.
`let ... else` ist die kurze Form für den Fall, dass der leere Fall die Funktion
verlässt; der Wert steht danach für den Rest der Funktion bereit, ohne dass sich
der Rest um eine Ebene einrückt.

`unwrap` gibt den Wert her und hält das Programm an, wenn keiner da ist. Es ist
keine Behandlung des Falls, sondern die Ansage, dass er nicht vorkommt. In einem
kurzen Versuch ist das in Ordnung, und in allem anderen ist es die Stelle, an
der später ein Programm abbricht.

### Die Erklärung

Ein `Option` erzeugen, mit `if let` lesen und mit `let ... else` verlassen.

```rust
fn first_of(zahlen: &[i32]) -> Option<i32> {
    // Deutsch: `first` gibt selbst ein `Option` zurück, denn ein leerer Slice
    // hat kein erstes Element. `copied` macht aus `Option<&i32>` ein
    // `Option<i32>`.
    zahlen.first().copied()
}

fn beschreibung(wert: Option<i32>) -> String {
    // Deutsch: `if let` nimmt den einen Fall heraus, der Daten trägt, und
    // `else` fängt den anderen ab.
    if let Some(zahl) = wert {
        format!("Wert {zahl}")
    } else {
        String::from("kein Wert")
    }
}

fn verdoppelt_oder_null(wert: Option<i32>) -> i32 {
    // Deutsch: `let ... else` bindet den Wert für den Rest der Funktion. Der
    // `else`-Zweig muss sie verlassen.
    let Some(zahl) = wert else {
        return 0;
    };

    zahl * 2
}

fn main() {
    let zahlen = [7, 8, 9];

    println!("{}", beschreibung(first_of(&zahlen)));
    println!("{}", beschreibung(first_of(&[])));
    println!("{}", verdoppelt_oder_null(first_of(&zahlen)));
    println!("{}", verdoppelt_oder_null(None));
}
```

Dass die Standardbibliothek selbst `Option` zurückgibt, ist der übliche Fall.
`first` auf einem Slice kann nichts anderes tun, denn ein leerer Slice hat kein
erstes Element, und eine erfundene Null wäre eine Lüge.

### Häufige Fehler

`unwrap` auf einem leeren Wert.

```rust
fn main() {
    let zahlen: [i32; 0] = [];

    let erste = zahlen.first().copied();

    println!("{}", erste.unwrap());
}
```

Das übersetzt. Beim Laufen sagt das Programm:

```text
thread 'main' (51292) panicked at leer.rs:6:26:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die Zahl in Klammern ist die Nummer des laufenden Vorgangs und bei jedem Lauf
eine andere.

Der Übersetzer hat den leeren Fall angeboten, und `unwrap` hat ihn abgelehnt.
Was der Typ noch wusste, weiß das Programm ab dieser Zeile nicht mehr, und der
Abbruch kommt genau dann, wenn der Fall wirklich eintritt, also spät.

Die Antwort ist eine der drei Formen aus der Erklärung. Wer wirklich sicher ist,
dass der Fall nicht vorkommt, schreibt `expect` mit einem Satz, warum, und dann
steht der Grund wenigstens in der Meldung.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe hat einen Test für den vorhandenen und einen für
den fehlenden Wert. `unwrap` kommt in keiner Lösung vor.

- `grade_for` gibt zu einer Punktzahl ein Urteil zurück, und `None` über 100
- `describe` beschreibt ein `Option` als Text, mit `if let`
- `doubled_or_zero` verdoppelt den Wert oder gibt null zurück, mit `let ... else`

```console
cd units/03-05-option-und-if-let
cargo test
```

### Quelle

    Buch, Kapitel 6 "Enums and Pattern Matching", Abschnitt 6.3 "Concise Control Flow with if let and let...else",
    https://doc.rust-lang.org/book/ch06-03-if-let.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

There is no empty value in Rust. No `null`, no `nil`, nothing that sits inside a
number and is not a number. What there is, is an `enum` from the standard
library with two variants:

`Option<T>` is either `Some(wert)` or `None`. The type stands in the program and
says that something can be missing here.

The value is pulled out with the same means as for any other `enum`, so with
`match`, or shorter with `if let` and `let ... else`.

### What it is good for

A missing value stands in the type with that. Whoever reads a function with
`-> Option<i32>` knows it without searching the documentation, and whoever
forgets the empty case is told so by the compiler, because `Option<i32>` is not
an `i32`.

`if let` is the short form for the case where only one variant is of interest.
`let ... else` is the short form for the case where the empty one leaves the
function; the value then stands ready for the rest of the function without the
rest moving in by one level.

`unwrap` hands the value out and stops the program if there is none. It is not a
treatment of the case but the announcement that it does not occur. In a short
try that is in order, and in everything else it is the place where a program
breaks off later.

### The explanation

Creating an `Option`, reading it with `if let` and leaving with `let ... else`.

```rust
fn first_of(zahlen: &[i32]) -> Option<i32> {
    // Deutsch: `first` gibt selbst ein `Option` zurück, denn ein leerer Slice
    // hat kein erstes Element. `copied` macht aus `Option<&i32>` ein
    // `Option<i32>`.
    zahlen.first().copied()
}

fn beschreibung(wert: Option<i32>) -> String {
    // Deutsch: `if let` nimmt den einen Fall heraus, der Daten trägt, und
    // `else` fängt den anderen ab.
    if let Some(zahl) = wert {
        format!("Wert {zahl}")
    } else {
        String::from("kein Wert")
    }
}

fn verdoppelt_oder_null(wert: Option<i32>) -> i32 {
    // Deutsch: `let ... else` bindet den Wert für den Rest der Funktion. Der
    // `else`-Zweig muss sie verlassen.
    let Some(zahl) = wert else {
        return 0;
    };

    zahl * 2
}

fn main() {
    let zahlen = [7, 8, 9];

    println!("{}", beschreibung(first_of(&zahlen)));
    println!("{}", beschreibung(first_of(&[])));
    println!("{}", verdoppelt_oder_null(first_of(&zahlen)));
    println!("{}", verdoppelt_oder_null(None));
}
```

That the standard library itself returns `Option` is the usual case. `first` on
a slice can do nothing else, because an empty slice has no first element, and an
invented zero would be a lie.

### Common mistakes

`unwrap` on an empty value.

```rust
fn main() {
    let zahlen: [i32; 0] = [];

    let erste = zahlen.first().copied();

    println!("{}", erste.unwrap());
}
```

That compiles. While running the program says:

```text
thread 'main' (51292) panicked at leer.rs:6:26:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The number in brackets is the number of the running process and a different one
on every run.

The compiler offered the empty case and `unwrap` refused it. What the type still
knew, the program does not know from that line on, and the break comes exactly
when the case really happens, so late.

The answer is one of the three forms from the explanation. Whoever really is
sure the case does not occur writes `expect` with a sentence saying why, and
then at least the reason stands in the message.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise has one test for the value that
is there and one for the value that is missing. `unwrap` appears in no solution.

- `grade_for` returns a verdict for a score, and `None` above 100
- `describe` describes an `Option` as text, with `if let`
- `doubled_or_zero` doubles the value or returns zero, with `let ... else`

```console
cd units/03-05-option-und-if-let
cargo test
```

### Source

    Book, chapter 6 "Enums and Pattern Matching", section 6.3 "Concise Control Flow with if let and let...else",
    https://doc.rust-lang.org/book/ch06-03-if-let.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
