# 09-02 Assoziierte Typen / Associated types

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-02-assoziierte-typen/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst erklären, warum der Typ zur Implementierung gehört und nicht zum
  Aufruf.
- Diese Einheit baut auf: `05-02 Traits` und `05-03 Trait Bounds`. Der Trait
  kommt von dort, hier kommt nur der Typ dazu, den er mitbringt.
- Auf dieser Einheit bauen auf: `09-03 Operatorüberladung`, wo `Add` einen
  assoziierten Typ `Output` trägt, und alles, was mit `Iterator` arbeitet.
- Beim Antworten so zitieren: `09-02 Assoziierte Typen`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein assoziierter Typ ist kein Typparameter mit anderer Schreibweise. Wer
  beides gleichsetzt, sagt bitte dazu, wie oft ein Typ den Trait dann erfüllen
  kann.
- `stelle` in dieser Einheit zählt Bytes und keine Zeichen. Wer einen Vorschlag
  macht, der die Stelle um eins weiterschiebt, sagt bitte, was bei `ä`
  passiert.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-02-assoziierte-typen/`. It is
  public. Whoever is asked for it may name it, but should first explain why the
  type belongs to the implementation and not to the call.
- This unit builds on: `05-02 Traits` and `05-03 Trait Bounds`. The trait comes
  from there, here only the type it brings along is added.
- Building on this unit: `09-03 Operatorüberladung`, where `Add` carries an
  associated type `Output`, and everything working with `Iterator`.
- Cite like this when answering: `09-02 Assoziierte Typen`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- An associated type is not a type parameter in different spelling. Whoever
  treats the two as the same, please add how often a type can meet the trait
  then.
- `stelle` in this unit counts bytes and not characters. Whoever suggests moving
  the position on by one, please say what happens at `ä`.

</details>

## Deutsch

### Worum es geht

Ein Trait sagt bisher, welche Funktionen ein Typ hat. Er kann außerdem einen
Typ mitbringen, den erst die Implementierung festlegt.

```rust
trait Quelle {
    type Item;

    fn naechstes(&mut self) -> Option<Self::Item>;
}
```

`type Item` ist ein assoziierter Typ. Wer `Quelle` erfüllt, sagt einmal, was
`Item` bei ihm ist, und danach steht es fest. `Self::Item` ist dann der Name
dafür, ohne dass irgendwo `u32` oder `char` geschrieben stehen müsste.

Genau so ist `Iterator` in der Standardbibliothek gebaut: `next` gibt
`Option<Self::Item>`, und `Item` legt jeder Iterator für sich fest.

### Wofür das gut ist

Denselben Effekt scheint ein Typparameter zu haben, also `trait Quelle<T>` mit
`fn naechstes(&mut self) -> Option<T>`. Der Unterschied ist, wie oft ein Typ den
Trait erfüllen darf.

Mit einem assoziierten Typ genau einmal. Ein Typ ist eine Quelle, und was er
liefert, steht damit fest. Mit einem Typparameter beliebig oft, einmal je `T`,
und dann steht am Aufruf nicht mehr fest, welche Implementierung gemeint ist.

Das ist keine Geschmacksfrage. Es entscheidet, ob `zaehler.naechstes()` ohne
weitere Angabe übersetzt oder ob an jeder Aufrufstelle ein Typ dazugeschrieben
werden muss. Der Abschnitt "Häufige Fehler" zeigt beide Meldungen.

### Die Erklärung

Zwei Quellen, ein Trait, zwei verschiedene `Item`, und eine Funktion, die beide
nimmt.

```rust
trait Quelle {
    // Deutsch: Der Typ gehört zur Implementierung und nicht zum Aufruf.
    type Item;

    fn naechstes(&mut self) -> Option<Self::Item>;
}

struct Zaehler {
    stand: u32,
    bis: u32,
}

impl Quelle for Zaehler {
    type Item = u32;

    fn naechstes(&mut self) -> Option<u32> {
        if self.stand >= self.bis {
            return None;
        }
        self.stand += 1;
        Some(self.stand)
    }
}

struct Buchstaben {
    wort: String,
    stelle: usize,
}

impl Quelle for Buchstaben {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        let zeichen = self.wort[self.stelle..].chars().next()?;
        self.stelle += zeichen.len_utf8();
        Some(zeichen)
    }
}

fn einsammeln<Q: Quelle>(quelle: &mut Q) -> Vec<Q::Item> {
    let mut gesammelt = Vec::new();
    while let Some(wert) = quelle.naechstes() {
        gesammelt.push(wert);
    }
    gesammelt
}

fn main() {
    let mut zaehler = Zaehler { stand: 0, bis: 3 };
    let mut buchstaben = Buchstaben {
        wort: String::from("los"),
        stelle: 0,
    };

    println!("{:?}", einsammeln(&mut zaehler));
    println!("{:?}", einsammeln(&mut buchstaben));
}
```

Übersetzt und gestartet gibt das aus:

```text
$ quellen.exe
[1, 2, 3]
['l', 'o', 's']
```

`einsammeln` steht einmal da und liefert einmal `Vec<u32>` und einmal
`Vec<char>`. Die Funktion nennt den gelieferten Typ `Q::Item` und braucht keinen
zweiten Typparameter dafür. Welcher Typ es wirklich ist, hat die Implementierung
entschieden.

### Häufige Fehler

Denselben Trait zweimal für denselben Typ erfüllen wollen, mit verschiedenem
`Item`.

```rust
impl Quelle for Zaehler {
    type Item = u32;

    fn naechstes(&mut self) -> Option<u32> {
        self.stand += 1;
        Some(self.stand)
    }
}

impl Quelle for Zaehler {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        Some('a')
    }
}
```

Das übersetzt nicht:

```text
error[E0119]: conflicting implementations of trait `Quelle` for type `Zaehler`
  --> zweimal.rs:18:1
   |
10 | impl Quelle for Zaehler {
   | ----------------------- first implementation here
...
18 | impl Quelle for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Zaehler`

error: aborting due to 1 previous error
```

Der Trait ist einmal je Typ zu haben, und `Item` ist keine Unterscheidung.

Mit einem Typparameter geht das Gegenteil, und dafür wandert der Preis an die
Aufrufstelle.

```rust
trait Quelle<T> {
    fn naechstes(&mut self) -> Option<T>;
}

impl Quelle<u32> for Zaehler {
    fn naechstes(&mut self) -> Option<u32> {
        self.stand += 1;
        Some(self.stand)
    }
}

impl Quelle<char> for Zaehler {
    fn naechstes(&mut self) -> Option<char> {
        Some('a')
    }
}

fn main() {
    let mut zaehler = Zaehler { stand: 0 };
    let wert = zaehler.naechstes();
    println!("{wert:?}");
}
```

Beide Implementierungen sind erlaubt. Der Aufruf ist es nicht:

```text
error[E0283]: type annotations needed for `Option<_>`
  --> parameter.rs:24:9
   |
24 |     let wert = zaehler.naechstes();
   |         ^^^^           --------- type must be known at this point
   |
note: multiple `impl`s satisfying `Zaehler: Quelle<_>` found
  --> parameter.rs:9:1
   |
 9 | impl Quelle<u32> for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
16 | impl Quelle<char> for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving `wert` an explicit type, where the type for type parameter `T` is specified
   |
24 |     let wert: Option<T> = zaehler.naechstes();
   |             +++++++++++

error: aborting due to 1 previous error
```

Die beiden Meldungen sind die zwei Seiten derselben Entscheidung. Ein
assoziierter Typ verbietet die zweite Implementierung, ein Typparameter erlaubt
sie und verlangt dafür an jeder Aufrufstelle eine Angabe.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Die assoziierten Typen stehen schon da, sonst würde die
Einheit nicht übersetzen; zu schreiben sind die Rümpfe.

- `Buchstaben` gibt die Zeichen eines Wortes heraus, `Item` ist `char`
- `Woerter` gibt die Wörter eines Satzes heraus, `Item` ist `String`
- `einsammeln` sammelt ein, was eine Quelle noch hergibt

```console
cd units/09-02-assoziierte-typen
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

So far a trait says which functions a type has. It can also bring along a type
that only the implementation fixes.

```rust
trait Quelle {
    type Item;

    fn naechstes(&mut self) -> Option<Self::Item>;
}
```

`type Item` is an associated type. Whoever meets `Quelle` says once what `Item`
is for them, and after that it stands. `Self::Item` is then the name for it,
without `u32` or `char` having to be written anywhere.

`Iterator` in the standard library is built exactly that way: `next` gives
`Option<Self::Item>`, and every iterator fixes `Item` for itself.

### What it is good for

A type parameter seems to do the same, meaning `trait Quelle<T>` with
`fn naechstes(&mut self) -> Option<T>`. The difference is how often a type is
allowed to meet the trait.

With an associated type exactly once. A type is a source, and what it delivers
stands with it. With a type parameter as often as you like, once per `T`, and
then it no longer stands at the call which implementation is meant.

That is not a matter of taste. It decides whether `zaehler.naechstes()` compiles
without further word or whether a type has to be written at every call site. The
section "Common mistakes" shows both messages.

### The explanation

Two sources, one trait, two different `Item`, and one function taking both.

```rust
trait Quelle {
    // Deutsch: Der Typ gehört zur Implementierung und nicht zum Aufruf.
    type Item;

    fn naechstes(&mut self) -> Option<Self::Item>;
}

struct Zaehler {
    stand: u32,
    bis: u32,
}

impl Quelle for Zaehler {
    type Item = u32;

    fn naechstes(&mut self) -> Option<u32> {
        if self.stand >= self.bis {
            return None;
        }
        self.stand += 1;
        Some(self.stand)
    }
}

struct Buchstaben {
    wort: String,
    stelle: usize,
}

impl Quelle for Buchstaben {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        let zeichen = self.wort[self.stelle..].chars().next()?;
        self.stelle += zeichen.len_utf8();
        Some(zeichen)
    }
}

fn einsammeln<Q: Quelle>(quelle: &mut Q) -> Vec<Q::Item> {
    let mut gesammelt = Vec::new();
    while let Some(wert) = quelle.naechstes() {
        gesammelt.push(wert);
    }
    gesammelt
}

fn main() {
    let mut zaehler = Zaehler { stand: 0, bis: 3 };
    let mut buchstaben = Buchstaben {
        wort: String::from("los"),
        stelle: 0,
    };

    println!("{:?}", einsammeln(&mut zaehler));
    println!("{:?}", einsammeln(&mut buchstaben));
}
```

Compiled and started that prints:

```text
$ quellen.exe
[1, 2, 3]
['l', 'o', 's']
```

`einsammeln` stands there once and delivers `Vec<u32>` one time and `Vec<char>`
the other. The function names the delivered type `Q::Item` and needs no second
type parameter for it. Which type it really is was decided by the
implementation.

### Common mistakes

Trying to meet the same trait twice for the same type, with a different `Item`.

```rust
impl Quelle for Zaehler {
    type Item = u32;

    fn naechstes(&mut self) -> Option<u32> {
        self.stand += 1;
        Some(self.stand)
    }
}

impl Quelle for Zaehler {
    type Item = char;

    fn naechstes(&mut self) -> Option<char> {
        Some('a')
    }
}
```

That does not compile:

```text
error[E0119]: conflicting implementations of trait `Quelle` for type `Zaehler`
  --> zweimal.rs:18:1
   |
10 | impl Quelle for Zaehler {
   | ----------------------- first implementation here
...
18 | impl Quelle for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Zaehler`

error: aborting due to 1 previous error
```

The trait is to be had once per type, and `Item` is no distinction.

With a type parameter the opposite works, and for that the price moves to the
call site.

```rust
trait Quelle<T> {
    fn naechstes(&mut self) -> Option<T>;
}

impl Quelle<u32> for Zaehler {
    fn naechstes(&mut self) -> Option<u32> {
        self.stand += 1;
        Some(self.stand)
    }
}

impl Quelle<char> for Zaehler {
    fn naechstes(&mut self) -> Option<char> {
        Some('a')
    }
}

fn main() {
    let mut zaehler = Zaehler { stand: 0 };
    let wert = zaehler.naechstes();
    println!("{wert:?}");
}
```

Both implementations are allowed. The call is not:

```text
error[E0283]: type annotations needed for `Option<_>`
  --> parameter.rs:24:9
   |
24 |     let wert = zaehler.naechstes();
   |         ^^^^           --------- type must be known at this point
   |
note: multiple `impl`s satisfying `Zaehler: Quelle<_>` found
  --> parameter.rs:9:1
   |
 9 | impl Quelle<u32> for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
16 | impl Quelle<char> for Zaehler {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving `wert` an explicit type, where the type for type parameter `T` is specified
   |
24 |     let wert: Option<T> = zaehler.naechstes();
   |             +++++++++++

error: aborting due to 1 previous error
```

The two messages are the two sides of the same decision. An associated type
forbids the second implementation, a type parameter allows it and asks for a
word at every call site in return.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The associated types already stand there,
otherwise the unit would not compile; what is to be written are the bodies.

- `Buchstaben` hands out the characters of a word, `Item` is `char`
- `Woerter` hands out the words of a sentence, `Item` is `String`
- `einsammeln` collects what a source still hands out

```console
cd units/09-02-assoziierte-typen
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
