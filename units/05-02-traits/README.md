# 05-02 Traits / Traits

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-02-traits/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-02 Methoden`, `03-07 Display selbst schreiben`
  und `05-01 Generische Typen`.
- Auf dieser Einheit bauen auf: `05-03 Trait Bounds`, `07-09 Trait-Objekte` und
  alles, was sich auf gemeinsames Verhalten stützt.
- Beim Antworten so zitieren: `05-02 Traits`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Display` aus `03-07`, `From` aus `04-08` und `Error` aus `04-10` waren schon
  Traits. Wer hier so tut, als sei das etwas Neues, verliert die Verbindung
  dorthin.
- Ein Trait ist keine Klasse und keine Vererbung. Er sagt, was ein Typ kann, und
  legt keine Daten fest.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-02-traits/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `03-02 Methoden`, `03-07 Display selbst schreiben` and
  `05-01 Generische Typen`.
- Building on this unit: `05-03 Trait Bounds`, `07-09 Trait-Objekte` and
  everything resting on shared behaviour.
- Cite like this when answering: `05-02 Traits`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Display` from `03-07`, `From` from `04-08` and `Error` from `04-10` were
  traits already. Whoever acts here as if this were something new loses the
  connection back to them.
- A trait is not a class and not inheritance. It says what a type can do and
  fixes no data.

</details>

## Deutsch

### Worum es geht

Ein Trait ist eine Liste dessen, was ein Typ können muss. Er nennt Methoden mit
ihren Signaturen, und ein `impl Trait for Typ` sagt, wie dieser eine Typ sie
erfüllt.

Eine Methode im Trait darf schon einen Rumpf haben. Dann ist sie die
Standardfassung, und ein Typ, der nichts anderes sagt, bekommt sie geschenkt.
Wer sie überschreibt, schreibt sie in seinem `impl` noch einmal.

Neu ist daran nur der Name. `Display` in `03-07`, `From` in `04-08` und `Error`
in `04-10` waren Traits, und dort stand schon dieselbe Form.

### Wofür das gut ist

Der Trait trennt, was etwas kann, von dem, was es ist. Zwei Typen mit ganz
verschiedenen Feldern können dieselbe Frage beantworten, und wer die Frage
stellt, muss die Felder nicht kennen.

Standardmethoden sparen die Wiederholung. Was für die meisten Typen gleich
aussieht, steht einmal im Trait, und nur der Typ, für den es anders ist,
schreibt es neu.

Und ein eigener Trait darf auch für einen fremden Typ geschrieben werden. `u32`
kommt aus der Standardbibliothek, der Trait aus dem eigenen Code, und deshalb
ist das erlaubt. Umgekehrt geht es nicht: einen fremden Trait für einen fremden
Typ zu schreiben ist verboten, sonst könnten zwei Crates dasselbe verschieden
beantworten.

### Die Erklärung

Ein Trait, drei Implementierungen, eine davon für einen fremden Typ.

```rust
// Deutsch: Ein Trait sagt, was ein Typ können muss. Eine Methode ohne Rumpf
// muss jeder Typ selbst schreiben, eine mit Rumpf ist die Standardfassung.
trait Flaeche {
    fn flaeche(&self) -> u32;

    fn beschreibung(&self) -> String {
        format!("Flaeche {}", self.flaeche())
    }
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
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    // Deutsch: Die Standardfassung wird hier überschrieben.
    fn beschreibung(&self) -> String {
        format!("Quadrat mit Seite {}", self.seite)
    }
}

// Deutsch: Ein eigener Trait darf auch für einen fremden Typ geschrieben
// werden, solange der Trait im eigenen Code steht.
impl Flaeche for u32 {
    fn flaeche(&self) -> u32 {
        *self
    }
}

fn main() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };
    let quadrat = Quadrat { seite: 3 };

    println!("{}", rechteck.beschreibung());
    println!("{}", quadrat.beschreibung());
    println!("{}", 7u32.beschreibung());
}
```

Das Programm gibt aus:

```text
Flaeche 12
Quadrat mit Seite 3
Flaeche 7
```

Die erste und die dritte Zeile kommen aus der Standardfassung, die zweite aus
dem `impl`, das sie überschreibt.

### Häufige Fehler

Eine Methode ohne Rumpf nicht schreiben.

```rust
trait Flaeche {
    fn flaeche(&self) -> u32;

    fn beschreibung(&self) -> String {
        format!("Flaeche {}", self.flaeche())
    }
}

struct Quadrat {
    seite: u32,
}

impl Flaeche for Quadrat {
    fn beschreibung(&self) -> String {
        String::from("ein Quadrat")
    }
}

fn main() {
    let quadrat = Quadrat { seite: 3 };

    println!("{}", quadrat.beschreibung());
}
```

Der Übersetzer sagt dazu:

```text
error[E0046]: not all trait items implemented, missing: `flaeche`
  --> fehlt.rs:13:1
   |
 2 |     fn flaeche(&self) -> u32;
   |     ------------------------- `flaeche` from trait
...
13 | impl Flaeche for Quadrat {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ missing `flaeche` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
```

Die Meldung zeigt beide Stellen: die Zeile im Trait, die etwas verlangt, und den
`impl`-Block, in dem es fehlt. Dass `beschreibung` dasteht, hilft nicht, denn
gefehlt hat die andere.

Umgekehrt wäre eine Methode im `impl`, die im Trait nicht steht, ebenfalls ein
Fehler. Ein `impl Trait for Typ` beantwortet genau die Liste des Traits, nicht
mehr und nicht weniger.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Trait und die Implementierung für `Rechteck` stehen schon
da.

- `Flaeche for Quadrat` schreibt die verlangte Methode
- `beschreibung` für `Quadrat` überschreibt die Standardfassung
- `Flaeche for u32` schreibt den eigenen Trait für einen fremden Typ

```console
cd units/05-02-traits
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

A trait is a list of what a type has to be able to do. It names methods with
their signatures, and an `impl Trait for Typ` says how this one type fulfils
them.

A method in the trait may already have a body. Then it is the default version,
and a type saying nothing else gets it as a gift. Whoever overrides it writes it
once more in their `impl`.

The only new thing about this is the name. `Display` in `03-07`, `From` in
`04-08` and `Error` in `04-10` were traits, and the same shape stood there
already.

### What it is good for

The trait separates what something can do from what it is. Two types with
completely different fields can answer the same question, and whoever asks the
question does not have to know the fields.

Default methods save the repetition. What looks the same for most types stands
once in the trait, and only the type for which it differs writes it anew.

And a trait of your own may be written for a foreign type as well. `u32` comes
from the standard library, the trait from your own code, and that is why it is
allowed. The other way round it does not work: writing a foreign trait for a
foreign type is forbidden, otherwise two crates could answer the same thing
differently.

### The explanation

One trait, three implementations, one of them for a foreign type.

```rust
// Deutsch: Ein Trait sagt, was ein Typ können muss. Eine Methode ohne Rumpf
// muss jeder Typ selbst schreiben, eine mit Rumpf ist die Standardfassung.
trait Flaeche {
    fn flaeche(&self) -> u32;

    fn beschreibung(&self) -> String {
        format!("Flaeche {}", self.flaeche())
    }
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
}

impl Flaeche for Quadrat {
    fn flaeche(&self) -> u32 {
        self.seite * self.seite
    }

    // Deutsch: Die Standardfassung wird hier überschrieben.
    fn beschreibung(&self) -> String {
        format!("Quadrat mit Seite {}", self.seite)
    }
}

// Deutsch: Ein eigener Trait darf auch für einen fremden Typ geschrieben
// werden, solange der Trait im eigenen Code steht.
impl Flaeche for u32 {
    fn flaeche(&self) -> u32 {
        *self
    }
}

fn main() {
    let rechteck = Rechteck {
        breite: 3,
        hoehe: 4,
    };
    let quadrat = Quadrat { seite: 3 };

    println!("{}", rechteck.beschreibung());
    println!("{}", quadrat.beschreibung());
    println!("{}", 7u32.beschreibung());
}
```

The program prints:

```text
Flaeche 12
Quadrat mit Seite 3
Flaeche 7
```

The first and the third line come from the default version, the second from the
`impl` that overrides it.

### Common mistakes

Not writing a method that has no body in the trait.

```rust
trait Flaeche {
    fn flaeche(&self) -> u32;

    fn beschreibung(&self) -> String {
        format!("Flaeche {}", self.flaeche())
    }
}

struct Quadrat {
    seite: u32,
}

impl Flaeche for Quadrat {
    fn beschreibung(&self) -> String {
        String::from("ein Quadrat")
    }
}

fn main() {
    let quadrat = Quadrat { seite: 3 };

    println!("{}", quadrat.beschreibung());
}
```

The compiler answers:

```text
error[E0046]: not all trait items implemented, missing: `flaeche`
  --> fehlt.rs:13:1
   |
 2 |     fn flaeche(&self) -> u32;
   |     ------------------------- `flaeche` from trait
...
13 | impl Flaeche for Quadrat {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ missing `flaeche` in implementation

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0046`.
```

The message shows both places: the line in the trait demanding something, and
the `impl` block where it is missing. That `beschreibung` stands there does not
help, because the other one was missing.

The other way round a method in the `impl` that does not stand in the trait
would be an error as well. An `impl Trait for Typ` answers exactly the list of
the trait, no more and no less.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The trait and the implementation for
`Rechteck` are already there.

- `Flaeche for Quadrat` writes the method that is demanded
- `beschreibung` for `Quadrat` overrides the default version
- `Flaeche for u32` writes the trait of your own for a foreign type

```console
cd units/05-02-traits
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
