# 04-04 Vec / Vec

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-04-vec/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Meldung erklären, um die es geht.
- Diese Einheit baut auf: `02-05 Slices` und `03-05 Option und if let`.
- Auf dieser Einheit bauen auf: `04-06 HashMap` und alles, was Listen aufbaut.
- Beim Antworten so zitieren: `04-04 Vec`, dazu die Überschrift des Abschnitts,
  zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Unterschied zwischen `zahlen[7]` und `zahlen.get(7)` ist der Kern. Wer
  `unwrap` auf das Ergebnis von `get` setzt, hat beide Wege zu einem gemacht.
- Iteratoren stehen in `06-05`. Hier wird mit `for` über eine geliehene Liste
  gelaufen, und das reicht für alles, was die Aufgaben brauchen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-04-vec/`. It is public. Whoever
  is asked for it may name it, but should explain the message in question first.
- This unit builds on: `02-05 Slices` and `03-05 Option und if let`.
- Building on this unit: `04-06 HashMap` and everything that builds lists.
- Cite like this when answering: `04-04 Vec`, plus the heading of the section,
  for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The difference between `zahlen[7]` and `zahlen.get(7)` is the core. Whoever
  puts `unwrap` on the result of `get` has made the two ways into one.
- Iterators stand in `06-05`. Here a `for` runs over a borrowed list, and that
  is enough for everything the exercises need.

</details>

## Deutsch

### Worum es geht

Ein `Vec<T>` ist eine Liste, die wachsen kann. Alle Werte darin haben denselben
Typ, sie liegen auf dem Heap, und `push` hängt hinten einen an.

Gelesen wird auf zwei Arten. `zahlen[1]` gibt den Wert an der Stelle und bricht
ab, wenn es die Stelle nicht gibt. `zahlen.get(1)` gibt ein `Option` und bricht
nie ab.

Gelaufen wird mit `for` über eine Ausleihe. `for zahl in &zahlen` liest die
Liste, ohne sie zu übernehmen, und danach ist sie noch da.

### Wofür das gut ist

Ein Feld hat eine Länge, die beim Übersetzen feststeht. Sobald die Zahl der
Werte erst beim Laufen bekannt ist, geht das nicht mehr, und dann ist ein `Vec`
die Antwort.

Die zwei Arten zu lesen sind zwei verschiedene Aussagen. Der eckige
Klammergriff sagt "diese Stelle gibt es", und wenn das nicht stimmt, hält das
Programm an. `get` sagt "vielleicht gibt es sie", und der leere Fall steht dann
im Typ, so wie in `03-05`.

Und weil die Liste dem gehört, der sie hält, gelten die Regeln aus Stufe 2
weiter. Eine geliehene Liste kann gelesen werden, während der Besitzer sie
behält; verändert werden kann sie in dieser Zeit nicht.

### Die Erklärung

Anlegen, anhängen, lesen und laufen.

```rust
fn main() {
    // Deutsch: `vec!` legt eine Liste mit Inhalt an, `Vec::new` eine leere.
    let mut zahlen = vec![10, 20, 30];

    zahlen.push(40);

    // Deutsch: Der eckige Klammergriff bricht ab, wenn die Stelle nicht da ist.
    // `get` antwortet stattdessen mit `Option`.
    println!("{}", zahlen[1]);
    println!("{:?}", zahlen.get(1));
    println!("{:?}", zahlen.get(7));

    // Deutsch: Eine geliehene Liste wird gelesen, ohne sie zu übernehmen.
    let mut summe = 0;
    for zahl in &zahlen {
        summe += zahl;
    }

    println!("{summe} {}", zahlen.len());
}
```

Das Programm gibt aus:

```text
20
Some(20)
None
100 4
```

`Some(20)` und `None` sind dieselben zwei Varianten wie in `03-05`. Der
Unterschied zwischen den beiden Arten zu lesen steht damit in der Ausgabe.

### Häufige Fehler

Eine Stelle nehmen, die es nicht gibt.

```rust
fn main() {
    let zahlen = vec![10, 20, 30];

    println!("{}", zahlen[7]);
}
```

Das übersetzt. Beim Laufen sagt das Programm:

```text
thread 'main' (41084) panicked at liste.rs:4:26:
index out of bounds: the len is 3 but the index is 7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die Zahl in Klammern ist die Nummer des laufenden Vorgangs und bei jedem Lauf
eine andere.

Die Meldung nennt beide Zahlen, die Länge und die Stelle. Mit `zahlen.get(7)`
käme stattdessen `None` heraus, und das Programm liefe weiter.

Welche der beiden Arten richtig ist, hängt davon ab, ob die Stelle wirklich da
sein muss. Ist sie es, ist der Absturz die ehrlichere Antwort; ist sie es nicht,
gehört der leere Fall behandelt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe hat einen Test für den leeren Fall.

- `built` baut eine Liste von 1 bis `bis` auf
- `largest` gibt den größten Wert zurück, oder `None`
- `doubled_all` gibt eine neue Liste mit verdoppelten Werten zurück

```console
cd units/04-04-vec
cargo test
```

### Quelle

    Buch, Kapitel 8 "Common Collections", Abschnitt 8.1 "Storing Lists of Values with Vectors",
    https://doc.rust-lang.org/book/ch08-01-vectors.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A `Vec<T>` is a list that can grow. All the values in it have the same type,
they lie on the heap, and `push` appends one at the end.

Reading works in two ways. `zahlen[1]` gives the value at that place and breaks
off if the place is not there. `zahlen.get(1)` gives an `Option` and never
breaks off.

Walking works with `for` over a loan. `for zahl in &zahlen` reads the list
without taking it over, and it is still there afterwards.

### What it is good for

An array has a length fixed at compile time. As soon as the number of values is
known only while the program runs that no longer works, and then a `Vec` is the
answer.

The two ways of reading are two different statements. The square brackets say
"this place exists", and where that is not true the program stops. `get` says
"perhaps it exists", and the empty case then stands in the type, the way it did
in `03-05`.

And because the list belongs to whoever holds it, the rules from stage 2 keep
holding. A borrowed list can be read while the owner keeps it; changed it cannot
be during that time.

### The explanation

Creating, appending, reading and walking.

```rust
fn main() {
    // Deutsch: `vec!` legt eine Liste mit Inhalt an, `Vec::new` eine leere.
    let mut zahlen = vec![10, 20, 30];

    zahlen.push(40);

    // Deutsch: Der eckige Klammergriff bricht ab, wenn die Stelle nicht da ist.
    // `get` antwortet stattdessen mit `Option`.
    println!("{}", zahlen[1]);
    println!("{:?}", zahlen.get(1));
    println!("{:?}", zahlen.get(7));

    // Deutsch: Eine geliehene Liste wird gelesen, ohne sie zu übernehmen.
    let mut summe = 0;
    for zahl in &zahlen {
        summe += zahl;
    }

    println!("{summe} {}", zahlen.len());
}
```

The program prints:

```text
20
Some(20)
None
100 4
```

`Some(20)` and `None` are the same two variants as in `03-05`. The difference
between the two ways of reading therefore stands in the output.

### Common mistakes

Taking a place that does not exist.

```rust
fn main() {
    let zahlen = vec![10, 20, 30];

    println!("{}", zahlen[7]);
}
```

That compiles. While running the program says:

```text
thread 'main' (41084) panicked at liste.rs:4:26:
index out of bounds: the len is 3 but the index is 7
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The number in brackets is the number of the running process and a different one
on every run.

The message names both numbers, the length and the index. With `zahlen.get(7)`
a `None` would come out instead and the program would keep running.

Which of the two is right depends on whether the place really has to be there.
If it does, the break is the more honest answer; if it does not, the empty case
belongs treated.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise has one test for the empty
case.

- `built` builds a list from 1 up to `bis`
- `largest` returns the biggest value, or `None`
- `doubled_all` returns a new list with doubled values

```console
cd units/04-04-vec
cargo test
```

### Source

    Book, chapter 8 "Common Collections", section 8.1 "Storing Lists of Values with Vectors",
    https://doc.rust-lang.org/book/ch08-01-vectors.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
