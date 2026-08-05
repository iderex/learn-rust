# 03-06 derive mit Debug / derive with Debug

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-06-derive-mit-debug/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-01 struct` und `03-03 enum`.
- Auf dieser Einheit bauen auf: `03-07 Display selbst schreiben`, und jede
  spätere Einheit, deren Tests einen eigenen Typ vergleichen.
- Beim Antworten so zitieren: `03-06 derive mit Debug`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Was `derive` erzeugt, ist gewöhnlicher Code und keine Sondereigenschaft des
  Typs. Wer es als Zauber beschreibt, verliert die Verbindung zu `03-07`, wo
  dieselbe Sache von Hand geschrieben wird.
- `Debug` ist die Ausgabe für die Fehlersuche und nicht die für Menschen. Der
  Unterschied ist die Aussage von `03-07`; wer ihn hier schon einebnet, nimmt
  die Einheit vorweg.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-06-derive-mit-debug/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `03-01 struct` and `03-03 enum`.
- Building on this unit: `03-07 Display selbst schreiben`, and every later unit
  whose tests compare a type of their own.
- Cite like this when answering: `03-06 derive mit Debug`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- What `derive` produces is ordinary code and not a special property of the
  type. Whoever describes it as magic loses the connection to `03-07`, where the
  same thing is written by hand.
- `Debug` is the output for fault finding and not the one for people. The
  difference is the point of `03-07`; whoever levels it here takes that unit in
  advance.

</details>

## Deutsch

### Worum es geht

`#[derive(...)]` steht über einem Typ und lässt den Übersetzer eine
Implementierung schreiben, die man sonst von Hand hinschreiben müsste. Was dabei
entsteht, ist gewöhnlicher Code.

`Debug` ist die erste, die man braucht. Sie gehört zu `{:?}` in `println!` und
zu `{:#?}`, das eine Zeile je Feld ausgibt. Ohne sie lässt sich ein eigener Typ
gar nicht ansehen.

Daneben stehen `PartialEq` für `==`, `Clone` für eine ausdrückliche Kopie und
`Copy` für die Kopie bei der Zuweisung. Ein Typ bekommt `Copy` nur, wenn jeder
seiner Teile es hat.

### Wofür das gut ist

Ohne `Debug` steht in einem fehlgeschlagenen Test nur, dass etwas nicht
gestimmt hat, und nicht was. `assert_eq!` gibt beide Seiten mit `{:?}` aus, und
genau deshalb verlangt es `Debug` von dem Typ, den es vergleicht.

Ohne `PartialEq` gibt es kein `==`, und damit auch kein `assert_eq!` über einen
eigenen Typ. Genau das war der Grund, warum die Tests in `03-01` Feld für Feld
gelesen haben.

Und `derive` ist billiger als die Handarbeit, weil es mit dem Typ mitwächst. Ein
neues Feld steht sofort in der Ausgabe und im Vergleich, ohne dass jemand daran
denken muss.

### Die Erklärung

Vier abgeleitete Implementierungen an einem `struct` und eine an einem `enum`.

```rust
// Deutsch: `derive` schreibt die Implementierung, die man sonst von Hand
// hinschreiben müsste. `Debug` gehört zu `{:?}`.
#[derive(Debug, PartialEq, Clone, Copy)]
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

#[derive(Debug, PartialEq)]
enum Reading {
    Missing,
    Temperature(i32),
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    // Deutsch: `{:?}` gibt eine Zeile aus, `{:#?}` eine Zeile je Feld.
    println!("{rechteck:?}");
    println!("{rechteck:#?}");

    // Deutsch: `Copy` und `Clone` machen aus der Zuweisung wieder eine Kopie,
    // und `PartialEq` erlaubt den Vergleich mit `==`.
    let zweites = rechteck;

    println!("{}", rechteck == zweites);

    println!("{:?} {:?}", Reading::Missing, Reading::Temperature(17));
}
```

Die beiden Ausgaben des Rechtecks sehen so aus:

```text
Rectangle { breite: 3, hoehe: 4 }
Rectangle {
    breite: 3,
    hoehe: 4,
}
```

Der Name des Typs steht mit darin, und die Felder stehen mit ihren Namen da. Das
ist eine Ausgabe für die Fehlersuche; eine für Menschen schreibt `03-07`.

### Häufige Fehler

Einen Typ ohne `Debug` mit `{:?}` ausgeben.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    println!("{rechteck:?}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> ohne.rs:12:15
   |
12 |     println!("{rechteck:?}");
   |               ^^^^^^^^^^^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
help: consider annotating `Rectangle` with `#[derive(Debug)]`
   |
 1 + #[derive(Debug)]
 2 | struct Rectangle {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Die Meldung nennt beide Wege: `derive` oder von Hand. Dass beide dastehen, ist
die Aussage dieser Einheit, denn `derive` nimmt einem die Handarbeit ab und
nicht die Möglichkeit dazu.

`E0277` heißt, dass ein Typ etwas nicht kann, was hier verlangt wird. Die
Meldung wird ab Stufe 5 oft kommen, denn dort heißt dieses "etwas" Trait.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Rectangle` und `Reading` tragen ihre `derive`-Zeilen schon.

- `debug_line` gibt den Typ als eine Zeile zurück
- `debug_block` gibt ihn mit einer Zeile je Feld zurück
- `same` vergleicht zwei Rechtecke

```console
cd units/03-06-derive-mit-debug
cargo test
```

### Quelle

    Buch, Anhang C "Derivable Traits",
    https://doc.rust-lang.org/book/appendix-03-derivable-traits.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 5 "Using Structs to Structure Related Data", Abschnitt 5.2 "An Example Program Using Structs",
    https://doc.rust-lang.org/book/ch05-02-example-structs.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`#[derive(...)]` stands above a type and lets the compiler write an
implementation that would otherwise have to be written by hand. What comes out
of it is ordinary code.

`Debug` is the first one needed. It belongs to `{:?}` in `println!` and to
`{:#?}`, which prints one line per field. Without it a type of your own cannot
be looked at at all.

Beside it stand `PartialEq` for `==`, `Clone` for an explicit copy and `Copy`
for the copy on assignment. A type only gets `Copy` if every one of its parts
has it.

### What it is good for

Without `Debug` a failing test says only that something was wrong and not what.
`assert_eq!` prints both sides with `{:?}`, and that is exactly why it demands
`Debug` of the type it compares.

Without `PartialEq` there is no `==`, and therefore no `assert_eq!` over a type
of your own. That was precisely the reason why the tests in `03-01` read field
by field.

And `derive` is cheaper than the handwork because it grows with the type. A new
field stands in the output and in the comparison at once, without anybody having
to think of it.

### The explanation

Four derived implementations on a `struct` and one on an `enum`.

```rust
// Deutsch: `derive` schreibt die Implementierung, die man sonst von Hand
// hinschreiben müsste. `Debug` gehört zu `{:?}`.
#[derive(Debug, PartialEq, Clone, Copy)]
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

#[derive(Debug, PartialEq)]
enum Reading {
    Missing,
    Temperature(i32),
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    // Deutsch: `{:?}` gibt eine Zeile aus, `{:#?}` eine Zeile je Feld.
    println!("{rechteck:?}");
    println!("{rechteck:#?}");

    // Deutsch: `Copy` und `Clone` machen aus der Zuweisung wieder eine Kopie,
    // und `PartialEq` erlaubt den Vergleich mit `==`.
    let zweites = rechteck;

    println!("{}", rechteck == zweites);

    println!("{:?} {:?}", Reading::Missing, Reading::Temperature(17));
}
```

The two outputs of the rectangle look like this:

```text
Rectangle { breite: 3, hoehe: 4 }
Rectangle {
    breite: 3,
    hoehe: 4,
}
```

The name of the type stands in it, and the fields stand there with their names.
That is an output for fault finding; one for people is written in `03-07`.

### Common mistakes

Printing a type without `Debug` using `{:?}`.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    println!("{rechteck:?}");
}
```

The compiler answers:

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
  --> ohne.rs:12:15
   |
12 |     println!("{rechteck:?}");
   |               ^^^^^^^^^^^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
help: consider annotating `Rectangle` with `#[derive(Debug)]`
   |
 1 + #[derive(Debug)]
 2 | struct Rectangle {
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

The message names both ways: `derive` or by hand. That both stand there is the
point of this unit, because `derive` takes the handwork away and not the
possibility of it.

`E0277` means a type cannot do something that is asked of it here. The message
will come often from stage 5 on, because there that "something" is called a
trait.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Rectangle` and `Reading` already carry their
`derive` lines.

- `debug_line` returns the type as one line
- `debug_block` returns it with one line per field
- `same` compares two rectangles

```console
cd units/03-06-derive-mit-debug
cargo test
```

### Source

    Book, appendix C "Derivable Traits",
    https://doc.rust-lang.org/book/appendix-03-derivable-traits.html,
    checked against 1.97.1

    Book, chapter 5 "Using Structs to Structure Related Data", section 5.2 "An Example Program Using Structs",
    https://doc.rust-lang.org/book/ch05-02-example-structs.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
