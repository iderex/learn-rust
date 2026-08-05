# 01-04 Kommentare und cargo fmt / Comments and cargo fmt

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/01-04-kommentare-und-cargo-fmt/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `01-03 Funktionen`.
- Auf dieser Einheit bauen auf: `01-05 if und else` und `01-06 Schleifen und
  cargo clippy`, wo das zweite Werkzeug dazukommt.
- Beim Antworten so zitieren: `01-04 Kommentare und cargo fmt`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Unterschied zwischen `//` und `///` ist hier der Kern. `//` erklärt einer
  Person etwas, `///` gehört zu dem, was darunter steht, und wird mitgetestet.

</details>

## Deutsch

### Worum es geht

Zwei Zeichenfolgen und ein Befehl. `//` beginnt einen gewöhnlichen Kommentar,
der bis zum Zeilenende geht und den der Übersetzer überliest. `///` beginnt
einen Doku-Kommentar; er gehört zu dem, was direkt darunter steht, und
`cargo doc` macht daraus eine Seite.

Der Befehl ist `cargo fmt`. Er legt Einrückung, Zeilenumbrüche und Abstände fest,
und zwar für alle gleich.

### Wofür das gut ist

Ein Doku-Kommentar ist mehr als ein hübscher Kommentar: sein Beispiel wird
mitgetestet. Steht in einem `///`-Block ein Beispiel mit drei Rückstrichen
darin, dann übersetzt und läuft `cargo test` genau dieses Beispiel. Ein
Kommentar, der lügt, wird damit rot statt bloß falsch.

`cargo fmt` nimmt eine ganze Klasse von Streit aus dem Weg. Wo die Klammer steht
und wie tief eingerückt wird, entscheidet niemand mehr, sondern das Werkzeug.
Der Nutzen ist nicht Schönheit, sondern dass ein Unterschied im Text danach ein
Unterschied im Inhalt ist.

### Die Erklärung

Beide Kommentararten nebeneinander, dazu ein Doku-Beispiel.

````rust
/// Gibt den Bruttobetrag zu einem Nettobetrag in Cent zurück.
///
/// ```
/// assert_eq!(brutto(100), 119);
/// ```
pub fn brutto(netto_cent: u32) -> u32 {
    // Deutsch: In Cent rechnen, damit keine Fließkommazahlen nötig sind.
    netto_cent + netto_cent * 19 / 100
}
````

Der obere Block gehört zur Funktion und beschreibt sie nach außen. Der untere
steht im Rumpf und erklärt eine Entscheidung. Die Faustregel: `///` sagt, was
etwas tut, `//` sagt, warum es so gemacht ist.

`cargo fmt` wird ohne Argumente aufgerufen und schreibt die Dateien um. Wer nur
wissen will, ob etwas zu ändern wäre, hängt `--check` an; dann ändert sich nichts
und der Rückgabewert sagt es.

### Häufige Fehler

Ein Doku-Kommentar, unter dem nichts steht.

```rust
fn main() {
    println!("hallo");
}

/// Diese Zeile beschreibt nichts.
```

Der Übersetzer sagt dazu:

```text
error: expected item after doc comment
 --> dok.rs:5:1
  |
5 | /// Diese Zeile beschreibt nichts.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this doc comment doesn't document anything

error: aborting due to 1 previous error
```

Diese Meldung hat keine Nummer, so wie die Meldungen von cargo in `00-02`. Sie
sagt trotzdem alles: ein `///` gehört zu etwas, und hier steht nichts mehr, zu
dem es gehören könnte. Die Antwort ist entweder, ein `//` daraus zu machen, oder
den Kommentar über das zu setzen, was er beschreiben soll.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot. Gerechnet wird durchweg in Cent.

- `vat_of` gibt die Mehrwertsteuer von 19 Prozent zurück
- `discounted` gibt einen Betrag mit zehn Prozent Nachlass zurück
- `rounded_up_to_full_euro` rundet auf den nächsten vollen Euro auf

```console
cd units/01-04-kommentare-und-cargo-fmt
cargo test
```

### Quelle

    Buch, Kapitel 3 "Common Programming Concepts", Abschnitt 3.4 "Comments",
    https://doc.rust-lang.org/book/ch03-04-comments.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Two character sequences and one command. `//` begins an ordinary comment running
to the end of the line, which the compiler reads past. `///` begins a doc
comment; it belongs to whatever stands directly below it, and `cargo doc` makes
a page out of it.

The command is `cargo fmt`. It fixes indentation, line breaks and spacing, and
it does so the same way for everybody.

### What it is good for

A doc comment is more than a pretty comment: its example is tested along with
everything else. If a `///` block holds an example inside three backticks, then
`cargo test` compiles and runs exactly that example. A comment that lies turns
red rather than merely being wrong.

`cargo fmt` takes a whole class of argument out of the way. Where the brace goes
and how deep the indentation runs is decided by nobody any more but by the tool.
The use is not beauty but that a difference in the text afterwards is a
difference in the content.

### The explanation

Both kinds of comment next to each other, with a doc example.

````rust
/// Gibt den Bruttobetrag zu einem Nettobetrag in Cent zurück.
///
/// ```
/// assert_eq!(brutto(100), 119);
/// ```
pub fn brutto(netto_cent: u32) -> u32 {
    // Deutsch: In Cent rechnen, damit keine Fließkommazahlen nötig sind.
    netto_cent + netto_cent * 19 / 100
}
````

The upper block belongs to the function and describes it outwards. The lower one
stands in the body and explains a decision. The rule of thumb: `///` says what
something does, `//` says why it is done that way.

`cargo fmt` is called without arguments and rewrites the files. Whoever only
wants to know whether something would change appends `--check`; then nothing
changes and the return code says it.

### Common mistakes

A doc comment with nothing below it.

```rust
fn main() {
    println!("hallo");
}

/// Diese Zeile beschreibt nichts.
```

The compiler answers:

```text
error: expected item after doc comment
 --> dok.rs:5:1
  |
5 | /// Diese Zeile beschreibt nichts.
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this doc comment doesn't document anything

error: aborting due to 1 previous error
```

This message has no number, like the messages from cargo in `00-02`. It still
says everything: a `///` belongs to something, and here nothing is left for it to
belong to. The answer is either to make it a `//` or to put the comment above the
thing it is meant to describe.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Everything is computed in cents.

- `vat_of` returns the value added tax of 19 percent
- `discounted` returns an amount with ten percent taken off
- `rounded_up_to_full_euro` rounds up to the next full euro

```console
cd units/01-04-kommentare-und-cargo-fmt
cargo test
```

### Source

    Book, chapter 3 "Common Programming Concepts", section 3.4 "Comments",
    https://doc.rust-lang.org/book/ch03-04-comments.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
