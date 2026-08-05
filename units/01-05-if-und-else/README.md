# 01-05 if und else / if and else

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/01-05-if-und-else/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `01-03 Funktionen` und `01-04 Kommentare und
  cargo fmt`.
- Auf dieser Einheit bauen auf: `01-06 Schleifen und cargo clippy`, und später
  `03-04 match`, wo aus der Verzweigung eine Fallunterscheidung wird.
- Beim Antworten so zitieren: `01-05 if und else`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Bedingung muss ein `bool` sein. Wer hier auf andere Sprachen verweist, in
  denen eine Zahl als Bedingung durchgeht, sagt bitte dazu, dass Rust das nicht
  tut und mit `E0308` antwortet.
- Das Einlesen der Eingabe steht im Beispiel und nicht in den Aufgaben. Die
  Aufgaben bekommen die gelesene Zeile als `&str`, weil ein Test nichts tippen
  kann.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/01-05-if-und-else/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `01-03 Funktionen` and `01-04 Kommentare und cargo fmt`.
- Building on this unit: `01-06 Schleifen und cargo clippy`, and later
  `03-04 match`, where the branch becomes a case distinction.
- Cite like this when answering: `01-05 if und else`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The condition has to be a `bool`. Whoever points at other languages, where a
  number passes as a condition, please say alongside it that Rust does not and
  answers with `E0308`.
- Reading the input stands in the example and not in the exercises. The
  exercises get the line that was read as a `&str`, because a test cannot type.

</details>

## Deutsch

### Worum es geht

Ein Programm soll nicht immer dasselbe tun. `if` prüft eine Bedingung, und der
Block dahinter läuft nur, wenn sie zutrifft. `else` fängt den anderen Fall auf,
`else if` schiebt eine weitere Prüfung dazwischen.

Zwei Dinge sind an Rust dabei eigen. Die Bedingung muss ein `bool` sein, eine
Zahl reicht nicht. Und `if` ist ein Ausdruck: es hat einen Wert und darf deshalb
rechts von einem `let` stehen.

### Wofür das gut ist

Dass die Bedingung ein `bool` sein muss, nimmt eine ganze Fehlerklasse weg. Wo
eine Sprache jede Zahl außer null als wahr liest, geht ein vertippter Vergleich
still durch. Hier bleibt er stehen, und zwar schon beim Übersetzen.

Dass `if` einen Wert hat, spart die Zwischenstufe. Statt eine Bindung leer
anzulegen und sie in beiden Zweigen zu füllen, wird sie einmal mit dem Ergebnis
der Verzweigung angelegt. Sie kann dann auch unveränderbar bleiben, was der
üblichere Fall ist.

### Die Erklärung

`if` rechts von einem `let`. Beide Zweige müssen denselben Typ liefern, denn die
Bindung bekommt genau einen Typ.

```rust
fn main() {
    let punkte = 61;

    // Deutsch: Der Wert der Verzweigung ist der Wert des Zweigs, der läuft.
    let ergebnis = if punkte >= 60 { "bestanden" } else { "nicht bestanden" };

    println!("{ergebnis}");
}
```

Die Eingabe kommt mit `read_line` von der Tastatur und landet in einem `String`.
Die gelesene Zeile trägt den Zeilenumbruch noch mit sich, deshalb steht `trim()`
dazwischen.

```rust
use std::io::stdin;

fn main() {
    let mut zeile = String::new();
    stdin().read_line(&mut zeile).expect("keine Eingabe gelesen");

    let antwort = if zeile.trim() == "ja" { "weiter" } else { "abbruch" };

    println!("{antwort}");
}
```

In den Aufgaben steht das Lesen nicht. Sie bekommen die Zeile als `&str`
übergeben, weil ein Test nichts tippen kann und ein Text als Eingabe genauso
verzweigt wie eine getippte Zeile.

### Häufige Fehler

Eine Zahl als Bedingung.

```rust
fn main() {
    let punkte = 61;

    if punkte {
        println!("bestanden");
    }
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> verzweigung.rs:4:8
  |
4 |     if punkte {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`E0308` ist der Fehler, bei dem zwei Typen nicht zusammenpassen, und er wird
noch oft kommen. Hier steht die Antwort in der Meldung selbst: erwartet wurde
`bool`, dagestanden hat eine Zahl. Gemeint war fast immer ein Vergleich, also
`if punkte >= 60`.

Dieselbe Nummer kommt, wenn die beiden Zweige verschiedene Typen liefern, etwa
eine Zahl im einen und ein Text im anderen. Auch dort ist der Grund derselbe:
die Bindung links vom `let` bekommt einen Typ und nicht zwei.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot.

- `larger` gibt die größere von zwei Zahlen zurück
- `grade_of` gibt zu einer Punktzahl "sehr gut", "bestanden" oder
  "nicht bestanden" zurück
- `answer_to` liest eine gelesene Zeile als Antwort und gibt "weiter" oder
  "abbruch" zurück

```console
cd units/01-05-if-und-else
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

A program should not always do the same thing. `if` checks a condition, and the
block behind it runs only when the condition holds. `else` catches the other
case, `else if` puts a further check in between.

Two things about this are particular to Rust. The condition has to be a `bool`,
a number is not enough. And `if` is an expression: it has a value and may
therefore stand on the right of a `let`.

### What it is good for

That the condition has to be a `bool` takes a whole class of fault away. Where a
language reads every number except zero as true, a mistyped comparison passes in
silence. Here it stops, and it stops at compile time.

That `if` has a value saves the step in between. Instead of creating a binding
empty and filling it in both branches, it is created once with the result of the
branch. It can then stay unchangeable as well, which is the more usual case.

### The explanation

`if` on the right of a `let`. Both branches have to deliver the same type,
because the binding gets exactly one type.

```rust
fn main() {
    let punkte = 61;

    // Deutsch: Der Wert der Verzweigung ist der Wert des Zweigs, der läuft.
    let ergebnis = if punkte >= 60 { "bestanden" } else { "nicht bestanden" };

    println!("{ergebnis}");
}
```

The input comes from the keyboard with `read_line` and lands in a `String`. The
line that was read still carries the line break with it, which is why `trim()`
stands in between.

```rust
use std::io::stdin;

fn main() {
    let mut zeile = String::new();
    stdin().read_line(&mut zeile).expect("keine Eingabe gelesen");

    let antwort = if zeile.trim() == "ja" { "weiter" } else { "abbruch" };

    println!("{antwort}");
}
```

The reading does not stand in the exercises. They are handed the line as a
`&str`, because a test cannot type and because a text as input branches exactly
like a typed line.

### Common mistakes

A number as the condition.

```rust
fn main() {
    let punkte = 61;

    if punkte {
        println!("bestanden");
    }
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> verzweigung.rs:4:8
  |
4 |     if punkte {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`E0308` is the error where two types do not fit together, and it will come often
still. Here the answer stands in the message itself: a `bool` was expected and a
number stood there. What was meant is nearly always a comparison, so
`if punkte >= 60`.

The same number comes up when the two branches deliver different types, a number
in one and a text in the other. The reason there is the same: the binding on the
left of the `let` gets one type and not two.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `larger` returns the bigger of two numbers
- `grade_of` returns "sehr gut", "bestanden" or "nicht bestanden" for a score
- `answer_to` reads a line that was read as an answer and returns "weiter" or
  "abbruch"

```console
cd units/01-05-if-und-else
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
