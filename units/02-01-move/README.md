# 02-01 Verschieben / Move

## Deutsch

### Worum es geht

Jeder Wert in Rust hat genau einen Eigentümer. Wird ein Wert an eine andere
Bindung oder an eine Funktion übergeben, wandert das Eigentum mit, und die alte
Bindung ist danach nicht mehr benutzbar. Das nennt sich Verschieben, auf
Englisch Move. Es gibt keinen Sammler, der später aufräumt: der Wert wird
aufgeräumt, wenn sein Eigentümer aus dem Gültigkeitsbereich fällt, und weil es
immer nur einen Eigentümer gibt, geschieht das genau einmal.

Typen wie `i32` oder `bool` verhalten sich anders. Sie liegen vollständig auf
dem Stapel, ihre Kopie kostet nichts, und deshalb tragen sie `Copy`. Beim
Zuweisen entsteht eine Kopie und nichts wird verschoben. `String` trägt `Copy`
nicht, weil er Daten auf dem Haufen besitzt, und deshalb verschiebt er.

### Wofür das gut ist

Die Regel klingt streng, und sie ist der Grund dafür, dass ein Rust-Programm
ohne Laufzeitprüfung und ohne Sammler auskommt und trotzdem keinen Speicher
doppelt freigibt. Zwei Bindungen auf denselben Haufenspeicher gäbe es nur mit
zwei Freigaben, und genau diesen Fall verhindert der Übersetzer, bevor das
Programm läuft.

Wer die Regel einmal verstanden hat, liest fast jede spätere Fehlermeldung von
Rust leichter. Ausleihen, Lebensdauern und `Rc` bauen alle auf dieser einen
Frage auf: wem gehört der Wert gerade.

### Die Erklärung

Diese Funktion nimmt den `String` an sich und gibt ihn verändert zurück. Der
Aufrufer verliert sein Eigentum und bekommt mit dem Rückgabewert ein neues.

```rust
pub fn exclaimed(mut s: String) -> String {
    s.push('!');
    s
}

let greeting = String::from("hallo");
let loud = exclaimed(greeting);
// `greeting` ist verschoben und ab hier nicht mehr benutzbar.
assert_eq!(loud, "hallo!");
```

Das lauffähige Beispiel steht als Doc-Kommentar in `src/lib.rs` und wird
mitgetestet. Wer den Wert behalten will, hat zwei Wege: ausleihen mit `&`, dann
bleibt das Eigentum beim Aufrufer, oder `clone`, dann entsteht eine zweite,
eigenständige Kopie auf dem Haufen.

### Häufige Fehler

Der häufigste Fehler ist, einen Wert an eine Funktion zu übergeben und ihn
danach noch einmal zu benutzen.

```rust
fn takes(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hallo");
    takes(s);
    println!("{s}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0382]: borrow of moved value: `s`
 --> arg.rs:8:16
  |
6 |     let s = String::from("hallo");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
7 |     takes(s);
  |           - value moved here
8 |     println!("{s}");
  |                ^ value borrowed here after move
  |
note: consider changing this parameter type in function `takes` to borrow instead if owning the value isn't necessary
 --> arg.rs:1:13
  |
1 | fn takes(s: String) -> usize {
  |    -----    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     takes(s.clone());
  |            ++++++++
```

Dieselbe Meldung mit derselben Nummer erscheint bei `let s2 = s1;` gefolgt von
einer Benutzung von `s1`. Der Hinweis auf `clone` ist bequem, aber er kostet
eine zweite Ablage auf dem Haufen. Wenn die Funktion den Wert gar nicht
besitzen muss, ist der Parameter als `&String` die bessere Antwort, und genau
das ist Aufgabe 1.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot.

- `length_borrowed` gibt die Länge zurück, ohne das Eigentum zu übernehmen
- `duplicated` gibt eine eigenständige Kopie zurück, das Original bleibt heil
- `joined` schiebt zwei `String` hinein und gibt einen zurück

```console
cd units/02-01-move
cargo test
```

### Quelle

The Rust Programming Language, Kapitel 4 "Understanding Ownership", Abschnitt
4.1 "What is Ownership?",
https://doc.rust-lang.org/1.97.1/book/ch04-01-what-is-ownership.html, gepinnte
Version 1.97.1.

## English

### What it is about

Every value in Rust has exactly one owner. Passing a value to another binding
or to a function moves ownership along with it, and the old binding cannot be
used afterwards. That is called a move. There is no collector cleaning up
later: a value is cleaned up when its owner goes out of scope, and because
there is only ever one owner, that happens exactly once.

Types like `i32` or `bool` behave differently. They sit entirely on the stack,
copying them costs nothing, and so they carry `Copy`. Assigning one produces a
copy and moves nothing. `String` does not carry `Copy`, because it owns data on
the heap, and so it moves.

### What it is good for

The rule sounds strict, and it is the reason a Rust program needs neither a
runtime check nor a collector and still never frees the same memory twice. Two
bindings pointing at one piece of heap memory would mean two frees, and that is
exactly the case the compiler rules out before the program runs.

Whoever has understood this rule once reads almost every later Rust error
message more easily. Borrowing, lifetimes and `Rc` all build on this one
question: who owns the value right now.

### The explanation

This function takes the `String` and gives it back changed. The caller loses
ownership and receives a new one with the return value.

```rust
pub fn exclaimed(mut s: String) -> String {
    s.push('!');
    s
}

let greeting = String::from("hallo");
let loud = exclaimed(greeting);
// `greeting` has moved and cannot be used from here on.
assert_eq!(loud, "hallo!");
```

The runnable example lives as a doc comment in `src/lib.rs` and is tested along
with everything else. Whoever wants to keep the value has two routes: borrow
with `&`, which leaves ownership with the caller, or `clone`, which creates a
second, standalone copy on the heap.

### Common mistakes

The most common mistake is passing a value to a function and then using it
again afterwards.

```rust
fn takes(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hallo");
    takes(s);
    println!("{s}");
}
```

The compiler answers:

```text
error[E0382]: borrow of moved value: `s`
 --> arg.rs:8:16
  |
6 |     let s = String::from("hallo");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
7 |     takes(s);
  |           - value moved here
8 |     println!("{s}");
  |                ^ value borrowed here after move
  |
note: consider changing this parameter type in function `takes` to borrow instead if owning the value isn't necessary
 --> arg.rs:1:13
  |
1 | fn takes(s: String) -> usize {
  |    -----    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     takes(s.clone());
  |            ++++++++
```

The same message with the same number appears for `let s2 = s1;` followed by a
use of `s1`. The hint about `clone` is convenient, but it costs a second
allocation on the heap. Where the function does not need to own the value at
all, a `&String` parameter is the better answer, and that is exactly exercise 1.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `length_borrowed` returns the length without taking ownership
- `duplicated` returns a standalone copy and leaves the original intact
- `joined` moves two `String` values in and returns one

```console
cd units/02-01-move
cargo test
```

### Source

The Rust Programming Language, chapter 4 "Understanding Ownership", section 4.1
"What is Ownership?",
https://doc.rust-lang.org/1.97.1/book/ch04-01-what-is-ownership.html, pinned
version 1.97.1.
