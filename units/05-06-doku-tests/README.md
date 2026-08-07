# 05-06 Doku-Tests / Doc tests

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-06-doku-tests/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `01-04 Kommentare und cargo fmt` und
  `05-04 Lifetimes`. Der Doku-Kommentar kommt aus der Stufe 1, die Lebensdauer
  in Aufgabe 3 aus dieser Stufe.
- Auf dieser Einheit bauen auf: alles, was ein Beispiel in einem
  Doku-Kommentar mitliefert.
- Beim Antworten so zitieren: `05-06 Doku-Tests`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein Doku-Test läuft als eigenes Programm gegen die öffentliche
  Schnittstelle. Was privat ist, sieht er nicht, und darin liegt der Unterschied
  zu einem Test in `src/`.
- `should_panic` an einem Doku-Test nimmt keine erwartete Meldung entgegen. Wer
  das Gegenteil behauptet, sagt bitte, an welcher Schreibweise.
- Die Beispiele stehen in dieser Einheit auch in der Lösung, anders als sonst.
  Sie sind hier der Gegenstand und nicht die Verzierung.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-06-doku-tests/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: `01-04 Kommentare und cargo fmt` and `05-04 Lifetimes`.
  The doc comment comes from stage 1, the lifetime in exercise 3 from this
  stage.
- Building on this unit: everything that ships an example inside a doc comment.
- Cite like this when answering: `05-06 Doku-Tests`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A doc test runs as a program of its own against the public interface. It does
  not see what is private, and that is the difference to a test inside `src/`.
- `should_panic` on a doc test takes no expected message. Whoever claims the
  opposite, please say in which spelling.
- The examples stand in the solution of this unit as well, unlike elsewhere.
  Here they are the subject and not the decoration.

</details>

## Deutsch

### Worum es geht

Ein Doku-Kommentar beginnt mit `///` und beschreibt das, was direkt darunter
steht. Was darin zwischen drei Rückstrichen steht, ist kein Zitat, sondern ein
Programm.

`cargo test` sammelt diese Beispiele ein, baut aus jedem ein eigenes kleines
Programm und führt es aus. Am Ende des Laufs steht ein eigener Abschnitt
"Doc-tests" mit einer Zeile je Beispiel.

Weil das Beispiel ein eigenes Programm ist, sieht es nur die öffentliche
Schnittstelle des Pakets. Deshalb steht in ihm ein `use`, wie in jedem anderen
Programm auch, und deshalb kann es nichts benutzen, was nicht `pub` ist.

### Wofür das gut ist

Ein Beispiel im Text veraltet still. Wird eine Funktion umbenannt oder bekommt
sie einen Parameter mehr, dann bleibt der Text daneben stehen und behauptet
weiter etwas, das nicht mehr stimmt. Niemand merkt es, bis jemand das Beispiel
abtippt und die Fehlermeldung bekommt.

Ein Doku-Test kann das nicht. Er wird bei jedem Lauf gebaut, also fällt die
Umbenennung im selben Moment auf, in dem sie passiert. Das Beispiel ist damit
nicht mehr eine Behauptung über den Code, sondern eine Aussage, die der
Übersetzer nachrechnet.

Dazu kommt, dass ein Beispiel für einen Leser geschrieben wird. Ein Test in
`tests/` prüft Grenzfälle, ein Doku-Test zeigt den geraden Weg. Beide braucht
es, und sie ersetzen einander nicht.

### Die Erklärung

Ein Paket `notiz` mit drei Beispielen: eines einfach, eines mit versteckten
Zeilen und `?`, und eines, das abbrechen soll.

```rust
//! Ein winziges Paket, dessen Doku-Kommentare Beispiele tragen.

/// Zählt die Wörter in einem Text.
///
/// Alles zwischen den Rückstrichen ist ein Beispiel. `cargo test` baut es als
/// eigenes kleines Programm und führt es aus.
///
/// ```
/// use notiz::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// assert_eq!(word_count("   "), 0);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Liest eine Prozentzahl aus einem Text.
///
/// Die Zeilen mit `#` am Anfang laufen mit, werden aber nicht abgedruckt. So
/// darf das Beispiel `?` benutzen, ohne dass der Leser das `main` drumherum
/// sieht.
///
/// ```
/// # use notiz::percent;
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// assert_eq!(percent(" 42 ")?, 42);
/// # Ok(())
/// # }
/// ```
pub fn percent(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.trim().parse()
}

/// Gibt das Wort an `stelle` zurück und bricht ab, wenn es dort keines gibt.
///
/// ```
/// use notiz::word_at;
///
/// assert_eq!(word_at("ein zwei drei", 1), "zwei");
/// ```
///
/// `should_panic` sagt, dass dieses Beispiel abbrechen soll.
///
/// ```should_panic
/// use notiz::word_at;
///
/// word_at("ein", 5);
/// ```
pub fn word_at(text: &str, stelle: usize) -> &str {
    text.split_whitespace()
        .nth(stelle)
        .expect("an dieser Stelle steht kein Wort")
}
```

`cargo test` gibt dazu aus:

```text
   Doc-tests notiz

running 4 tests
test src\lib.rs - percent (line 24) ... ok
test src\lib.rs - word_at (line 45) - should panic ... ok
test src\lib.rs - word_count (line 8) ... ok
test src\lib.rs - word_at (line 37) ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.11s
```

Vier Tests aus drei Funktionen, denn `word_at` trägt zwei Beispiele. Jede Zeile
nennt die Datei und die Zeile, in der das Beispiel anfängt, und das ist die
Stelle, an der es repariert wird.

Ein Wort zu `should_panic` an dieser Stelle. Es nimmt keine erwartete Meldung
entgegen, es nimmt also jeden Abbruch an, auch einen aus einem ganz anderen
Grund. Wer den Grund festhalten will, schreibt den Test in `tests/`, wo
`#[should_panic(expected = "...")]` zur Verfügung steht.

### Häufige Fehler

Die Funktion umbenennen und das Beispiel stehen lassen.

```rust
//! Ein winziges Paket, dessen Doku-Kommentar ein Beispiel trägt.

/// Zählt die Wörter in einem Text.
///
/// ```
/// use notiz::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// ```
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
```

`cargo test` sagt dazu:

```text
running 1 test
test src\lib.rs - count_words (line 5) ... FAILED

failures:

---- src\lib.rs - count_words (line 5) stdout ----
error[E0432]: unresolved import `notiz::word_count`
 --> src\lib.rs:7:5
  |
7 | use notiz::word_count;
  |     ^^^^^^^^^^^^^^^^^ no `word_count` in the root

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.

failures:
    src\lib.rs - count_words (line 5)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s
```

Genau das ist der Sinn der Sache. Ohne den Doku-Test stünde die alte Zeile
weiter da und niemand wüsste es. Die Meldung nennt die Zeile im Beispiel und
nicht im Rumpf, denn übersetzt wurde das Beispiel.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`. Rot sind deshalb zweierlei Tests: die
in `tests/exercise.rs` und die Beispiele in den Doku-Kommentaren der drei
Aufgaben. `word_count` und `word_at` stehen fertig da, ihre Beispiele sind grün.

- `initials` baut die Initialen eines Namens
- `percent` liest eine Prozentzahl, mit versteckten Zeilen und `?` im Beispiel
- `longest` gibt den längeren zweier Texte zurück, mit einer Lebensdauer

```console
cd units/05-06-doku-tests
cargo test
```

### Quelle

    Buch, Kapitel 14 "More about Cargo and Crates.io", Abschnitt 14.2
    "Publishing a Crate to Crates.io", Unterabschnitt "Documentation Comments as
    Tests",
    https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A doc comment starts with `///` and describes whatever stands directly below it.
What sits inside it between three backticks is not a quotation, it is a program.

`cargo test` collects those examples, builds a small program of its own out of
each one and runs it. At the end of the run there is a section of its own,
"Doc-tests", with one line per example.

Because the example is a program of its own, it only sees the public interface
of the package. That is why a `use` stands inside it, like in any other program,
and why it cannot use anything that is not `pub`.

### What it is good for

An example in prose goes stale in silence. When a function is renamed or gains
one more parameter, the prose next to it stays where it is and keeps claiming
something that no longer holds. Nobody notices until somebody types the example
out and gets the compiler message.

A doc test cannot do that. It is built on every run, so the rename shows up in
the same moment it happens. The example is then no longer a claim about the
code, it is a statement the compiler recomputes.

On top of that, an example is written for a reader. A test in `tests/` checks
edge cases, a doc test shows the straight path. Both are needed, and they do not
replace each other.

### The explanation

A package `notiz` with three examples: one plain, one with hidden lines and `?`,
and one that is meant to abort.

```rust
//! Ein winziges Paket, dessen Doku-Kommentare Beispiele tragen.

/// Zählt die Wörter in einem Text.
///
/// Alles zwischen den Rückstrichen ist ein Beispiel. `cargo test` baut es als
/// eigenes kleines Programm und führt es aus.
///
/// ```
/// use notiz::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// assert_eq!(word_count("   "), 0);
/// ```
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Liest eine Prozentzahl aus einem Text.
///
/// Die Zeilen mit `#` am Anfang laufen mit, werden aber nicht abgedruckt. So
/// darf das Beispiel `?` benutzen, ohne dass der Leser das `main` drumherum
/// sieht.
///
/// ```
/// # use notiz::percent;
/// # fn main() -> Result<(), std::num::ParseIntError> {
/// assert_eq!(percent(" 42 ")?, 42);
/// # Ok(())
/// # }
/// ```
pub fn percent(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.trim().parse()
}

/// Gibt das Wort an `stelle` zurück und bricht ab, wenn es dort keines gibt.
///
/// ```
/// use notiz::word_at;
///
/// assert_eq!(word_at("ein zwei drei", 1), "zwei");
/// ```
///
/// `should_panic` sagt, dass dieses Beispiel abbrechen soll.
///
/// ```should_panic
/// use notiz::word_at;
///
/// word_at("ein", 5);
/// ```
pub fn word_at(text: &str, stelle: usize) -> &str {
    text.split_whitespace()
        .nth(stelle)
        .expect("an dieser Stelle steht kein Wort")
}
```

`cargo test` prints for it:

```text
   Doc-tests notiz

running 4 tests
test src\lib.rs - percent (line 24) ... ok
test src\lib.rs - word_at (line 45) - should panic ... ok
test src\lib.rs - word_count (line 8) ... ok
test src\lib.rs - word_at (line 37) ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.11s
```

Four tests out of three functions, because `word_at` carries two examples. Every
line names the file and the line where the example begins, and that is the place
where it gets repaired.

A word on `should_panic` here. It takes no expected message, so it accepts every
abort, including one for an entirely different reason. Whoever wants to pin the
reason down writes the test in `tests/`, where
`#[should_panic(expected = "...")]` is available.

### Common mistakes

Renaming the function and leaving the example where it is.

```rust
//! Ein winziges Paket, dessen Doku-Kommentar ein Beispiel trägt.

/// Zählt die Wörter in einem Text.
///
/// ```
/// use notiz::word_count;
///
/// assert_eq!(word_count("ein zwei drei"), 3);
/// ```
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
```

`cargo test` answers:

```text
running 1 test
test src\lib.rs - count_words (line 5) ... FAILED

failures:

---- src\lib.rs - count_words (line 5) stdout ----
error[E0432]: unresolved import `notiz::word_count`
 --> src\lib.rs:7:5
  |
7 | use notiz::word_count;
  |     ^^^^^^^^^^^^^^^^^ no `word_count` in the root

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0432`.
Couldn't compile the test.

failures:
    src\lib.rs - count_words (line 5)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s
```

That is exactly the point of the thing. Without the doc test the old line would
still be standing there and nobody would know. The message names the line inside
the example and not inside the body, because what was compiled is the example.

### The exercises

The bodies in `src/lib.rs` are `todo!()`. Two kinds of test are red for that
reason: the ones in `tests/exercise.rs` and the examples in the doc comments of
the three exercises. `word_count` and `word_at` stand there finished, their
examples are green.

- `initials` builds the initials of a name
- `percent` reads a percentage, with hidden lines and `?` in the example
- `longest` returns the longer of two texts, with a lifetime

```console
cd units/05-06-doku-tests
cargo test
```

### Source

    Book, chapter 14 "More about Cargo and Crates.io", section 14.2 "Publishing
    a Crate to Crates.io", subsection "Documentation Comments as Tests",
    https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
