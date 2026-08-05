# 04-05 String / String

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-05-string/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-05 Slices`, wo der Schnitt mitten in ein Zeichen
  schon einmal vorkam, und `04-04 Vec`.
- Auf dieser Einheit bauen auf: `04-06 HashMap` und alles, was Text zusammenbaut.
- Beim Antworten so zitieren: `04-05 String`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `len` zählt Bytes. Wer es als Zahl der Zeichen erklärt, macht genau den Fehler,
  den die Einheit ausräumt, und die Tests halten den Unterschied an "Grüße"
  fest.
- Die abgedruckte Fehlermeldung ist gekürzt: in der Mitte stehen Pfade in die
  Quellen des Übersetzers, die auf jedem Rechner anders lauten. Der Text sagt
  das an Ort und Stelle.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-05-string/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `02-05 Slices`, where the cut into the middle of a
  character already appeared, and `04-04 Vec`.
- Building on this unit: `04-06 HashMap` and everything that builds text.
- Cite like this when answering: `04-05 String`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `len` counts bytes. Whoever explains it as the number of characters makes
  exactly the mistake the unit clears away, and the tests hold the difference
  down on "Grüße".
- The compiler message printed is shortened: in the middle stand paths into the
  sources of the compiler which read differently on every machine. The text says
  so on the spot.

</details>

## Deutsch

### Worum es geht

Es gibt zwei Textarten, und beide kommen ständig vor. Ein `&str` ist geliehener
Text, den jemand anderes besitzt; ein Text in Anführungszeichen im Programm ist
einer. Ein `String` gehört sich selbst, liegt auf dem Heap und kann wachsen.

Gebaut wird ein `String` mit `String::from` oder `to_string`, verlängert mit
`push_str` und `push`, zusammengesetzt mit `format!`.

Und dann ist da die Zahl. `len` zählt Bytes, nicht Zeichen. Ein `ü` braucht zwei
Bytes, also hat "Grüße" fünf Zeichen und sieben Bytes.

### Wofür das gut ist

Die Trennung spart Kopien. Eine Funktion, die Text nur liest, nimmt `&str` und
kann damit einen `String`, einen Ausschnitt daraus und einen Text aus dem
Programm annehmen, ohne dass irgendwo etwas kopiert wird.

Die Zählung in Bytes ist keine Schikane, sondern die Wahrheit über die Ablage.
Text steht in UTF-8, und dort belegen Zeichen verschieden viel Platz. Wer nach
dem fünften Zeichen schneiden will, muss die Zeichen zählen und nicht die Bytes,
und `chars()` tut genau das.

Deshalb gibt es auch keinen Zugriff über eine Zahl. `text[0]` müsste entweder
ein Byte oder ein Zeichen liefern, und beides wäre für die Hälfte der Fälle
falsch. Die Sprache verlangt, dass man sich entscheidet.

### Die Erklärung

Anlegen, verlängern, zusammensetzen und zählen.

```rust
fn main() {
    // Deutsch: Ein Text im Programm ist ein `&str`. `to_string` oder
    // `String::from` machen daraus einen `String`, der wachsen kann.
    let mut gruss = String::from("Hallo");

    gruss.push_str(", Welt");
    gruss.push('!');

    // Deutsch: `format!` setzt zusammen und lässt beide Teile stehen.
    let zeile = format!("{gruss} ({} Bytes)", gruss.len());

    println!("{zeile}");

    // Deutsch: `len` zählt Bytes, `chars().count()` zählt Zeichen. Bei
    // Umlauten sind das zwei verschiedene Zahlen.
    let umlaute = String::from("Grüße");

    println!("{} {}", umlaute.len(), umlaute.chars().count());
}
```

Das Programm gibt aus:

```text
Hallo, Welt! (12 Bytes)
7 5
```

Sieben Bytes und fünf Zeichen, aus demselben Wort. Genau diese beiden Zahlen
hält ein Test dieser Einheit fest.

### Häufige Fehler

Auf ein Zeichen über seine Nummer zugreifen.

```rust
fn main() {
    let text = String::from("Grüße");

    println!("{}", text[0]);
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> index.rs:4:25
  |
4 |     println!("{}", text[0]);
  |                         ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  ...
  = note: required for `String` to implement `Index<{integer}>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

An der Stelle mit den drei Punkten stehen im Lauf Pfade in die Quellen des
Übersetzers; sie lauten auf jedem Rechner anders und sagen für diese Einheit
nichts.

Die Meldung nennt beide Wege, `chars()` und `bytes()`, und verlangt damit die
Entscheidung, um die es geht. Ein Ausschnitt über Bytes geht weiterhin, also
`&text[0..2]`, und der bricht ab, wenn er mitten in ein Zeichen fällt; das war
`02-05`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe wird an einem Wort mit Umlauten geprüft.

- `char_count` zählt die Zeichen eines Textes
- `joined` setzt zwei Texte mit einem Leerzeichen dazwischen zusammen
- `shortened` gibt die ersten `zeichen` Zeichen zurück

```console
cd units/04-05-string
cargo test
```

### Quelle

    Buch, Kapitel 8 "Common Collections", Abschnitt 8.2 "Storing UTF-8 Encoded Text with Strings",
    https://doc.rust-lang.org/book/ch08-02-strings.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

There are two kinds of text, and both appear constantly. A `&str` is borrowed
text owned by somebody else; a text in quotes in the program is one. A `String`
owns itself, lies on the heap and can grow.

A `String` is built with `String::from` or `to_string`, lengthened with
`push_str` and `push`, and put together with `format!`.

And then there is the number. `len` counts bytes, not characters. A `ü` needs
two bytes, so "Grüße" has five characters and seven bytes.

### What it is good for

The separation saves copies. A function that only reads text takes `&str` and
can then accept a `String`, a part of one and a text from the program, without
anything being copied anywhere.

Counting in bytes is not chicanery but the truth about the storage. Text stands
in UTF-8, and there characters take up different amounts of room. Whoever wants
to cut after the fifth character has to count characters and not bytes, and
`chars()` does exactly that.

That is also why there is no access through a number. `text[0]` would have to
deliver either a byte or a character, and both would be wrong for half the
cases. The language demands that you decide.

### The explanation

Creating, lengthening, putting together and counting.

```rust
fn main() {
    // Deutsch: Ein Text im Programm ist ein `&str`. `to_string` oder
    // `String::from` machen daraus einen `String`, der wachsen kann.
    let mut gruss = String::from("Hallo");

    gruss.push_str(", Welt");
    gruss.push('!');

    // Deutsch: `format!` setzt zusammen und lässt beide Teile stehen.
    let zeile = format!("{gruss} ({} Bytes)", gruss.len());

    println!("{zeile}");

    // Deutsch: `len` zählt Bytes, `chars().count()` zählt Zeichen. Bei
    // Umlauten sind das zwei verschiedene Zahlen.
    let umlaute = String::from("Grüße");

    println!("{} {}", umlaute.len(), umlaute.chars().count());
}
```

The program prints:

```text
Hallo, Welt! (12 Bytes)
7 5
```

Seven bytes and five characters, out of the same word. Exactly those two numbers
are held down by a test of this unit.

### Common mistakes

Reaching for a character through its number.

```rust
fn main() {
    let text = String::from("Grüße");

    println!("{}", text[0]);
}
```

The compiler answers:

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> index.rs:4:25
  |
4 |     println!("{}", text[0]);
  |                         ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
          for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
  ...
  = note: required for `String` to implement `Index<{integer}>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

At the place with the three dots stand paths into the sources of the compiler
during the run; they read differently on every machine and say nothing for this
unit.

The message names both ways, `chars()` and `bytes()`, and demands with that the
decision this is about. A cut over bytes keeps working, so `&text[0..2]`, and it
breaks off when it falls into the middle of a character; that was `02-05`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise is checked on a word with
umlauts in it.

- `char_count` counts the characters of a text
- `joined` puts two texts together with a space in between
- `shortened` returns the first `zeichen` characters

```console
cd units/04-05-string
cargo test
```

### Source

    Book, chapter 8 "Common Collections", section 8.2 "Storing UTF-8 Encoded Text with Strings",
    https://doc.rust-lang.org/book/ch08-02-strings.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
