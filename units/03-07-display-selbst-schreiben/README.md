# 03-07 Display selbst schreiben / Writing Display by hand

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/03-07-display-selbst-schreiben/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `03-04 match`, `03-05 Option und if let` und
  `03-06 derive mit Debug`. Mit ihr ist die Stufe 3 zu Ende.
- Auf dieser Einheit bauen auf: `05-02 Traits`, wo erklärt wird, was ein Trait
  ist, und `04-10 std::error::Error und Box<dyn Error>`, wo ein Fehlertyp beides
  braucht.
- Beim Antworten so zitieren: `03-07 Display selbst schreiben`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Hier wird eine vorgegebene Form ausgefüllt. Was ein Trait ist, steht in
  `05-02`, und wer es hier erklärt, nimmt die Einheit vorweg.
- `Display` ist für Menschen, `Debug` für die Fehlersuche. Wer beide dasselbe
  ausgeben lässt, hat die Einheit verfehlt, und der Text sagt warum.
- `unwrap` kommt in dieser Einheit nicht vor. Der fehlende Messwert ist eine
  Variante und wird als Fall behandelt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-07-display-selbst-schreiben/`.
  It is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `03-04 match`, `03-05 Option und if let` and
  `03-06 derive mit Debug`. With it stage 3 ends.
- Building on this unit: `05-02 Traits`, where what a trait is gets explained,
  and `04-10 std::error::Error und Box<dyn Error>`, where an error type needs
  both.
- Cite like this when answering: `03-07 Display selbst schreiben`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- What happens here is filling in a given shape. What a trait is stands in
  `05-02`, and whoever explains it here takes that unit in advance.
- `Display` is for people, `Debug` is for fault finding. Whoever lets both print
  the same thing has missed the unit, and the text says why.
- `unwrap` does not appear in this unit. The missing reading is a variant and is
  treated as a case.

</details>

## Deutsch

### Worum es geht

`{:?}` kam mit `derive` von selbst. `{}` kommt nicht von selbst: dahinter steht
`Display`, und die schreibt man von Hand.

Die Form ist immer dieselbe. Man schreibt einen `impl`-Block für seinen Typ und
darin eine Funktion `fmt`, die einen Schreibplatz bekommt und mit `write!`
hineinschreibt. Zurück kommt `fmt::Result`, und das Ergebnis von `write!` ist
schon dieses Result.

Warum es zwei Ausgaben gibt, ist der Punkt der Einheit. `Debug` zeigt den Aufbau
mit Feldnamen und Typnamen, `Display` zeigt den Satz, den ein Mensch lesen soll.

### Wofür das gut ist

Die beiden Leser sind verschieden. Wer einen Fehler sucht, will wissen, welche
Variante mit welchen Feldern dasteht. Wer eine Ausgabe liest, will "Flur: 17
Grad" und nicht `Sensor { name: "Flur", wert: Temperature(17) }`.

Weil beide getrennt sind, kann man die eine ändern, ohne die andere anzufassen.
`Debug` wächst mit dem Typ mit, weil `derive` sie schreibt, und `Display` bleibt
so, wie sie für den Leser gedacht war.

`Display` zieht außerdem etwas nach sich: ein Typ, der sie hat, bekommt
`to_string` geschenkt. Umgekehrt gibt es keinen `derive(Display)`, und das ist
Absicht, denn der Übersetzer weiß nicht, welcher Satz gemeint ist.

### Die Erklärung

Ein Typ mit beiden Ausgaben.

```rust
use std::fmt;

#[derive(Debug)]
struct Percent(u8);

// Deutsch: `Display` wird von Hand geschrieben. `fmt` bekommt einen
// Schreibplatz und schreibt mit `write!` hinein.
impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} %", self.0)
    }
}

fn main() {
    let anteil = Percent(42);

    // Deutsch: `{}` nimmt `Display`, `{:?}` nimmt `Debug`. Zwei Ausgaben für
    // zwei Leser.
    println!("{anteil}");
    println!("{anteil:?}");
}
```

Das Programm gibt aus:

```text
42 %
Percent(42)
```

Zwei Zeilen aus einem Wert, und keine der beiden ist die falsche. Sie haben
verschiedene Leser.

### Häufige Fehler

Einen Typ mit `{}` ausgeben, der nur `Debug` hat.

```rust
#[derive(Debug)]
struct Sensor {
    name: String,
}

fn main() {
    let sensor = Sensor {
        name: String::from("Flur"),
    };

    println!("{sensor}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `Sensor` doesn't implement `std::fmt::Display`
  --> ohnedisplay.rs:11:15
   |
11 |     println!("{sensor}");
   |               ^^^^^^^^ `Sensor` cannot be formatted with the default formatter
   |
help: the trait `std::fmt::Display` is not implemented for `Sensor`
  --> ohnedisplay.rs:2:1
   |
 2 | struct Sensor {
   | ^^^^^^^^^^^^^
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Es ist dieselbe Nummer wie in `03-06`, denn es ist derselbe Fall: dem Typ fehlt
etwas, das hier verlangt wird. Der Hinweis am Ende schlägt `{:?}` vor, und das
ist der bequeme Ausweg. Er ist richtig, solange die Ausgabe für die Fehlersuche
gedacht ist, und falsch, sobald ein Mensch sie lesen soll.

Anders als bei `Debug` gibt es hier kein `derive`. Die Meldung schlägt auch
keines vor, denn es gibt keines.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Das Modell aus `Sensor` und `Reading` steht schon da, mit
`Debug` abgeleitet, und `Percent` zeigt die Form einmal ausgefüllt.

- `Display` für `Reading`, mit einem Fall je Variante und ohne `unwrap`
- `Display` für `Sensor`, die die Ausgabe von `Reading` mitbenutzt
- `for_people` gibt die Ausgabe für Menschen als `String` zurück

```console
cd units/03-07-display-selbst-schreiben
cargo test
```

### Quelle

    Buch, Kapitel 10 "Generic Types, Traits, and Lifetimes", Abschnitt 10.2 "Defining Shared Behavior with Traits",
    https://doc.rust-lang.org/book/ch10-02-traits.html,
    geprüft gegen 1.97.1

    Standardbibliothek, "Display in std::fmt",
    https://doc.rust-lang.org/std/fmt/trait.Display.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`{:?}` came by itself with `derive`. `{}` does not come by itself: behind it
stands `Display`, and that one is written by hand.

The shape is always the same. You write an `impl` block for your type and inside
it a function `fmt`, which gets a place to write and writes into it with
`write!`. What comes back is `fmt::Result`, and the result of `write!` is
already that result.

Why there are two outputs is the point of the unit. `Debug` shows the build with
field names and type names, `Display` shows the sentence a person is meant to
read.

### What it is good for

The two readers are different. Whoever is looking for a fault wants to know
which variant with which fields stands there. Whoever reads an output wants
"Flur: 17 Grad" and not `Sensor { name: "Flur", wert: Temperature(17) }`.

Because the two are separate, one can be changed without touching the other.
`Debug` grows with the type because `derive` writes it, and `Display` stays the
way it was meant for the reader.

`Display` also brings something with it: a type that has it gets `to_string` for
free. The other way round there is no `derive(Display)`, and that is deliberate,
because the compiler does not know which sentence is meant.

### The explanation

One type with both outputs.

```rust
use std::fmt;

#[derive(Debug)]
struct Percent(u8);

// Deutsch: `Display` wird von Hand geschrieben. `fmt` bekommt einen
// Schreibplatz und schreibt mit `write!` hinein.
impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} %", self.0)
    }
}

fn main() {
    let anteil = Percent(42);

    // Deutsch: `{}` nimmt `Display`, `{:?}` nimmt `Debug`. Zwei Ausgaben für
    // zwei Leser.
    println!("{anteil}");
    println!("{anteil:?}");
}
```

The program prints:

```text
42 %
Percent(42)
```

Two lines out of one value, and neither of them is the wrong one. They have
different readers.

### Common mistakes

Printing a type with `{}` that only has `Debug`.

```rust
#[derive(Debug)]
struct Sensor {
    name: String,
}

fn main() {
    let sensor = Sensor {
        name: String::from("Flur"),
    };

    println!("{sensor}");
}
```

The compiler answers:

```text
error[E0277]: `Sensor` doesn't implement `std::fmt::Display`
  --> ohnedisplay.rs:11:15
   |
11 |     println!("{sensor}");
   |               ^^^^^^^^ `Sensor` cannot be formatted with the default formatter
   |
help: the trait `std::fmt::Display` is not implemented for `Sensor`
  --> ohnedisplay.rs:2:1
   |
 2 | struct Sensor {
   | ^^^^^^^^^^^^^
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

It is the same number as in `03-06`, because it is the same case: the type is
missing something that is asked of it here. The note at the end suggests `{:?}`,
and that is the convenient way out. It is right as long as the output is meant
for fault finding, and wrong as soon as a person is to read it.

Unlike with `Debug` there is no `derive` here. The message does not suggest one
either, because there is none.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The model of `Sensor` and `Reading` already
stands there with `Debug` derived, and `Percent` shows the shape filled in once.

- `Display` for `Reading`, with one case per variant and without `unwrap`
- `Display` for `Sensor`, using the output of `Reading` inside it
- `for_people` returns the output for people as a `String`

```console
cd units/03-07-display-selbst-schreiben
cargo test
```

### Source

    Book, chapter 10 "Generic Types, Traits, and Lifetimes", section 10.2 "Defining Shared Behavior with Traits",
    https://doc.rust-lang.org/book/ch10-02-traits.html,
    checked against 1.97.1

    Standard library, "Display in std::fmt",
    https://doc.rust-lang.org/std/fmt/trait.Display.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
