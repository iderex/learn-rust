# 01-02 Zahlen und andere einfache Typen / Numbers and other simple types

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/01-02-zahlen-und-einfache-typen/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `01-01 Variablen und Veränderbarkeit`.
- Auf dieser Einheit bauen auf: `01-03 Funktionen` und alles, was rechnet.
- Beim Antworten so zitieren: `01-02 Zahlen und andere einfache Typen`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Überlauf beim Rechnen ist hier ausdrücklich nicht das Thema, nur der
  Bereich der Typen. Wer danach gefragt wird, sagt, dass es später kommt.

</details>

## Deutsch

### Worum es geht

Eine Zahl in Rust hat immer einen Typ, und der Typ sagt zwei Dinge: wie viele
Bytes sie belegt und welche Werte hineinpassen.

Ganze Zahlen ohne Vorzeichen heißen `u8`, `u16`, `u32`, `u64`. Die Zahl im Namen
ist die Anzahl der Bits. Mit Vorzeichen heißen sie `i8` bis `i64`.
Fließkommazahlen sind `f32` und `f64`. Dazu kommen `bool` mit den beiden Werten
`true` und `false` und `char` für ein einzelnes Zeichen.

### Wofür das gut ist

Der Bereich ist keine Formalie. Ein `u8` reicht von 0 bis 255, und was darüber
liegt, passt nicht hinein. Wer das nicht weiß, schreibt irgendwann eine Zahl in
eine Schublade, die zu klein ist.

Rust rechnet außerdem nie stillschweigend von einem Typ in den anderen um. Wer
ein `u8` als `u32` braucht, schreibt die Umwandlung hin. Das ist eine Zeile mehr
und dafür keine Überraschung an einer Stelle, an der niemand hinsieht.

### Die Erklärung

Die Typen nebeneinander, jeder mit einem Wert darin.

```rust
fn main() {
    let ohne_vorzeichen: u8 = 255;
    let mit_vorzeichen: i32 = -7;
    let komma: f64 = 2.5;
    let wahr: bool = true;
    let zeichen: char = 'a';

    let breiter: u32 = u32::from(ohne_vorzeichen);

    println!("{ohne_vorzeichen} {mit_vorzeichen} {komma} {wahr} {zeichen} {breiter}");
}
```

Drei Dinge stehen darin. Erstens der Bereich: `u8` geht von 0 bis 255, weil acht
Bits genau 256 verschiedene Werte tragen. Zweitens `char`, der in einfachen
Anführungszeichen steht, während Text in doppelten steht. Drittens die
Umwandlung `u32::from`, die den Wert von einem kleinen in einen größeren Typ
trägt. In diese Richtung geht sie immer, denn alles aus `u8` passt in `u32`.

### Häufige Fehler

Eine Zahl mit Komma in eine ganze Zahl schreiben.

```rust
fn main() {
    let ganz: i32 = 2.5;
    println!("{ganz}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> komma.rs:2:21
  |
2 |     let ganz: i32 = 2.5;
  |               ---   ^^^ expected `i32`, found floating-point number
  |               |
  |               expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Es ist dieselbe Nummer wie in `00-01`, `E0308`, und derselbe Aufbau: erwartet
`i32`, gefunden eine Fließkommazahl. Die richtige Antwort hängt davon ab, was
gemeint war. Soll die Zahl 2.5 bleiben, dann muss der Typ `f64` heißen. Soll es
eine ganze Zahl sein, dann gehört dort 2 hin, und der Verlust der Nachkommastelle
ist dann eine Entscheidung und kein Versehen.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.

- `fits_in_u8` gibt zurück, ob ein `u32` in ein `u8` passt
- `widened` gibt ein `u8` als `u32` zurück
- `half` gibt die Hälfte einer `f64` zurück
- `is_hex_letter` gibt zurück, ob ein `char` einer der Buchstaben a bis f einer Hexadezimalziffer ist

Bei der letzten Aufgabe reicht ein Vergleich mit `>=` und `<=`. Die Lösung
schreibt stattdessen `('a'..='f').contains(&zeichen)`, weil `cargo clippy` genau
diese Form vorschlägt. Was clippy ist, kommt in `01-06`; hier steht nur, warum
die Lösung anders aussieht als der offensichtliche Weg.

```console
cd units/01-02-zahlen-und-einfache-typen
cargo test
```

### Quelle

    Buch, Kapitel 3 "Common Programming Concepts", Abschnitt 3.2 "Data Types",
    https://doc.rust-lang.org/book/ch03-02-data-types.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A number in Rust always has a type, and the type says two things: how many bytes
it takes and which values fit into it.

Whole numbers without a sign are called `u8`, `u16`, `u32`, `u64`. The number in
the name is the count of bits. With a sign they are called `i8` to `i64`.
Floating point numbers are `f32` and `f64`. Next to them stand `bool` with the
two values `true` and `false` and `char` for a single character.

### What it is good for

The range is not a formality. A `u8` runs from 0 to 255, and whatever lies above
that does not fit into it. Whoever does not know that will at some point write a
number into a drawer that is too small.

Rust also never converts silently from one type into another. Whoever needs a
`u8` as a `u32` writes the conversion out. That is one line more and in exchange
no surprise at a place nobody is looking at.

### The explanation

The types next to each other, each with a value in it.

```rust
fn main() {
    let ohne_vorzeichen: u8 = 255;
    let mit_vorzeichen: i32 = -7;
    let komma: f64 = 2.5;
    let wahr: bool = true;
    let zeichen: char = 'a';

    let breiter: u32 = u32::from(ohne_vorzeichen);

    println!("{ohne_vorzeichen} {mit_vorzeichen} {komma} {wahr} {zeichen} {breiter}");
}
```

Three things are in there. First the range: `u8` runs from 0 to 255, because
eight bits carry exactly 256 different values. Second `char`, which stands in
single quotation marks while text stands in double ones. Third the conversion
`u32::from`, carrying the value from a small type into a larger one. In that
direction it always works, because everything from `u8` fits into `u32`.

### Common mistakes

Writing a number with a decimal point into a whole number.

```rust
fn main() {
    let ganz: i32 = 2.5;
    println!("{ganz}");
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> komma.rs:2:21
  |
2 |     let ganz: i32 = 2.5;
  |               ---   ^^^ expected `i32`, found floating-point number
  |               |
  |               expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

It is the same number as in `00-01`, `E0308`, and the same shape: expected
`i32`, found a floating point number. The right answer depends on what was
meant. If the number is to stay 2.5, then the type has to be `f64`. If it is
meant to be a whole number, then 2 belongs there, and losing the decimal place is
then a decision and not an oversight.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `fits_in_u8` returns whether a `u32` fits into a `u8`
- `widened` returns a `u8` as a `u32`
- `half` returns half of an `f64`
- `is_hex_letter` returns whether a `char` is one of the letters a to f of a hexadecimal digit

For the last exercise a comparison with `>=` and `<=` is enough. The solution
writes `('a'..='f').contains(&zeichen)` instead, because `cargo clippy` proposes
exactly that form. What clippy is comes in `01-06`; what stands here is only why
the solution looks different from the obvious way.

```console
cd units/01-02-zahlen-und-einfache-typen
cargo test
```

### Source

    Book, chapter 3 "Common Programming Concepts", section 3.2 "Data Types",
    https://doc.rust-lang.org/book/ch03-02-data-types.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
