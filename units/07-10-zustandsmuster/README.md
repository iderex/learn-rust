# 07-10 Das Zustandsmuster / The state pattern

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-10-zustandsmuster/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-02 Traits` und `07-04 Deref und Drop`. Das Muster
  steht auf Trait-Objekten, und die kommen von dort.
- Auf dieser Einheit bauen auf: alles, was Verhalten hinter einem `dyn Trait`
  austauscht.
- Beim Antworten so zitieren: `07-10 Das Zustandsmuster`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `self: Box<Self>` ist der Kern und keine Schreibweise. Ein Übergang
  verbraucht den alten Zustand, statt ihn zu verändern.
- Das `Option` am Feld ist kein "vielleicht keiner". Es ist der Griff, mit dem
  der alte Zustand herausgenommen wird. Wer das Gegenteil behauptet, sagt bitte,
  wie der Übergang ohne `take` aussehen soll.
- Dieses Muster ist nicht die einzige Antwort. Ein `enum` mit `match` tut
  dasselbe mit weniger Zeilen und mit anderen Kosten. Was hier steht, ist ein
  Weg und keine Empfehlung gegen den anderen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-10-zustandsmuster/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `05-02 Traits` and `07-04 Deref und Drop`. The pattern
  stands on trait objects, and those come from there.
- Building on this unit: everything that swaps behaviour behind a `dyn Trait`.
- Cite like this when answering: `07-10 Das Zustandsmuster`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `self: Box<Self>` is the core and not a spelling. A transition uses the old
  state up rather than changing it.
- The `Option` on the field is not a "maybe none". It is the handle with which
  the old state is taken out. Whoever claims the opposite, please say what the
  transition should look like without `take`.
- This pattern is not the only answer. An `enum` with `match` does the same with
  fewer lines and with different costs. What stands here is one way and not a
  recommendation against the other.

</details>

## Deutsch

### Worum es geht

Ein Beitrag ist ein Entwurf, dann in Prüfung, dann freigegeben. Das lässt sich
als Zahl im struct führen und an jeder Stelle mit einem `match` abfragen. Das
Zustandsmuster tut etwas anderes: Jeder Zustand ist ein eigener Typ.

Der Beitrag hält einen `Box<dyn State>`. Was in einem Zustand erlaubt ist, steht
in dessen eigener Implementierung, und ein Übergang ist eine Methode, die einen
neuen `Box<dyn State>` zurückgibt.

Diese Methoden nehmen `self: Box<Self>` und nicht `&self`. Sie verbrauchen den
alten Zustand, statt ihn zu verändern, und deshalb hält danach niemand mehr den
alten in der Hand.

### Wofür das gut ist

Ein neuer Zustand ist ein neuer Typ und sonst nichts. Kein `match` an fünf
Stellen muss um einen Zweig ergänzt werden, denn es gibt keine fünf Stellen. Der
Beitrag fragt seinen Zustand und weiß nicht, welche es gibt.

Und was in einem Zustand nicht erlaubt ist, muss nirgends verboten werden. Ein
Übergang, den es von hier aus nicht gibt, gibt einfach `self` zurück. Es gibt
keinen Fehlerfall, der behandelt werden müsste, und keine Bedingung, die jemand
vergessen könnte.

Der Preis steht in dieser Einheit mit im Text. `self: Box<Self>` heißt, dass
jeder Zustand auf dem Heap liegt, und das `Option` am Feld ist reine Mechanik.
Wer einen `enum` mit `match` nimmt, hat weniger Zeilen und dafür die fünf
Stellen. Diese Einheit zeigt einen Weg und behauptet nicht, dass der andere
falsch ist.

### Die Erklärung

Drei Zustände, ein Beitrag, und ein Übergang, den es nicht gibt.

```rust
// Deutsch: Ein Zustand ist ein eigener Typ. Was in ihm erlaubt ist, steht in
// seinem Trait und nicht in einem `match` an fünf Stellen.
trait Zustand {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand>;
    fn freigeben(self: Box<Self>) -> Box<dyn Zustand>;
    fn name(&self) -> &'static str;
    fn sichtbar(&self) -> bool;
}

struct Entwurf;

impl Zustand for Entwurf {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        Box::new(InPruefung)
    }

    // Deutsch: Freigeben geht aus dem Entwurf heraus nicht. Der Zustand bleibt,
    // wie er ist, und das ist die ganze Regel.
    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn name(&self) -> &'static str {
        "Entwurf"
    }

    fn sichtbar(&self) -> bool {
        false
    }
}

struct InPruefung;

impl Zustand for InPruefung {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        Box::new(Freigegeben)
    }

    fn name(&self) -> &'static str {
        "InPruefung"
    }

    fn sichtbar(&self) -> bool {
        false
    }
}

struct Freigegeben;

impl Zustand for Freigegeben {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn name(&self) -> &'static str {
        "Freigegeben"
    }

    fn sichtbar(&self) -> bool {
        true
    }
}

struct Beitrag {
    // Deutsch: `Option` ist hier kein "vielleicht keiner", sondern der Griff,
    // mit dem der alte Zustand herausgenommen wird, um ihn zu verbrauchen.
    zustand: Option<Box<dyn Zustand>>,
    text: String,
}

impl Beitrag {
    fn neu(text: &str) -> Beitrag {
        Beitrag {
            zustand: Some(Box::new(Entwurf)),
            text: text.to_string(),
        }
    }

    fn pruefen(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.pruefen());
        }
    }

    fn freigeben(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.freigeben());
        }
    }

    fn name(&self) -> &'static str {
        self.zustand.as_ref().expect("ein Zustand ist da").name()
    }

    fn inhalt(&self) -> &str {
        if self.zustand.as_ref().expect("ein Zustand ist da").sichtbar() {
            &self.text
        } else {
            ""
        }
    }
}

fn main() {
    let mut beitrag = Beitrag::neu("Heute gelernt");
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    // Deutsch: Freigeben aus dem Entwurf heraus tut nichts.
    beitrag.freigeben();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    beitrag.pruefen();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    beitrag.freigeben();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());
}
```

Das Programm gibt aus:

```text
Entwurf ""
Entwurf ""
InPruefung ""
Freigegeben "Heute gelernt"
```

Die zweiten beiden Zeilen sind die Aussage. Zwischen ihnen steht ein
`freigeben`, und der Zustand ist danach derselbe. Nichts hat das verboten, und
nichts hat einen Fehler gemeldet: `Entwurf::freigeben` gibt `self` zurück, und
damit ist der Fall erledigt.

Der Text erscheint erst in der letzten Zeile. `inhalt` entscheidet das nicht
selbst, es fragt den Zustand, und deshalb muss `Beitrag` nicht wissen, welche
Zustände es gibt.

### Häufige Fehler

Den Übergang ohne `take` schreiben, also den alten Zustand aus einem `&mut self`
herausnehmen wollen.

```rust
struct Beitrag {
    zustand: Box<dyn Zustand>,
}

impl Beitrag {
    fn weiter(&mut self) {
        self.zustand = self.zustand.weiter();
    }
}
```

Der Übersetzer sagt dazu:

```text
error[E0507]: cannot move out of `self.zustand` which is behind a mutable reference
  --> ohne-take.rs:36:24
   |
36 |         self.zustand = self.zustand.weiter();
   |                        ^^^^^^^^^^^^ -------- `self.zustand` moved due to this method call
   |                        |
   |                        move occurs because `self.zustand` has type `Box<dyn Zustand>`, which does not implement the `Copy` trait
   |
note: `Zustand::weiter` takes ownership of the receiver `self`, which moves `self.zustand`
  --> ohne-take.rs:2:15
   |
 2 |     fn weiter(self: Box<Self>) -> Box<dyn Zustand>;
   |               ^^^^
help: you could `clone` the value and consume it, if the following trait bounds could be satisfied: `dyn Zustand: Sized` and `dyn Zustand: Clone`
   |
36 |         self.zustand = <Box<dyn Zustand> as Clone>::clone(&self.zustand).weiter();
   |                        ++++++++++++++++++++++++++++++++++++            +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0507`.
```

Die Anmerkung nennt den Grund: Der Übergang nimmt den Zustand an sich. Aus einem
`&mut self` lässt sich aber nichts herausnehmen, denn danach stünde dort für
einen Moment nichts, und ein Abbruch mittendrin fände ein Feld ohne Wert vor.

Genau dafür ist das `Option`. `take` legt `None` hinein und gibt den alten Wert
heraus, und dann steht dort etwas Gültiges, während der Übergang läuft. Der
Vorschlag mit `clone` am Ende der Meldung führt hier nicht weiter, denn
`dyn Zustand` ist weder `Sized` noch `Clone`, und die Meldung sagt das mit.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Approved` steht fertig da und ist das Muster für die beiden
anderen, und sein Doku-Test ist grün.

- `State for Draft` sagt, dass `review` weiterführt und `approve` nicht
- `State for Pending` sagt es andersherum
- `Post::content` fragt den Zustand, statt selbst zu entscheiden

```console
cd units/07-10-zustandsmuster
cargo test
```

### Quelle

    Buch, Kapitel 18 "Object Oriented Programming Features", Abschnitt 18.3
    "Implementing an Object-Oriented Design Pattern",
    https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A post is a draft, then under review, then approved. That can be kept as a number
in the struct and asked about with a `match` at every place. The state pattern
does something else: every state is a type of its own.

The post holds a `Box<dyn State>`. What is allowed in a state stands in that
state's own implementation, and a transition is a method returning a new
`Box<dyn State>`.

Those methods take `self: Box<Self>` and not `&self`. They use the old state up
rather than changing it, and that is why nobody is holding the old one afterwards.

### What it is good for

A new state is a new type and nothing else. No `match` at five places has to gain
a further arm, because there are no five places. The post asks its state and does
not know which ones there are.

And what is not allowed in a state need not be forbidden anywhere. A transition
that does not exist from here simply returns `self`. There is no error case that
would have to be handled and no condition somebody could forget.

The price stands in the text of this unit as well. `self: Box<Self>` means every
state lies on the heap, and the `Option` on the field is pure mechanics. Whoever
takes an `enum` with `match` has fewer lines and the five places instead. This
unit shows one way and does not claim the other one is wrong.

### The explanation

Three states, one post, and a transition that does not exist.

```rust
// Deutsch: Ein Zustand ist ein eigener Typ. Was in ihm erlaubt ist, steht in
// seinem Trait und nicht in einem `match` an fünf Stellen.
trait Zustand {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand>;
    fn freigeben(self: Box<Self>) -> Box<dyn Zustand>;
    fn name(&self) -> &'static str;
    fn sichtbar(&self) -> bool;
}

struct Entwurf;

impl Zustand for Entwurf {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        Box::new(InPruefung)
    }

    // Deutsch: Freigeben geht aus dem Entwurf heraus nicht. Der Zustand bleibt,
    // wie er ist, und das ist die ganze Regel.
    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn name(&self) -> &'static str {
        "Entwurf"
    }

    fn sichtbar(&self) -> bool {
        false
    }
}

struct InPruefung;

impl Zustand for InPruefung {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        Box::new(Freigegeben)
    }

    fn name(&self) -> &'static str {
        "InPruefung"
    }

    fn sichtbar(&self) -> bool {
        false
    }
}

struct Freigegeben;

impl Zustand for Freigegeben {
    fn pruefen(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn freigeben(self: Box<Self>) -> Box<dyn Zustand> {
        self
    }

    fn name(&self) -> &'static str {
        "Freigegeben"
    }

    fn sichtbar(&self) -> bool {
        true
    }
}

struct Beitrag {
    // Deutsch: `Option` ist hier kein "vielleicht keiner", sondern der Griff,
    // mit dem der alte Zustand herausgenommen wird, um ihn zu verbrauchen.
    zustand: Option<Box<dyn Zustand>>,
    text: String,
}

impl Beitrag {
    fn neu(text: &str) -> Beitrag {
        Beitrag {
            zustand: Some(Box::new(Entwurf)),
            text: text.to_string(),
        }
    }

    fn pruefen(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.pruefen());
        }
    }

    fn freigeben(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.freigeben());
        }
    }

    fn name(&self) -> &'static str {
        self.zustand.as_ref().expect("ein Zustand ist da").name()
    }

    fn inhalt(&self) -> &str {
        if self.zustand.as_ref().expect("ein Zustand ist da").sichtbar() {
            &self.text
        } else {
            ""
        }
    }
}

fn main() {
    let mut beitrag = Beitrag::neu("Heute gelernt");
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    // Deutsch: Freigeben aus dem Entwurf heraus tut nichts.
    beitrag.freigeben();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    beitrag.pruefen();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());

    beitrag.freigeben();
    println!("{} {:?}", beitrag.name(), beitrag.inhalt());
}
```

The program prints:

```text
Entwurf ""
Entwurf ""
InPruefung ""
Freigegeben "Heute gelernt"
```

The first two lines are the statement. Between them stands a `freigeben`, and the
state is the same afterwards. Nothing forbade it and nothing reported an error:
`Entwurf::freigeben` returns `self`, and with that the case is settled.

The text appears only in the last line. `inhalt` does not decide that itself, it
asks the state, and that is why `Beitrag` need not know which states there are.

### Common mistakes

Writing the transition without `take`, meaning wanting to take the old state out
of a `&mut self`.

```rust
struct Beitrag {
    zustand: Box<dyn Zustand>,
}

impl Beitrag {
    fn weiter(&mut self) {
        self.zustand = self.zustand.weiter();
    }
}
```

The compiler answers:

```text
error[E0507]: cannot move out of `self.zustand` which is behind a mutable reference
  --> ohne-take.rs:36:24
   |
36 |         self.zustand = self.zustand.weiter();
   |                        ^^^^^^^^^^^^ -------- `self.zustand` moved due to this method call
   |                        |
   |                        move occurs because `self.zustand` has type `Box<dyn Zustand>`, which does not implement the `Copy` trait
   |
note: `Zustand::weiter` takes ownership of the receiver `self`, which moves `self.zustand`
  --> ohne-take.rs:2:15
   |
 2 |     fn weiter(self: Box<Self>) -> Box<dyn Zustand>;
   |               ^^^^
help: you could `clone` the value and consume it, if the following trait bounds could be satisfied: `dyn Zustand: Sized` and `dyn Zustand: Clone`
   |
36 |         self.zustand = <Box<dyn Zustand> as Clone>::clone(&self.zustand).weiter();
   |                        ++++++++++++++++++++++++++++++++++++            +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0507`.
```

The note names the reason: the transition takes the state for itself. Out of a
`&mut self`, however, nothing can be taken, because there would be nothing there
for a moment afterwards, and an abort in the middle would find a field without a
value.

That is exactly what the `Option` is for. `take` puts `None` in and hands the old
value out, and then something valid stands there while the transition runs. The
suggestion with `clone` at the end of the message leads nowhere here, because
`dyn Zustand` is neither `Sized` nor `Clone`, and the message says so itself.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Approved` stands there finished and is the
model for the other two, and its doc test is green.

- `State for Draft` says that `review` leads on and `approve` does not
- `State for Pending` says it the other way round
- `Post::content` asks the state instead of deciding itself

```console
cd units/07-10-zustandsmuster
cargo test
```

### Source

    Book, chapter 18 "Object Oriented Programming Features", section 18.3
    "Implementing an Object-Oriented Design Pattern",
    https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
