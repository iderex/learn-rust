# 03-02 Methoden / Methods

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-02-methoden/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-01 struct` und die Ausleihregeln aus
  `02-03` und `02-04`.
- Auf dieser Einheit bauen auf: die weiteren Einheiten der Stufe 3 und später
  `05-02 Traits`, wo Methoden aus einem Trait kommen.
- Beim Antworten so zitieren: `03-02 Methoden`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `self`, `&self` und `&mut self` sind dieselben drei Fälle wie bei einem
  Parameter in Stufe 2. Wer das anders erklärt, erklärt zweimal dasselbe auf
  zwei Arten.
- `new` ist hier eine gewöhnliche zugeordnete Funktion und kein Schlüsselwort.
  Wer sie einen Konstruktor nennt, sagt bitte dazu, dass die Sprache davon
  nichts weiß.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-02-methoden/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `03-01 struct` and the borrowing rules from `02-03` and
  `02-04`.
- Building on this unit: the further units of stage 3 and later `05-02 Traits`,
  where methods come out of a trait.
- Cite like this when answering: `03-02 Methoden`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `self`, `&self` and `&mut self` are the same three cases as for a parameter in
  stage 2. Whoever explains that differently explains the same thing twice in
  two ways.
- `new` is an ordinary associated function here and not a keyword. Whoever calls
  it a constructor, please say alongside it that the language knows nothing of
  the term.

</details>

## Deutsch

### Worum es geht

Ein `impl`-Block gehört zu einem Typ und sammelt, was man mit ihm machen kann.
Was darin steht, ist entweder eine Methode oder eine zugeordnete Funktion.

Eine Methode hat `self` als ersten Parameter und wird mit dem Punkt aufgerufen.
Für `self` gibt es dieselben drei Fälle wie für jeden anderen Parameter in
Stufe 2: `&self` leiht den Wert und liest nur, `&mut self` leiht ihn zum
Verändern, und `self` nimmt ihn an sich.

Eine zugeordnete Funktion hat kein `self` und wird mit zwei Doppelpunkten
aufgerufen, also `Rectangle::new(3, 4)`. `new` ist dabei ein üblicher Name und
kein Schlüsselwort.

### Wofür das gut ist

Eine Methode steht dort, wo der Typ steht. Wer wissen will, was ein Rechteck
kann, liest einen Block statt das ganze Programm nach Funktionen mit
`&Rectangle` zu durchsuchen.

Der Aufruf sagt außerdem mehr. `rechteck.double()` nennt den Wert zuerst, und an
`&mut self` in der Signatur steht, dass er dabei verändert wird. Bei einer
zugeordneten Funktion gibt es noch keinen Wert, den man vorne hinschreiben
könnte, und genau deshalb hat `new` kein `self`.

`Self` mit großem S ist der Typ, in dessen `impl` man steht. Er spart die
Wiederholung des Namens und macht eine Umbenennung zu einer Änderung an einer
Stelle.

### Die Erklärung

Ein `impl`-Block mit allen vier Formen.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

impl Rectangle {
    // Deutsch: Eine zugeordnete Funktion ohne `self`. Aufgerufen wird sie mit
    // zwei Doppelpunkten. `Self` ist der Typ, in dessen `impl` sie steht.
    fn new(breite: u32, hoehe: u32) -> Self {
        Self { breite, hoehe }
    }

    // Deutsch: `&self` liest nur.
    fn area(&self) -> u32 {
        self.breite * self.hoehe
    }

    // Deutsch: `&mut self` verändert den Wert, zu dem die Methode gehört.
    fn double(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }

    // Deutsch: `self` nimmt den Wert an sich. Danach gibt es ihn unter dem
    // alten Namen nicht mehr.
    fn into_square(self) -> Rectangle {
        let seite = if self.breite > self.hoehe {
            self.breite
        } else {
            self.hoehe
        };

        Rectangle {
            breite: seite,
            hoehe: seite,
        }
    }
}

fn main() {
    let mut rechteck = Rectangle::new(3, 4);

    println!("{}", rechteck.area());

    rechteck.double();

    println!("{} {}", rechteck.breite, rechteck.hoehe);

    let quadrat = rechteck.into_square();

    println!("{}", quadrat.area());
}
```

Nach `into_square` ist `rechteck` verschoben, genau wie in `02-01`. Wer es
danach noch benutzt, bekommt `E0382`.

### Häufige Fehler

Eine verändernde Methode auf einer Bindung ohne `mut`.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

impl Rectangle {
    fn double(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    rechteck.double();

    println!("{} {}", rechteck.breite, rechteck.hoehe);
}
```

Der Übersetzer sagt dazu:

```text
error[E0596]: cannot borrow `rechteck` as mutable, as it is not declared as mutable
  --> methode.rs:19:5
   |
19 |     rechteck.double();
   |     ^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
14 |     let mut rechteck = Rectangle {
   |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

Am Aufruf steht kein `&mut`, und trotzdem geht es um eine veränderbare Ausleihe:
der Punkt nimmt sie sich, weil die Methode `&mut self` verlangt. Die Antwort
steht in der Meldung, nämlich `mut` an der Bindung.

Das ist derselbe Fall wie in `02-04`, nur eine Zeile früher: dort fehlte die
Erlaubnis mitten im Programm, hier fehlt sie schon beim Anlegen.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Rectangle` und die Methode `area` stehen schon da.

- `Rectangle::new` legt ein Rechteck an, ohne `self`
- `perimeter` gibt den Umfang zurück und liest nur
- `double` verdoppelt beide Seiten an Ort und Stelle

```console
cd units/03-02-methoden
cargo test
```

### Quelle

    Buch, Kapitel 5 "Using Structs to Structure Related Data", Abschnitt 5.3 "Methods",
    https://doc.rust-lang.org/book/ch05-03-method-syntax.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

An `impl` block belongs to a type and gathers what can be done with it. What
stands inside it is either a method or an associated function.

A method has `self` as its first parameter and is called with the dot. For
`self` there are the same three cases as for any other parameter in stage 2:
`&self` borrows the value and only reads, `&mut self` borrows it for changing,
and `self` takes it.

An associated function has no `self` and is called with two colons, so
`Rectangle::new(3, 4)`. `new` is a customary name there and not a keyword.

### What it is good for

A method stands where the type stands. Whoever wants to know what a rectangle
can do reads one block instead of searching the whole program for functions
taking `&Rectangle`.

The call also says more. `rechteck.double()` names the value first, and `&mut
self` in the signature says that it gets changed in the process. For an
associated function there is no value yet that could be written in front, and
that is exactly why `new` has no `self`.

`Self` with a capital S is the type whose `impl` you are inside. It saves
repeating the name and makes a rename a change in one place.

### The explanation

One `impl` block with all four forms.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

impl Rectangle {
    // Deutsch: Eine zugeordnete Funktion ohne `self`. Aufgerufen wird sie mit
    // zwei Doppelpunkten. `Self` ist der Typ, in dessen `impl` sie steht.
    fn new(breite: u32, hoehe: u32) -> Self {
        Self { breite, hoehe }
    }

    // Deutsch: `&self` liest nur.
    fn area(&self) -> u32 {
        self.breite * self.hoehe
    }

    // Deutsch: `&mut self` verändert den Wert, zu dem die Methode gehört.
    fn double(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }

    // Deutsch: `self` nimmt den Wert an sich. Danach gibt es ihn unter dem
    // alten Namen nicht mehr.
    fn into_square(self) -> Rectangle {
        let seite = if self.breite > self.hoehe {
            self.breite
        } else {
            self.hoehe
        };

        Rectangle {
            breite: seite,
            hoehe: seite,
        }
    }
}

fn main() {
    let mut rechteck = Rectangle::new(3, 4);

    println!("{}", rechteck.area());

    rechteck.double();

    println!("{} {}", rechteck.breite, rechteck.hoehe);

    let quadrat = rechteck.into_square();

    println!("{}", quadrat.area());
}
```

After `into_square` the value `rechteck` is moved, exactly as in `02-01`.
Whoever uses it afterwards gets `E0382`.

### Common mistakes

A changing method on a binding without `mut`.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

impl Rectangle {
    fn double(&mut self) {
        self.breite *= 2;
        self.hoehe *= 2;
    }
}

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    rechteck.double();

    println!("{} {}", rechteck.breite, rechteck.hoehe);
}
```

The compiler answers:

```text
error[E0596]: cannot borrow `rechteck` as mutable, as it is not declared as mutable
  --> methode.rs:19:5
   |
19 |     rechteck.double();
   |     ^^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
14 |     let mut rechteck = Rectangle {
   |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

No `&mut` stands at the call, and it is still about a mutable loan: the dot
takes one because the method asks for `&mut self`. The answer stands in the
message, namely `mut` at the binding.

It is the same case as in `02-04`, only one line earlier: there the permission
was missing in the middle of the program, here it is already missing where the
value is created.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Rectangle` and the method `area` are already
there.

- `Rectangle::new` creates a rectangle, without `self`
- `perimeter` returns the perimeter and only reads
- `double` doubles both sides in place

```console
cd units/03-02-methoden
cargo test
```

### Source

    Book, chapter 5 "Using Structs to Structure Related Data", section 5.3 "Methods",
    https://doc.rust-lang.org/book/ch05-03-method-syntax.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
