# 07-01 Box / Box

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-01-box/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-03 enum` und `02-02 Stack und Heap`. Der Typ kommt
  aus der Stufe 3, die beiden Speicherorte aus der Stufe 2.
- Auf dieser Einheit bauen auf: `07-02 Rc` und `07-03 RefCell`, und alles, was
  einen Wert hinter einem Zeiger hält.
- Beim Antworten so zitieren: `07-01 Box`, dazu die Überschrift des Abschnitts,
  zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Box` ist kein Werkzeug gegen den Borrow-Checker. Sie beantwortet eine Frage
  nach der Größe und nach dem Ort, und wer sie gegen einen Ausleihfehler
  vorschlägt, sagt bitte, welche der beiden Fragen dort offen war.
- Die Meldung `E0072` nennt nicht die Rekursion als Fehler, sondern die fehlende
  Indirektion. Ein rekursiver Typ ist erlaubt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-01-box/`. It is public. Whoever
  is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `03-03 enum` and `02-02 Stack und Heap`. The type comes
  from stage 3, the two places in memory from stage 2.
- Building on this unit: `07-02 Rc` and `07-03 RefCell`, and everything holding
  a value behind a pointer.
- Cite like this when answering: `07-01 Box`, plus the heading of the section,
  for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Box` is not a tool against the borrow checker. It answers a question about
  size and about place, and whoever suggests it against a borrowing error,
  please say which of those two questions was open there.
- The message `E0072` does not name recursion as the fault, it names the missing
  indirection. A recursive type is allowed.

</details>

## Deutsch

### Worum es geht

Ein Wert liegt normalerweise dort, wo er steht: in einer Variablen auf dem
Stack, in einem struct, in einem enum. Der Übersetzer muss dafür wissen, wie
viele Bytes er belegt, und zwar beim Übersetzen.

`Box::new(wert)` legt den Wert stattdessen auf den Heap. Was auf dem Stack
bleibt, ist ein Zeiger, und ein Zeiger ist immer gleich groß, ganz gleich wie
groß das ist, worauf er zeigt.

Genau daran hängt der Fall, um den es hier geht. Ein Typ, der sich selbst
enthält, hat keine ausrechenbare Größe. Ein Typ, der einen Zeiger auf sich
selbst enthält, hat eine.

### Wofür das gut ist

Eine Liste, deren Glied den Rest der Liste trägt, ist der kürzeste Fall. Ohne
`Box` müsste ein Glied so groß sein wie ein Glied plus eine Zahl, und das hört
nie auf. Der Übersetzer sagt das mit `E0072` und schlägt die Indirektion vor.

Mit `Box` ist ein Glied so groß wie eine Zahl plus ein Zeiger. Der Rest liegt
woanders, und die Rechnung geht auf.

Der zweite Grund ist der Umzug. Einen großen Wert weiterzugeben heißt sonst, ihn
zu kopieren. Liegt er hinter einer `Box`, wandert nur der Zeiger, und der Wert
selbst bleibt liegen, wo er ist.

`Box` gibt den Speicher von selbst wieder frei, wenn sie am Ende ihres
Gültigkeitsbereichs angekommen ist. Es gibt nichts zurückzugeben und nichts zu
vergessen.

### Die Erklärung

Eine Zahl auf dem Heap, und eine Liste, deren Rest hinter einer `Box` steht.

```rust
// Deutsch: Ein Wert auf dem Heap. Die Box liegt auf dem Stack und ist so gross
// wie ein Zeiger; die 5 liegt am anderen Ende.
fn auf_dem_heap() {
    let zahl = Box::new(5_i64);
    println!("{zahl}");
    println!("{}", *zahl + 1);
}

// Deutsch: Ein Glied traegt eine Zahl und den Rest der Liste. Der Rest steht
// hinter einer Box, sonst haette der Typ keine bekannte Groesse.
enum Liste {
    Glied(i64, Box<Liste>),
    Ende,
}

fn summe(liste: &Liste) -> i64 {
    match liste {
        Liste::Glied(zahl, rest) => zahl + summe(rest),
        Liste::Ende => 0,
    }
}

fn main() {
    auf_dem_heap();

    let liste = Liste::Glied(1, Box::new(Liste::Glied(2, Box::new(Liste::Ende))));
    println!("{}", summe(&liste));
    println!("{}", size_of::<Box<Liste>>());
}
```

Das Programm gibt aus:

```text
5
6
3
8
```

Die erste Zeile zeigt, dass eine `Box` sich beim Ausgeben wie ihr Inhalt
verhält. Die zweite holt den Inhalt mit `*` heraus, um mit ihm zu rechnen. Die
dritte ist die Summe der Liste. Die vierte ist die Größe einer `Box<Liste>` in
Bytes auf der Maschine, auf der dieser Lauf stattfand, also die eines Zeigers
und nicht die der ganzen Liste.

### Häufige Fehler

Denselben Typ ohne `Box` schreiben.

```rust
enum Liste {
    Glied(i64, Liste),
    Ende,
}

fn main() {
    let liste = Liste::Glied(1, Liste::Ende);

    match liste {
        Liste::Glied(zahl, _) => println!("{zahl}"),
        Liste::Ende => println!("leer"),
    }
}
```

Der Übersetzer sagt dazu:

```text
error[E0072]: recursive type `Liste` has infinite size
 --> ohne_box.rs:1:1
  |
1 | enum Liste {
  | ^^^^^^^^^^
2 |     Glied(i64, Liste),
  |                ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Glied(i64, Box<Liste>),
  |                ++++     +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0072`.
```

`recursive without indirection` zeigt auf die Stelle und sagt, was fehlt. Nicht
die Rekursion ist verboten, sondern der unmittelbare Einbau ohne Zeiger
dazwischen. Der Vorschlag darunter ist die Antwort.

Die Antwort ist nicht, die Liste zu einem `Vec<i64>` umzubauen, weil der Fehler
dann weg ist. Das wäre ein anderer Typ mit anderen Eigenschaften, und die Frage,
warum der erste nicht ging, bliebe unbeantwortet.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Typ `Liste` steht schon da, mit der `Box` an der Stelle,
an der sie hingehört.

- `from_slice` baut aus einem Slice eine Liste, ein Glied je Zahl
- `sum` addiert alle Zahlen einer Liste
- `contains` sagt, ob eine Zahl in der Liste vorkommt

```console
cd units/07-01-box
cargo test
```

### Quelle

    Buch, Kapitel 15 "Smart Pointers", Abschnitt 15.1 "Using Box<T> to Point to Data on the Heap",
    https://doc.rust-lang.org/book/ch15-01-box.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A value normally lies where it stands: in a variable on the stack, in a struct,
in an enum. For that the compiler has to know how many bytes it takes up, and it
has to know at compile time.

`Box::new(wert)` puts the value on the heap instead. What stays on the stack is
a pointer, and a pointer is always the same size, no matter how large the thing
it points at is.

Exactly that is what the case here hangs on. A type containing itself has no
size anybody can work out. A type containing a pointer to itself has one.

### What it is good for

A list whose link carries the rest of the list is the shortest case. Without
`Box` a link would have to be as large as a link plus a number, and that never
stops. The compiler says so with `E0072` and suggests the indirection.

With `Box` a link is as large as a number plus a pointer. The rest lies
somewhere else, and the sum adds up.

The second reason is moving. Passing a large value on otherwise means copying
it. Where it lies behind a `Box`, only the pointer travels, and the value itself
stays where it is.

`Box` gives the memory back by itself once it reaches the end of its scope.
There is nothing to hand back and nothing to forget.

### The explanation

A number on the heap, and a list whose rest stands behind a `Box`.

```rust
// Deutsch: Ein Wert auf dem Heap. Die Box liegt auf dem Stack und ist so gross
// wie ein Zeiger; die 5 liegt am anderen Ende.
fn auf_dem_heap() {
    let zahl = Box::new(5_i64);
    println!("{zahl}");
    println!("{}", *zahl + 1);
}

// Deutsch: Ein Glied traegt eine Zahl und den Rest der Liste. Der Rest steht
// hinter einer Box, sonst haette der Typ keine bekannte Groesse.
enum Liste {
    Glied(i64, Box<Liste>),
    Ende,
}

fn summe(liste: &Liste) -> i64 {
    match liste {
        Liste::Glied(zahl, rest) => zahl + summe(rest),
        Liste::Ende => 0,
    }
}

fn main() {
    auf_dem_heap();

    let liste = Liste::Glied(1, Box::new(Liste::Glied(2, Box::new(Liste::Ende))));
    println!("{}", summe(&liste));
    println!("{}", size_of::<Box<Liste>>());
}
```

The program prints:

```text
5
6
3
8
```

The first line shows that a `Box` behaves like its contents when printed. The
second takes the contents out with `*` to compute with them. The third is the
sum of the list. The fourth is the size of a `Box<Liste>` in bytes on the
machine this run happened on, meaning that of a pointer and not that of the
whole list.

### Common mistakes

Writing the same type without `Box`.

```rust
enum Liste {
    Glied(i64, Liste),
    Ende,
}

fn main() {
    let liste = Liste::Glied(1, Liste::Ende);

    match liste {
        Liste::Glied(zahl, _) => println!("{zahl}"),
        Liste::Ende => println!("leer"),
    }
}
```

The compiler answers:

```text
error[E0072]: recursive type `Liste` has infinite size
 --> ohne_box.rs:1:1
  |
1 | enum Liste {
  | ^^^^^^^^^^
2 |     Glied(i64, Liste),
  |                ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Glied(i64, Box<Liste>),
  |                ++++     +

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0072`.
```

`recursive without indirection` points at the place and says what is missing. It
is not recursion that is forbidden, it is putting the type straight in with no
pointer in between. The suggestion below it is the answer.

The answer is not to rebuild the list as a `Vec<i64>` because the error goes
away then. That would be a different type with different properties, and the
question why the first one did not work would stay unanswered.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The type `Liste` already stands there, with
the `Box` in the place it belongs.

- `from_slice` builds a list out of a slice, one link per number
- `sum` adds up every number of a list
- `contains` says whether a number turns up in the list

```console
cd units/07-01-box
cargo test
```

### Source

    Book, chapter 15 "Smart Pointers", section 15.1 "Using Box<T> to Point to Data on the Heap",
    https://doc.rust-lang.org/book/ch15-01-box.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
