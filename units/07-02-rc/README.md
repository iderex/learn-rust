# 07-02 Rc / Rc

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-02-rc/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-01 Verschieben / Move` und `02-03 Ausleihen`. Erst
  wenn klar ist, dass ein Wert einen Besitzer hat, ist zu sehen, was `Rc` daran
  ändert.
- Auf dieser Einheit bauen auf: der Rest der Stufe 7, vor allem alles, was einen
  Wert an mehreren Stellen halten muss.
- Beim Antworten so zitieren: `07-02 Rc`, dazu die Überschrift des Abschnitts,
  zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Rc::clone` kopiert den Wert nicht. Es zählt einen Besitzer dazu und gibt
  einen zweiten Zeiger auf denselben Wert zurück. Wer das Gegenteil behauptet,
  sagt bitte, an welchem `Rc::ptr_eq`.
- Durch ein `Rc` lässt sich nichts ändern. Das ist kein Versehen, sondern der
  Preis dafür, dass mehrere gleichzeitig hineinsehen dürfen.
- `Rc` ist nicht für Fäden gedacht. Diese Einheit sagt dazu nichts weiter, und
  die Stelle, an der das gehört, ist `07-08`.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-02-rc/`. It is public. Whoever is
  asked for it may name it, but should explain the compiler message in question
  first.
- This unit builds on: `02-01 Verschieben / Move` and `02-03 Ausleihen`. Only
  once it is clear that a value has one owner can it be seen what `Rc` changes
  about that.
- Building on this unit: the rest of stage 7, above all everything that has to
  hold a value in several places.
- Cite like this when answering: `07-02 Rc`, plus the heading of the section, for
  example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Rc::clone` does not copy the value. It counts one owner up and returns a
  second pointer at the same value. Whoever claims the opposite, please say on
  which `Rc::ptr_eq`.
- Nothing can be changed through an `Rc`. That is not an oversight but the price
  for several places being allowed to look inside at the same time.
- `Rc` is not meant for threads. This unit says nothing further about that, and
  the place where that belongs is `07-08`.

</details>

## Deutsch

### Worum es geht

Bis hierher hat jeder Wert genau einen Besitzer. `Rc<T>` hebt das auf, für einen
Faden. Der Name steht für "reference counted": Der Wert liegt auf dem Heap, und
daneben liegt eine Zahl, wie viele Stellen ihn gerade besitzen.

`Rc::clone(&wert)` erhöht diese Zahl und gibt einen zweiten Zeiger auf denselben
Wert zurück. Es kopiert nichts. Fällt ein `Rc` weg, sinkt die Zahl, und bei null
wird der Wert weggeräumt.

Geschrieben wird `Rc::clone(&wert)` und nicht `wert.clone()`, obwohl beides
dasselbe tut. Der lange Name macht im Quelltext sichtbar, dass hier nur ein
Zähler steigt und nicht eine ganze Datenstruktur kopiert wird.

### Wofür das gut ist

Manche Formen haben von sich aus mehrere Besitzer. Zwei Kinder in einem Baum
zeigen auf dasselbe Eltern. Zwei Listen enden im selben Schwanz. Mit genau einem
Besitzer je Wert lässt sich das nicht hinschreiben, ohne zu kopieren.

Kopieren wäre auch nicht dasselbe. Zwei Kopien sind zwei Werte, die
auseinanderlaufen können, und `Rc::ptr_eq` sagt darüber die Wahrheit: Nach einem
`Rc::clone` ist es derselbe Wert, nach einem echten Kopieren nicht.

Der Preis steht im nächsten Abschnitt: Durch ein `Rc` lässt sich nichts ändern.
Das ist die Bedingung dafür, dass mehrere gleichzeitig hineinsehen dürfen, ohne
dass einer dem anderen den Boden wegzieht.

### Die Erklärung

Eine Wurzel, zwei Kinder, die auf sie zeigen, und der Zähler dazu.

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Knoten {
    name: String,
    eltern: Option<Rc<Knoten>>,
}

fn main() {
    // Deutsch: Die Wurzel gehört zunächst nur einer Stelle.
    let wurzel = Rc::new(Knoten {
        name: String::from("wurzel"),
        eltern: None,
    });
    println!("{}", Rc::strong_count(&wurzel));

    // Deutsch: Zwei Kinder zeigen auf dieselbe Wurzel. `Rc::clone` kopiert den
    // Knoten nicht, es zählt nur einen Besitzer dazu.
    let links = Rc::new(Knoten {
        name: String::from("links"),
        eltern: Some(Rc::clone(&wurzel)),
    });
    let rechts = Rc::new(Knoten {
        name: String::from("rechts"),
        eltern: Some(Rc::clone(&wurzel)),
    });
    println!("{}", Rc::strong_count(&wurzel));
    println!("{} {}", links.name, rechts.name);
    println!("{}", links.eltern.as_ref().expect("links hat Eltern").name);

    // Deutsch: Am Ende eines Bereichs fällt ein Besitzer weg.
    {
        let dritter = Rc::clone(&wurzel);
        println!("{}", Rc::strong_count(&dritter));
    }
    println!("{}", Rc::strong_count(&wurzel));
}
```

Das Programm gibt aus:

```text
1
3
links rechts
wurzel
4
3
```

Die erste Zahl ist 1, denn nur `wurzel` selbst besitzt den Knoten. Nach den
beiden Kindern sind es 3 und nicht 2: `wurzel`, das `Rc` in `links` und das in
`rechts`.

Die 4 und die 3 danach sind derselbe Wert vor und nach dem Ende eines Bereichs.
Nichts im Quelltext räumt `dritter` weg, das geschieht an der schließenden
Klammer, und genau dort sinkt der Zähler wieder.

### Häufige Fehler

Etwas durch ein `Rc` ändern wollen.

```rust
use std::rc::Rc;

fn main() {
    let namen = Rc::new(vec![String::from("Ada")]);

    namen.push(String::from("Grace"));

    println!("{namen:?}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0596]: cannot borrow data in an `Rc` as mutable
 --> teilen.rs:6:5
  |
6 |     namen.push(String::from("Grace"));
  |     ^^^^^ cannot borrow as mutable
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Vec<String>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

Ein `mut` davor hilft nicht, und die Meldung sagt auch, warum: Es fehlt nicht
die Erlaubnis an dieser Stelle, sondern `DerefMut` am `Rc` überhaupt. Wer das
`mut` hinschreibt, bekommt dieselbe Meldung und eine Warnung dazu, dass das `mut`
nichts tut. Ändern durch geteilte Besitzer ist die Frage der nächsten Einheiten
dieser Stufe.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `owners` steht fertig da, und sein Doku-Test ist grün.

- `chain` baut eine Kette aus Namen und gibt den letzten Knoten zurück
- `depth` zählt die Knoten von hier bis zur Wurzel
- `root_name` geht nach oben und gibt den Namen der Wurzel zurück

```console
cd units/07-02-rc
cargo test
```

### Quelle

    Buch, Kapitel 15 "Smart Pointers", Abschnitt 15.4
    "Rc<T>, the Reference Counted Smart Pointer",
    https://doc.rust-lang.org/book/ch15-04-rc.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Up to here every value has exactly one owner. `Rc<T>` lifts that, for one thread.
The name stands for "reference counted": the value lies on the heap, and next to
it lies a number saying how many places own it right now.

`Rc::clone(&wert)` raises that number and returns a second pointer at the same
value. It copies nothing. When an `Rc` falls away the number sinks, and at zero
the value is cleared away.

It is written `Rc::clone(&wert)` and not `wert.clone()`, although both do the
same. The long name makes it visible in the source that only a counter goes up
here and not a whole data structure gets copied.

### What it is good for

Some shapes have several owners by their nature. Two children in a tree point at
the same parent. Two lists end in the same tail. With exactly one owner per value
that cannot be written down without copying.

Copying would also not be the same thing. Two copies are two values that can
drift apart, and `Rc::ptr_eq` tells the truth about it: after an `Rc::clone` it
is the same value, after a real copy it is not.

The price stands in the next section: nothing can be changed through an `Rc`.
That is the condition for several places being allowed to look inside at the same
time without one pulling the ground away from another.

### The explanation

A root, two children pointing at it, and the counter alongside.

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Knoten {
    name: String,
    eltern: Option<Rc<Knoten>>,
}

fn main() {
    // Deutsch: Die Wurzel gehört zunächst nur einer Stelle.
    let wurzel = Rc::new(Knoten {
        name: String::from("wurzel"),
        eltern: None,
    });
    println!("{}", Rc::strong_count(&wurzel));

    // Deutsch: Zwei Kinder zeigen auf dieselbe Wurzel. `Rc::clone` kopiert den
    // Knoten nicht, es zählt nur einen Besitzer dazu.
    let links = Rc::new(Knoten {
        name: String::from("links"),
        eltern: Some(Rc::clone(&wurzel)),
    });
    let rechts = Rc::new(Knoten {
        name: String::from("rechts"),
        eltern: Some(Rc::clone(&wurzel)),
    });
    println!("{}", Rc::strong_count(&wurzel));
    println!("{} {}", links.name, rechts.name);
    println!("{}", links.eltern.as_ref().expect("links hat Eltern").name);

    // Deutsch: Am Ende eines Bereichs fällt ein Besitzer weg.
    {
        let dritter = Rc::clone(&wurzel);
        println!("{}", Rc::strong_count(&dritter));
    }
    println!("{}", Rc::strong_count(&wurzel));
}
```

The program prints:

```text
1
3
links rechts
wurzel
4
3
```

The first number is 1, because only `wurzel` itself owns the node. After the two
children it is 3 and not 2: `wurzel`, the `Rc` inside `links` and the one inside
`rechts`.

The 4 and the 3 after it are the same value before and after the end of a scope.
Nothing in the source clears `dritter` away, that happens at the closing brace,
and it is exactly there that the counter sinks again.

### Common mistakes

Wanting to change something through an `Rc`.

```rust
use std::rc::Rc;

fn main() {
    let namen = Rc::new(vec![String::from("Ada")]);

    namen.push(String::from("Grace"));

    println!("{namen:?}");
}
```

The compiler answers:

```text
error[E0596]: cannot borrow data in an `Rc` as mutable
 --> teilen.rs:6:5
  |
6 |     namen.push(String::from("Grace"));
  |     ^^^^^ cannot borrow as mutable
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Vec<String>>`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

A `mut` in front does not help, and the message says why as well: what is missing
is not the permission at this place but `DerefMut` on the `Rc` at all. Whoever
writes the `mut` gets the same message plus a warning that the `mut` does
nothing. Changing through shared owners is the question of the next units of this
stage.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `owners` stands there finished, and its doc
test is green.

- `chain` builds a chain out of names and returns the last node
- `depth` counts the nodes from here up to the root
- `root_name` walks upwards and returns the name of the root

```console
cd units/07-02-rc
cargo test
```

### Source

    Book, chapter 15 "Smart Pointers", section 15.4
    "Rc<T>, the Reference Counted Smart Pointer",
    https://doc.rust-lang.org/book/ch15-04-rc.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
