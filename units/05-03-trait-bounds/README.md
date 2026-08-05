# 05-03 Trait Bounds / Trait bounds

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-03-trait-bounds/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-01 Generische Typen` und `05-02 Traits`. Sie löst
  die Meldung auf, an der `05-01` endet.
- Auf dieser Einheit bauen auf: `05-04 Lifetimes` und alles Generische, das mehr
  tut als Werte weiterreichen.
- Beim Antworten so zitieren: `05-03 Trait Bounds`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Eine Schranke ist eine Bedingung an den Typ und keine Prüfung beim Laufen. Sie
  wird an der Aufrufstelle geprüft, und die Meldung dort nennt beide Seiten.
- `where` und die kurze Schreibweise bedeuten dasselbe. Wer einen Unterschied
  behauptet, sagt bitte, welchen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-03-trait-bounds/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `05-01 Generische Typen` and `05-02 Traits`. It resolves
  the message `05-01` ends on.
- Building on this unit: `05-04 Lifetimes` and everything generic that does more
  than hand values on.
- Cite like this when answering: `05-03 Trait Bounds`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A bound is a condition on the type and not a check while running. It is
  checked at the call site, and the message there names both sides.
- `where` and the short form mean the same thing. Whoever claims a difference,
  please say which one.

</details>

## Deutsch

### Worum es geht

Eine Schranke sagt, was ein Typparameter können muss. `T: PartialOrd` heißt:
für diesen `T` gibt es einen Vergleich, und deshalb darf die Funktion ihn
benutzen.

Mehrere Schranken an einem Typ werden mit `+` verbunden, also
`T: PartialOrd + Display`. Mehrere Typparameter mit je eigenen Schranken passen
dann nicht mehr in eine Zeile, und dafür gibt es `where`, das dasselbe sagt und
den Kopf frei lässt.

Geprüft wird beides beim Übersetzen, und zwar an der Aufrufstelle. Wer eine
Funktion mit einem Typ aufruft, der die Schranke nicht erfüllt, bekommt
`E0277`.

### Wofür das gut ist

`05-01` endete an einer Wand: ein freies `T` kann fast nichts, und ein Vergleich
war nicht erlaubt. Die Schranke ist der Weg durch diese Wand, und sie schreibt
in den Kopf der Funktion, was bis dahin nur im Rumpf stand.

Damit steht der Vertrag am Namen. Wer die Signatur liest, weiß, welche Typen
hineinpassen, ohne den Rumpf zu lesen, und wer den Rumpf ändert, muss den Kopf
mitändern.

Und die Meldung kommt an der richtigen Stelle. Nicht beim Laufen, nicht in der
generischen Funktion, sondern dort, wo jemand einen Typ einsetzt, der es nicht
kann.

### Die Erklärung

Eine Schranke, zwei Schranken, und dieselben zwei mit `where`.

```rust
use std::fmt::Display;

// Deutsch: `T: PartialOrd` ist die Schranke. Sie sagt, was `T` können muss,
// und erst damit ist der Vergleich erlaubt.
fn groesster<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    let mut groesster = werte.first()?;

    for wert in werte {
        if wert > groesster {
            groesster = wert;
        }
    }

    Some(groesster)
}

// Deutsch: Zwei Schranken an einem Typ, mit `+` verbunden.
fn groesster_gemeldet<T: PartialOrd + Display>(werte: &[T]) -> String {
    match groesster(werte) {
        Some(wert) => format!("groesster Wert {wert}"),
        None => String::from("keine Werte"),
    }
}

// Deutsch: Dieselben Schranken in der Schreibweise mit `where`. Bei mehreren
// Typparametern bleibt der Kopf damit lesbar.
fn beide_gemeldet<T, U>(links: T, rechts: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{links} und {rechts}")
}

fn main() {
    println!("{:?}", groesster(&[3, 9, 4]));
    println!("{:?}", groesster::<i32>(&[]));
    println!("{}", groesster_gemeldet(&[3, 9, 4]));
    println!("{}", groesster_gemeldet::<i32>(&[]));
    println!("{}", beide_gemeldet(42, "Text"));
}
```

Das Programm gibt aus:

```text
Some(9)
None
groesster Wert 9
keine Werte
42 und Text
```

Das `?` in der ersten Funktion ist dasselbe wie in `04-08`, hier auf einem
`Option`: ist die Liste leer, gibt die Funktion sofort `None` zurück.

### Häufige Fehler

Eine gebundene Funktion mit einem Typ aufrufen, der die Schranke nicht erfüllt.

```rust
use std::fmt::Display;

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

fn gemeldet<T: Display>(wert: T) -> String {
    format!("Wert {wert}")
}

fn main() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };

    println!("{}", gemeldet(rechteck));
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `Rechteck` doesn't implement `std::fmt::Display`
  --> aufruf.rs:18:29
   |
18 |     println!("{}", gemeldet(rechteck));
   |                    -------- ^^^^^^^^ unsatisfied trait bound
   |                    |
   |                    required by a bound introduced by this call
   |
help: the trait `std::fmt::Display` is not implemented for `Rechteck`
  --> aufruf.rs:3:1
   |
 3 | struct Rechteck {
   | ^^^^^^^^^^^^^^^
note: required by a bound in `gemeldet`
  --> aufruf.rs:8:16
   |
 8 | fn gemeldet<T: Display>(wert: T) -> String {
   |                ^^^^^^^ required by this bound in `gemeldet`

error: aborting due to 1 previous error
```

Die Meldung zeigt beide Seiten: die Stelle, an der der Typ eingesetzt wird, und
die Schranke, die ihn ablehnt. Das ist der Unterschied zu `05-01`, wo die
Meldung noch in der generischen Funktion selbst stand.

Die Antwort ist nicht, die Schranke wegzunehmen, sondern `Display` für
`Rechteck` zu schreiben, so wie in `03-07`. Eine Schranke, die stört, ist
meistens eine Frage nach etwas, das dem Typ wirklich fehlt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe trägt ihre Schranken schon im Kopf.

- `largest` gibt den größten Wert einer Liste zurück, mit `PartialOrd`
- `reported` gibt einen Wert als Text zurück, mit `Display`
- `largest_reported` verbindet beide, in der Schreibweise mit `where`

```console
cd units/05-03-trait-bounds
cargo test
```

### Quelle

    Buch, Kapitel 10 "Generic Types, Traits, and Lifetimes", Abschnitt 10.2 "Defining Shared Behavior with Traits",
    https://doc.rust-lang.org/book/ch10-02-traits.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A bound says what a type parameter has to be able to do. `T: PartialOrd` means:
for this `T` there is a comparison, and that is why the function may use it.

Several bounds on one type are joined with `+`, so `T: PartialOrd + Display`.
Several type parameters with bounds of their own then no longer fit into one
line, and for that there is `where`, which says the same and leaves the head
clear.

Both are checked at compile time, and at the call site. Whoever calls a function
with a type that does not fulfil the bound gets `E0277`.

### What it is good for

`05-01` ended at a wall: a free `T` can do almost nothing, and a comparison was
not allowed. The bound is the way through that wall, and it writes into the head
of the function what until then stood only in the body.

With it the contract stands at the name. Whoever reads the signature knows which
types fit in without reading the body, and whoever changes the body has to
change the head with it.

And the message comes at the right place. Not while running, not inside the
generic function, but where somebody puts in a type that cannot do it.

### The explanation

One bound, two bounds, and the same two with `where`.

```rust
use std::fmt::Display;

// Deutsch: `T: PartialOrd` ist die Schranke. Sie sagt, was `T` können muss,
// und erst damit ist der Vergleich erlaubt.
fn groesster<T: PartialOrd>(werte: &[T]) -> Option<&T> {
    let mut groesster = werte.first()?;

    for wert in werte {
        if wert > groesster {
            groesster = wert;
        }
    }

    Some(groesster)
}

// Deutsch: Zwei Schranken an einem Typ, mit `+` verbunden.
fn groesster_gemeldet<T: PartialOrd + Display>(werte: &[T]) -> String {
    match groesster(werte) {
        Some(wert) => format!("groesster Wert {wert}"),
        None => String::from("keine Werte"),
    }
}

// Deutsch: Dieselben Schranken in der Schreibweise mit `where`. Bei mehreren
// Typparametern bleibt der Kopf damit lesbar.
fn beide_gemeldet<T, U>(links: T, rechts: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{links} und {rechts}")
}

fn main() {
    println!("{:?}", groesster(&[3, 9, 4]));
    println!("{:?}", groesster::<i32>(&[]));
    println!("{}", groesster_gemeldet(&[3, 9, 4]));
    println!("{}", groesster_gemeldet::<i32>(&[]));
    println!("{}", beide_gemeldet(42, "Text"));
}
```

The program prints:

```text
Some(9)
None
groesster Wert 9
keine Werte
42 und Text
```

The `?` in the first function is the same one as in `04-08`, here on an
`Option`: if the list is empty the function returns `None` at once.

### Common mistakes

Calling a bounded function with a type that does not fulfil the bound.

```rust
use std::fmt::Display;

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

fn gemeldet<T: Display>(wert: T) -> String {
    format!("Wert {wert}")
}

fn main() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };

    println!("{}", gemeldet(rechteck));
}
```

The compiler answers:

```text
error[E0277]: `Rechteck` doesn't implement `std::fmt::Display`
  --> aufruf.rs:18:29
   |
18 |     println!("{}", gemeldet(rechteck));
   |                    -------- ^^^^^^^^ unsatisfied trait bound
   |                    |
   |                    required by a bound introduced by this call
   |
help: the trait `std::fmt::Display` is not implemented for `Rechteck`
  --> aufruf.rs:3:1
   |
 3 | struct Rechteck {
   | ^^^^^^^^^^^^^^^
note: required by a bound in `gemeldet`
  --> aufruf.rs:8:16
   |
 8 | fn gemeldet<T: Display>(wert: T) -> String {
   |                ^^^^^^^ required by this bound in `gemeldet`

error: aborting due to 1 previous error
```

The message shows both sides: the place where the type is put in, and the bound
that refuses it. That is the difference from `05-01`, where the message still
stood inside the generic function itself.

The answer is not to take the bound away but to write `Display` for `Rechteck`,
the way `03-07` did. A bound that gets in the way is mostly a question about
something the type really lacks.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise already carries its bounds in
its head.

- `largest` returns the biggest value of a list, with `PartialOrd`
- `reported` returns a value as text, with `Display`
- `largest_reported` joins both, in the form with `where`

```console
cd units/05-03-trait-bounds
cargo test
```

### Source

    Book, chapter 10 "Generic Types, Traits, and Lifetimes", section 10.2 "Defining Shared Behavior with Traits",
    https://doc.rust-lang.org/book/ch10-02-traits.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
