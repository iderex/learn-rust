# 10-05 Drop check / Drop check

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-05-drop-check/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `07-04 Deref und Drop` und `10-04 Varianz`. Dort stand,
  was `Drop` tut und wann eine Lebenszeit sich anpassen darf, hier steht, was
  ein `Drop` an dieser Anpassung ändert.
- Auf dieser Einheit bauen auf: alles, was einen eigenen Behälter mit `Drop`
  über geliehenen Daten baut.
- Beim Antworten so zitieren: `10-05 Drop check`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der drop check ist keine Prüfung des `drop`-Rumpfs. Er sieht nicht nach, ob
  `drop` die Referenz wirklich anfasst; er nimmt an, dass es das darf, sobald
  ein `Drop` da ist.
- Ein `Drop` verkürzt die erlaubte Lebensdauer, nicht die tatsächliche. Der Wert
  fällt genauso spät wie vorher; abgelehnt wird nur, was der Übersetzer nicht
  mehr beweisen kann.
- Die Meldung heißt `error[E0597]` und nennt in ihrer letzten Zeile den Satz,
  auf den es ankommt: Werte fallen in der umgekehrten Reihenfolge ihrer
  Vereinbarung. Sie steht unter "Häufige Fehler" und ist echte Ausgabe von
  1.97.1.
- Dass es am `Drop` liegt und nicht an der Referenz, ist gemessen: dieselben
  Zeilen ohne die `Drop`-Implementierung werden angenommen, und der Abschnitt
  "Die Erklärung" hat diesen Fall mit im laufenden Programm.
- Ein `Vec` räumt seine Werte von vorne nach hinten auf. Wer die drei Spuren in
  einen `Vec` legt, bekommt deshalb nicht die umgekehrte Reihenfolge, und ein
  Test hält genau das fest.
- `#[may_dangle]` ist die Ausnahme, mit der die Standardbibliothek die Strenge
  zurücknimmt. Sie braucht `unsafe` und steht nicht in dieser Einheit.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-05-drop-check/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: `07-04 Deref und Drop` and `10-04 Varianz`. There it
  stood what `Drop` does and when a lifetime may adapt, here it stands what a
  `Drop` changes about that adapting.
- Building on this unit: everything that builds a container of its own with a
  `Drop` over borrowed data.
- Cite like this when answering: `10-05 Drop check`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The drop check is not a check of the `drop` body. It does not look at whether
  `drop` really touches the reference; it assumes it may, as soon as a `Drop` is
  there.
- A `Drop` shortens the allowed lifetime, not the actual one. The value falls
  exactly as late as before; what is refused is only what the compiler can no
  longer prove.
- The message is called `error[E0597]` and names in its last line the sentence
  it comes down to: values fall in the opposite order of their declaration. It
  is under "Common mistakes" and is real output of 1.97.1.
- That it is down to the `Drop` and not to the reference is measured: the same
  lines without the `Drop` implementation are accepted, and the section "The
  explanation" carries that case in the running program.
- A `Vec` cleans its values up from front to back. Whoever puts the three traces
  into a `Vec` therefore does not get the reverse order, and a test pins down
  exactly that.
- `#[may_dangle]` is the exception the standard library takes the strictness
  back with. It needs `unsafe` and is not in this unit.

</details>

## Deutsch

### Worum es geht

Ein Wert, der eine Referenz hält, muss nicht länger leben als das, worauf er
zeigt. Er muss nur aufhören, hinzusehen, bevor es weg ist. Solange niemand mehr
hinsieht, ist eine Referenz, die ins Leere zeigt, kein Problem, denn sie wird
nie gelesen.

Genau diese Freiheit nimmt ein `Drop`. Ein Typ mit `Drop` bekommt am Ende noch
einmal das Wort, und in diesem letzten Wort darf er seine Felder lesen. Also
kann er die Referenz lesen, und also muss das, worauf sie zeigt, in diesem
Moment noch da sein.

Der drop check ist die Rechnung, die der Übersetzer dazu aufmacht. Sie ist
grob: Sie sieht nicht in den Rumpf von `drop` hinein und fragt nicht, ob die
Referenz dort wirklich vorkommt. Sobald ein `Drop` da ist, wird angenommen, dass
sie vorkommen könnte.

### Wofür das gut ist

Die Strenge fällt nicht beim Lesen des Nomicons auf, sondern beim Umstellen von
zwei Zeilen. Zwei `let` in der falschen Reihenfolge, und derselbe Code, der
gestern übersetzt hat, tut es heute nicht mehr, weil in der Zwischenzeit ein
`Drop` dazugekommen ist.

Wer weiß, was los ist, verschiebt eine Zeile nach oben und ist fertig. Wer es
nicht weiß, sucht den Fehler in der Lebenszeit, schreibt `'static` an eine
Stelle, an die es nicht gehört, und baut dort einen zweiten Fehler ein.

Der Satz, der die Sache löst, steht in der Meldung selbst: Werte fallen in der
umgekehrten Reihenfolge ihrer Vereinbarung. Was länger leben soll, wird also
früher vereinbart. Das ist die ganze Regel, und sie steht in einer Notiz, die
man leicht überliest.

### Die Erklärung

Ein Programm, das läuft, mit beiden Fällen nebeneinander.

```rust
use std::cell::RefCell;

struct Spur<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

impl Drop for Spur<'_> {
    fn drop(&mut self) {
        self.buch.borrow_mut().push(self.name.clone());
    }
}

// Deutsch: Derselbe Aufbau ohne Drop. Damit fällt die Strenge weg.
struct Ohne<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

fn main() {
    let buch = RefCell::new(Vec::new());

    {
        let _eins = Spur {
            name: String::from("eins"),
            buch: &buch,
        };
        let _zwei = Spur {
            name: String::from("zwei"),
            buch: &buch,
        };
        println!("{:?}", buch.borrow());
    }

    println!("{:?}", buch.borrow());

    // Deutsch: `ohne` wird vor `spaeter` vereinbart und fällt deshalb nach ihm.
    // Mit einem Drop wäre genau das die abgelehnte Stelle; ohne eines geht es.
    let ohne;
    let spaeter = RefCell::new(vec![String::from("nichts")]);
    ohne = Ohne {
        name: String::from("ohne"),
        buch: &spaeter,
    };
    println!("{} {:?}", ohne.name, ohne.buch.borrow());
}
```

`cargo run` gibt aus:

```text
[]
["zwei", "eins"]
ohne ["nichts"]
```

Die erste Zeile ist leer, weil noch nichts gefallen ist. Die zweite zeigt die
umgekehrte Reihenfolge: `zwei` wurde später vereinbart und fällt zuerst. Die
dritte ist der Fall ohne `Drop`, der angenommen wird, obwohl `ohne` sein
geliehenes Buch überlebt.

### Häufige Fehler

Das Buch nach der Spur vereinbaren.

```rust
use std::cell::RefCell;

struct Spur<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

impl Drop for Spur<'_> {
    fn drop(&mut self) {
        self.buch.borrow_mut().push(self.name.clone());
    }
}

fn main() {
    let eins;
    let buch = RefCell::new(Vec::new());
    eins = Spur {
        name: String::from("eins"),
        buch: &buch,
    };
    println!("{}", eins.name);
}
```

`cargo build` sagt dazu:

```text
error[E0597]: `buch` does not live long enough
  --> src\main.rs:19:15
   |
16 |     let buch = RefCell::new(Vec::new());
   |         ---- binding `buch` declared here
...
19 |         buch: &buch,
   |               ^^^^^ borrowed value does not live long enough
...
22 | }
   | -
   | |
   | `buch` dropped here while still borrowed
   | borrow might be used here, when `eins` is dropped and runs the `Drop` code for type `Spur`
   |
   = note: values in a scope are dropped in the opposite order they are defined
```

Drei Stellen dieser Meldung lohnen sich. Die vorletzte Zeile nennt den Grund
beim Namen und sagt, dass die Ausleihe benutzt werden könnte, wenn `eins` fällt
und der `Drop`-Code läuft. Sie sagt "könnte", denn nachgesehen hat niemand.
Die letzte Zeile nennt die Regel, aus der die Reihenfolge folgt. Und die Meldung
zeigt auf `&buch` und nicht auf das `impl Drop`, obwohl das `impl Drop` die
Ursache ist.

Der Weg heraus ist eine Zeile nach oben. `let buch` vor `let eins`, und
derselbe Code wird angenommen. Nicht der Weg heraus ist `'static` an der
Referenz: Das Buch wird davon nicht älter, und die Meldung wandert nur weiter.

Dass es wirklich am `Drop` liegt, ist nachgemessen und nicht behauptet: Ohne die
`impl Drop`-Zeilen werden dieselben `let` in derselben Reihenfolge angenommen.
Das ist der dritte Teil des Programms unter "Die Erklärung".

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `Spur`, `neues_buch` und `eintraege` stehen fertig da, und
der Doku-Test von `Spur` ist grün.

Alle drei Aufgaben legen zuerst das Buch an und danach die Spuren. Andersherum
übersetzt keine von ihnen, und was dann kommt, steht oben.

- `reihenfolge` zeigt, dass drei Spuren in der umgekehrten Reihenfolge fallen
- `mit_eigenem_block` zieht die mittlere Spur mit einem Block nach vorn
- `frueh_fallen_lassen` zieht die erste Spur mit `drop` nach vorn

```console
cd units/10-05-drop-check
cargo test
```

### Quelle

    The Rustonomicon, Kapitel 3.9 "Drop Check",
    https://doc.rust-lang.org/nomicon/dropck.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 10.9 "Destructors",
    https://doc.rust-lang.org/reference/destructors.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A value holding a reference does not have to live shorter than the thing it
points at. It only has to stop looking before that thing is gone. As long as
nobody looks any more, a reference pointing into nothing is not a problem,
because it never gets read.

That freedom is exactly what a `Drop` takes away. A type with a `Drop` gets the
last word at the end, and in that last word it may read its fields. So it can
read the reference, and so the thing it points at has to still be there at that
moment.

The drop check is the reasoning the compiler sets up for this. It is coarse: it
does not look inside the body of `drop` and does not ask whether the reference
really appears there. As soon as a `Drop` is present, it is assumed that it
could appear.

### What it is good for

The strictness does not show up while reading the Nomicon but while moving two
lines around. Two `let` lines in the wrong order, and the same code that
compiled yesterday does not today, because a `Drop` has arrived in the meantime.

Whoever knows what is going on moves one line up and is done. Whoever does not
looks for the fault in the lifetime, writes `'static` in a place it does not
belong, and builds a second fault in there.

The sentence that settles the matter stands in the message itself: values fall
in the opposite order of their declaration. So whatever is meant to live longer
gets declared earlier. That is the whole rule, and it stands in a note that is
easy to read past.

### The explanation

A program that runs, with both cases side by side.

```rust
use std::cell::RefCell;

struct Spur<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

impl Drop for Spur<'_> {
    fn drop(&mut self) {
        self.buch.borrow_mut().push(self.name.clone());
    }
}

// English: the same build without a Drop. That drops the strictness.
struct Ohne<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

fn main() {
    let buch = RefCell::new(Vec::new());

    {
        let _eins = Spur {
            name: String::from("eins"),
            buch: &buch,
        };
        let _zwei = Spur {
            name: String::from("zwei"),
            buch: &buch,
        };
        println!("{:?}", buch.borrow());
    }

    println!("{:?}", buch.borrow());

    // English: `ohne` is declared before `spaeter` and therefore falls after
    // it. With a Drop that would be the refused place; without one it goes.
    let ohne;
    let spaeter = RefCell::new(vec![String::from("nichts")]);
    ohne = Ohne {
        name: String::from("ohne"),
        buch: &spaeter,
    };
    println!("{} {:?}", ohne.name, ohne.buch.borrow());
}
```

`cargo run` prints:

```text
[]
["zwei", "eins"]
ohne ["nichts"]
```

The first line is empty because nothing has fallen yet. The second shows the
reverse order: `zwei` was declared later and falls first. The third is the case
without a `Drop`, which is accepted although `ohne` outlives its borrowed book.

### Common mistakes

Declaring the book after the trace.

```rust
use std::cell::RefCell;

struct Spur<'a> {
    name: String,
    buch: &'a RefCell<Vec<String>>,
}

impl Drop for Spur<'_> {
    fn drop(&mut self) {
        self.buch.borrow_mut().push(self.name.clone());
    }
}

fn main() {
    let eins;
    let buch = RefCell::new(Vec::new());
    eins = Spur {
        name: String::from("eins"),
        buch: &buch,
    };
    println!("{}", eins.name);
}
```

`cargo build` answers:

```text
error[E0597]: `buch` does not live long enough
  --> src\main.rs:19:15
   |
16 |     let buch = RefCell::new(Vec::new());
   |         ---- binding `buch` declared here
...
19 |         buch: &buch,
   |               ^^^^^ borrowed value does not live long enough
...
22 | }
   | -
   | |
   | `buch` dropped here while still borrowed
   | borrow might be used here, when `eins` is dropped and runs the `Drop` code for type `Spur`
   |
   = note: values in a scope are dropped in the opposite order they are defined
```

Three places in this message are worth the time. The second to last line names
the reason and says the borrow might be used when `eins` is dropped and the
`Drop` code runs. It says "might", because nobody looked. The last line names
the rule the order follows from. And the message points at `&buch` and not at
the `impl Drop`, although the `impl Drop` is the cause.

The way out is one line further up. `let buch` before `let eins`, and the same
code is accepted. What is not the way out is a `'static` on the reference: the
book does not get any older from it, and the message only moves along.

That it really is down to the `Drop` is measured rather than claimed: without
the `impl Drop` lines the same `let` lines in the same order are accepted. That
is the third part of the program under "The explanation".

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `Spur`, `neues_buch` and `eintraege` stand
there finished, and the doc test of `Spur` is green.

All three exercises create the book first and the traces after it. The other way
round none of them compiles, and what comes then stands above.

- `reihenfolge` shows that three traces fall in the reverse order
- `mit_eigenem_block` pulls the middle trace forward with a block
- `frueh_fallen_lassen` pulls the first trace forward with `drop`

```console
cd units/10-05-drop-check
cargo test
```

### Source

    The Rustonomicon, chapter 3.9 "Drop Check",
    https://doc.rust-lang.org/nomicon/dropck.html,
    checked against 1.97.1

    The Rust Reference, chapter 10.9 "Destructors",
    https://doc.rust-lang.org/reference/destructors.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
