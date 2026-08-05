# 02-05 Slices / Slices

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/02-05-slices/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Meldung erklären, um die es geht.
- Diese Einheit baut auf: `02-03 Ausleihen` und `02-04 Veränderbares Ausleihen`.
- Auf dieser Einheit bauen auf: `04-05 String` und `04-04 Vec`, und mit ihr ist
  die Stufe 2 zu Ende.
- Beim Antworten so zitieren: `02-05 Slices`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Grenzen eines Slice auf einem Text sind Bytes und keine Zeichen. Wer hier
  von Zeichen spricht, sagt bitte dazu, dass ein `ü` zwei Bytes belegt und ein
  Schnitt mittendrin das Programm anhält.
- Dieser Fehler kommt beim Laufen und nicht beim Übersetzen. Das ist der
  Unterschied zu allen Meldungen der Stufe 2 davor, und der Text sagt es an Ort
  und Stelle.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/02-05-slices/`. It is public.
  Whoever is asked for it may name it, but should explain the message in
  question first.
- This unit builds on: `02-03 Ausleihen` and `02-04 Veränderbares Ausleihen`.
- Building on this unit: `04-05 String` and `04-04 Vec`, and with it stage 2
  ends.
- Cite like this when answering: `02-05 Slices`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The bounds of a slice over a text are bytes and not characters. Whoever talks
  about characters here, please say alongside it that a `ü` takes two bytes and
  that a cut inside it stops the program.
- This fault comes while the program runs and not while it is compiled. That is
  the difference from every message in stage 2 before it, and the text says so
  on the spot.

</details>

## Deutsch

### Worum es geht

Ein Slice ist eine Ausleihe auf einen Teil. `&text[0..5]` leiht die ersten fünf
Bytes eines Textes aus, `&zahlen[..2]` die ersten zwei Zahlen eines Feldes. Der
Slice merkt sich zwei Dinge, nämlich wo der Teil anfängt und wie lang er ist.

Ein Slice auf einen Text heißt `&str`. Genau das ist auch der Typ eines Texts in
Anführungszeichen im Programm, und deshalb passen beide in dieselbe Funktion.

Weil ein Slice eine Ausleihe ist, gelten die Regeln aus `02-03` und `02-04`. Er
hält den Wert dahinter fest, solange er benutzt wird, und der Wert darf in
dieser Zeit nicht verändert werden.

### Wofür das gut ist

Eine Funktion, die auf einem Slice arbeitet, kopiert nichts. Sie bekommt die
Stelle und die Länge, mehr nicht, und sie kann deshalb auf einem Teil arbeiten,
ohne dass jemand diesen Teil erst herausschneiden und ablegen muss.

Sie ist außerdem allgemeiner. `&[i32]` passt auf ein Feld, auf einen Teil davon
und später auf ein `Vec`, ohne dass die Funktion davon etwas weiß. `&str` passt
auf einen `String`, auf einen Teil davon und auf einen Text im Programm.

Und der Slice bindet die Länge an die Daten. Wer eine Anfangsadresse und eine
Länge getrennt weiterreicht, kann beide auseinanderlaufen lassen; hier gehören
sie zusammen.

### Die Erklärung

Ein Slice auf einen Text und einer auf ein Feld.

```rust
fn main() {
    let text = String::from("hallo welt");

    // Deutsch: Ein Slice auf einen `String` ist ein `&str`. Er merkt sich die
    // Stelle im Text und die Länge, und er kopiert nichts.
    let wort = &text[0..5];

    println!("{wort} {}", wort.len());

    // Deutsch: Dieselbe Stelle im Speicher, also wurde nichts kopiert.
    println!("{}", wort.as_ptr() == text.as_ptr());

    let zahlen = [1, 2, 3, 4];

    // Deutsch: Auf einem Feld heißt der Slice `&[i32]`, und `..` ohne Zahl
    // meint den Anfang oder das Ende.
    let anfang = &zahlen[..2];

    println!("{} {}", anfang.len(), anfang[1]);
}
```

`as_ptr` gibt die Adresse zurück, an der die Daten liegen. Dass die des Slice
und die des Textes gleich sind, ist der Beweis, dass nichts kopiert wurde. Ein
Test in dieser Einheit macht genau das.

### Häufige Fehler

Mitten in ein Zeichen schneiden.

```rust
fn main() {
    let text = String::from("Grüße");

    let anfang = &text[0..3];

    println!("{anfang}");
}
```

Das übersetzt. Beim Laufen sagt das Programm:

```text
thread 'main' (60420) panicked at grenze.rs:4:23:
end byte index 3 is not a char boundary; it is inside 'ü' (bytes 2..4 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die Zahl in Klammern ist die Nummer des laufenden Vorgangs und bei jedem Lauf
eine andere.

Der Grund ist, dass die Grenzen Bytes zählen und nicht Zeichen. `G` und `r`
brauchen je ein Byte, `ü` braucht zwei, und der Schnitt bei 3 fällt mitten
hinein. Mit `0..2` oder `0..4` geht es.

Das ist die erste Meldung der Stufe 2, die erst beim Laufen kommt. Bis hierher
hat der Übersetzer jeden Fehler dieser Stufe vorher abgefangen; welche Stelle
ein Text hat, weiß er nicht.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Keine Aufgabe legt eine Kopie an, und ein Test sieht das über
die Adresse nach.

- `first_word` gibt den Teil bis zum ersten Leerzeichen zurück
- `without_first` gibt alles außer der ersten Zahl zurück
- `sum_of` addiert die Zahlen eines Slice

```console
cd units/02-05-slices
cargo test
```

### Quelle

    Buch, Kapitel 4 "Understanding Ownership", Abschnitt 4.3 "The Slice Type",
    https://doc.rust-lang.org/book/ch04-03-slices.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A slice is a loan on a part. `&text[0..5]` lends out the first five bytes of a
text, `&zahlen[..2]` the first two numbers of an array. The slice remembers two
things, namely where the part begins and how long it is.

A slice over a text is called `&str`. That is also the type of a text in quotes
in the program, and that is why both fit into the same function.

Because a slice is a loan, the rules from `02-03` and `02-04` hold. It holds on
to the value behind it as long as it is used, and during that time the value may
not be changed.

### What it is good for

A function working on a slice copies nothing. It gets the place and the length
and nothing else, so it can work on a part without anybody having to cut that
part out and store it first.

It is also more general. `&[i32]` fits an array, a part of one and later a
`Vec`, without the function knowing anything about it. `&str` fits a `String`, a
part of one and a text written in the program.

And the slice ties the length to the data. Whoever passes a starting address and
a length on separately can let the two drift apart; here they belong together.

### The explanation

A slice over a text and one over an array.

```rust
fn main() {
    let text = String::from("hallo welt");

    // Deutsch: Ein Slice auf einen `String` ist ein `&str`. Er merkt sich die
    // Stelle im Text und die Länge, und er kopiert nichts.
    let wort = &text[0..5];

    println!("{wort} {}", wort.len());

    // Deutsch: Dieselbe Stelle im Speicher, also wurde nichts kopiert.
    println!("{}", wort.as_ptr() == text.as_ptr());

    let zahlen = [1, 2, 3, 4];

    // Deutsch: Auf einem Feld heißt der Slice `&[i32]`, und `..` ohne Zahl
    // meint den Anfang oder das Ende.
    let anfang = &zahlen[..2];

    println!("{} {}", anfang.len(), anfang[1]);
}
```

`as_ptr` returns the address at which the data lies. That the one of the slice
and the one of the text are equal is the proof that nothing was copied. One test
in this unit does exactly that.

### Common mistakes

Cutting into the middle of a character.

```rust
fn main() {
    let text = String::from("Grüße");

    let anfang = &text[0..3];

    println!("{anfang}");
}
```

That compiles. While running the program says:

```text
thread 'main' (60420) panicked at grenze.rs:4:23:
end byte index 3 is not a char boundary; it is inside 'ü' (bytes 2..4 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The number in brackets is the number of the running process and a different one
on every run.

The reason is that the bounds count bytes and not characters. `G` and `r` need
one byte each, `ü` needs two, and the cut at 3 falls into the middle of it. With
`0..2` or `0..4` it works.

This is the first message in stage 2 that comes only while the program runs.
Until here the compiler caught every fault of this stage beforehand; which
places a text has is not something it knows.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. No exercise makes a copy, and one test checks
that through the address.

- `first_word` returns the part up to the first space
- `without_first` returns everything except the first number
- `sum_of` adds up the numbers of a slice

```console
cd units/02-05-slices
cargo test
```

### Source

    Book, chapter 4 "Understanding Ownership", section 4.3 "The Slice Type",
    https://doc.rust-lang.org/book/ch04-03-slices.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
