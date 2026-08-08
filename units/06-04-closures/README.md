# 06-04 Closures / Closures

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/06-04-closures/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-01 Verschieben / Move` und `05-03 Trait Bounds`.
  Das Einfangen ist Ownership, die Schranke ist eine Trait-Schranke.
- Auf dieser Einheit bauen auf: der Rest der Stufe 6, vor allem alles mit
  Iteratoren, und jede Funktion, die ein Stück Verhalten entgegennimmt.
- Beim Antworten so zitieren: `06-04 Closures`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `Fn`, `FnMut` und `FnOnce` sind keine Auswahl, sondern gestaffelt. Jede
  `Fn`-Closure erfüllt auch `FnMut` und `FnOnce`. Wer das Gegenteil behauptet,
  sagt bitte, an welcher Closure.
- Welches Trait eine Closure erfüllt, entscheidet ihr Rumpf und nicht das Wort
  `move`. `move` sagt, wie eingefangen wird, und nicht, wie oft aufgerufen
  werden darf.
- Der Typ einer Closure hat keinen Namen. Deshalb steht bei der Rückgabe
  `impl Fn(i32) -> i32` und nicht ein Typ, den man hinschreiben könnte.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/06-04-closures/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `02-01 Verschieben / Move` and `05-03 Trait Bounds`.
  Capturing is ownership, the bound is a trait bound.
- Building on this unit: the rest of stage 6, above all everything with
  iterators, and every function that takes a piece of behaviour.
- Cite like this when answering: `06-04 Closures`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `Fn`, `FnMut` and `FnOnce` are not a choice but a staircase. Every `Fn`
  closure also fulfils `FnMut` and `FnOnce`. Whoever claims the opposite, please
  say on which closure.
- Which trait a closure fulfils is decided by its body and not by the word
  `move`. `move` says how the capture happens, not how often it may be called.
- The type of a closure has no name. That is why the return says
  `impl Fn(i32) -> i32` and not a type that could be written down.

</details>

## Deutsch

### Worum es geht

Eine Closure ist eine Funktion, die man mitten im Code hinschreibt und die
etwas aus ihrer Umgebung mitnimmt. `|zahl| zahl + summand` benutzt `summand`,
obwohl `summand` nicht in der Klammer steht.

Wie sie es mitnimmt, sucht sich der Übersetzer aus, und zwar so sparsam wie
möglich. Nur lesen heißt eine gemeinsame Referenz. Verändern heißt eine
veränderbare Referenz. Herausgeben heißt den Wert selbst.

Daran hängen drei Traits. `Fn` heißt beliebig oft aufrufbar. `FnMut` heißt
aufrufbar, verändert sich dabei aber. `FnOnce` heißt einmal, denn beim Aufruf
wird die Closure aufgebraucht. Sie sind gestaffelt: Was `Fn` erfüllt, erfüllt
auch die anderen beiden.

### Wofür das gut ist

Eine Funktion, die ein Stück Verhalten entgegennimmt, muss nicht wissen, was
dieses Verhalten mitbringt. `for_each_even` schickt Zahlen an etwas, das der
Aufrufer geschrieben hat, und ob dieses Etwas in eine Liste schreibt, mitzählt
oder nichts tut, geht die Funktion nichts an.

Die Schranke sagt, was die Funktion mit der Closure vorhat, und nicht, was sie
gern hätte. `Fn` verlangt am meisten von der Closure und erlaubt dem Rumpf am
meisten, `FnOnce` ist umgekehrt. Wer `Fn` fordert, wo `FnOnce` reichen würde,
schließt Closures aus, die einen Wert übernehmen, ohne etwas dafür zu bekommen.

Und weil der Typ einer Closure keinen Namen hat, ist ein generischer Parameter
mit Schranke die einzige Art, sie entgegenzunehmen, ohne sie in eine `Box` zu
stecken. Zwei Closures mit demselben Rumpf sind trotzdem zwei Typen.

### Die Erklärung

Drei Closures, drei Arten einzufangen, und die drei Traits daran.

```rust
// Deutsch: Drei Closures, drei Arten einzufangen, und daran hängen die drei
// Traits `Fn`, `FnMut` und `FnOnce`.
fn main() {
    // Deutsch: Nur lesen. Die Closure fängt eine Referenz ein, `namen` bleibt
    // danach benutzbar. Das ist `Fn`.
    let namen = vec![String::from("Ada"), String::from("Grace")];
    let zaehle = || namen.len();
    println!("{}", zaehle());
    println!("{}", zaehle());
    println!("{}", namen[0]);

    // Deutsch: Verändern. Die Closure fängt eine veränderbare Referenz ein und
    // muss deshalb selbst `mut` sein. Das ist `FnMut`.
    let mut liste = Vec::new();
    let mut merke = |wert: i32| liste.push(wert);
    merke(1);
    merke(2);
    println!("{liste:?}");

    // Deutsch: Übernehmen. `move` schiebt den Wert in die Closure hinein. Weil
    // sie ihn danach herausgibt, lässt sie sich nur einmal aufrufen, und das
    // ist `FnOnce`.
    let gruss = String::from("Hallo");
    let verbrauche = move || gruss;
    println!("{}", verbrauche());
}
```

Das Programm gibt aus:

```text
2
2
Ada
[1, 2]
Hallo
```

Die dritte Zeile ist die eigentliche Aussage des ersten Teils. `namen` steht
nach zwei Aufrufen von `zaehle` immer noch da, denn die Closure hat sich nur
eine Referenz genommen.

Beim zweiten Teil steht `mut` zweimal, an `liste` und an `merke`. Das zweite
sieht überflüssig aus, ist es aber nicht: Die Closure hält die veränderbare
Referenz in sich, also ändert sich beim Aufruf die Closure selbst.

### Häufige Fehler

Eine Closure zweimal aufrufen, die beim ersten Aufruf aufgebraucht wird.

```rust
fn main() {
    let name = String::from("Ada");

    let verbrauche = move || name;

    println!("{}", verbrauche());
    println!("{}", verbrauche());
}
```

Der Übersetzer sagt dazu:

```text
error[E0382]: use of moved value: `verbrauche`
 --> einmal.rs:7:20
  |
6 |     println!("{}", verbrauche());
  |                    ------------ `verbrauche` moved due to this call
7 |     println!("{}", verbrauche());
  |                    ^^^^^^^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `name` out of its environment
 --> einmal.rs:4:30
  |
4 |     let verbrauche = move || name;
  |                              ^^^^
note: this value implements `FnOnce`, which causes it to be moved when called
 --> einmal.rs:6:20
  |
6 |     println!("{}", verbrauche());
  |                    ^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

Die zweite Anmerkung nennt den Grund genau: Der Rumpf gibt `name` heraus, also
ist die Closure `FnOnce`, und der Aufruf verbraucht sie. Nicht `move` ist
schuld. Mit `move || name.len()` bliebe dieselbe Closure `Fn`, denn dann bleibt
`name` in ihr liegen, statt herauszugehen.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `apply` steht fertig da, und sein Doku-Test ist grün.

- `apply_twice` wendet dieselbe Closure zweimal an, mit `Fn`
- `for_each_even` meldet die geraden Zahlen an eine Closure, mit `FnMut`
- `make_adder` gibt eine Closure zurück, mit `move` und `impl Fn`

```console
cd units/06-04-closures
cargo test
```

### Quelle

    Buch, Kapitel 13 "Functional Language Features: Iterators and Closures",
    Abschnitt 13.1 "Closures",
    https://doc.rust-lang.org/book/ch13-01-closures.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A closure is a function written down in the middle of the code that takes
something along from its surroundings. `|zahl| zahl + summand` uses `summand`
although `summand` does not stand inside the bars.

How it takes it along is chosen by the compiler, and as frugally as possible.
Only reading means a shared reference. Changing means a mutable reference.
Giving out means the value itself.

Three traits hang on that. `Fn` means callable any number of times. `FnMut`
means callable, but changing itself while doing so. `FnOnce` means once, because
the call uses the closure up. They are a staircase: what fulfils `Fn` fulfils the
other two as well.

### What it is good for

A function that takes a piece of behaviour need not know what that behaviour
brings along. `for_each_even` sends numbers to something the caller wrote, and
whether that something writes into a list, counts along or does nothing is none
of the function's business.

The bound says what the function intends to do with the closure, not what it
would like. `Fn` demands the most of the closure and allows the body the most,
`FnOnce` is the other way round. Whoever asks for `Fn` where `FnOnce` would do
shuts out closures that take a value over without getting anything for it.

And because the type of a closure has no name, a generic parameter with a bound
is the only way to take one without putting it into a `Box`. Two closures with
the same body are still two types.

### The explanation

Three closures, three ways of capturing, and the three traits on them.

```rust
// Deutsch: Drei Closures, drei Arten einzufangen, und daran hängen die drei
// Traits `Fn`, `FnMut` und `FnOnce`.
fn main() {
    // Deutsch: Nur lesen. Die Closure fängt eine Referenz ein, `namen` bleibt
    // danach benutzbar. Das ist `Fn`.
    let namen = vec![String::from("Ada"), String::from("Grace")];
    let zaehle = || namen.len();
    println!("{}", zaehle());
    println!("{}", zaehle());
    println!("{}", namen[0]);

    // Deutsch: Verändern. Die Closure fängt eine veränderbare Referenz ein und
    // muss deshalb selbst `mut` sein. Das ist `FnMut`.
    let mut liste = Vec::new();
    let mut merke = |wert: i32| liste.push(wert);
    merke(1);
    merke(2);
    println!("{liste:?}");

    // Deutsch: Übernehmen. `move` schiebt den Wert in die Closure hinein. Weil
    // sie ihn danach herausgibt, lässt sie sich nur einmal aufrufen, und das
    // ist `FnOnce`.
    let gruss = String::from("Hallo");
    let verbrauche = move || gruss;
    println!("{}", verbrauche());
}
```

The program prints:

```text
2
2
Ada
[1, 2]
Hallo
```

The third line is the actual statement of the first part. `namen` is still there
after two calls of `zaehle`, because the closure took nothing but a reference.

In the second part `mut` stands twice, at `liste` and at `merke`. The second one
looks superfluous but is not: the closure holds the mutable reference inside
itself, so the call changes the closure itself.

### Common mistakes

Calling a closure twice that is used up by the first call.

```rust
fn main() {
    let name = String::from("Ada");

    let verbrauche = move || name;

    println!("{}", verbrauche());
    println!("{}", verbrauche());
}
```

The compiler answers:

```text
error[E0382]: use of moved value: `verbrauche`
 --> einmal.rs:7:20
  |
6 |     println!("{}", verbrauche());
  |                    ------------ `verbrauche` moved due to this call
7 |     println!("{}", verbrauche());
  |                    ^^^^^^^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `name` out of its environment
 --> einmal.rs:4:30
  |
4 |     let verbrauche = move || name;
  |                              ^^^^
note: this value implements `FnOnce`, which causes it to be moved when called
 --> einmal.rs:6:20
  |
6 |     println!("{}", verbrauche());
  |                    ^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

The second note names the reason exactly: the body gives `name` out, so the
closure is `FnOnce`, and the call uses it up. `move` is not at fault. With
`move || name.len()` the same closure would stay `Fn`, because then `name` stays
lying inside it instead of leaving.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `apply` stands there finished, and its doc test
is green.

- `apply_twice` applies the same closure twice, with `Fn`
- `for_each_even` reports the even numbers to a closure, with `FnMut`
- `make_adder` returns a closure, with `move` and `impl Fn`

```console
cd units/06-04-closures
cargo test
```

### Source

    Book, chapter 13 "Functional Language Features: Iterators and Closures",
    section 13.1 "Closures",
    https://doc.rust-lang.org/book/ch13-01-closures.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
