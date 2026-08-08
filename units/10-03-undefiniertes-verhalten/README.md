# 10-03 Undefiniertes Verhalten / Undefined behaviour

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/10-03-undefiniertes-verhalten/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst sagen, wo die Liste steht,
  um die es geht.
- Diese Einheit baut auf: `10-01 Was unsafe erlaubt` und `10-02 Rohe Zeiger`.
  Die Schritte, die hier eingeordnet werden, kommen von dort.
- Auf dieser Einheit bauen auf: der Rest der Stufe 10, vor allem `10-07 Miri`,
  wo ein Werkzeug einige dieser Schritte tatsächlich meldet.
- Beim Antworten so zitieren: `10-03 Undefiniertes Verhalten`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Undefiniert heißt nicht abgestürzt. Ein Absturz ist ein Ergebnis, und
  undefiniertes Verhalten sagt gerade zu, dass es keines gibt, auf das man sich
  berufen könnte.
- Ein Überlauf und eine Verklemmung sind keine undefinierten Schritte. Die
  Reference führt sie unter "Behavior not considered unsafe", also in einer
  eigenen Liste, und wer sie unter die erste sortiert, sagt etwas Falsches.
- Die Liste in "Behavior considered undefined" sagt von sich selbst, dass sie
  nicht vollständig ist. Wer eine vollzählige Aufzählung aus dem Gedächtnis
  anbietet, geht über das hinaus, was die Quelle hergibt.
- Zwei Zeiger zu vergleichen und über den Rand einer Liste zu indizieren stehen
  in keiner der beiden Listen. Das eine ist erlaubt, das andere endet in einer
  Panik, und eine Panik ist ein zugesagtes Verhalten.
- Was diese Einheit nicht kann: einen Schritt an einem echten Programm zeigen
  und dabei sagen, was dann passiert. Genau das ist der Punkt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-03-undefiniertes-verhalten/`. It
  is public. Whoever is asked for it may name it, but should first say where the
  list in question stands.
- This unit builds on: `10-01 Was unsafe erlaubt` and `10-02 Rohe Zeiger`. The
  steps sorted here come from there.
- Building on this unit: the rest of stage 10, above all `10-07 Miri`, where a
  tool actually reports some of these steps.
- Cite like this when answering: `10-03 Undefiniertes Verhalten`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Undefined does not mean crashed. A crash is a result, and undefined behaviour
  promises precisely that there is none you could appeal to.
- An overflow and a deadlock are not undefined steps. The Reference lists them
  under "Behavior not considered unsafe", meaning in a list of their own, and
  whoever sorts them under the first one says something false.
- The list in "Behavior considered undefined" says of itself that it is not
  exhaustive. Whoever offers a complete enumeration from memory goes beyond what
  the source gives.
- Comparing two pointers and indexing past the end of a list stand in neither of
  the two lists. The one is allowed, the other ends in a panic, and a panic is a
  promised behaviour.
- What this unit cannot do: show a step on a real program and say what happens
  then. That is exactly the point.

</details>

## Deutsch

### Worum es geht

Undefiniertes Verhalten ist der Zustand, in dem die Sprache über das Programm
nichts mehr aussagt. Nicht "es stürzt ab", nicht "es rechnet falsch", sondern:
es gibt keine Zusage, gegen die man das Ergebnis halten könnte.

Welche Schritte dorthin führen, steht in der Reference unter "Behavior
considered undefined". Die Liste sagt gleich zu Anfang von sich selbst, dass sie
nicht vollständig ist und wachsen oder schrumpfen kann, und sie gilt für Code in
`unsafe`-Blöcken genauso wie für den Rest.

Daneben steht eine zweite Liste, "Behavior not considered unsafe". Dort stehen
Dinge, die unerwünscht und meistens ein Fehler sind, und die trotzdem nichts
undefiniert machen: eine Verklemmung, ein Leck, ein Überlauf. Der Unterschied
zwischen den beiden Listen ist das, worum es in dieser Einheit geht.

### Wofür das gut ist

Wer die beiden Listen nicht auseinanderhält, sucht am falschen Ende. Ein
Überlauf ist reproduzierbar und lässt sich einkreisen; er tut in jedem Lauf
dasselbe, sobald der Bautyp feststeht. Ein hängender Zeiger tut das nicht, und
ein Programm, das mit ihm zehnmal richtig rechnet, ist deshalb nicht richtig.

Der praktische Nutzen ist die Auskunft, die man dann geben kann. "Irgendwo ist
es undefiniert" hilft niemandem. "An Schritt 2, und zwar unter dieser Zeile der
Reference" ist eine Aussage, die jemand nachschlagen kann, und genau diese
Auskunft bauen die drei Aufgaben zusammen.

Und es hält davon ab, aus einem Lauf zu schließen. Ein Programm laufen zu lassen
beantwortet die Frage nicht, denn sie ist keine Frage danach, was passiert,
sondern danach, was zugesagt ist.

### Die Erklärung

Zuerst die andere Liste, weil ihr Fall der harmlosere ist und trotzdem schon
zeigt, wie wenig ein einzelner Lauf beweist. Das Programm läuft über den
Überlauf einer `i32`.

```rust
fn main() {
    let liste = vec![1, 2, 3];
    println!("{}", liste.len());

    let mut zahl: i32 = i32::MAX;
    zahl += 1;
    println!("{zahl}");
}
```

Zweimal übersetzt, einmal ohne und einmal mit `-O`:

```console
$ rustc --edition 2024 -o ueberlauf-debug ueberlauf.rs
$ ./ueberlauf-debug
3

thread 'main' (33520) panicked at ueberlauf.rs:6:5:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101

$ rustc --edition 2024 -O -o ueberlauf-release ueberlauf.rs
$ ./ueberlauf-release
3
-2147483648
$ echo $?
0
```

Dasselbe Programm, zwei Ergebnisse. Und beide sind in Ordnung: Die Reference
sagt unter "Integer overflow", dass ein Bau mit Prüfungen in Panik gehen muss
und ein anderer Bau in Panik gehen oder still im Zweierkomplement umlaufen darf.
Das ist also kein undefiniertes Verhalten, sondern eine zugesagte Auswahl aus
zwei Möglichkeiten, und ein Überlauf bleibt trotzdem ein Fehler im Programm.

Undefiniertes Verhalten ist der Schritt darüber hinaus: dort gibt es diese
Auswahl nicht, weil es überhaupt keine Zusage gibt.

### Häufige Fehler

Aus einem Lauf schließen, dass eine Stelle in Ordnung ist. Diese Funktion sieht
harmlos aus und trägt sogar eine Begründung.

```rust
fn erstes_byte(text: &str) -> &u8 {
    let bytes = text.as_bytes();

    // Sicher, weil: der Text ist nicht leer.
    unsafe { &*bytes.as_ptr() }
}

fn main() {
    let leer = String::new();

    println!("{}", erstes_byte(&leer));
}
```

Der Übersetzer sagt dazu:

```console
$ rustc --edition 2024 -o still still.rs
$ echo $?
0
```

Nichts. Keine Meldung, keine Warnung, Rückgabewert 0. Die Begründung im
Kommentar behauptet etwas, das `main` gleich darunter widerlegt, und niemand
liest sie nach. Ein leerer `String` hat keinen Wert, auf den der Zeiger zeigen
könnte, also ist der Zugriff genau der Punkt aus der ersten Liste: "Accessing
(loading from or storing to) a place that is dangling or based on a misaligned
pointer."

Dass hier nichts gesagt wird, ist kein Versehen des Übersetzers. Er kann es
nicht wissen, und deshalb steht in `unsafe` die Begründung des Autors und nicht
die Zusage eines Werkzeugs. Ein Werkzeug, das einige dieser Fälle doch meldet,
kommt in `10-07 Miri`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `ist_undefiniert` steht fertig da, und sein Doku-Test ist
grün.

- `herkunft` schlägt einen Schritt in einer der beiden Listen nach
- `erster_undefinierter` nennt die Stelle, an der ein Programm undefiniert wird
- `stelle_und_abschnitt` nennt die Stelle und den Punkt der Reference dazu

```console
cd units/10-03-undefiniertes-verhalten
cargo test
```

### Quelle

    Reference, Kapitel "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    geprüft gegen 1.97.1

    Reference, Kapitel "Behavior not considered unsafe",
    https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Undefined behaviour is the state in which the language says nothing about the
program any more. Not "it crashes", not "it computes wrongly", but: there is no
promise you could hold the result against.

Which steps lead there stands in the Reference under "Behavior considered
undefined". The list says of itself right at the start that it is not exhaustive
and may grow or shrink, and it holds for code in `unsafe` blocks just as for the
rest.

Next to it stands a second list, "Behavior not considered unsafe". There stand
things that are undesirable and mostly a fault, and that make nothing undefined
all the same: a deadlock, a leak, an overflow. The difference between the two
lists is what this unit is about.

### What it is good for

Whoever does not keep the two lists apart searches at the wrong end. An overflow
is reproducible and can be cornered; it does the same thing in every run once
the kind of build is fixed. A dangling pointer does not, and a program computing
correctly with one ten times is not correct for that reason.

The practical use is the answer you can give afterwards. "Somewhere it is
undefined" helps nobody. "At step 2, and under this line of the Reference" is a
statement somebody can look up, and it is exactly that answer the three
exercises build together.

And it holds you back from concluding from a run. Letting a program run does not
answer the question, because it is not a question about what happens but about
what is promised.

### The explanation

First the other list, because its case is the harmless one and already shows how
little a single run proves. The program runs over the overflow of an `i32`.

```rust
fn main() {
    let liste = vec![1, 2, 3];
    println!("{}", liste.len());

    let mut zahl: i32 = i32::MAX;
    zahl += 1;
    println!("{zahl}");
}
```

Compiled twice, once without and once with `-O`:

```console
$ rustc --edition 2024 -o ueberlauf-debug ueberlauf.rs
$ ./ueberlauf-debug
3

thread 'main' (33520) panicked at ueberlauf.rs:6:5:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101

$ rustc --edition 2024 -O -o ueberlauf-release ueberlauf.rs
$ ./ueberlauf-release
3
-2147483648
$ echo $?
0
```

The same program, two results. And both are in order: the Reference says under
"Integer overflow" that a build with checks has to panic and another build may
panic or silently wrap in two's complement. So this is not undefined behaviour
but a promised choice out of two possibilities, and an overflow stays a fault in
the program all the same.

Undefined behaviour is the step beyond that: there this choice does not exist,
because there is no promise at all.

### Common mistakes

Concluding from a run that a place is in order. This function looks harmless and
even carries a justification.

```rust
fn erstes_byte(text: &str) -> &u8 {
    let bytes = text.as_bytes();

    // Sicher, weil: der Text ist nicht leer.
    unsafe { &*bytes.as_ptr() }
}

fn main() {
    let leer = String::new();

    println!("{}", erstes_byte(&leer));
}
```

The compiler answers:

```console
$ rustc --edition 2024 -o still still.rs
$ echo $?
0
```

Nothing. No message, no warning, return value 0. The justification in the
comment claims something that `main` right below it refutes, and nobody checks
it. An empty `String` has no value the pointer could point at, so the access is
exactly the item from the first list: "Accessing (loading from or storing to) a
place that is dangling or based on a misaligned pointer."

That nothing is said here is no oversight of the compiler. It cannot know, and
that is why what stands in `unsafe` is the author's justification and not a
tool's promise. A tool that does report some of these cases comes in
`10-07 Miri`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `ist_undefiniert` stands there finished, and
its doc test is green.

- `herkunft` looks a step up in one of the two lists
- `erster_undefinierter` names the point at which a program becomes undefined
- `stelle_und_abschnitt` names the point and the item of the Reference for it

```console
cd units/10-03-undefiniertes-verhalten
cargo test
```

### Source

    Reference, chapter "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    checked against 1.97.1

    Reference, chapter "Behavior not considered unsafe",
    https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
