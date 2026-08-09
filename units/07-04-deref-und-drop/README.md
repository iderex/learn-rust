# 07-04 Deref und Drop / Deref and Drop

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-04-deref-und-drop/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-02 Traits` und `07-02 Rc`. Beides hier sind
  Traits, und `Rc` war der erste Typ, der sich schon so benutzen ließ.
- Auf dieser Einheit bauen auf: der Rest der Stufe 7 und alles, was einen
  eigenen Behälter schreibt.
- Beim Antworten so zitieren: `07-04 Deref und Drop`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Deref` verändert nichts am Wert. Es sagt dem Übersetzer nur, wohin ein `*`
  führt und wo er weitersuchen darf, wenn eine Methode am eigenen Typ nicht
  steht.
- `drop` lässt sich nicht als Methode aufrufen. Wer den Wert früher wegräumen
  will, nimmt die Funktion `drop(wert)`, und die Meldung dazu steht unter
  "Häufige Fehler".
- Die Reihenfolge beim Wegfallen ist umgekehrt zur Reihenfolge beim Anlegen.
  Das steht im Beispiel und in seiner Ausgabe, nicht nur als Behauptung.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-04-deref-und-drop/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `05-02 Traits` and `07-02 Rc`. Both things here are
  traits, and `Rc` was the first type that could already be used this way.
- Building on this unit: the rest of stage 7 and everything that writes a
  container of its own.
- Cite like this when answering: `07-04 Deref und Drop`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Deref` changes nothing about the value. It only tells the compiler where a
  `*` leads and where it may keep looking when a method is not on the type
  itself.
- `drop` cannot be called as a method. Whoever wants to clear the value away
  earlier takes the function `drop(wert)`, and the message for that is under
  "Common mistakes".
- The order of falling away is the reverse of the order of creation. That stands
  in the example and in its output, not only as a claim.

</details>

## Deutsch

### Worum es geht

`Deref` ist das Trait hinter dem Stern. Wer es für einen eigenen Typ schreibt,
sagt damit, welcher Wert dahinter liegt, und ab dann bedeutet `*wert` genau
diesen.

Dazu kommt etwas, das man leicht für Zauberei hält. Findet der Übersetzer eine
Methode am eigenen Typ nicht, sieht er durch `Deref` hindurch weiter. Deshalb
geht `karton.len()` an einem `Karton<String>`, obwohl `Karton` kein `len` hat.

`Drop` ist das Gegenstück am Ende. Es läuft, wenn ein Wert wegfällt, und der
Übersetzer setzt den Aufruf selbst ein. Aufrufen lässt es sich nicht.

### Wofür das gut ist

Ein eigener Behälter soll sich benutzen lassen wie das, was er enthält. Ohne
`Deref` müsste jeder Aufrufer den Inhalt erst auspacken, und der Behälter wäre
im Weg statt hilfreich. `Rc` aus der letzten Einheit war schon so gebaut, und
hier steht, wie das gemacht ist.

`Drop` nimmt einer ganzen Klasse von Fehlern den Boden. Eine Datei schließen,
eine Sperre freigeben, einen Zähler zurücksetzen: Das sind Dinge, die jemand
vergisst, wenn er sie hinschreiben muss. Steht es in `Drop`, geschieht es auch
auf dem Weg über einen Abbruch.

Die Reihenfolge ist dabei festgelegt und nicht zufällig. Werte fallen in der
umgekehrten Reihenfolge weg, in der sie angelegt wurden, sodass ein Wert, der
einen anderen braucht, noch da ist, wenn der andere geht.

### Die Erklärung

Ein eigener Behälter mit `Deref` und zwei Wächter mit `Drop` in einem Programm.

```rust
use std::ops::Deref;

// Deutsch: Ein eigener Zeiger. `Deref` sagt dem Übersetzer, worauf ein `*`
// führt, und damit auch, was ein Methodenaufruf durch ihn hindurch findet.
struct Karton<T>(T);

impl<T> Deref for Karton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Deutsch: `Drop` läuft, wenn der Wert wegfällt. Aufrufen lässt es sich nicht.
struct Wache {
    name: String,
}

impl Drop for Wache {
    fn drop(&mut self) {
        println!("{} geht", self.name);
    }
}

fn main() {
    let text = Karton(String::from("Ada"));

    // Deutsch: `*text` folgt dem Deref, und `text.len()` findet die Methode von
    // String, ohne dass hier ein Stern steht.
    println!("{}", *text);
    println!("{}", text.len());

    let _erste = Wache {
        name: String::from("erste"),
    };
    let _zweite = Wache {
        name: String::from("zweite"),
    };

    println!("Ende von main");
}
```

Das Programm gibt aus:

```text
Ada
3
Ende von main
zweite geht
erste geht
```

Die ersten beiden Zeilen sind `Deref`. Einmal mit Stern, einmal ohne, und beide
Male kommt der `String` im Karton heraus.

Die letzten drei sind `Drop`. "Ende von main" steht vor beiden Abgängen, denn
die Wächter fallen erst an der schließenden Klammer weg. Und "zweite" geht vor
"erste", weil das Wegfallen in umgekehrter Reihenfolge geschieht.

### Häufige Fehler

`drop` als Methode aufrufen wollen.

```rust
struct Wache {
    name: String,
}

impl Drop for Wache {
    fn drop(&mut self) {
        println!("{} geht", self.name);
    }
}

fn main() {
    let erste = Wache {
        name: String::from("erste"),
    };

    erste.drop();

    println!("danach");
}
```

Der Übersetzer sagt dazu:

```text
error[E0040]: explicit use of destructor method
  --> raeumen.rs:16:11
   |
16 |     erste.drop();
   |           ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
16 -     erste.drop();
16 +     drop(erste);
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0040`.
```

Die Meldung nennt gleich den Weg. `drop(erste)` nimmt den Wert an sich und lässt
ihn dann wegfallen, was das Aufräumen genau einmal auslöst. `erste.drop()` würde
den Aufruf machen und den Wert danach trotzdem noch einmal wegfallen lassen,
also zweimal aufräumen, und deshalb ist er verboten und nicht nur unschön.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Etikett` steht fertig da, und sein Doku-Test ist grün.

- `Deref for Karton<T>` gibt den gehaltenen Wert heraus
- `length` zählt die Bytes im Karton, ohne Stern
- `Drop for Wachhund` setzt das Kreuz beim Wegfallen

```console
cd units/07-04-deref-und-drop
cargo test
```

### Quelle

    Buch, Kapitel 15 "Smart Pointers", Abschnitt 15.2
    "Treating Smart Pointers Like Regular References",
    https://doc.rust-lang.org/book/ch15-02-deref.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 15 "Smart Pointers", Abschnitt 15.3
    "Running Code on Cleanup with the Drop Trait",
    https://doc.rust-lang.org/book/ch15-03-drop.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`Deref` is the trait behind the star. Whoever writes it for a type of their own
says with it which value lies behind, and from then on `*wert` means exactly
that one.

On top of that comes something easily taken for magic. When the compiler does not
find a method on the type itself, it keeps looking through `Deref`. That is why
`karton.len()` works on a `Karton<String>` although `Karton` has no `len`.

`Drop` is the counterpart at the end. It runs when a value falls away, and the
compiler inserts the call itself. It cannot be called.

### What it is good for

A container of your own should be usable like what it contains. Without `Deref`
every caller would have to unpack the content first, and the container would be
in the way instead of helpful. `Rc` from the last unit was already built this
way, and here stands how that is done.

`Drop` takes the ground away from a whole class of mistakes. Closing a file,
releasing a lock, resetting a counter: those are things somebody forgets when
they have to be written down. Standing in `Drop`, they happen on the way out
through an abort as well.

The order is fixed and not accidental. Values fall away in the reverse order of
their creation, so that a value needing another one is still there when the other
one goes.

### The explanation

A container of your own with `Deref` and two guards with `Drop` in one program.

```rust
use std::ops::Deref;

// Deutsch: Ein eigener Zeiger. `Deref` sagt dem Übersetzer, worauf ein `*`
// führt, und damit auch, was ein Methodenaufruf durch ihn hindurch findet.
struct Karton<T>(T);

impl<T> Deref for Karton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Deutsch: `Drop` läuft, wenn der Wert wegfällt. Aufrufen lässt es sich nicht.
struct Wache {
    name: String,
}

impl Drop for Wache {
    fn drop(&mut self) {
        println!("{} geht", self.name);
    }
}

fn main() {
    let text = Karton(String::from("Ada"));

    // Deutsch: `*text` folgt dem Deref, und `text.len()` findet die Methode von
    // String, ohne dass hier ein Stern steht.
    println!("{}", *text);
    println!("{}", text.len());

    let _erste = Wache {
        name: String::from("erste"),
    };
    let _zweite = Wache {
        name: String::from("zweite"),
    };

    println!("Ende von main");
}
```

The program prints:

```text
Ada
3
Ende von main
zweite geht
erste geht
```

The first two lines are `Deref`. Once with a star, once without, and both times
the `String` inside the container comes out.

The last three are `Drop`. "Ende von main" stands before both exits, because the
guards only fall away at the closing brace. And "zweite" goes before "erste",
because falling away happens in the reverse order.

### Common mistakes

Wanting to call `drop` as a method.

```rust
struct Wache {
    name: String,
}

impl Drop for Wache {
    fn drop(&mut self) {
        println!("{} geht", self.name);
    }
}

fn main() {
    let erste = Wache {
        name: String::from("erste"),
    };

    erste.drop();

    println!("danach");
}
```

The compiler answers:

```text
error[E0040]: explicit use of destructor method
  --> raeumen.rs:16:11
   |
16 |     erste.drop();
   |           ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
16 -     erste.drop();
16 +     drop(erste);
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0040`.
```

The message names the way out right away. `drop(erste)` takes the value and then
lets it fall away, which triggers the cleanup exactly once. `erste.drop()` would
make the call and then still let the value fall away afterwards, so cleaning up
twice, and that is why it is forbidden and not merely inelegant.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Etikett` stands there finished, and its doc
test is green.

- `Deref for Karton<T>` hands the held value out
- `length` counts the bytes in the container, without a star
- `Drop for Wachhund` ticks the box on falling away

```console
cd units/07-04-deref-und-drop
cargo test
```

### Source

    Book, chapter 15 "Smart Pointers", section 15.2
    "Treating Smart Pointers Like Regular References",
    https://doc.rust-lang.org/book/ch15-02-deref.html,
    checked against 1.97.1

    Book, chapter 15 "Smart Pointers", section 15.3
    "Running Code on Cleanup with the Drop Trait",
    https://doc.rust-lang.org/book/ch15-03-drop.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
