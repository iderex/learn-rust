# 09-03 Operatorüberladung / Operator overloading

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-03-operatorueberladung/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-02 Traits` und `03-07 Display selbst schreiben`.
  Ein Operator ist ein Trait wie jedes andere, und ein Trait für einen eigenen
  Typ zu schreiben ist von dort bekannt.
- Auf dieser Einheit bauen auf: der Rest der Stufe 9 und jeder eigene Typ, der
  sich wie eine Zahl oder wie eine Liste benutzen lassen soll.
- Beim Antworten so zitieren: `09-03 Operatorüberladung`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Nicht jeder Operator lässt sich überladen. `&&`, `||` und die Zuweisung
  gehören nicht dazu. Wer das Gegenteil behauptet, sagt bitte, an welchem Trait.
- `Output` muss nicht der eigene Typ sein. Bei `Index` ist es hier `str`, und
  das kaufmännische Und steht in der Signatur von `index` und nicht im
  zugeordneten Typ.
- `add` nimmt `self` an sich. Dass `a + b` die beiden nicht aufbraucht, liegt an
  `Copy` und nicht an `Add`.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-03-operatorueberladung/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `05-02 Traits` and `03-07 Display selbst schreiben`. An
  operator is a trait like any other, and writing a trait for a type of your own
  is known from there.
- Building on this unit: the rest of stage 9 and every type of your own that is
  meant to be usable like a number or like a list.
- Cite like this when answering: `09-03 Operatorüberladung`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Not every operator can be overloaded. `&&`, `||` and assignment are not among
  them. Whoever claims the opposite, please say on which trait.
- `Output` need not be the type itself. At `Index` it is `str` here, and the
  ampersand stands in the signature of `index` and not in the associated type.
- `add` takes `self` for itself. That `a + b` does not use the two up comes from
  `Copy` and not from `Add`.

</details>

## Deutsch

### Worum es geht

In Rust ist ein Operator ein Trait. `a + b` heißt `Add::add(a, b)`, `-a` heißt
`Neg::neg(a)`, und `a[i]` heißt `Index::index(&a, i)`. Die Traits liegen in
`std::ops`.

Wer eines davon für einen eigenen Typ schreibt, gibt dem Operator dort eine
Bedeutung. Anderswo ändert sich nichts: `+` an zwei Zahlen bleibt, was es war.

Zu jedem dieser Traits gehört ein zugeordneter Typ `Output`. Er sagt, was
herauskommt, und muss nicht der eigene Typ sein. Bei `Index` ist es hier `str`,
obwohl der Zugriff eine Referenz zurückgibt.

### Wofür das gut ist

Ein Typ, der eine Größe darstellt, soll sich rechnen lassen wie eine Größe.
`links + rechts` sagt, was gemeint ist. `links.zusammen_mit(rechts)` sagt
dasselbe und verlangt, dass der Leser den Namen erst liest.

Der Gewinn ist nicht die Kürze, sondern dass der Rest des Programms den Typ nicht
mehr kennen muss. Aufgabe 3 dieser Einheit zählt eine Liste von Punkten
zusammen, ohne zu wissen, wie das Zusammenzählen gemacht ist, und genau das ist
der Punkt.

Die Grenze steht daneben. Nicht jeder Operator lässt sich überladen, `&&` und
`||` etwa nicht, und ein `+`, das etwas anderes tut als zusammenzählen, ist eine
Falle für jeden, der es liest. Ein Operator ist ein Versprechen über die
Bedeutung, und wer es bricht, hat nichts gewonnen.

### Die Erklärung

Ein Punkt mit `Add` und eine Woche mit `Index`, in einem Programm.

```rust
use std::ops::{Add, Index};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

// Deutsch: `Add` gibt dem Plus eine Bedeutung für diesen Typ. `Output` sagt,
// was dabei herauskommt, und das muss nicht derselbe Typ sein.
impl Add for Punkt {
    type Output = Punkt;

    fn add(self, andere: Punkt) -> Punkt {
        Punkt {
            x: self.x + andere.x,
            y: self.y + andere.y,
        }
    }
}

struct Woche {
    tage: [&'static str; 7],
}

// Deutsch: `Index` gibt den eckigen Klammern eine Bedeutung. Zurück kommt eine
// Referenz, denn der Wert bleibt, wo er ist.
impl Index<usize> for Woche {
    type Output = str;

    fn index(&self, stelle: usize) -> &str {
        self.tage[stelle]
    }
}

fn main() {
    let links = Punkt { x: 1, y: 2 };
    let rechts = Punkt { x: 3, y: 4 };

    println!("{:?}", links + rechts);

    // Deutsch: Beide sind `Copy`, also stehen sie danach noch da.
    println!("{:?}", links);

    let woche = Woche {
        tage: ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
    };

    println!("{}", &woche[0]);
    println!("{}", &woche[6]);
}
```

Das Programm gibt aus:

```text
Punkt { x: 4, y: 6 }
Punkt { x: 1, y: 2 }
Mo
So
```

Die zweite Zeile ist die, die man leicht übersieht. `add` nimmt `self` an sich,
also wäre `links` nach dem Plus weg. Dass es noch da ist, liegt am `Copy` in der
`derive`-Zeile und nicht am `Add`.

Bei `Index` steht `type Output = str` und nicht `&str`. Das kaufmännische Und
steht schon in der Signatur von `index`, und ein zweites wäre eine Referenz auf
eine Referenz.

### Häufige Fehler

Den Operator benutzen, ohne ihn zu schreiben.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

fn main() {
    let links = Punkt { x: 1, y: 2 };
    let rechts = Punkt { x: 3, y: 4 };

    println!("{:?}", links + rechts);
}
```

Der Übersetzer sagt dazu:

```text
error[E0369]: cannot add `Punkt` to `Punkt`
  --> ohne-impl.rs:11:28
   |
11 |     println!("{:?}", links + rechts);
   |                      ----- ^ ------ Punkt
   |                      |
   |                      Punkt
   |
note: an implementation of `Add` might be missing for `Punkt`
  --> ohne-impl.rs:2:1
   |
 2 | struct Punkt {
   | ^^^^^^^^^^^^ must implement `Add`
note: the trait `Add` must be implemented
  --> <std>/core/src/ops/arith.rs:76:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

An `<std>` stand der Pfad zur Standardbibliothek dieses Rechners, mit der
Prüfsumme des Übersetzers darin. Das ist die einzige Ersetzung, sonst steht die
Meldung so da, wie sie kam.

Die Meldung sagt beides: welchem Typ das Trait fehlt und welches Trait das ist.
Sie nennt kein `derive`, denn `Add` lässt sich nicht ableiten. Was `+` für zwei
Punkte heißen soll, kann nur ein Mensch entscheiden.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Neg for Point` steht fertig da, und sein Doku-Test ist grün.

- `Add for Point` zählt je Achse zusammen
- `Index<usize> for Week` gibt das Kürzel an dieser Stelle heraus
- `sum` zählt eine Liste von Punkten zusammen und benutzt dafür das `+`

```console
cd units/09-03-operatorueberladung
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.2 "Advanced Traits",
    https://doc.rust-lang.org/book/ch20-02-advanced-traits.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

In Rust an operator is a trait. `a + b` means `Add::add(a, b)`, `-a` means
`Neg::neg(a)`, and `a[i]` means `Index::index(&a, i)`. The traits live in
`std::ops`.

Whoever writes one of them for a type of their own gives the operator a meaning
there. Elsewhere nothing changes: `+` on two numbers stays what it was.

Every one of these traits has an associated type `Output`. It says what comes
out, and it need not be the type itself. At `Index` it is `str` here, although
the access returns a reference.

### What it is good for

A type standing for a quantity should be computable like a quantity.
`links + rechts` says what is meant. `links.zusammen_mit(rechts)` says the same
and asks the reader to read the name first.

The gain is not the brevity but that the rest of the program need not know the
type any more. Exercise 3 of this unit adds a list of points up without knowing
how the adding is done, and that is exactly the point.

The limit stands next to it. Not every operator can be overloaded, `&&` and `||`
for instance cannot, and a `+` doing something other than adding is a trap for
everybody who reads it. An operator is a promise about meaning, and whoever
breaks it has gained nothing.

### The explanation

A point with `Add` and a week with `Index`, in one program.

```rust
use std::ops::{Add, Index};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

// Deutsch: `Add` gibt dem Plus eine Bedeutung für diesen Typ. `Output` sagt,
// was dabei herauskommt, und das muss nicht derselbe Typ sein.
impl Add for Punkt {
    type Output = Punkt;

    fn add(self, andere: Punkt) -> Punkt {
        Punkt {
            x: self.x + andere.x,
            y: self.y + andere.y,
        }
    }
}

struct Woche {
    tage: [&'static str; 7],
}

// Deutsch: `Index` gibt den eckigen Klammern eine Bedeutung. Zurück kommt eine
// Referenz, denn der Wert bleibt, wo er ist.
impl Index<usize> for Woche {
    type Output = str;

    fn index(&self, stelle: usize) -> &str {
        self.tage[stelle]
    }
}

fn main() {
    let links = Punkt { x: 1, y: 2 };
    let rechts = Punkt { x: 3, y: 4 };

    println!("{:?}", links + rechts);

    // Deutsch: Beide sind `Copy`, also stehen sie danach noch da.
    println!("{:?}", links);

    let woche = Woche {
        tage: ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
    };

    println!("{}", &woche[0]);
    println!("{}", &woche[6]);
}
```

The program prints:

```text
Punkt { x: 4, y: 6 }
Punkt { x: 1, y: 2 }
Mo
So
```

The second line is the one easily overlooked. `add` takes `self` for itself, so
`links` would be gone after the plus. That it is still there comes from the
`Copy` in the `derive` line and not from the `Add`.

At `Index` it says `type Output = str` and not `&str`. The ampersand already
stands in the signature of `index`, and a second one would be a reference to a
reference.

### Common mistakes

Using the operator without writing it.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

fn main() {
    let links = Punkt { x: 1, y: 2 };
    let rechts = Punkt { x: 3, y: 4 };

    println!("{:?}", links + rechts);
}
```

The compiler answers:

```text
error[E0369]: cannot add `Punkt` to `Punkt`
  --> ohne-impl.rs:11:28
   |
11 |     println!("{:?}", links + rechts);
   |                      ----- ^ ------ Punkt
   |                      |
   |                      Punkt
   |
note: an implementation of `Add` might be missing for `Punkt`
  --> ohne-impl.rs:2:1
   |
 2 | struct Punkt {
   | ^^^^^^^^^^^^ must implement `Add`
note: the trait `Add` must be implemented
  --> <std>/core/src/ops/arith.rs:76:0

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

Where `<std>` stands, the path to the standard library of this machine stood,
with the checksum of the compiler inside it. That is the only substitution,
otherwise the message stands as it came.

The message says both things: which type is missing the trait and which trait
that is. It names no `derive`, because `Add` cannot be derived. What `+` should
mean for two points can only be decided by a person.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Neg for Point` stands there finished, and its
doc test is green.

- `Add for Point` adds per axis
- `Index<usize> for Week` hands the abbreviation at that place out
- `sum` adds a list of points up and uses the `+` for it

```console
cd units/09-03-operatorueberladung
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.2 "Advanced Traits",
    https://doc.rust-lang.org/book/ch20-02-advanced-traits.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
