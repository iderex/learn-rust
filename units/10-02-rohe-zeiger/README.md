# 10-02 Rohe Zeiger / Raw pointers

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-02-rohe-zeiger/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-03 Ausleihen` und `02-04 Veränderbares
  Ausleihen`. Ein roher Zeiger ist das, was von einer Referenz übrig bleibt,
  wenn die Regeln wegfallen.
- Auf dieser Einheit bauen auf: der Rest der Stufe 10, also undefiniertes
  Verhalten, Varianz, FFI und Miri.
- Beim Antworten so zitieren: `10-02 Rohe Zeiger`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Einen rohen Zeiger anzulegen ist erlaubt und braucht kein `unsafe`. Erst das
  Lesen und das Schreiben durch ihn braucht eines.
- `unsafe` schaltet keine Prüfung des Übersetzers ab. Es sagt, dass der Autor
  die Bedingungen nachgesehen hat, die kein Werkzeug hier nachsehen kann.
- Eine `unsafe fn` verlangt in der Ausgabe 2024 auch innen einen
  `unsafe`-Block. Wer das Gegenteil behauptet, sagt bitte, gegen welche Ausgabe.
- Die Bedingungen für ein Dereferenzieren stehen in der Reference unter
  "Behavior considered undefined" und sind nicht vollzählig aus dem Gedächtnis
  aufzuzählen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-02-rohe-zeiger/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: `02-03 Ausleihen` and `02-04 Veränderbares Ausleihen`. A
  raw pointer is what is left of a reference once the rules fall away.
- Building on this unit: the rest of stage 10, meaning undefined behaviour,
  variance, FFI and Miri.
- Cite like this when answering: `10-02 Rohe Zeiger`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Creating a raw pointer is allowed and needs no `unsafe`. Only reading and
  writing through it needs one.
- `unsafe` switches off no check of the compiler. It says that the author has
  looked up the conditions no tool can look up here.
- An `unsafe fn` asks for an `unsafe` block inside as well in edition 2024.
  Whoever claims the opposite, please say against which edition.
- The conditions for a dereference stand in the Reference under "Behavior
  considered undefined" and are not to be listed in full from memory.

</details>

## Deutsch

### Worum es geht

Ein roher Zeiger ist eine Adresse. `*const T` liest, `*mut T` liest und
schreibt, und mehr steht nicht darin.

Er darf alles, was eine Referenz nicht darf. Er darf null sein, er darf auf
etwas zeigen, das es nicht mehr gibt, es dürfen mehrere veränderbare Zeiger auf
dieselbe Stelle zeigen, und niemand rechnet ihm eine Lebenszeit nach.

Angelegt und herumgereicht wird er ohne `unsafe`. Erst wer durch ihn liest oder
schreibt, braucht einen `unsafe`-Block, und dieser Block sagt nicht, dass hier
etwas Gefährliches passiert. Er sagt, dass der Autor die Bedingungen nachgesehen
hat, die der Übersetzer an dieser Stelle nicht nachsehen kann.

### Wofür das gut ist

Die meisten Programme brauchen keinen rohen Zeiger. Drei Fälle bleiben. Der
erste ist die Schnittstelle nach draußen, denn eine fremde Bibliothek kennt
Rusts Referenzen nicht. Der zweite sind Datenstrukturen, deren Form die
Ausleihregeln nicht abbilden, etwa eine Liste, in der zwei Knoten
aufeinander zeigen. Der dritte ist die Standardbibliothek selbst, deren sichere
Typen innen aus genau diesen Zeigern gebaut sind.

Der Gewinn ist in allen drei Fällen derselbe: Die unsichere Stelle wird klein
und sie wird sichtbar. Aufgabe 3 dieser Einheit ist die Gegenprobe dazu, denn
sie vergleicht zwei Zeiger und braucht dafür kein `unsafe`, weil nichts gelesen
wird.

Der Preis steht daneben. Ab hier prüft niemand mehr mit. Ein Fehler an dieser
Stelle ist kein falsches Ergebnis, sondern ein Programm, dessen Verhalten nicht
mehr festgelegt ist, und das kann in einem Lauf richtig aussehen und im
nächsten nicht.

### Die Erklärung

Zwei Zeiger, einmal ohne und einmal mit `unsafe`.

```rust
fn main() {
    let zahl = 5;
    let mut andere = 10;

    // Deutsch: Einen rohen Zeiger anzulegen ist erlaubt und braucht kein
    // `unsafe`. Er ist eine Adresse und sonst nichts.
    let nur_lesen = &zahl as *const i32;
    let schreiben = &mut andere as *mut i32;

    // Deutsch: Vergleichen und auf null prüfen geht ohne `unsafe`, denn dabei
    // wird nichts gelesen.
    println!("{}", std::ptr::eq(nur_lesen, &zahl));
    println!("{}", nur_lesen.is_null());

    // Deutsch: Lesen und Schreiben geht nur im `unsafe`-Block, und die
    // Begründung gehört daneben.
    //
    // Sicher, weil: Beide Zeiger kommen aus Referenzen auf Werte, die in diesem
    // Rahmen leben. Sie sind deshalb nicht null, ausgerichtet und gültig, und
    // auf dieselbe Stelle zeigt hier nichts sonst.
    unsafe {
        println!("{}", *nur_lesen);

        *schreiben += 1;

        println!("{}", *schreiben);
    }

    println!("{andere}");
}
```

Das Programm gibt aus:

```text
true
false
5
11
11
```

Die ersten beiden Zeilen sind die, die überrascht, wer `unsafe` für ansteckend
hält. Beide entstehen ohne einen einzigen `unsafe`-Block, denn ein Vergleich
zweier Adressen liest keine der beiden Stellen.

Der Block darunter trägt eine Begründung, und die ist kein Kommentar zur
Verzierung. Sie ist das Einzige, was an dieser Stelle die Arbeit tut, die sonst
der Übersetzer tut, und sie nennt die Bedingungen einzeln: nicht null,
ausgerichtet, gültig, und keine zweite Referenz auf dieselbe Stelle.

Die letzte Zeile zeigt, dass durch `schreiben` wirklich `andere` verändert
wurde. Die Adresse war dieselbe, und deshalb war es derselbe Wert.

### Häufige Fehler

Durch einen rohen Zeiger lesen, ohne einen `unsafe`-Block darum.

```rust
fn main() {
    let zahl = 5;
    let zeiger = &zahl as *const i32;

    println!("{}", *zeiger);
}
```

Der Übersetzer sagt dazu:

```text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> roh.rs:5:20
  |
5 |     println!("{}", *zeiger);
  |                    ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0133`.
```

Die Meldung nennt in ihrer Notiz genau die Bedingungen, die eine Begründung
abarbeiten muss: nicht null, nicht baumelnd, ausgerichtet, und keine Verletzung
der Aliasregeln. Die vollständige Liste steht in der Reference unter "Behavior
considered undefined" und nicht in dieser Meldung.

Die richtige Antwort ist nicht, einen `unsafe`-Block darum zu setzen und
weiterzugehen. Sie ist, die Bedingungen nachzusehen und die Begründung daneben
zu schreiben. Wo das nicht geht, ist der rohe Zeiger die falsche Wahl.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `adresse_von` steht fertig da, und sein Doku-Test ist grün.

- `lies` liest den Wert hinter einem `*const i32`
- `ersetzen` schreibt hinter einen `*mut i32` und gibt den alten Wert heraus
- `zeigen_auf_dasselbe` sagt, ob zwei Zeiger auf dieselbe Stelle zeigen

```console
cd units/10-02-rohe-zeiger
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    geprüft gegen 1.97.1

    The Rust Reference, "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben. Die Reference ist nach Abschnitten und nicht nach Kapiteln
geordnet, deshalb steht bei ihr der Abschnittstitel an der Stelle, an der beim
Buch der Kapiteltitel steht.

## English

### What it is about

A raw pointer is an address. `*const T` reads, `*mut T` reads and writes, and
there is nothing more in it.

It may do everything a reference may not. It may be null, it may point at
something that no longer exists, several writable pointers may point at the same
place, and nobody works out a lifetime for it.

Creating it and passing it around happens without `unsafe`. Only whoever reads
or writes through it needs an `unsafe` block, and that block does not say that
something dangerous happens here. It says that the author has looked up the
conditions the compiler cannot look up at this place.

### What it is good for

Most programs need no raw pointer. Three cases remain. The first is the
interface to the outside, because a foreign library does not know Rust's
references. The second is data structures whose shape the borrowing rules do not
map, a list in which two nodes point at each other for instance. The third is
the standard library itself, whose safe types are built inside out of exactly
these pointers.

The gain is the same in all three cases: the unsafe place becomes small and it
becomes visible. Exercise 3 of this unit is the counter-check for that, because
it compares two pointers and needs no `unsafe` for it, since nothing is read.

The price stands next to it. From here on nobody checks along. A mistake at this
place is not a wrong result but a program whose behaviour is no longer settled,
and that can look right in one run and not in the next.

### The explanation

Two pointers, once without and once with `unsafe`.

```rust
fn main() {
    let zahl = 5;
    let mut andere = 10;

    // Deutsch: Einen rohen Zeiger anzulegen ist erlaubt und braucht kein
    // `unsafe`. Er ist eine Adresse und sonst nichts.
    let nur_lesen = &zahl as *const i32;
    let schreiben = &mut andere as *mut i32;

    // Deutsch: Vergleichen und auf null prüfen geht ohne `unsafe`, denn dabei
    // wird nichts gelesen.
    println!("{}", std::ptr::eq(nur_lesen, &zahl));
    println!("{}", nur_lesen.is_null());

    // Deutsch: Lesen und Schreiben geht nur im `unsafe`-Block, und die
    // Begründung gehört daneben.
    //
    // Sicher, weil: Beide Zeiger kommen aus Referenzen auf Werte, die in diesem
    // Rahmen leben. Sie sind deshalb nicht null, ausgerichtet und gültig, und
    // auf dieselbe Stelle zeigt hier nichts sonst.
    unsafe {
        println!("{}", *nur_lesen);

        *schreiben += 1;

        println!("{}", *schreiben);
    }

    println!("{andere}");
}
```

The program prints:

```text
true
false
5
11
11
```

The first two lines are the ones surprising whoever holds `unsafe` to be
catching. Both come about without a single `unsafe` block, because a comparison
of two addresses reads neither of the two places.

The block below carries a justification, and that is no comment for decoration.
It is the only thing doing the work at this place that the compiler does
otherwise, and it names the conditions one by one: not null, aligned, valid, and
no second reference to the same place.

The last line shows that `andere` really was changed through `schreiben`. The
address was the same, and therefore it was the same value.

### Common mistakes

Reading through a raw pointer without an `unsafe` block around it.

```rust
fn main() {
    let zahl = 5;
    let zeiger = &zahl as *const i32;

    println!("{}", *zeiger);
}
```

The compiler answers:

```text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> roh.rs:5:20
  |
5 |     println!("{}", *zeiger);
  |                    ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0133`.
```

In its note the message names exactly the conditions a justification has to work
through: not null, not dangling, aligned, and no violation of the aliasing
rules. The full list stands in the Reference under "Behavior considered
undefined" and not in this message.

The right answer is not to put an `unsafe` block around it and move on. It is to
look the conditions up and write the justification next to it. Where that cannot
be done, the raw pointer is the wrong choice.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `adresse_von` stands there finished, and its
doc test is green.

- `lies` reads the value behind a `*const i32`
- `ersetzen` writes behind a `*mut i32` and hands the old value out
- `zeigen_auf_dasselbe` says whether two pointers point at the same place

```console
cd units/10-02-rohe-zeiger
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    checked against 1.97.1

    The Rust Reference, "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.
The Reference is ordered by sections and not by chapters, which is why the
section title stands with it where the chapter title stands with the book.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
