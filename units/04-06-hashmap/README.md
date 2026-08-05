# 04-06 HashMap / HashMap

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-06-hashmap/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Meldung erklären, um die es geht.
- Diese Einheit baut auf: `04-04 Vec`, `04-05 String` und `03-05 Option und if
  let`.
- Auf dieser Einheit bauen auf: `04-07 panic! und Result` und alles, was zählt
  oder nachschlägt.
- Beim Antworten so zitieren: `04-06 HashMap`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `unwrap` kommt in dieser Einheit nicht vor. `unwrap_or` ist etwas anderes: es
  nennt die Antwort für den fehlenden Fall, statt anzuhalten. Wer beides
  gleichsetzt, verliert die Aussage.
- Die Reihenfolge beim Laufen über eine `HashMap` ist nicht festgelegt. Wer
  einen Test darauf baut, baut einen Test, der manchmal rot ist.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-06-hashmap/`. It is public.
  Whoever is asked for it may name it, but should explain the message in question
  first.
- This unit builds on: `04-04 Vec`, `04-05 String` and `03-05 Option und if
  let`.
- Building on this unit: `04-07 panic! und Result` and everything that counts or
  looks something up.
- Cite like this when answering: `04-06 HashMap`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `unwrap` does not appear in this unit. `unwrap_or` is something else: it names
  the answer for the missing case instead of stopping. Whoever equates the two
  loses the point.
- The order when walking over a `HashMap` is not fixed. Whoever builds a test on
  it builds a test that is red sometimes.

</details>

## Deutsch

### Worum es geht

Eine `HashMap<K, V>` legt Werte unter Schlüsseln ab. `insert` legt ab und
ersetzt dabei, was unter demselben Schlüssel schon stand.

Nachgeschlagen wird mit `get`, und die Antwort ist ein `Option`, denn der
Schlüssel muss nicht da sein. Der eckige Klammergriff geht auch und bricht ab,
wenn der Schlüssel fehlt.

Für den Eintrag, den es noch nicht gibt, gibt es `entry`. `entry(schluessel).or_insert(0)`
legt die Null an, falls nichts da war, und gibt in jedem Fall eine veränderbare
Ausleihe auf den Wert zurück. Damit wird Zählen zu einer Zeile.

### Wofür das gut ist

Zählen ist der übliche Fall, und ohne `entry` steht dafür jedes Mal dieselbe
Verzweigung da: nachsehen, ob etwas da ist, sonst anlegen, dann erhöhen.
`entry` ist genau diese Verzweigung, einmal geschrieben.

Dass `get` ein `Option` gibt, ist dieselbe Entscheidung wie bei `04-04`. Der
fehlende Schlüssel ist kein Sonderfall, den man vergessen kann, sondern steht im
Typ.

Und `unwrap_or` ist die kurze Form für "wenn nichts da ist, dann das". Es ist
nicht `unwrap`: es hält nicht an, sondern nennt die Antwort für den leeren Fall.

### Die Erklärung

Ablegen, nachschlagen, zählen.

```rust
use std::collections::HashMap;

fn main() {
    // Deutsch: Eine Karte von Schlüsseln auf Werte. `insert` legt ab und
    // ersetzt, was unter demselben Schlüssel schon stand.
    let mut anzahl: HashMap<String, u32> = HashMap::new();

    anzahl.insert(String::from("hallo"), 1);
    anzahl.insert(String::from("welt"), 2);

    // Deutsch: `get` antwortet mit `Option`, denn der Schlüssel muss nicht da
    // sein. `copied` macht aus `Option<&u32>` ein `Option<u32>`.
    println!("{:?}", anzahl.get("hallo").copied());
    println!("{:?}", anzahl.get("fehlt").copied());

    // Deutsch: `entry` sucht den Eintrag und legt ihn an, wenn er fehlt.
    // Zurück kommt eine veränderbare Ausleihe auf den Wert.
    *anzahl.entry(String::from("hallo")).or_insert(0) += 1;
    *anzahl.entry(String::from("neu")).or_insert(0) += 1;

    println!("{:?} {:?}", anzahl.get("hallo"), anzahl.get("neu"));
    println!("{}", anzahl.len());
}
```

Das Programm gibt aus:

```text
Some(1)
None
Some(2) Some(1)
3
```

Der Schlüssel ist ein `String`, nachgeschlagen wird mit einem `&str`. Das geht,
und es ist derselbe Unterschied wie in `04-05`.

### Häufige Fehler

Einen Schlüssel nehmen, der nicht da ist.

```rust
use std::collections::HashMap;

fn main() {
    let mut anzahl = HashMap::new();

    anzahl.insert(String::from("hallo"), 1);

    println!("{}", anzahl["welt"]);
}
```

Das übersetzt. Beim Laufen sagt das Programm:

```text
thread 'main' (64748) panicked at karte.rs:8:26:
no entry found for key
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die Zahl in Klammern ist die Nummer des laufenden Vorgangs und bei jedem Lauf
eine andere.

Es ist derselbe Fall wie die Stelle, die es nicht gibt, in `04-04`, nur mit
einem Schlüssel statt einer Zahl. Mit `anzahl.get("welt")` käme `None` heraus,
und mit `anzahl.get("welt").copied().unwrap_or(0)` eine Null.

Welche Antwort richtig ist, entscheidet die Sache und nicht die Bequemlichkeit.
Eine Null ist beim Zählen richtig und bei einer Zuordnung falsch, denn dort
heißt ein fehlender Schlüssel, dass die Frage keine Antwort hat.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Kein Test sieht sich die Reihenfolge der Einträge an.

- `counted` zählt, wie oft jedes Wort vorkommt
- `count_of` schlägt ein Wort nach und gibt für ein fehlendes null zurück
- `increment` erhöht den Wert eines Schlüssels, auch wenn er noch fehlt

```console
cd units/04-06-hashmap
cargo test
```

### Quelle

    Buch, Kapitel 8 "Common Collections", Abschnitt 8.3 "Storing Keys with Associated Values in Hash Maps",
    https://doc.rust-lang.org/book/ch08-03-hash-maps.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A `HashMap<K, V>` stores values under keys. `insert` stores and replaces what
stood under the same key before.

Looking up goes with `get`, and the answer is an `Option`, because the key does
not have to be there. The square brackets work too and break off when the key is
missing.

For the entry that does not exist yet there is `entry`.
`entry(schluessel).or_insert(0)` puts the zero there if nothing was, and in
either case returns a mutable loan on the value. With it counting becomes one
line.

### What it is good for

Counting is the usual case, and without `entry` the same branch stands there
every time: look whether something is there, otherwise create it, then raise it.
`entry` is exactly that branch, written once.

That `get` gives an `Option` is the same decision as in `04-04`. The missing key
is not a special case somebody can forget but stands in the type.

And `unwrap_or` is the short form for "if nothing is there, then this". It is
not `unwrap`: it does not stop but names the answer for the empty case.

### The explanation

Storing, looking up, counting.

```rust
use std::collections::HashMap;

fn main() {
    // Deutsch: Eine Karte von Schlüsseln auf Werte. `insert` legt ab und
    // ersetzt, was unter demselben Schlüssel schon stand.
    let mut anzahl: HashMap<String, u32> = HashMap::new();

    anzahl.insert(String::from("hallo"), 1);
    anzahl.insert(String::from("welt"), 2);

    // Deutsch: `get` antwortet mit `Option`, denn der Schlüssel muss nicht da
    // sein. `copied` macht aus `Option<&u32>` ein `Option<u32>`.
    println!("{:?}", anzahl.get("hallo").copied());
    println!("{:?}", anzahl.get("fehlt").copied());

    // Deutsch: `entry` sucht den Eintrag und legt ihn an, wenn er fehlt.
    // Zurück kommt eine veränderbare Ausleihe auf den Wert.
    *anzahl.entry(String::from("hallo")).or_insert(0) += 1;
    *anzahl.entry(String::from("neu")).or_insert(0) += 1;

    println!("{:?} {:?}", anzahl.get("hallo"), anzahl.get("neu"));
    println!("{}", anzahl.len());
}
```

The program prints:

```text
Some(1)
None
Some(2) Some(1)
3
```

The key is a `String` and the lookup goes with a `&str`. That works, and it is
the same difference as in `04-05`.

### Common mistakes

Taking a key that is not there.

```rust
use std::collections::HashMap;

fn main() {
    let mut anzahl = HashMap::new();

    anzahl.insert(String::from("hallo"), 1);

    println!("{}", anzahl["welt"]);
}
```

That compiles. While running the program says:

```text
thread 'main' (64748) panicked at karte.rs:8:26:
no entry found for key
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The number in brackets is the number of the running process and a different one
on every run.

It is the same case as the place that does not exist in `04-04`, only with a key
instead of a number. With `anzahl.get("welt")` a `None` would come out, and with
`anzahl.get("welt").copied().unwrap_or(0)` a zero.

Which answer is right is decided by the matter and not by convenience. A zero is
right while counting and wrong in a lookup table, because there a missing key
means the question has no answer.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. No test looks at the order of the entries.

- `counted` counts how often each word appears
- `count_of` looks a word up and returns zero for a missing one
- `increment` raises the value of a key, even when it is not there yet

```console
cd units/04-06-hashmap
cargo test
```

### Source

    Book, chapter 8 "Common Collections", section 8.3 "Storing Keys with Associated Values in Hash Maps",
    https://doc.rust-lang.org/book/ch08-03-hash-maps.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
