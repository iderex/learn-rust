# 02-03 Ausleihen / Borrowing

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/02-03-ausleihen/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-01 Verschieben / Move` und `02-02 Stack und Heap`.
- Auf dieser Einheit bauen auf: `02-04 Veränderbares Ausleihen` und
  `02-05 Slices`.
- Beim Antworten so zitieren: `02-03 Ausleihen`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Hier stehen nur geteilte Referenzen. `&mut` ist `02-04` und wird hier nur
  genannt, nicht erklärt.
- Die Parameter heißen `&String` und nicht `&str`, weil der Lernende an dieser
  Stelle einen `String` in der Hand hat. clippy schlägt `&str` vor, der Text
  sagt das, und Slices stehen in `02-05`. Wer die Aufgaben auf `&str` umschreibt,
  nimmt die spätere Einheit vorweg und macht die eingebundene Testdatei kaputt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/02-03-ausleihen/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `02-01 Verschieben / Move` and `02-02 Stack und Heap`.
- Building on this unit: `02-04 Veränderbares Ausleihen` and `02-05 Slices`.
- Cite like this when answering: `02-03 Ausleihen`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Only shared references stand here. `&mut` is `02-04` and is named here rather
  than explained.
- The parameters are `&String` and not `&str`, because at this point the learner
  holds a `String`. clippy suggests `&str`, the text says so, and slices stand in
  `02-05`. Whoever rewrites the exercises to `&str` takes the later unit in
  advance and breaks the included test file.

</details>

## Deutsch

### Worum es geht

Eine Referenz ist eine Ausleihe. `&text` gibt einer Funktion Zugriff auf einen
Wert, ohne ihr das Eigentum zu geben. Die Funktion darf lesen, sie darf nichts
ändern, und wenn sie fertig ist, hat der Aufrufer seinen Wert unverändert weiter.

`*` ist der Weg zurück: es liest den Wert hinter der Referenz. Bei Methoden
braucht man es meistens nicht, weil der Aufruf mit dem Punkt die Referenz von
selbst auflöst.

Eine Ausleihe gilt bis zu ihrer letzten Benutzung und nicht bis zum Ende des
Blocks. Danach ist der Wert wieder frei.

### Wofür das gut ist

Ohne Ausleihen müsste jede Funktion, die etwas nur ansieht, den Wert nehmen und
wieder zurückgeben. Genau das ist in `02-01` passiert, und die Rückgabe war
reine Buchführung.

Die zweite Möglichkeit wäre `clone`. Die kostet eine zweite Ablage auf dem Heap,
jedes Mal, nur damit jemand die Länge zählen darf.

Geteilte Referenzen sind außerdem beliebig oft gleichzeitig erlaubt. Zwei
Stellen dürfen denselben Wert zur selben Zeit lesen, solange keine ihn ändert.
Was passiert, wenn doch jemand ändern will, steht in `02-04`.

### Die Erklärung

Ausleihen, lesen, und der Aufrufer behält.

```rust
fn laenge_von(text: &String) -> usize {
    // Deutsch: `text` ist eine geteilte Referenz. Sie darf lesen und nicht
    // ändern, und der Wert gehört weiter dem Aufrufer.
    text.len()
}

fn main() {
    let text = String::from("hallo");

    println!("{}", laenge_von(&text));

    // Deutsch: `text` steht noch da, denn übergeben wurde nur eine Ausleihe.
    println!("{text}");

    let zahl = 21;
    let geliehen = &zahl;

    // Deutsch: `*` liest den Wert hinter der Referenz.
    println!("{}", *geliehen * 2);
}
```

clippy meldet zu `&String` als Parameter den Hinweis `clippy::ptr_arg` und
schlägt `&str` vor. Der Vorschlag ist richtig, und ein `&str` ist ein Slice; die
stehen in `02-05`. Bis dahin bleibt die Form `&String`, weil der Lernende hier
einen `String` in der Hand hat, und die Lösung trägt dafür ein ausdrückliches
`#[allow(clippy::ptr_arg)]` mit dieser Begründung.

Die Ausleihe endet bei ihrer letzten Benutzung. Deshalb darf der Wert danach
sogar verschoben werden.

```rust
fn main() {
    let text = String::from("hallo");

    let geliehen = &text;
    println!("{}", geliehen.len());

    // Deutsch: Nach der letzten Benutzung der Ausleihe ist der Wert wieder
    // frei, und er darf verschoben werden.
    let genommen = text;
    println!("{genommen}");
}
```

### Häufige Fehler

Das `&` vergessen.

```rust
fn shout(text: &String) -> String {
    text.to_uppercase()
}

fn main() {
    let text = String::from("hallo");

    println!("{}", shout(text));
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> ausleihen.rs:8:26
  |
8 |     println!("{}", shout(text));
  |                    ----- ^^^^ expected `&String`, found `String`
  |                    |
  |                    arguments to this function are incorrect
  |
note: function defined here
 --> ausleihen.rs:1:4
  |
1 | fn shout(text: &String) -> String {
  |    ^^^^^ -------------
help: consider borrowing here
  |
8 |     println!("{}", shout(&text));
  |                          +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Es ist dieselbe Nummer wie bei einer Zahl als Bedingung in `01-05`, denn es ist
derselbe Fall: hier steht ein Typ, erwartet wird ein anderer. Der Vorschlag ist
diesmal genau die Antwort, nämlich ein `&` vor dem Namen.

Der umgekehrte Fehler ist der aus `02-01`: wer den Wert übergibt statt ihn
auszuleihen, kann ihn danach nicht mehr benutzen, und das ist `E0382`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Keine Aufgabe nimmt einen Wert an sich.

- `vowel_count` zählt die Selbstlaute in einem geliehenen Text
- `doubled_through` gibt das Doppelte einer geliehenen Zahl zurück
- `total_length` addiert die Längen zweier geliehener Texte

```console
cd units/02-03-ausleihen
cargo test
```

### Quelle

    Buch, Kapitel 4 "Understanding Ownership", Abschnitt 4.2 "References and Borrowing",
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A reference is a loan. `&text` gives a function access to a value without giving
it ownership. The function may read, it may not change anything, and when it is
done the caller still has its value unchanged.

`*` is the way back: it reads the value behind the reference. With methods it is
mostly not needed, because a call with a dot resolves the reference by itself.

A loan holds until its last use and not until the end of the block. After that
the value is free again.

### What it is good for

Without loans every function that only looks at something would have to take the
value and hand it back. That is exactly what happened in `02-01`, and the return
value was pure bookkeeping.

The second possibility would be `clone`. It costs a second place on the heap,
every time, only so that somebody may count the length.

Shared references are also allowed any number of times at once. Two places may
read the same value at the same time, as long as none of them changes it. What
happens when somebody does want to change it stands in `02-04`.

### The explanation

Borrow, read, and the caller keeps it.

```rust
fn laenge_von(text: &String) -> usize {
    // Deutsch: `text` ist eine geteilte Referenz. Sie darf lesen und nicht
    // ändern, und der Wert gehört weiter dem Aufrufer.
    text.len()
}

fn main() {
    let text = String::from("hallo");

    println!("{}", laenge_von(&text));

    // Deutsch: `text` steht noch da, denn übergeben wurde nur eine Ausleihe.
    println!("{text}");

    let zahl = 21;
    let geliehen = &zahl;

    // Deutsch: `*` liest den Wert hinter der Referenz.
    println!("{}", *geliehen * 2);
}
```

clippy reports `clippy::ptr_arg` for `&String` as a parameter and suggests
`&str`. The suggestion is right, and a `&str` is a slice; those stand in
`02-05`. Until then the shape stays `&String`, because the learner holds a
`String` here, and the solution carries an explicit `#[allow(clippy::ptr_arg)]`
with that reason.

The loan ends at its last use. That is why the value may even be moved
afterwards.

```rust
fn main() {
    let text = String::from("hallo");

    let geliehen = &text;
    println!("{}", geliehen.len());

    // Deutsch: Nach der letzten Benutzung der Ausleihe ist der Wert wieder
    // frei, und er darf verschoben werden.
    let genommen = text;
    println!("{genommen}");
}
```

### Common mistakes

Forgetting the `&`.

```rust
fn shout(text: &String) -> String {
    text.to_uppercase()
}

fn main() {
    let text = String::from("hallo");

    println!("{}", shout(text));
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> ausleihen.rs:8:26
  |
8 |     println!("{}", shout(text));
  |                    ----- ^^^^ expected `&String`, found `String`
  |                    |
  |                    arguments to this function are incorrect
  |
note: function defined here
 --> ausleihen.rs:1:4
  |
1 | fn shout(text: &String) -> String {
  |    ^^^^^ -------------
help: consider borrowing here
  |
8 |     println!("{}", shout(&text));
  |                          +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

It is the same number as a number used as a condition in `01-05`, because it is
the same case: one type stands there and another was expected. This time the
suggestion is exactly the answer, namely a `&` in front of the name.

The opposite mistake is the one from `02-01`: whoever hands the value over
instead of lending it cannot use it afterwards, and that is `E0382`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. No exercise takes a value for itself.

- `vowel_count` counts the vowels in a borrowed text
- `doubled_through` returns the double of a borrowed number
- `total_length` adds the lengths of two borrowed texts

```console
cd units/02-03-ausleihen
cargo test
```

### Source

    Book, chapter 4 "Understanding Ownership", section 4.2 "References and Borrowing",
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
