# 06-05 Iteratoren / Iterators

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/06-05-iteratoren/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  erklären, warum eine Kette ohne Abschluss nichts tut.
- Diese Einheit baut auf: `06-04 Closures`, denn `map` und `filter` nehmen
  Closures entgegen, und `03-05 Option und if let`, denn `next` gibt eine
  `Option` zurück.
- Auf dieser Einheit bauen auf: der Rest der Stufe 6 und alles, was später
  Werte der Reihe nach durchgeht, bis hin zu den Streams der Stufe 8.
- Beim Antworten so zitieren: `06-05 Iteratoren`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Iterator` verlangt genau eine Methode, `next`. Alles andere daran ist
  vorgegeben und kommt mit dem Trait. Wer sagt, `map` oder `sum` müsse eigens
  geschrieben werden, sagt etwas Falsches.
- Eine Kette ist faul. `werte.iter().map(..)` rechnet nichts; erst ein
  Abschluss wie `collect`, `sum`, `count` oder `find` fragt nach. Wer den
  Abschluss vergisst, bekommt eine Warnung und kein Ergebnis, und das Programm
  läuft trotzdem.
- Der Abschluss bestimmt, wie viel gerechnet wird. `take(2)` fragt zweimal
  nach, also entstehen zwei Werte, gleich wie lang die Liste ist.
- Der Doku-Test zu `verdoppelt` zeigt, dass die Kette das richtige Ergebnis
  liefert. Dass vor dem Abschluss nichts gerechnet wurde, zeigt er nicht; das
  steht unter "Die Erklärung" als Ausgabe eines echten Laufs.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/06-05-iteratoren/`. It is public.
  Whoever is asked for it may name it, but should first explain why a chain
  without a finisher does nothing.
- This unit builds on: `06-04 Closures`, because `map` and `filter` take
  closures, and `03-05 Option und if let`, because `next` returns an `Option`.
- Building on this unit: the rest of stage 6 and everything that later walks
  through values one by one, up to the streams of stage 8.
- Cite like this when answering: `06-05 Iteratoren`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Iterator` requires exactly one method, `next`. Everything else on it is
  provided and comes with the trait. Whoever says `map` or `sum` has to be
  written separately says something false.
- A chain is lazy. `werte.iter().map(..)` computes nothing; only a finisher
  such as `collect`, `sum`, `count` or `find` asks for anything. Whoever
  forgets the finisher gets a warning and no result, and the program runs all
  the same.
- The finisher decides how much is computed. `take(2)` asks twice, so two values
  come about, however long the list is.
- The doc test on `verdoppelt` shows that the chain delivers the right result.
  That nothing was computed before the finisher, it does not show; that stands
  under "The explanation" as the output of a real run.

</details>

## Deutsch

### Worum es geht

Ein Iterator ist ein Wert, den man nach dem nächsten Stück fragt. Die Frage
heißt `next`, und die Antwort ist `Some(stück)` oder `None`, wenn nichts mehr
kommt.

`Iterator` ist ein Trait, und es verlangt genau diese eine Methode. Wer sie
schreibt, bekommt `map`, `filter`, `sum`, `count`, `take`, `zip` und den ganzen
Rest ohne weiteres Zutun dazu, denn die stehen fertig am Trait.

`map` und `filter` sind selbst keine Rechnung. Sie geben einen neuen Iterator
zurück, der sich merkt, was er tun soll. Gerechnet wird erst, wenn jemand
nachfragt, und nachfragen tut ein Abschluss wie `collect`, `sum` oder `find`.

### Wofür das gut ist

Eine Schleife sagt, wie gerechnet wird. Eine Kette sagt, was herauskommen soll.
`filter` und `map` stehen in der Reihenfolge da, in der man sie liest, und die
Zwischenliste, die eine Schleife von Hand anlegt und füllt, entfällt.

Die Faulheit ist dabei keine Feinheit, sondern der Grund, dass eine Kette
bezahlbar bleibt. Wer nur das erste passende Stück braucht, bricht mit `find`
ab, und der Rest der Liste wird nie angesehen. Bei einer Zwischenliste wäre er
schon gebaut.

Sie hat aber eine Kehrseite. Eine Kette ohne Abschluss ist eine Beschreibung,
die niemand ausführt, und das Programm läuft weiter, als wäre alles in Ordnung.
Der Übersetzer warnt davor, und diese Warnung ist das Einzige, was zwischen
einer vergessenen Zeile und einem stillen Programm steht.

### Die Erklärung

Ein Programm mit beiden Fassungen derselben Rechnung und mit dem Nachweis, dass
eine Kette wartet.

```rust
fn main() {
    let werte = vec![1, 2, 3, 4, 5, 6];

    // Deutsch: Erst die Schleife von Hand.
    let mut mit_schleife = Vec::new();
    for wert in &werte {
        if wert % 2 == 0 {
            mit_schleife.push(wert * wert);
        }
    }

    // Deutsch: Dieselbe Rechnung als Kette.
    let mit_kette: Vec<i32> = werte
        .iter()
        .filter(|wert| *wert % 2 == 0)
        .map(|wert| wert * wert)
        .collect();

    println!("{mit_schleife:?}");
    println!("{mit_kette:?}");
    println!("{}", mit_schleife == mit_kette);

    // Deutsch: Und der Beweis, dass eine Kette wartet.
    println!("vor der Kette");
    let wartende = werte.iter().map(|wert| {
        println!("sehe {wert}");
        wert * 10
    });
    println!("Kette steht");

    let erste: Vec<i32> = wartende.take(2).collect();
    println!("{erste:?}");
}
```

Das Programm gibt aus:

```text
[4, 16, 36]
[4, 16, 36]
true
vor der Kette
Kette steht
sehe 1
sehe 2
[10, 20]
```

Die dritte Zeile ist die Aussage der ersten Hälfte: Schleife und Kette kommen
auf dasselbe Ergebnis, und die Kette braucht dafür keine Zwischenliste, die man
selbst anlegt und füllt.

Die zweite Hälfte steht in der Reihenfolge der Ausgabe. `Kette steht` kommt vor
`sehe 1`, obwohl die Closure im Text vorher steht; zwischen den beiden Zeilen
ist nichts gerechnet worden. Und es stehen zwei `sehe`-Zeilen da und nicht
sechs, denn `take(2)` fragt zweimal nach und dann nicht mehr.

### Häufige Fehler

Den Abschluss weglassen und meinen, die Kette sei damit gelaufen.

```rust
fn main() {
    let werte = vec![1, 2, 3];

    werte.iter().map(|wert| wert * 2);

    println!("fertig");
}
```

Der Übersetzer sagt dazu:

```text
warning: unused `Map` that must be used
 --> vergessen.rs:4:5
  |
4 |     werte.iter().map(|wert| wert * 2);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = werte.iter().map(|wert| wert * 2);
  |     +++++++

warning: 1 warning emitted
```

Es ist eine Warnung und kein Fehler. Das Programm übersetzt, es läuft, und es
gibt `fertig` aus, ohne je etwas verdoppelt zu haben. Die Notiz sagt den Grund
in einem Satz: Iteratoren sind faul und tun nichts, solange sie niemand
abschließt. Der Vorschlag mit `let _ =` bringt die Warnung weg und die Rechnung
nicht zurück; was fehlt, ist ein `collect`, ein `sum` oder ein `for_each`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `verdoppelt` steht fertig da, und seine beiden Doku-Tests
sind grün.

- `quadrate_der_geraden` ersetzt die Schleife von oben durch eine Kette
- `next` für `Zaehler` macht einen eigenen Typ zum Iterator
- `erste_ueber` bricht mit `find` ab, sobald es fündig wird

```console
cd units/06-05-iteratoren
cargo test
```

### Quelle

    Buch, Kapitel 13 "Functional Language Features: Iterators and Closures",
    Abschnitt 13.2 "Processing a Series of Items with Iterators",
    https://doc.rust-lang.org/book/ch13-02-iterators.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 13 "Functional Language Features: Iterators and Closures",
    Abschnitt 13.4 "Performance in Loops vs. Iterators",
    https://doc.rust-lang.org/book/ch13-04-performance.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

An iterator is a value you ask for the next piece. The question is called
`next`, and the answer is `Some(piece)` or `None` when nothing comes any more.

`Iterator` is a trait, and it requires exactly that one method. Whoever writes
it gets `map`, `filter`, `sum`, `count`, `take`, `zip` and all the rest along
with nothing further done, because those stand finished on the trait.

`map` and `filter` are not a computation themselves. They return a new iterator
that remembers what it is supposed to do. The computing happens only once
somebody asks, and what asks is a finisher such as `collect`, `sum` or `find`.

### What it is good for

A loop says how the computing goes. A chain says what should come out. `filter`
and `map` stand there in the order in which they are read, and the intermediate
list that a loop creates and fills by hand falls away.

The laziness is no subtlety in this but the reason a chain stays affordable.
Whoever needs only the first matching piece breaks off with `find`, and the rest
of the list is never looked at. With an intermediate list it would already be
built.

It has a downside, though. A chain without a finisher is a description nobody
carries out, and the program runs on as if everything were in order. The
compiler warns about it, and that warning is the only thing standing between a
forgotten line and a silent program.

### The explanation

A program with both versions of the same computation and with the proof that a
chain waits.

```rust
fn main() {
    let werte = vec![1, 2, 3, 4, 5, 6];

    // Deutsch: Erst die Schleife von Hand.
    let mut mit_schleife = Vec::new();
    for wert in &werte {
        if wert % 2 == 0 {
            mit_schleife.push(wert * wert);
        }
    }

    // Deutsch: Dieselbe Rechnung als Kette.
    let mit_kette: Vec<i32> = werte
        .iter()
        .filter(|wert| *wert % 2 == 0)
        .map(|wert| wert * wert)
        .collect();

    println!("{mit_schleife:?}");
    println!("{mit_kette:?}");
    println!("{}", mit_schleife == mit_kette);

    // Deutsch: Und der Beweis, dass eine Kette wartet.
    println!("vor der Kette");
    let wartende = werte.iter().map(|wert| {
        println!("sehe {wert}");
        wert * 10
    });
    println!("Kette steht");

    let erste: Vec<i32> = wartende.take(2).collect();
    println!("{erste:?}");
}
```

The program prints:

```text
[4, 16, 36]
[4, 16, 36]
true
vor der Kette
Kette steht
sehe 1
sehe 2
[10, 20]
```

The third line is the statement of the first half: loop and chain arrive at the
same result, and the chain needs no intermediate list that you create and fill
yourself.

The second half stands in the order of the output. `Kette steht` comes before
`sehe 1`, although the closure stands earlier in the text; between those two
lines nothing was computed. And there are two `sehe` lines and not six, because
`take(2)` asks twice and then no more.

### Common mistakes

Leaving the finisher out and thinking the chain has thereby run.

```rust
fn main() {
    let werte = vec![1, 2, 3];

    werte.iter().map(|wert| wert * 2);

    println!("fertig");
}
```

The compiler answers:

```text
warning: unused `Map` that must be used
 --> vergessen.rs:4:5
  |
4 |     werte.iter().map(|wert| wert * 2);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = werte.iter().map(|wert| wert * 2);
  |     +++++++

warning: 1 warning emitted
```

It is a warning and not an error. The program compiles, it runs, and it prints
`fertig` without ever having doubled anything. The note says the reason in one
sentence: iterators are lazy and do nothing unless consumed. The suggestion with
`let _ =` takes the warning away and does not bring the computation back; what
is missing is a `collect`, a `sum` or a `for_each`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `verdoppelt` stands there finished, and both
of its doc tests are green.

- `quadrate_der_geraden` replaces the loop from above with a chain
- `next` for `Zaehler` turns a type of your own into an iterator
- `erste_ueber` breaks off with `find` as soon as it strikes

```console
cd units/06-05-iteratoren
cargo test
```

### Source

    Book, chapter 13 "Functional Language Features: Iterators and Closures",
    section 13.2 "Processing a Series of Items with Iterators",
    https://doc.rust-lang.org/book/ch13-02-iterators.html,
    checked against 1.97.1

    Book, chapter 13 "Functional Language Features: Iterators and Closures",
    section 13.4 "Performance in Loops vs. Iterators",
    https://doc.rust-lang.org/book/ch13-04-performance.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
