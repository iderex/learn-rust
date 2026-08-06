# 10-08 Die Reference als Nachschlagewerk / The Reference as a lookup

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/10-08-die-reference-als-nachschlagewerk/`. Sie ist öffentlich. Wer
  nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die Stelle der
  Reference nennen, aus der die Antwort kommt.
- Diese Einheit baut auf: `10-01 unsafe`, `10-02 Rohe Zeiger` und
  `10-03 Undefiniertes Verhalten`. Dort stand, was undefiniert ist, hier steht,
  wo man es nachliest.
- Auf dieser Einheit baut auf: alles, was nach der Stufe 10 kommt, denn ab dort
  ist Nachschlagen die übliche Bewegung und nicht mehr die Ausnahme.
- Beim Antworten so zitieren: `10-08 Die Reference als Nachschlagewerk`, dazu
  die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Reference ist keine kürzere Fassung des Buchs. Das Buch führt durch einen
  Stoff, die Reference beantwortet eine Frage. Wer sie von vorn liest, benutzt
  sie falsch herum.
- Eine Antwort aus der Reference nennt die Stelle. Ohne Stelle ist sie geraten,
  auch wenn sie stimmt.
- `as` zwischen Kommazahl und ganzer Zahl sättigt, `as` zwischen zwei ganzen
  Zahlen schneidet ab. Wer beides für dasselbe hält, sagt bitte, an welchem
  Beispiel.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/10-08-die-reference-als-nachschlagewerk/`. It is public. Whoever is
  asked for it may name it, but should first name the place in the Reference the
  answer comes from.
- This unit builds on: `10-01 unsafe`, `10-02 Rohe Zeiger` and
  `10-03 Undefiniertes Verhalten`. What is undefined stood there, where to look
  it up stands here.
- Building on this unit: everything after stage 10, because from there on
  looking something up is the usual movement and no longer the exception.
- Cite like this when answering: `10-08 Die Reference als Nachschlagewerk`, plus
  the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The Reference is not a shorter version of the book. The book leads through a
  subject, the Reference answers a question. Whoever reads it from the front is
  using it the wrong way round.
- An answer from the Reference names the place. Without the place it is guessed,
  even when it is right.
- `as` between a float and an integer saturates, `as` between two integers
  truncates. Whoever holds those to be the same thing, please say on which
  example.

</details>

## Deutsch

### Worum es geht

Die Reference ist die genaue Beschreibung der Sprache. Sie ist nach Bauteilen
geordnet und nicht nach Lernschritten, und ihre Abschnitte sind durchnummeriert:
8.2.4 sind die Operatorausdrücke, 10.3 ist das Speicherbild der Typen, 17.2 ist
die Liste des undefinierten Verhaltens.

Seit einiger Zeit trägt zusätzlich fast jeder einzelne Satz darin eine Marke in
eckigen Klammern, etwa `[expr.as.numeric.float-as-int]`. Diese Marke benennt
genau eine Regel und verschiebt sich nicht, wenn die Kapitel umgeordnet werden.

Das Buch und die Reference beantworten verschiedene Fragen. Das Buch führt durch
einen Stoff und lässt weg, was gerade stören würde. Die Reference lässt nichts
weg und führt durch nichts.

### Wofür das gut ist

Ab der Stufe 10 hört das Raten auf zu funktionieren. Was ein Übersetzer mit
einem Überlauf macht, was `as` an einer Grenze tut, ob ein Zugriff erlaubt ist:
das sind Fragen mit genau einer Antwort, und die steht an genau einer Stelle.

Der Unterschied zu C ist dabei der eigentliche Grund. In C ist der Überlauf
einer vorzeichenbehafteten Zahl undefiniert, und die Umwandlung einer zu großen
Kommazahl in eine ganze Zahl ebenfalls. In Rust ist beides festgelegt. Wer die
C-Gewohnheit mitbringt, hält hier etwas für gefährlich, was es nicht ist, und
übersieht daneben die Liste unter 17.2, in der die wirklichen Fälle stehen.

Eine Antwort ohne Stelle nützt außerdem niemandem weiter. Sie lässt sich nicht
nachprüfen, sie altert still, und beim nächsten Mal wird wieder geraten.

### Die Erklärung

Vier Fragen, vier Antworten, jede mit ihrer Stelle.

```rust
use std::mem::align_of;

// Deutsch: Vier Antworten, die nachgeschlagen und nicht geraten sind. Die
// Reference sagt zu jeder, unter welcher Regel sie steht.
fn main() {
    // 8.2.4 "Operator expressions": zur Null hin gerundet, NaN wird 0, zu
    // grosse Werte saettigen an der Grenze statt umzulaufen.
    println!("{}", 42.9f64 as u8);
    println!("{}", 300.0f64 as u8);
    println!("{}", (-1.5f64) as u8);
    println!("{}", f64::NAN as u8);

    // 8.2.4, dieselbe Stelle und eine andere Regel: zwischen zwei ganzen
    // Zahlen bleiben die unteren Bits stehen, hier wird nichts gesaettigt.
    println!("{}", 300i32 as u8);
    println!("{}", (-1i32) as u8);

    // 8.2.4, Abschnitt "Overflow": im Debug-Bau haelt `+` bei einem Ueberlauf
    // an. `checked_add` fragt danach, statt zu rechnen.
    println!("{:?}", 250u8.checked_add(10));

    // 17.2 "Behavior considered undefined": ein Zugriff ueber einen falsch
    // ausgerichteten Zeiger ist undefiniert. `align_of` sagt, was noetig ist.
    println!("{}", align_of::<u32>());
}
```

Das Programm gibt aus:

```text
42
255
0
0
44
255
None
4
```

Die dritte und die fünfte Zeile stehen nebeneinander, weil sie zusammen die
ganze Regel ergeben. `300.0f64 as u8` ist 255, `300i32 as u8` ist 44. Beides ist
`as`, beides geht nach `u8`, und die Antworten sind verschieden. Wer eine davon
kennt und die andere daraus ableitet, liegt falsch.

### Häufige Fehler

Annehmen, `as` wandle zwischen beliebigen Typen um.

```rust
fn main() {
    let zahl = 3i32;

    let text = zahl as String;

    println!("{text}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0605]: non-primitive cast: `i32` as `String`
 --> nachschlagen.rs:4:16
  |
4 |     let text = zahl as String;
  |                ^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0605`.
```

Die Meldung sagt, wofür `as` da ist, und nennt damit auch die Stelle, an der die
erlaubten Umwandlungen aufgezählt sind. Für alles andere gibt es `From` und
`TryFrom` aus `04-08` und `04-09`. `rustc --explain E0605` ist der kürzeste Weg
von einer Fehlernummer zu ihrer Erklärung und braucht kein Netz.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe ist eine Frage an die Reference, und die
fertige Funktion darüber zeigt, wie eine beantwortete aussieht.

- `truncating_to_u8` wandelt eine ganze Zahl nach `u8`, mit `as`
- `sum_without_panic` addiert zwei `u8` und antwortet, statt anzuhalten
- `is_aligned_for_u32` sagt, ob an einer Adresse ein `u32` liegen dürfte

```console
cd units/10-08-die-reference-als-nachschlagewerk
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 8.2.4 "Operator expressions",
    https://doc.rust-lang.org/reference/expressions/operator-expr.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 17.2 "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

The Reference is the exact description of the language. It is ordered by parts
of the language and not by learning steps, and its sections are numbered: 8.2.4
are the operator expressions, 10.3 is the memory image of types, 17.2 is the
list of undefined behaviour.

For a while now almost every single sentence in it also carries a tag in square
brackets, such as `[expr.as.numeric.float-as-int]`. That tag names exactly one
rule and does not move when the chapters are rearranged.

The book and the Reference answer different questions. The book leads through a
subject and leaves out whatever would get in the way. The Reference leaves
nothing out and leads through nothing.

### What it is good for

From stage 10 on, guessing stops working. What a compiler does with an overflow,
what `as` does at a bound, whether an access is allowed: those are questions with
exactly one answer, and it stands at exactly one place.

The difference to C is the real reason here. In C the overflow of a signed
number is undefined, and so is converting a floating point number that is too
large into an integer. In Rust both are laid down. Whoever brings the habit from
C holds something to be dangerous here that is not, and next to it overlooks the
list under 17.2, where the real cases stand.

An answer without its place is of no further use to anybody either. It cannot be
checked, it goes stale in silence, and next time it is guessed again.

### The explanation

Four questions, four answers, each with its place.

```rust
use std::mem::align_of;

// Deutsch: Vier Antworten, die nachgeschlagen und nicht geraten sind. Die
// Reference sagt zu jeder, unter welcher Regel sie steht.
fn main() {
    // 8.2.4 "Operator expressions": zur Null hin gerundet, NaN wird 0, zu
    // grosse Werte saettigen an der Grenze statt umzulaufen.
    println!("{}", 42.9f64 as u8);
    println!("{}", 300.0f64 as u8);
    println!("{}", (-1.5f64) as u8);
    println!("{}", f64::NAN as u8);

    // 8.2.4, dieselbe Stelle und eine andere Regel: zwischen zwei ganzen
    // Zahlen bleiben die unteren Bits stehen, hier wird nichts gesaettigt.
    println!("{}", 300i32 as u8);
    println!("{}", (-1i32) as u8);

    // 8.2.4, Abschnitt "Overflow": im Debug-Bau haelt `+` bei einem Ueberlauf
    // an. `checked_add` fragt danach, statt zu rechnen.
    println!("{:?}", 250u8.checked_add(10));

    // 17.2 "Behavior considered undefined": ein Zugriff ueber einen falsch
    // ausgerichteten Zeiger ist undefiniert. `align_of` sagt, was noetig ist.
    println!("{}", align_of::<u32>());
}
```

The program prints:

```text
42
255
0
0
44
255
None
4
```

The third and the fifth line stand next to each other because together they make
up the whole rule. `300.0f64 as u8` is 255, `300i32 as u8` is 44. Both are `as`,
both go to `u8`, and the answers differ. Whoever knows one of them and derives
the other from it is wrong.

### Common mistakes

Assuming `as` converts between arbitrary types.

```rust
fn main() {
    let zahl = 3i32;

    let text = zahl as String;

    println!("{text}");
}
```

The compiler answers:

```text
error[E0605]: non-primitive cast: `i32` as `String`
 --> nachschlagen.rs:4:16
  |
4 |     let text = zahl as String;
  |                ^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0605`.
```

The message says what `as` is there for, and with that it also names the place
where the allowed conversions are listed. For everything else there are `From`
and `TryFrom` from `04-08` and `04-09`. `rustc --explain E0605` is the shortest
way from an error number to its explanation and needs no network.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise is a question put to the
Reference, and the finished function above them shows what an answered one looks
like.

- `truncating_to_u8` converts an integer to `u8`, with `as`
- `sum_without_panic` adds two `u8` and answers instead of stopping
- `is_aligned_for_u32` says whether a `u32` would be allowed at an address

```console
cd units/10-08-die-reference-als-nachschlagewerk
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    checked against 1.97.1

    The Rust Reference, chapter 8.2.4 "Operator expressions",
    https://doc.rust-lang.org/reference/expressions/operator-expr.html,
    checked against 1.97.1

    The Rust Reference, chapter 17.2 "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
