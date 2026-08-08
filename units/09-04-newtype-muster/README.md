# 09-04 Das Newtype-Muster / The newtype pattern

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-04-newtype-muster/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-01 struct`, `05-02 Traits` und
  `04-08 From, Into und der Operator ?`. Der Tupel-Struct, das Trait und `From`
  kommen von dort, hier kommt zusammen, wofür sie zusammen gut sind.
- Auf dieser Einheit bauen auf: `09-05 Fortgeschrittene Typen`, wo derselbe
  Typ noch einmal auftaucht, und jede Stelle, an der eine Zahl eine Bedeutung
  bekommen soll.
- Beim Antworten so zitieren: `09-04 Das Newtype-Muster`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein Newtype kostet zur Laufzeit nichts. Wer sagt, er sei eine zusätzliche
  Schachtel im Speicher, sagt etwas Falsches: ein Struct mit genau einem Feld
  hat dessen Größe.
- Der Doku-Test mit `compile_fail` zeigt, dass die Verwechslung
  zurückgewiesen wird. Er zeigt nicht, mit welcher Meldung, denn ein
  `compile_fail` ist auch dann grün, wenn der Grund ein anderer ist. Die Meldung
  selbst steht unter "Häufige Fehler" und stammt aus einem echten Lauf.
- Das Verbot, ein fremdes Trait auf einen fremden Typ zu schreiben, heißt
  Waisenregel. Sie fällt durch den Newtype nicht weg; der eigene Typ ist nur
  kein fremder mehr.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-04-newtype-muster/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `03-01 struct`, `05-02 Traits` and
  `04-08 From, Into und der Operator ?`. The tuple struct, the trait and `From`
  come from there, what comes together here is what they are good for together.
- Building on this unit: `09-05 Fortgeschrittene Typen`, where the same type
  turns up once more, and every place where a number should get a meaning.
- Cite like this when answering: `09-04 Das Newtype-Muster`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A newtype costs nothing at run time. Whoever says it is an extra box in memory
  says something false: a struct with exactly one field has that field's size.
- The doc test with `compile_fail` shows that the mix-up is refused. It does not
  show with which message, because a `compile_fail` is green even when the
  reason is a different one. The message itself stands under "Common mistakes"
  and comes from a real run.
- The ban on writing a foreign trait onto a foreign type is called the orphan
  rule. It does not fall away through the newtype; the type of your own is
  merely no longer a foreign one.

</details>

## Deutsch

### Worum es geht

Ein Newtype ist ein Struct mit genau einem Feld, das einen anderen Wert
umschließt. `struct Zentimeter(u32)` ist einer. Er trägt dieselbe Zahl wie
vorher und ist trotzdem ein anderer Typ, und das ist der ganze Trick.

Zwei Newtypes über demselben `u32` sind füreinander unbrauchbar. Wo eine
`Zentimeter` erwartet wird, passt keine `Gramm` hinein, obwohl beide dieselben
vier Byte tragen. Der Übersetzer lehnt die Verwechslung ab, bevor das Programm
läuft.

Ausgepackt wird mit `.0`, denn ein Tupel-Struct nummeriert seine Felder. Nach
dem Auspacken ist es wieder eine gewöhnliche Zahl ohne Bedeutung, und das
Einpacken macht sie wieder zu einer Länge.

### Wofür das gut ist

Der erste Grund ist die Verwechslung. Eine Funktion mit drei `u32` in den
Parametern nimmt jede Reihenfolge an, und ein vertauschtes Paar fällt erst auf,
wenn eine Zahl unsinnig wird. Mit drei Newtypes fällt es beim Übersetzen auf, und
zwar an der Aufrufstelle.

Der zweite Grund ist die Waisenregel. Ein Trait darf man nur dann für einen Typ
schreiben, wenn eines von beiden aus dem eigenen Paket kommt. `Display` für
`Vec<String>` ist deshalb verboten: beide gehören der Standardbibliothek. Ein
eigener Typ um den `Vec` herum macht die Hälfte eigen, und danach geht es.

Das kostet nichts. Ein Struct mit genau einem Feld hat die Größe dieses Feldes,
und das Ein- und Auspacken steht nach dem Übersetzen nicht mehr im Programm. Was
bleibt, ist die Trennung im Typ.

Der Preis steht woanders. Der Newtype erbt nichts: was der innere Typ konnte,
kann er nicht, solange es niemand durchreicht. Deshalb steht am Ende dieser
Einheit ein `From`, das den Übergang einmal aufschreibt, statt ihn an jeder
Stelle neu zu erfinden.

### Die Erklärung

Ein Programm mit zwei Längen, einer Masse und einem fremden Trait auf einem
fremden Typ.

```rust
use std::fmt;

struct Zentimeter(u32);
struct Gramm(u32);

// Deutsch: Ein eigener Typ um einen fremden herum. Erst dadurch lässt sich
// Display für eine Liste schreiben, die uns nicht gehört.
struct Liste(Vec<String>);

impl fmt::Display for Liste {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

fn main() {
    let breite = Zentimeter(80);
    let hoehe = Zentimeter(120);

    println!("{}", addiere(breite, hoehe).0);

    let gewicht = Gramm(80);
    println!("{}", gewicht.0);

    println!("{}", Liste(vec![String::from("a"), String::from("b")]));
}
```

Das Programm gibt aus:

```console
$ ./newtype
200
80
[a, b]
```

Die zweite Zeile ist die interessante. `Gramm(80)` und `Zentimeter(80)` tragen
dieselbe Zahl, und ausgepackt sind sie nicht mehr auseinanderzuhalten.
Eingepackt sind sie es, und die Grenze verläuft genau an `.0`.

### Häufige Fehler

Zwei Newtypes für austauschbar halten, weil dieselbe Zahl darin steht.

```rust
struct Zentimeter(u32);
struct Gramm(u32);

fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

fn main() {
    let breite = Zentimeter(80);
    let gewicht = Gramm(120);

    println!("{}", addiere(breite, gewicht).0);
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
  --> verwechslung.rs:12:36
   |
12 |     println!("{}", addiere(breite, gewicht).0);
   |                    -------         ^^^^^^^ expected `Zentimeter`, found `Gramm`
   |                    |
   |                    arguments to this function are incorrect
   |
note: function defined here
  --> verwechslung.rs:4:4
   |
 4 | fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
   |    ^^^^^^^                -------------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Genau diese Meldung ist der Zweck des Musters. Ohne die beiden Typen stünde dort
zweimal `u32`, der Aufruf ginge durch, und die falsche Zahl käme erst beim
Rechnen heraus. Die Antwort ist nicht, den Newtype wegzulassen, sondern den
Übergang aufzuschreiben, und das ist Aufgabe 3.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `addiere` steht fertig da, und seine beiden Doku-Tests sind
grün, der zweite dadurch, dass er nicht übersetzt.

- `summe` zählt Längen zusammen und gibt wieder eine Länge zurück
- `Display` für `Liste` schreibt die Einträge in eckigen Klammern
- `From<Kilometer> for Zentimeter` schreibt den Übergang einmal auf

```console
cd units/09-04-newtype-muster
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features",
    Abschnitt 20.2 "Advanced Traits",
    https://doc.rust-lang.org/book/ch20-02-advanced-traits.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 20 "Advanced Features",
    Abschnitt 20.3 "Advanced Types",
    https://doc.rust-lang.org/book/ch20-03-advanced-types.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A newtype is a struct with exactly one field wrapping another value.
`struct Zentimeter(u32)` is one. It carries the same number as before and is a
different type all the same, and that is the whole trick.

Two newtypes over the same `u32` are useless to each other. Where a `Zentimeter`
is expected, no `Gramm` fits in, although both carry the same four bytes. The
compiler refuses the mix-up before the program runs.

Unwrapping goes through `.0`, because a tuple struct numbers its fields. After
unwrapping it is an ordinary number without a meaning again, and wrapping makes
it a length again.

### What it is good for

The first reason is the mix-up. A function with three `u32` in its parameters
accepts every order, and a swapped pair only shows up once a number turns
nonsensical. With three newtypes it shows up at compile time, and at the place
of the call.

The second reason is the orphan rule. A trait may be written for a type only
when one of the two comes from your own package. `Display` for `Vec<String>` is
therefore forbidden: both belong to the standard library. A type of your own
around the `Vec` makes half of it your own, and after that it works.

That costs nothing. A struct with exactly one field has the size of that field,
and the wrapping and unwrapping is no longer in the program after compiling.
What stays is the separation in the type.

The price stands elsewhere. The newtype inherits nothing: what the inner type
could do, it cannot, as long as nobody passes it through. That is why a `From`
stands at the end of this unit, writing the crossing down once instead of
inventing it anew at every place.

### The explanation

A program with two lengths, one mass and a foreign trait on a foreign type.

```rust
use std::fmt;

struct Zentimeter(u32);
struct Gramm(u32);

// Deutsch: Ein eigener Typ um einen fremden herum. Erst dadurch lässt sich
// Display für eine Liste schreiben, die uns nicht gehört.
struct Liste(Vec<String>);

impl fmt::Display for Liste {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

fn main() {
    let breite = Zentimeter(80);
    let hoehe = Zentimeter(120);

    println!("{}", addiere(breite, hoehe).0);

    let gewicht = Gramm(80);
    println!("{}", gewicht.0);

    println!("{}", Liste(vec![String::from("a"), String::from("b")]));
}
```

The program prints:

```console
$ ./newtype
200
80
[a, b]
```

The second line is the interesting one. `Gramm(80)` and `Zentimeter(80)` carry
the same number, and unwrapped they can no longer be told apart. Wrapped they
can, and the border runs exactly at `.0`.

### Common mistakes

Taking two newtypes for interchangeable because the same number stands in them.

```rust
struct Zentimeter(u32);
struct Gramm(u32);

fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
    Zentimeter(a.0 + b.0)
}

fn main() {
    let breite = Zentimeter(80);
    let gewicht = Gramm(120);

    println!("{}", addiere(breite, gewicht).0);
}
```

The compiler answers:

```text
error[E0308]: mismatched types
  --> verwechslung.rs:12:36
   |
12 |     println!("{}", addiere(breite, gewicht).0);
   |                    -------         ^^^^^^^ expected `Zentimeter`, found `Gramm`
   |                    |
   |                    arguments to this function are incorrect
   |
note: function defined here
  --> verwechslung.rs:4:4
   |
 4 | fn addiere(a: Zentimeter, b: Zentimeter) -> Zentimeter {
   |    ^^^^^^^                -------------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

This message is exactly the point of the pattern. Without the two types `u32`
would stand there twice, the call would go through, and the wrong number would
only come out while calculating. The answer is not to leave the newtype out but
to write the crossing down, and that is exercise 3.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `addiere` stands there finished, and both of
its doc tests are green, the second one by not compiling.

- `summe` adds lengths up and returns a length again
- `Display` for `Liste` writes the entries in square brackets
- `From<Kilometer> for Zentimeter` writes the crossing down once

```console
cd units/09-04-newtype-muster
cargo test
```

### Source

    Book, chapter 20 "Advanced Features",
    section 20.2 "Advanced Traits",
    https://doc.rust-lang.org/book/ch20-02-advanced-traits.html,
    checked against 1.97.1

    Book, chapter 20 "Advanced Features",
    section 20.3 "Advanced Types",
    https://doc.rust-lang.org/book/ch20-03-advanced-types.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
