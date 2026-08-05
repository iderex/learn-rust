# 02-02 Stack und Heap / Stack and heap

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/02-02-stack-und-heap/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-01 Verschieben / Move`.
- Auf dieser Einheit bauen auf: `02-03 Ausleihen`, `02-04 Veränderbares
  Ausleihen` und `02-05 Slices`.
- Beim Antworten so zitieren: `02-02 Stack und Heap`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ob eine Zuweisung kopiert oder verschiebt, entscheidet der Typ und nicht die
  Größe des Werts. Wer hier "kleine Werte werden kopiert" sagt, sagt bitte dazu,
  dass ein Feld aus tausend Zahlen `Copy` ist und ein `String` es nicht ist.
- `Copy` ist ein Trait. Traits stehen erst in Stufe 5, und der Text nennt hier
  nur die Regel, nach der ein Typ `Copy` ist. Wer mehr erklärt, nimmt `05-02`
  vorweg.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/02-02-stack-und-heap/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `02-01 Verschieben / Move`.
- Building on this unit: `02-03 Ausleihen`, `02-04 Veränderbares Ausleihen` and
  `02-05 Slices`.
- Cite like this when answering: `02-02 Stack und Heap`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Whether an assignment copies or moves is decided by the type and not by the
  size of the value. Whoever says "small values are copied" here, please say
  alongside it that an array of a thousand numbers is `Copy` and a `String` is
  not.
- `Copy` is a trait. Traits do not stand before stage 5, and the text here names
  only the rule by which a type is `Copy`. Whoever explains more takes `05-02`
  in advance.

</details>

## Deutsch

### Worum es geht

Ein Programm hat zwei Orte für Werte. Der Stack ist der Stapel der laufenden
Aufrufe. Was dort liegt, hat eine feste Größe, die schon beim Übersetzen
feststeht, und es verschwindet, wenn der Aufruf zu Ende ist. Der Heap ist der
Bereich für alles, dessen Größe erst beim Laufen feststeht. Wer dort etwas
ablegt, bekommt eine Adresse zurück, und diese Adresse liegt wieder auf dem
Stack.

Ein `String` ist beides zugleich. Auf dem Stack liegen drei Angaben, nämlich die
Adresse, die Länge und die Größe des angeforderten Platzes. Die Zeichen selbst
liegen auf dem Heap.

Daran hängt, was eine Zuweisung tut. Bei einem Typ, der `Copy` ist, wird der
Wert kopiert und die alte Bindung bleibt benutzbar. Bei jedem anderen Typ wird
verschoben, so wie in `02-01`.

### Wofür das gut ist

Der Unterschied erklärt, warum dieselbe Zeile einmal geht und einmal nicht. `let
b = a;` ist bei einer Zahl harmlos und bei einem `String` ein Verschieben. Wer
das am Typ festmacht, muss es nicht raten.

Er erklärt außerdem, warum niemand hinter dem Programm aufräumen muss. Der Wert
auf dem Heap gehört genau einer Bindung, und wenn die zu Ende ist, wird der
Platz freigegeben. Kopierte Werte kosten dabei nichts, weil ihre Größe
feststeht.

Die Regel, welcher Typ `Copy` ist, ist kurz. Alle ganzen Zahlen, die
Fließkommazahlen, `bool`, `char` und geteilte Referenzen wie `&str` sind es. Ein
Tupel ist es, wenn jeder seiner Teile es ist. Alles, was Platz auf dem Heap
besitzt und ihn wieder freigeben muss, ist es nicht, also `String` und `Vec`.

### Die Erklärung

Dieselbe Zeile zweimal, mit verschiedenem Ausgang.

```rust
fn main() {
    // Deutsch: `i32` ist `Copy`. Die Zuweisung legt eine zweite Zahl an, und
    // beide Bindungen sind danach benutzbar.
    let a = 5;
    let b = a;
    println!("{a} {b}");

    // Deutsch: `String` ist nicht `Copy`. `clone` legt hier ausdrücklich eine
    // zweite Zeichenkette auf dem Heap an, damit beide Bindungen etwas
    // besitzen.
    let s1 = String::from("hallo");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}
```

Ohne das `clone` wäre die zweite Zuweisung ein Verschieben. Der Unterschied
liegt nicht daran, dass die Zahl klein und die Zeichenkette lang ist. Ein Feld
aus tausend Zahlen ist `Copy`, und ein `String` mit einem Zeichen ist es nicht.
Entscheidend ist, ob der Typ Platz auf dem Heap besitzt, den jemand wieder
freigeben muss.

### Häufige Fehler

Zuweisen und danach beide benutzen.

```rust
fn main() {
    let s1 = String::from("hallo");
    let s2 = s1;

    println!("{s1} {s2}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0382]: borrow of moved value: `s1`
 --> verschieben.rs:5:16
  |
2 |     let s1 = String::from("hallo");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1} {s2}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

Die Meldung sagt den Grund selbst, und sie sagt ihn am Typ: `String` hat `Copy`
nicht. Genau dieselben fünf Zeilen mit `let a = 5;` statt der Zeichenkette
übersetzen ohne eine einzige Meldung.

Der Vorschlag mit `clone` ist richtig und kostet eine zweite Zeichenkette auf
dem Heap. Wer den Wert nur lesen will, nimmt lieber eine Referenz, und die steht
in `02-03`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot.

- `twice` gibt das Doppelte einer Zahl zurück, und der Aufrufer behält seine
  Zahl
- `with_exclamation` nimmt einen `String`, hängt ein Ausrufezeichen an und gibt
  ihn zurück
- `copies_on_assignment` sagt zu einem Typnamen, ob eine Zuweisung kopiert

```console
cd units/02-02-stack-und-heap
cargo test
```

### Quelle

    Buch, Kapitel 4 "Understanding Ownership", Abschnitt 4.1 "What is Ownership?",
    https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A program has two places for values. The stack is the pile of running calls.
What lies there has a fixed size, known at compile time, and it goes away when
the call ends. The heap is the area for everything whose size is known only
while the program runs. Whoever puts something there gets an address back, and
that address lies on the stack again.

A `String` is both at once. On the stack lie three pieces of information, namely
the address, the length and the size of the space asked for. The characters
themselves lie on the heap.

What an assignment does hangs on that. For a type that is `Copy` the value is
copied and the old binding stays usable. For every other type it is moved, the
way it went in `02-01`.

### What it is good for

The difference explains why the same line works one time and not the other.
`let b = a;` is harmless for a number and a move for a `String`. Whoever ties
that to the type does not have to guess.

It also explains why nobody has to clean up behind the program. The value on the
heap belongs to exactly one binding, and when that one ends the space is
released. Copied values cost nothing while that happens, because their size is
fixed.

The rule for which type is `Copy` is short. All the whole numbers, the floating
point numbers, `bool`, `char` and shared references such as `&str` are. A tuple
is when every one of its parts is. Everything that owns space on the heap and
has to release it again is not, so `String` and `Vec`.

### The explanation

The same line twice, with a different outcome.

```rust
fn main() {
    // Deutsch: `i32` ist `Copy`. Die Zuweisung legt eine zweite Zahl an, und
    // beide Bindungen sind danach benutzbar.
    let a = 5;
    let b = a;
    println!("{a} {b}");

    // Deutsch: `String` ist nicht `Copy`. `clone` legt hier ausdrücklich eine
    // zweite Zeichenkette auf dem Heap an, damit beide Bindungen etwas
    // besitzen.
    let s1 = String::from("hallo");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}
```

Without the `clone` the second assignment would be a move. The difference does
not come from the number being small and the string being long. An array of a
thousand numbers is `Copy`, and a `String` holding one character is not. What
decides is whether the type owns space on the heap that somebody has to release
again.

### Common mistakes

Assigning and then using both.

```rust
fn main() {
    let s1 = String::from("hallo");
    let s2 = s1;

    println!("{s1} {s2}");
}
```

The compiler answers:

```text
error[E0382]: borrow of moved value: `s1`
 --> verschieben.rs:5:16
  |
2 |     let s1 = String::from("hallo");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1} {s2}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

The message says the reason itself, and it says it from the type: `String` does
not have `Copy`. Exactly the same five lines with `let a = 5;` instead of the
string compile without a single message.

The suggestion with `clone` is right and costs a second string on the heap.
Whoever only wants to read the value takes a reference instead, and that stands
in `02-03`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `twice` returns the double of a number, and the caller keeps its number
- `with_exclamation` takes a `String`, appends an exclamation mark and gives it
  back
- `copies_on_assignment` says for a type name whether an assignment copies

```console
cd units/02-02-stack-und-heap
cargo test
```

### Source

    Book, chapter 4 "Understanding Ownership", section 4.1 "What is Ownership?",
    https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
