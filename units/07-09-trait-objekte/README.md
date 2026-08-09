# 07-09 Trait-Objekte / Trait objects

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-09-trait-objekte/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-02 Traits`, woher `Flaeche`, `Rechteck` und
  `Quadrat` kommen, und `07-01 Box`, denn ein Trait-Objekt liegt hinter einem
  Zeiger.
- Auf dieser Einheit bauen auf: alles, was mehrere Typen in einer Liste hält,
  ohne sie vorher aufzuzählen.
- Beim Antworten so zitieren: `07-09 Trait-Objekte`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Meldung zu einem Trait, der nicht hinter `dyn` darf, heißt in dieser
  gebundenen Fassung "is not dyn compatible". Die ältere Schreibweise "object
  safe" steht in vielen Texten und meint dasselbe, kommt aber aus dem
  Übersetzer nicht mehr heraus.
- Die vier Zahlen unter "Was der Umweg kostet" sind auf einem Rechner gemessen
  und keine Zusage der Sprache. Wer sie weitergibt, gibt den Befehl mit.
- Ob ein Rumpf von `Form::flaeche` ein `match` oder zwei `if let` benutzt, sagen
  die Tests dieser Einheit nicht. Das steht unter "Was diese Tests nicht
  beantworten" mit dem Lauf, der es zeigt, und diese Aussage bleibt negativ.
- Welcher der beiden Wege schneller läuft, ist hier nicht gemessen worden, und
  nichts in dieser Einheit ist ein Beleg dafür.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-09-trait-objekte/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `05-02 Traits`, where `Flaeche`, `Rechteck` and `Quadrat`
  come from, and `07-01 Box`, because a trait object lies behind a pointer.
- Building on this unit: everything that holds several types in one list without
  listing them beforehand.
- Cite like this when answering: `07-09 Trait-Objekte`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The message about a trait that may not go behind `dyn` reads "is not dyn
  compatible" in this pinned version. The older spelling "object safe" stands in
  many texts and means the same thing, but no longer comes out of the compiler.
- The four numbers under "What the detour costs" are measured on one machine and
  are not a promise of the language. Whoever passes them on passes the command
  on with them.
- Whether a body of `Form::flaeche` uses a `match` or two `if let` is not
  answered by the tests of this unit. That stands under "What these tests do not
  answer" with the run that shows it, and that statement stays negative.
- Which of the two ways runs faster has not been measured here, and nothing in
  this unit is evidence for it.

</details>

## Deutsch

### Worum es geht

Ein Trait sagt, was ein Typ können muss. Bis hierher stand er immer als Schranke
an einem generischen Typ: `fn flaeche_von<F: Flaeche>(form: &F)` nimmt irgendeine
Form, und beim Übersetzen steht fest, welche.

`dyn Flaeche` dreht das um. Es ist selbst ein Typ, und zwar einer, dessen Größe
niemand kennt, weil ein `Rechteck` und ein `Quadrat` verschieden groß sind.
Deshalb steht er nie allein da, sondern immer hinter einem Zeiger: `&dyn Flaeche`
oder `Box<dyn Flaeche>`.

Was dahinter liegt, weiß der Übersetzer dann nicht mehr. Er weiß nur, dass es den
Trait erfüllt, und schlägt die Methode beim Aufruf nach. Das ist der ganze
Unterschied, und alles Weitere folgt daraus.

### Wofür das gut ist

Eine Liste mit einem Rechteck und einem Quadrat darin. Mit einem generischen `F`
geht das nicht, denn `Vec<F>` ist eine Liste von genau einem Typ, und `F` wird
beim Aufruf zu genau einem.

Mit `Vec<Box<dyn Flaeche>>` geht es. Die Liste hält Zeiger, alle gleich groß, und
was am Ende jedes Zeigers liegt, darf verschieden sein. Genau das ist der Grund,
warum es Trait-Objekte gibt.

Dasselbe geht auch mit einem `enum`, und die letzten beiden Aufgaben bauen genau
das. Welcher der beiden Wege wann passt, steht weiter unten unter einer eigenen
Überschrift, denn das ist keine Geschmacksfrage.

### Die Erklärung

Eine Liste, zwei Typen, ein Trait.

```rust
trait Flaeche {
    fn flaeche(&self) -> u32;
    fn name(&self) -> &'static str;
}

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

struct Quadrat {
    seite: u32,
}

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    fn name(&self) -> &'static str {
        "Rechteck"
    }
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    fn name(&self) -> &'static str {
        "Quadrat"
    }
}

fn main() {
    // Deutsch: Eine Liste, zwei Typen. Das geht nur hinter einem Zeiger.
    let formen: Vec<Box<dyn Flaeche>> = vec![
        Box::new(Rechteck { breite: 3, hoehe: 4 }),
        Box::new(Quadrat { seite: 5 }),
    ];

    for form in &formen {
        println!("{} {}", form.name(), form.flaeche());
    }

    let gesamt: u32 = formen.iter().map(|form| form.flaeche()).sum();
    println!("{gesamt}");
}
```

Das Programm gibt aus:

```text
Rechteck 12
Quadrat 25
37
```

Die Schleife ruft zweimal dieselbe Zeile auf und landet zweimal in einem anderen
Rumpf. Das ist es, was ein Trait-Objekt tut, und der Rumpf der Schleife merkt
nichts davon.

### Was der Umweg kostet

Der Zeiger auf ein Trait-Objekt ist doppelt so breit wie ein gewöhnlicher. In ihm
steht die Adresse des Wertes und daneben die Adresse einer Tabelle mit den
Methoden, und über die zweite läuft jeder Aufruf.

```rust
use std::mem::size_of;

trait Flaeche {
    fn flaeche(&self) -> u32;
}

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

enum Form {
    Rechteck { breite: u32, hoehe: u32 },
    Quadrat { seite: u32 },
}

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }
}

impl Form {
    fn flaeche(&self) -> u32 {
        match self {
            Form::Rechteck { breite, hoehe } => breite * hoehe,
            Form::Quadrat { seite } => seite * seite,
        }
    }
}

fn main() {
    let rechteck = Rechteck { breite: 3, hoehe: 4 };
    let hinter_zeiger: &dyn Flaeche = &rechteck;
    let im_kasten: Box<dyn Flaeche> = Box::new(Rechteck { breite: 3, hoehe: 4 });

    println!("{}", hinter_zeiger.flaeche());
    println!("{}", im_kasten.flaeche());
    println!("{}", Form::Rechteck { breite: 3, hoehe: 4 }.flaeche());
    println!("{}", Form::Quadrat { seite: 5 }.flaeche());

    println!("{}", size_of::<&Rechteck>());
    println!("{}", size_of::<&dyn Flaeche>());
    println!("{}", size_of::<Box<dyn Flaeche>>());
    println!("{}", size_of::<Form>());
}
```

Das Programm gibt aus:

```text
12
12
12
25
8
16
16
12
```

Die vier Zahlen unten sind der Punkt. Ein `&Rechteck` ist 8 Bytes groß, ein
`&dyn Flaeche` 16, ein `Box<dyn Flaeche>` ebenso, und das ganze `Form` liegt bei
12 und braucht dafür gar keinen Zeiger.

Gemessen auf einem Rechner mit `rustc 1.97.1`, x86_64, mit dem Programm oben.
Größen von Zeigern hängen vom Ziel ab, deshalb gehören Zahl und Befehl zusammen.
Was hier nicht gemessen ist, ist die Laufzeit: Wer sagen will, welcher der beiden
Wege schneller ist, misst das und beruft sich nicht auf diese Zeilen.

### Häufige Fehler

Zwei Typen in eine Liste legen, ohne den Umweg über den Zeiger.

```rust
struct Rechteck {
    breite: u32,
    hoehe: u32,
}

struct Quadrat {
    seite: u32,
}

fn main() {
    let formen = vec![Rechteck { breite: 3, hoehe: 4 }, Quadrat { seite: 5 }];

    println!("{}", formen.len());
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
  --> gemischte-liste.rs:11:57
   |
11 |     let formen = vec![Rechteck { breite: 3, hoehe: 4 }, Quadrat { seite: 5 }];
   |                                                         ^^^^^^^^^^^^^^^^^^^^ expected `Rechteck`, found `Quadrat`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`vec!` nimmt den Typ des ersten Elements und erwartet ihn danach überall. Der Weg
ist `Vec<Box<dyn Flaeche>>` mit `Box::new` um jedes Element.

Der zweite Fehler kommt später und überrascht mehr: Nicht jeder Trait darf hinter
`dyn`.

```rust
trait Verdoppeln {
    fn doppelt(&self) -> Self;
}

fn zeigen(wert: &dyn Verdoppeln) {
    let _ = wert;
}

fn main() {}
```

Der Übersetzer sagt dazu:

```text
error[E0038]: the trait `Verdoppeln` is not dyn compatible
 --> nicht-objektsicher.rs:5:18
  |
5 | fn zeigen(wert: &dyn Verdoppeln) {
  |                  ^^^^^^^^^^^^^^ `Verdoppeln` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> nicht-objektsicher.rs:2:26
  |
1 | trait Verdoppeln {
  |       ---------- this trait is not dyn compatible...
2 |     fn doppelt(&self) -> Self;
  |                          ^^^^ ...because method `doppelt` references the `Self` type in its return type
  = help: consider moving `doppelt` to another trait

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0038`.
```

Der Grund steht in der Meldung. Hinter `dyn` ist der eigene Typ gerade das, was
niemand mehr kennt, und eine Methode, die ihn zurückgibt, ließe sich nicht
aufrufen. `Flaeche` in dieser Einheit gibt `u32` und `&'static str` zurück und
hat deshalb kein solches Problem.

### Wann welche Fassung passt

Beide Wege lösen dieselbe Aufgabe, und die Aufgaben 1 und 3 rechnen dieselbe
Zahl aus. Was sie unterscheiden, ist, wer später etwas dazuschreiben darf.

Ein `enum` zählt seine Fälle auf. Wer einen dazuschreibt, ändert den Typ, und
jedes `match` darüber wird rot, bis der neue Fall behandelt ist. Das ist ein
Vorteil, solange die Fälle bekannt und ihre Zahl klein ist: Der Übersetzer führt
Buch darüber, wo überall etwas fehlt, und in der Liste liegt der Wert selbst
statt eines Zeigers auf ihn.

Ein Trait-Objekt zählt nichts auf. Wer einen neuen Typ dazuschreibt, schreibt ihn
und sein `impl` irgendwo hin, und nichts anderes ändert sich. Das ist der Weg,
wenn die Menge offen ist, wenn Fremdcode eigene Typen beisteuern soll oder wenn
alle Fälle im selben `enum` unangenehm groß würden.

Die kürzeste Fassung der Regel: geschlossene Menge, bekannte Fälle, `enum`.
Offene Menge, fremde Typen, `dyn`. Und wo beides geht, entscheidet, welche
Änderung öfter kommt, ein neuer Fall oder eine neue Stelle, die alle Fälle
behandelt.

### Was diese Tests nicht beantworten

Aufgabe 2 verlangt ein `match` über die beiden Fälle. Ob ein Rumpf das benutzt
oder stattdessen zwei `if let` hintereinander, sagt kein Test dieser Einheit.

Nachgemessen und nicht vermutet: Die Lösung wurde auf zwei `if let` mit einer 0
am Ende umgestellt und die Testdatei danach ausgeführt.

```console
$ cargo test -q -p unit-07-09-trait-objekte --test exercise
running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Ein Test kann das auch nicht sehen, denn beide Rümpfe geben für jede Eingabe
dasselbe zurück. Der Unterschied liegt woanders: Kommt ein dritter Fall dazu,
wird das `match` rot und die Fassung mit `if let` gibt still 0 zurück. Wer die
zweite Fassung wählt, verliert genau die Buchführung, für die das `enum` in
diesem Vergleich steht.

Ebenso wenig steht hier, welcher der beiden Wege schneller läuft. Gemessen sind
Größen und nicht Zeiten.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `flaeche_von` steht fertig da, und sein Doku-Test ist grün.

- `gesamt_dyn` zählt die Flächen einer Liste aus `Box<dyn Flaeche>` zusammen
- `Form::flaeche` gibt die Fläche des jeweiligen Falls zurück
- `gesamt_enum` rechnet dieselbe Summe über eine Liste aus `Form`
- `groesste_dyn` sagt, wie die größte Form heißt, bei Gleichstand die erste

```console
cd units/07-09-trait-objekte
cargo test
```

### Quelle

    Buch, Kapitel 18 "Object Oriented Programming Features", Abschnitt 18.2
    "Using Trait Objects to Abstract over Shared Behavior",
    https://doc.rust-lang.org/book/ch18-02-trait-objects.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 6.1 "Traits", Abschnitt "Dyn compatibility",
    https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A trait says what a type has to be able to do. Up to here it always stood as a
bound on a generic type: `fn flaeche_von<F: Flaeche>(form: &F)` takes any shape,
and at compile time it is settled which one.

`dyn Flaeche` turns that around. It is a type in its own right, and one whose
size nobody knows, because a `Rechteck` and a `Quadrat` are of different sizes.
That is why it never stands alone but always behind a pointer: `&dyn Flaeche` or
`Box<dyn Flaeche>`.

What lies behind it is then not known to the compiler. It only knows that the
thing meets the trait, and it looks the method up at the call. That is the whole
difference, and everything else follows from it.

### What it is good for

A list with a rectangle and a square in it. With a generic `F` that does not
work, because `Vec<F>` is a list of exactly one type, and `F` becomes exactly one
at the call.

With `Vec<Box<dyn Flaeche>>` it works. The list holds pointers, all of the same
size, and what lies at the end of each pointer may differ. That is exactly why
trait objects exist.

The same thing also works with an `enum`, and the last two exercises build
precisely that. Which of the two ways fits when stands further down under a
heading of its own, because it is not a matter of taste.

### The explanation

One list, two types, one trait.

```rust
trait Flaeche {
    fn flaeche(&self) -> u32;
    fn name(&self) -> &'static str;
}

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

struct Quadrat {
    seite: u32,
}

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }

    fn name(&self) -> &'static str {
        "Rechteck"
    }
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    fn name(&self) -> &'static str {
        "Quadrat"
    }
}

fn main() {
    // Deutsch: Eine Liste, zwei Typen. Das geht nur hinter einem Zeiger.
    let formen: Vec<Box<dyn Flaeche>> = vec![
        Box::new(Rechteck { breite: 3, hoehe: 4 }),
        Box::new(Quadrat { seite: 5 }),
    ];

    for form in &formen {
        println!("{} {}", form.name(), form.flaeche());
    }

    let gesamt: u32 = formen.iter().map(|form| form.flaeche()).sum();
    println!("{gesamt}");
}
```

The program prints:

```text
Rechteck 12
Quadrat 25
37
```

The loop calls the same line twice and lands in a different body twice. That is
what a trait object does, and the body of the loop notices nothing of it.

### What the detour costs

A pointer to a trait object is twice as wide as an ordinary one. In it stands the
address of the value and next to it the address of a table with the methods, and
every call goes over the second one.

```rust
use std::mem::size_of;

trait Flaeche {
    fn flaeche(&self) -> u32;
}

struct Rechteck {
    breite: u32,
    hoehe: u32,
}

enum Form {
    Rechteck { breite: u32, hoehe: u32 },
    Quadrat { seite: u32 },
}

impl Flaeche for Rechteck {
    fn flaeche(&self) -> u32 {
        self.breite * self.hoehe
    }
}

impl Form {
    fn flaeche(&self) -> u32 {
        match self {
            Form::Rechteck { breite, hoehe } => breite * hoehe,
            Form::Quadrat { seite } => seite * seite,
        }
    }
}

fn main() {
    let rechteck = Rechteck { breite: 3, hoehe: 4 };
    let hinter_zeiger: &dyn Flaeche = &rechteck;
    let im_kasten: Box<dyn Flaeche> = Box::new(Rechteck { breite: 3, hoehe: 4 });

    println!("{}", hinter_zeiger.flaeche());
    println!("{}", im_kasten.flaeche());
    println!("{}", Form::Rechteck { breite: 3, hoehe: 4 }.flaeche());
    println!("{}", Form::Quadrat { seite: 5 }.flaeche());

    println!("{}", size_of::<&Rechteck>());
    println!("{}", size_of::<&dyn Flaeche>());
    println!("{}", size_of::<Box<dyn Flaeche>>());
    println!("{}", size_of::<Form>());
}
```

The program prints:

```text
12
12
12
25
8
16
16
12
```

The four numbers at the bottom are the point. A `&Rechteck` is 8 bytes, a
`&dyn Flaeche` is 16, a `Box<dyn Flaeche>` is the same, and the whole `Form` sits
at 12 and needs no pointer at all for it.

Measured on one machine with `rustc 1.97.1`, x86_64, with the program above.
Pointer sizes depend on the target, which is why the number and the command
belong together. What is not measured here is the running time: whoever wants to
say which of the two ways is faster measures that and does not lean on these
lines.

### Common mistakes

Putting two types into one list without the detour over the pointer.

```rust
struct Rechteck {
    breite: u32,
    hoehe: u32,
}

struct Quadrat {
    seite: u32,
}

fn main() {
    let formen = vec![Rechteck { breite: 3, hoehe: 4 }, Quadrat { seite: 5 }];

    println!("{}", formen.len());
}
```

The compiler answers:

```text
error[E0308]: mismatched types
  --> gemischte-liste.rs:11:57
   |
11 |     let formen = vec![Rechteck { breite: 3, hoehe: 4 }, Quadrat { seite: 5 }];
   |                                                         ^^^^^^^^^^^^^^^^^^^^ expected `Rechteck`, found `Quadrat`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`vec!` takes the type of the first element and expects it everywhere afterwards.
The way out is `Vec<Box<dyn Flaeche>>` with a `Box::new` around every element.

The second mistake comes later and surprises more: not every trait is allowed
behind `dyn`.

```rust
trait Verdoppeln {
    fn doppelt(&self) -> Self;
}

fn zeigen(wert: &dyn Verdoppeln) {
    let _ = wert;
}

fn main() {}
```

The compiler answers:

```text
error[E0038]: the trait `Verdoppeln` is not dyn compatible
 --> nicht-objektsicher.rs:5:18
  |
5 | fn zeigen(wert: &dyn Verdoppeln) {
  |                  ^^^^^^^^^^^^^^ `Verdoppeln` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> nicht-objektsicher.rs:2:26
  |
1 | trait Verdoppeln {
  |       ---------- this trait is not dyn compatible...
2 |     fn doppelt(&self) -> Self;
  |                          ^^^^ ...because method `doppelt` references the `Self` type in its return type
  = help: consider moving `doppelt` to another trait

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0038`.
```

The reason stands in the message. Behind `dyn` the own type is exactly what
nobody knows any more, and a method returning it could not be called. `Flaeche`
in this unit returns `u32` and `&'static str` and therefore has no such problem.

### When which version fits

Both ways solve the same task, and exercises 1 and 3 work out the same number.
What separates them is who is allowed to add something later.

An `enum` lists its cases. Whoever adds one changes the type, and every `match`
over it goes red until the new case is handled. That is an advantage as long as
the cases are known and their number is small: the compiler keeps the books on
where something is missing, and the value itself lies in the list instead of a
pointer to it.

A trait object lists nothing. Whoever adds a new type writes it and its `impl`
somewhere, and nothing else changes. That is the way when the set is open, when
foreign code is meant to contribute types of its own, or when all the cases in
one `enum` would grow uncomfortably large.

The shortest version of the rule: closed set, known cases, `enum`. Open set,
foreign types, `dyn`. And where both work, what decides is which change comes
more often, a new case or a new place that handles all the cases.

### What these tests do not answer

Exercise 2 asks for a `match` over the two cases. Whether a body uses that or two
`if let` one after another instead is not said by any test of this unit.

Measured rather than supposed: the solution was switched over to two `if let`
with a 0 at the end, and the test file was run afterwards.

```console
$ cargo test -q -p unit-07-09-trait-objekte --test exercise
running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

A test cannot see it either, because both bodies give back the same thing for
every input. The difference sits elsewhere: once a third case comes along, the
`match` goes red and the version with `if let` quietly gives back 0. Whoever
picks the second version loses exactly the bookkeeping the `enum` stands for in
this comparison.

Just as little does it stand here which of the two ways runs faster. What is
measured are sizes and not times.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `flaeche_von` stands there finished, and its
doc test is green.

- `gesamt_dyn` adds up the areas of a list of `Box<dyn Flaeche>`
- `Form::flaeche` gives back the area of the case at hand
- `gesamt_enum` works out the same sum over a list of `Form`
- `groesste_dyn` says what the largest shape is called, the first one on a tie

```console
cd units/07-09-trait-objekte
cargo test
```

### Source

    Book, chapter 18 "Object Oriented Programming Features", section 18.2
    "Using Trait Objects to Abstract over Shared Behavior",
    https://doc.rust-lang.org/book/ch18-02-trait-objects.html,
    checked against 1.97.1

    The Rust Reference, chapter 6.1 "Traits", section "Dyn compatibility",
    https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
