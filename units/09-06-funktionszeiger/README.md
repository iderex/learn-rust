# 09-06 Funktionszeiger / Function pointers

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-06-funktionszeiger/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `06-04 Closures` und `01-03 Funktionen`. Der
  Unterschied, um den es geht, ist der zwischen diesen beiden.
- Auf dieser Einheit bauen auf: `09-07`, wo ein Makro Namen einsetzt, und jede
  Stelle, an der eine Tabelle aus Namen und Verhalten gebaut wird.
- Beim Antworten so zitieren: `09-06 Funktionszeiger`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `fn` mit kleinem f ist ein Typ, `Fn` mit großem F ist ein Trait. Die beiden
  werden im Gespräch dauernd verwechselt, und die Einheit lebt von dem
  Unterschied.
- Eine Closure geht als `fn` durch, solange sie nichts einfängt. Sobald sie
  etwas einfängt, geht sie nicht mehr durch, und die Meldung dazu ist
  `error[E0308]` mit einer Notiz, die das Eingefangene beim Namen nennt. Die
  Meldung steht unter "Häufige Fehler" und ist echte Ausgabe von 1.97.1.
- Der Konstruktor eines Tupel-Structs ist selbst eine Funktion und passt
  deshalb dorthin, wo ein Funktionszeiger erwartet wird. Das ist keine
  Sonderregel für `map`.
- Funktionszeiger werden in dieser Einheit nicht miteinander verglichen. Wer
  wissen will, ob zwei Zeiger auf dieselbe Funktion zeigen, stellt eine andere
  Frage als die hier, und die Antwort darauf ist unzuverlässiger, als sie
  aussieht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-06-funktionszeiger/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `06-04 Closures` and `01-03 Funktionen`. The difference
  this is about is the one between those two.
- Building on this unit: `09-07`, where a macro puts names in, and every place
  where a table of names and behaviour gets built.
- Cite like this when answering: `09-06 Funktionszeiger`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `fn` with a small f is a type, `Fn` with a capital F is a trait. The two get
  mixed up constantly in conversation, and the unit lives off that difference.
- A closure passes as an `fn` for as long as it captures nothing. As soon as it
  captures something it no longer passes, and the message for that is
  `error[E0308]` with a note naming the captured thing. The message is under
  "Common mistakes" and is real output of 1.97.1.
- The constructor of a tuple struct is a function itself and therefore fits
  where a function pointer is expected. That is not a special rule for `map`.
- Function pointers are not compared with each other in this unit. Whoever wants
  to know whether two pointers point at the same function is asking a different
  question from this one, and the answer to it is less reliable than it looks.

</details>

## Deutsch

### Worum es geht

Eine Funktion hat einen Namen, und dieser Name lässt sich weitergeben wie eine
Zahl. Der Typ, den er dabei annimmt, heißt `fn`, klein geschrieben, und liest
sich wie die Signatur ohne Namen: `fn(i32) -> i32`.

Damit steht neben den Closures aus `06-04` ein zweiter Weg, Verhalten
weiterzureichen. `Fn`, `FnMut` und `FnOnce` mit großem F sind Traits, unter die
Closures und Funktionen gleichermaßen fallen. `fn` mit kleinem f ist dagegen ein
gewöhnlicher Typ, so wie `i32` einer ist.

### Wofür das gut ist

Ein `fn` ist so groß wie ein Zeiger und trägt nichts mit sich herum. Es lässt
sich in einer Tabelle ablegen, aus einer Funktion zurückgeben und über eine
Sprachgrenze schicken, ohne dass ein `Box` oder eine Lebenszeit dazukommt. Wer
einen Namen auf ein Verhalten abbilden will, kommt mit `fn` weiter als mit
`Box<dyn Fn(i32) -> i32>`, und der Unterschied steht in der Signatur, wo ihn
jeder Leser sieht.

Die Grenze ist genau das, was ein `fn` nicht kann. Es trägt nichts mit sich,
also kann es auch nichts einfangen. Sobald eine Closure eine Variable von
draußen braucht, ist `fn` der falsche Typ, und der Übersetzer sagt das mit
einer Meldung, die die eingefangene Variable beim Namen nennt.

### Die Erklärung

Drei Werte gehen hier an dieselbe Stelle: zwei benannte Funktionen und eine
Closure, die nichts einfängt.

```rust
fn verdoppeln(x: i32) -> i32 {
    x * 2
}

fn negieren(x: i32) -> i32 {
    -x
}

fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

fn main() {
    println!("{}", zweimal(verdoppeln, 3));
    println!("{}", zweimal(negieren, 3));

    let ohne_fang: fn(i32) -> i32 = |x| x + 1;
    println!("{}", zweimal(ohne_fang, 3));
}
```

`cargo run` gibt aus:

```text
12
3
5
```

Die erste Zeile ist 3 zweimal verdoppelt, die zweite ist 3 zweimal negiert und
damit wieder 3, die dritte ist 3 zweimal um eins erhöht. `zweimal` sieht keinen
Unterschied zwischen den dreien, denn alle drei sind zu dem Zeitpunkt schon
`fn(i32) -> i32`.

Bei `ohne_fang` steht die Angabe des Typs nicht zur Zierde da. Eine Closure hat
zuerst einen eigenen, namenlosen Typ, und erst die Angabe `fn(i32) -> i32`
bringt den Übersetzer dazu, sie in einen Funktionszeiger umzuwandeln. Diese
Umwandlung geht nur in diese Richtung: aus einer Funktion wird nie eine Closure
mit Gedächtnis.

Der Konstruktor eines Tupel-Structs zählt dabei als Funktion. `struct Marke(u32);`
legt neben dem Typ auch `Marke` als Funktion von `u32` nach `Marke` an, und die
passt überall dorthin, wo `fn(u32) -> Marke` steht.

### Häufige Fehler

Eine Closure übergeben, die etwas eingefangen hat.

```rust
fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

fn main() {
    let zugabe = 10;
    let addiere = |x: i32| x + zugabe;

    println!("{}", zweimal(addiere, 1));
}
```

`cargo build` sagt dazu:

```text
error[E0308]: mismatched types
 --> src\main.rs:9:28
  |
7 |     let addiere = |x: i32| x + zugabe;
  |                   -------- the found closure
8 |
9 |     println!("{}", zweimal(addiere, 1));
  |                    ------- ^^^^^^^ expected fn pointer, found closure
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected fn pointer `fn(i32) -> i32`
                found closure `{closure@src\main.rs:7:19: 7:27}`
note: closures can only be coerced to `fn` types if they do not capture any variables
 --> src\main.rs:7:32
  |
7 |     let addiere = |x: i32| x + zugabe;
  |                                ^^^^^^ `zugabe` captured here
note: function defined here
 --> src\main.rs:1:4
  |
1 | fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
  |    ^^^^^^^ -----------------
```

Die zweite Notiz ist die, auf die es ankommt. Sie sagt nicht nur, dass eine
Closure nicht passt, sondern unter welcher Bedingung sie gepasst hätte, und sie
zeigt mit `zugabe` auf die Stelle, an der die Bedingung gebrochen ist.

Der Weg heraus ist eine Entscheidung und kein Trick. Entweder das Eingefangene
wird ein weiterer Parameter, dann bleibt `fn` stehen. Oder die Signatur nimmt
statt `fn(i32) -> i32` ein `impl Fn(i32) -> i32`, dann sind Closures mit
Gedächtnis erlaubt und die Funktion trägt dafür einen generischen Typ.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `verdoppeln`, `negieren` und `zweimal` stehen fertig da, und
der Doku-Test von `zweimal` ist grün.

- `anwenden` bekommt einen Funktionszeiger und wendet ihn auf jeden Wert an
- `waehle` gibt zu einem Namen den passenden Funktionszeiger heraus, oder nichts
- `einpacken` steckt jede Zahl in eine `Marke`, ohne eine Closure zu schreiben

```console
cd units/09-06-funktionszeiger
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.4 "Advanced Functions
    and Closures",
    https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A function has a name, and that name can be passed on like a number. The type it
takes on while doing so is called `fn`, spelled small, and reads like the
signature without the name: `fn(i32) -> i32`.

That puts a second way of passing behaviour on next to the closures from
`06-04`. `Fn`, `FnMut` and `FnOnce` with a capital F are traits that closures and
functions fall under alike. `fn` with a small f, in contrast, is an ordinary
type, the way `i32` is one.

### What it is good for

An `fn` is as large as a pointer and carries nothing around with it. It can be
put in a table, returned out of a function and sent across a language boundary
without a `Box` or a lifetime coming along. Whoever wants to map a name onto a
behaviour gets further with `fn` than with `Box<dyn Fn(i32) -> i32>`, and the
difference stands in the signature, where every reader sees it.

The limit is exactly what an `fn` cannot do. It carries nothing with it, so it
cannot capture anything either. As soon as a closure needs a variable from
outside, `fn` is the wrong type, and the compiler says so with a message that
names the captured variable.

### The explanation

Three values go to the same place here: two named functions and a closure that
captures nothing.

```rust
fn verdoppeln(x: i32) -> i32 {
    x * 2
}

fn negieren(x: i32) -> i32 {
    -x
}

fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

fn main() {
    println!("{}", zweimal(verdoppeln, 3));
    println!("{}", zweimal(negieren, 3));

    let ohne_fang: fn(i32) -> i32 = |x| x + 1;
    println!("{}", zweimal(ohne_fang, 3));
}
```

`cargo run` prints:

```text
12
3
5
```

The first line is 3 doubled twice, the second is 3 negated twice and therefore 3
again, the third is 3 raised by one twice. `zweimal` sees no difference between
the three, because by then all three are already `fn(i32) -> i32`.

On `ohne_fang` the type annotation is not there for decoration. A closure first
has a type of its own with no name, and only the annotation `fn(i32) -> i32`
makes the compiler turn it into a function pointer. That conversion only goes in
this direction: a function never becomes a closure with a memory.

The constructor of a tuple struct counts as a function here. `struct Marke(u32);`
creates, next to the type, `Marke` as a function from `u32` to `Marke`, and that
one fits everywhere `fn(u32) -> Marke` stands.

### Common mistakes

Passing a closure that captured something.

```rust
fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
    f(f(wert))
}

fn main() {
    let zugabe = 10;
    let addiere = |x: i32| x + zugabe;

    println!("{}", zweimal(addiere, 1));
}
```

`cargo build` says about it:

```text
error[E0308]: mismatched types
 --> src\main.rs:9:28
  |
7 |     let addiere = |x: i32| x + zugabe;
  |                   -------- the found closure
8 |
9 |     println!("{}", zweimal(addiere, 1));
  |                    ------- ^^^^^^^ expected fn pointer, found closure
  |                    |
  |                    arguments to this function are incorrect
  |
  = note: expected fn pointer `fn(i32) -> i32`
                found closure `{closure@src\main.rs:7:19: 7:27}`
note: closures can only be coerced to `fn` types if they do not capture any variables
 --> src\main.rs:7:32
  |
7 |     let addiere = |x: i32| x + zugabe;
  |                                ^^^^^^ `zugabe` captured here
note: function defined here
 --> src\main.rs:1:4
  |
1 | fn zweimal(f: fn(i32) -> i32, wert: i32) -> i32 {
  |    ^^^^^^^ -----------------
```

The second note is the one that matters. It says not only that a closure does
not fit but under which condition it would have fitted, and it points with
`zugabe` at the place where that condition is broken.

The way out is a decision and not a trick. Either the captured thing becomes one
more parameter, and then `fn` stays. Or the signature takes an
`impl Fn(i32) -> i32` instead of an `fn(i32) -> i32`, and then closures with a
memory are allowed and the function carries a generic type for it.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `verdoppeln`, `negieren` and `zweimal` stand
there finished, and the doc test of `zweimal` is green.

- `anwenden` gets a function pointer and applies it to every value
- `waehle` hands out the function pointer matching a name, or nothing
- `einpacken` puts every number into a `Marke`, without writing a closure

```console
cd units/09-06-funktionszeiger
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.4 "Advanced Functions and
    Closures",
    https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
