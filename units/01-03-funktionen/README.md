# 01-03 Funktionen / Functions

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/01-03-funktionen/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `01-01 Variablen und Veränderbarkeit` und
  `01-02 Zahlen und andere einfache Typen`.
- Auf dieser Einheit bauen auf: alle weiteren Einheiten, denn ab hier wird jede
  Aufgabe als Funktion geschrieben.
- Beim Antworten so zitieren: `01-03 Funktionen`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Unterschied zwischen Ausdruck und Anweisung ist hier der Kern. Wer danach
  gefragt wird, erklärt ihn am Semikolon und nicht an einer Regelliste.

</details>

## Deutsch

### Worum es geht

Eine Funktion bündelt ein Stück Arbeit unter einem Namen. Sie steht mit `fn` da,
bekommt Parameter mit ihren Typen und nennt hinter einem Pfeil den Typ dessen,
was sie zurückgibt.

Der Rückgabewert ist der letzte Ausdruck im Rumpf. Er braucht kein `return`, er
braucht aber die Abwesenheit eines Semikolons.

### Wofür das gut ist

Ein Name für ein Stück Arbeit ist der billigste Weg, ein Programm lesbar zu
halten. Wer `area(3, 4)` liest, muss nicht wissen, wie die Fläche gerechnet
wird.

Der zweite Nutzen ist die Wiederverwendung. `square_area` in der dritten Aufgabe
rechnet nichts selbst, sondern ruft `area` mit zweimal derselben Seite auf. Wenn
sich die Flächenrechnung je ändert, ändert sie sich an einer Stelle.

### Die Erklärung

Eine Funktion mit zwei Parametern und einem Rückgabetyp.

```rust
fn umfang(breite: u32, hoehe: u32) -> u32 {
    2 * (breite + hoehe)
}

fn main() {
    let ergebnis = umfang(2, 3);
    println!("{ergebnis}");
}
```

Drei Dinge sind daran wichtig. Erstens: jeder Parameter nennt seinen Typ, und
Rust rät ihn nie. Zweitens: der Pfeil `->` nennt den Rückgabetyp. Drittens: die
letzte Zeile im Rumpf hat kein Semikolon, und deshalb ist sie der Rückgabewert.

Der Unterschied dahinter heißt Ausdruck gegen Anweisung. Ein Ausdruck hat einen
Wert, `2 * (breite + hoehe)` zum Beispiel. Eine Anweisung hat keinen; ein
Semikolon macht aus einem Ausdruck eine Anweisung und wirft seinen Wert weg.

### Häufige Fehler

Genau dieses eine Semikolon zu viel.

```rust
fn verdoppelt(n: u32) -> u32 {
    n * 2;
}

fn main() {
    println!("{}", verdoppelt(21));
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
 --> semikolon.rs:1:26
  |
1 | fn verdoppelt(n: u32) -> u32 {
  |    ----------            ^^^ expected `u32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * 2;
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Das Zeichen `()` heißt Unit und ist der Typ, den etwas hat, das keinen Wert
liefert. Die Meldung sagt genau das: erwartet `u32`, gefunden `()`, weil der
Rumpf mit einer Anweisung endet. Der Hinweis zeigt auf das Semikolon und schlägt
vor, es zu entfernen. Genau das ist die Antwort.

Es ist wieder `E0308`, dieselbe Nummer wie in `00-01` und `01-02`. Diese Nummer
steht für alles, was den falschen Typ hat, und deshalb sieht man sie oft.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.

- `area` gibt die Fläche eines Rechtecks zurück
- `celsius_from` rechnet eine Temperatur von Fahrenheit in Celsius um
- `square_area` gibt die Fläche eines Quadrats zurück, indem es `area` aufruft

```console
cd units/01-03-funktionen
cargo test
```

### Quelle

    Buch, Kapitel 3 "Common Programming Concepts", Abschnitt 3.3 "Functions",
    https://doc.rust-lang.org/book/ch03-03-how-functions-work.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A function bundles a piece of work under a name. It stands there with `fn`, gets
parameters with their types, and names the type of what it returns behind an
arrow.

The return value is the last expression in the body. It needs no `return`, but
it does need the absence of a semicolon.

### What it is good for

A name for a piece of work is the cheapest way to keep a program readable.
Whoever reads `area(3, 4)` does not have to know how the area is computed.

The second use is reuse. `square_area` in the third exercise computes nothing
itself but calls `area` with the same side twice. If the area calculation ever
changes, it changes in one place.

### The explanation

A function with two parameters and a return type.

```rust
fn umfang(breite: u32, hoehe: u32) -> u32 {
    2 * (breite + hoehe)
}

fn main() {
    let ergebnis = umfang(2, 3);
    println!("{ergebnis}");
}
```

Three things about it matter. First: every parameter names its type, and Rust
never guesses it. Second: the arrow `->` names the return type. Third: the last
line in the body has no semicolon, and that is why it is the return value.

The difference behind that is expression against statement. An expression has a
value, `2 * (breite + hoehe)` for example. A statement has none; a semicolon
turns an expression into a statement and throws its value away.

### Common mistakes

Exactly that one semicolon too many.

```rust
fn verdoppelt(n: u32) -> u32 {
    n * 2;
}

fn main() {
    println!("{}", verdoppelt(21));
}
```

The compiler answers:

```text
error[E0308]: mismatched types
 --> semikolon.rs:1:26
  |
1 | fn verdoppelt(n: u32) -> u32 {
  |    ----------            ^^^ expected `u32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     n * 2;
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

The sign `()` is called unit and is the type of something that delivers no
value. The message says exactly that: expected `u32`, found `()`, because the
body ends with a statement. The note points at the semicolon and proposes
removing it. That is the answer.

It is `E0308` again, the same number as in `00-01` and `01-02`. That number
stands for everything of the wrong type, which is why it turns up often.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `area` returns the area of a rectangle
- `celsius_from` converts a temperature from Fahrenheit to Celsius
- `square_area` returns the area of a square by calling `area`

```console
cd units/01-03-funktionen
cargo test
```

### Source

    Book, chapter 3 "Common Programming Concepts", section 3.3 "Functions",
    https://doc.rust-lang.org/book/ch03-03-how-functions-work.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
