# 01-01 Variablen und Veränderbarkeit / Variables and mutability

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/01-01-variablen-und-veraenderbarkeit/`. Sie ist öffentlich. Wer
  nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: die Stufe 0, also `00-01` bis `00-03`.
- Auf dieser Einheit bauen auf: die weiteren Einheiten der Stufe 1 und später
  `02-01 Verschieben`, denn Eigentum wird an Bindungen erklärt.
- Beim Antworten so zitieren: `01-01 Variablen und Veränderbarkeit`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Aufgabe 1 verlangt ausdrücklich eine veränderbare Bindung und zwei Schritte.
  `start + 2` besteht den Test und verfehlt die Aufgabe.

</details>

## Deutsch

### Worum es geht

`let` bindet einen Wert an einen Namen. In Rust ist diese Bindung standardmäßig
nicht veränderbar: was einmal darunter steht, bleibt darunter. Wer den Wert
später ändern will, schreibt `let mut`.

Daneben stehen zwei weitere Dinge. Eine Konstante mit `const` ist nie
veränderbar und braucht immer ihren Typ. Und das Beschatten legt mit einem
zweiten `let` einen neuen Wert unter denselben Namen, auch mit einem anderen
Typ.

### Wofür das gut ist

Die meisten Werte in einem Programm ändern sich nie, und man merkt es nur, wenn
die Sprache den Unterschied sichtbar macht. Steht `mut` nicht da, dann weiß
jeder Leser sofort: das hier bleibt, wie es ist. Steht es da, dann ist es ein
Hinweis, an dieser Stelle genauer hinzusehen.

Der Nutzen zeigt sich beim Lesen fremden Codes und noch mehr, sobald mehrere
Teile eines Programms auf dieselben Daten zeigen. Diese Regel ist der Anfang
dessen, was in Stufe 2 als Eigentum wiederkommt.

### Die Erklärung

Drei Formen nebeneinander. Die erste bleibt, wie sie ist. Die zweite darf sich
ändern. Die dritte legt einen neuen Wert unter den alten Namen.

```rust
fn main() {
    let fest = 3;

    let mut zaehler = 0;
    zaehler += 1;

    let text = "hallo";
    let text = text.len();

    println!("{fest} {zaehler} {text}");
}
```

Beim Beschatten entsteht wirklich ein neuer Wert. Der alte ist danach nicht mehr
erreichbar, und der neue darf einen anderen Typ haben: aus dem Text `"hallo"`
wird die Zahl `5`. Das ist etwas anderes als `mut`, denn mit `mut` bliebe der
Typ derselbe.

Eine Konstante sieht so aus und steht meist ganz oben, außerhalb jeder Funktion.

```rust
const MAX_ATTEMPTS: u32 = 3;
```

### Häufige Fehler

Der Klassiker ist die zweite Zuweisung an eine Bindung ohne `mut`.

```rust
fn main() {
    let zahl = 3;
    zahl = 4;
    println!("{zahl}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0384]: cannot assign twice to immutable variable `zahl`
 --> unveraenderlich.rs:3:5
  |
2 |     let zahl = 3;
  |         ---- first assignment to `zahl`
3 |     zahl = 4;
  |     ^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut zahl = 3;
  |         +++
```

Die Meldung zeigt beide Stellen: die erste Zuweisung und die zweite, die
abgelehnt wird. Der Hinweis am Ende schlägt `let mut zahl` vor, und das ist eine
von zwei richtigen Antworten. Die andere ist ein zweites `let`, also ein
Beschatten, wenn der alte Wert gar nicht weiterleben soll.

Welche der beiden passt, entscheidet die Aufgabe und nicht der Hinweis. `mut`
heißt: derselbe Wert ändert sich. Beschatten heißt: hier beginnt ein neuer Wert.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.

- `twice_incremented` zählt `start` in zwei Schritten um eins hoch, mit einer
  veränderbaren Bindung
- `quoted_length` gibt die Länge von `text` mit einem Anführungszeichen auf
  jeder Seite zurück, indem der Name `text` beschattet wird

```console
cd units/01-01-variablen-und-veraenderbarkeit
cargo test
```

### Quelle

    Buch, Kapitel 3 "Common Programming Concepts", Abschnitt 3.1 "Variables and
    Mutability",
    https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`let` binds a value to a name. In Rust that binding is not mutable by default:
what stands under it once stays under it. Whoever wants to change the value
later writes `let mut`.

Two further things stand next to it. A constant with `const` is never mutable
and always needs its type. And shadowing puts a new value under the same name
with a second `let`, with a different type as well.

### What it is good for

Most values in a program never change, and you only notice that when the
language makes the difference visible. If `mut` is not there, every reader knows
at once: this stays as it is. If it is there, it is a hint to look more closely
at that place.

The use shows when reading somebody else's code, and more so as soon as several
parts of a program point at the same data. This rule is the beginning of what
comes back in stage 2 as ownership.

### The explanation

Three forms next to each other. The first stays as it is. The second may change.
The third puts a new value under the old name.

```rust
fn main() {
    let fest = 3;

    let mut zaehler = 0;
    zaehler += 1;

    let text = "hallo";
    let text = text.len();

    println!("{fest} {zaehler} {text}");
}
```

Shadowing really does make a new value. The old one cannot be reached
afterwards, and the new one may have a different type: the text `"hallo"` turns
into the number `5`. That is something other than `mut`, because with `mut` the
type would stay the same.

A constant looks like this and usually stands at the very top, outside every
function.

```rust
const MAX_ATTEMPTS: u32 = 3;
```

### Common mistakes

The classic one is the second assignment to a binding without `mut`.

```rust
fn main() {
    let zahl = 3;
    zahl = 4;
    println!("{zahl}");
}
```

The compiler answers:

```text
error[E0384]: cannot assign twice to immutable variable `zahl`
 --> unveraenderlich.rs:3:5
  |
2 |     let zahl = 3;
  |         ---- first assignment to `zahl`
3 |     zahl = 4;
  |     ^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut zahl = 3;
  |         +++
```

The message shows both places: the first assignment and the second one, which is
refused. The note at the end proposes `let mut zahl`, and that is one of two
right answers. The other is a second `let`, meaning a shadowing, when the old
value is not meant to live on at all.

Which of the two fits is decided by the task and not by the note. `mut` means:
the same value changes. Shadowing means: a new value begins here.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `twice_incremented` counts `start` up by one in two steps, with a mutable
  binding
- `quoted_length` returns the length of `text` with a quotation mark on each
  side, by shadowing the name `text`

```console
cd units/01-01-variablen-und-veraenderbarkeit
cargo test
```

### Source

    Book, chapter 3 "Common Programming Concepts", section 3.1 "Variables and
    Mutability",
    https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
