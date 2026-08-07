# 10-01 Was unsafe erlaubt und was es nicht abschaltet / What unsafe allows and what it does not switch off

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-01-was-unsafe-erlaubt/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst sagen, welche der fünf Freiheiten gerade gebraucht wird.
- Diese Einheit baut auf: `02-03 Ausleihen`, `02-04 Veränderbares Ausleihen` und
  `02-05 Slices`. Was `unsafe` nicht abschaltet, ist genau der Prüfer aus diesen
  Einheiten.
- Auf dieser Einheit bauen auf: die übrige Stufe 10, also rohe Zeiger,
  undefiniertes Verhalten, FFI und Miri.
- Beim Antworten so zitieren: `10-01 Was unsafe erlaubt`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `unsafe` schaltet den Ausleihprüfer nicht ab. Wer einen Ausleihfehler mit
  einem `unsafe`-Block beantwortet, hat dem Fragenden nicht geholfen; der Block
  ist dann sogar unnötig, und der Übersetzer sagt das auch.
- Ein `unsafe`-Block ist eine Zusage und kein Schalter. Wer einen vorschlägt,
  sagt bitte dazu, welche Bedingung der Leser damit verspricht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-01-was-unsafe-erlaubt/`. It is
  public. Whoever is asked for it may name it, but should first say which of the
  five freedoms is needed right now.
- This unit builds on: `02-03 Ausleihen`, `02-04 Veränderbares Ausleihen` and
  `02-05 Slices`. What `unsafe` does not switch off is exactly the checker from
  those units.
- Building on this unit: the rest of stage 10, meaning raw pointers, undefined
  behaviour, FFI and Miri.
- Cite like this when answering: `10-01 Was unsafe erlaubt`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `unsafe` does not switch the borrow checker off. Whoever answers a borrow
  error with an `unsafe` block has not helped the asker; the block is even
  unnecessary then, and the compiler says so too.
- An `unsafe` block is a promise and not a switch. Whoever suggests one, please
  add which condition the reader is promising with it.

</details>

## Deutsch

### Worum es geht

`unsafe` erlaubt fünf Dinge, die sonst nicht gehen: einen rohen Zeiger lesen
oder schreiben, eine `unsafe`-Funktion oder -Methode rufen, ein veränderbares
`static` anfassen, einen `unsafe`-Trait erfüllen und ein Feld einer `union`
lesen.

Diese fünf sind die ganze Liste. Was sonst gilt, gilt weiter, und zwar Wort für
Wort: der Ausleihprüfer prüft, die Typen stimmen, ein verschobener Wert ist
verschoben, und `Drop` läuft, wann es laufen würde.

Das Wort ist also schlecht gewählt. `unsafe` heißt nicht "ohne Prüfung", es
heißt "hier verspreche ich etwas, das die Prüfung nicht sehen kann".

### Wofür das gut ist

Der Prüfer sagt nein, wo er nichts beweisen kann, nicht nur da, wo etwas falsch
ist. Zwei veränderbare Ausleihen in eine Liste sind falsch, wenn sie sich
überschneiden, und richtig, wenn nicht. Er sieht den Unterschied nicht und sagt
deshalb immer nein.

`split_at_mut` aus der Standardbibliothek ist genau dieser Fall. Es gibt zwei
veränderbare Hälften desselben Slices heraus, sein Rumpf ist `unsafe`, und seine
Schnittstelle ist es nicht. Das ist die Bauform, um die es in dieser Stufe geht:
eine sichere Hülle, deren Bedingung im Rumpf geprüft und im Text genannt wird.

Der Preis ist, dass die Prüfung dort aufhört und der Mensch anfängt. Deshalb ist
der Block so klein wie möglich zu halten, und deshalb steht über jedem eine
Zeile, welche Bedingung gerade versprochen wird.

### Die Erklärung

Ein roher Zeiger, den zu bauen erlaubt ist und den zu lesen es nicht ist, und
eine sichere Hülle um zwei veränderbare Hälften.

```rust
use std::slice;

// Deutsch: Der Zeiger wird ohne unsafe gebaut. Gelesen wird er mit.
fn adresse(wert: &i32) -> *const i32 {
    wert
}

// Deutsch: Sicher nach außen, roh nach innen. Die Bedingung steht im Rumpf.
fn teilen(werte: &mut [i32], mitte: usize) -> (&mut [i32], &mut [i32]) {
    let laenge = werte.len();
    let zeiger = werte.as_mut_ptr();

    assert!(mitte <= laenge);

    unsafe {
        (
            slice::from_raw_parts_mut(zeiger, mitte),
            slice::from_raw_parts_mut(zeiger.add(mitte), laenge - mitte),
        )
    }
}

fn main() {
    let wert = 7;
    let zeiger = adresse(&wert);

    println!("gelesen: {}", unsafe { *zeiger });

    let mut zahlen = [1, 2, 3, 4, 5, 6];
    let (links, rechts) = teilen(&mut zahlen, 2);

    links[0] = 10;
    rechts[0] = 30;

    println!("links: {links:?}");
    println!("rechts: {rechts:?}");
    println!("zusammen: {zahlen:?}");
}
```

Übersetzt und gestartet gibt das aus:

```text
$ unsafe_beispiel.exe
gelesen: 7
links: [10, 2]
rechts: [30, 4, 5, 6]
zusammen: [10, 2, 30, 4, 5, 6]
```

`adresse` hat kein `unsafe` im Rumpf, denn einen Zeiger hinzulegen tut niemandem
weh. In `teilen` steht der Block um genau die zwei Zeilen, die ihn brauchen, und
das `assert!` davor ist die Bedingung, die er voraussetzt. Ohne das `assert!`
wäre `mitte` größer als die Länge erlaubt, und die zweite Hälfte zeigte über das
Ende hinaus.

### Häufige Fehler

Einen rohen Zeiger ohne Block lesen.

```rust
fn main() {
    let wert = 7;
    let zeiger = &wert as *const i32;

    println!("{}", *zeiger);
}
```

Der Übersetzer weist das zurück:

```text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> ohne_unsafe.rs:5:20
  |
5 |     println!("{}", *zeiger);
  |                    ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: aborting due to 1 previous error
```

Das Bauen war erlaubt, das Lesen nicht. Die beiden Zeilen stehen direkt
untereinander, und nur die zweite wird angefasst.

Der zweite Fehler ist der teurere. Einen Ausleihfehler mit `unsafe` beantworten
wollen.

```rust
fn main() {
    let mut wert = 7;
    let erste = &mut wert;

    unsafe {
        let zweite = &mut wert;
        *zweite += 1;
    }

    *erste += 1;
    println!("{wert}");
}
```

Der Block hilft nicht, und der Übersetzer sagt dazu noch, dass er überflüssig
ist:

```text
warning: unnecessary `unsafe` block
 --> unsafe_hilft_nicht.rs:5:5
  |
5 |     unsafe {
  |     ^^^^^^ unnecessary `unsafe` block

error[E0499]: cannot borrow `wert` as mutable more than once at a time
  --> unsafe_hilft_nicht.rs:6:22
   |
 3 |     let erste = &mut wert;
   |                 --------- first mutable borrow occurs here
...
 6 |         let zweite = &mut wert;
   |                      ^^^^^^^^^ second mutable borrow occurs here
...
10 |     *erste += 1;
   |     ----------- first borrow later used here
```

Zwei Meldungen zu einer Sache: der Ausleihprüfer läuft weiter, und der Block hat
im selben Atemzug nichts freigeschaltet, was hier gebraucht würde. Wer eine
zweite Ausleihe will, geht über einen rohen Zeiger, so wie `teilen` es tut, und
sagt dazu, warum sich die beiden nicht überschneiden.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jeder Rumpf braucht einen `unsafe`-Block, und jeder soll so
klein sein, wie er sein kann.

- `lesen` liest durch einen rohen Zeiger und ist selbst `unsafe`
- `teilen` gibt zwei veränderbare Hälften eines Slices heraus
- `erstes_und_letztes` gibt beide Enden veränderbar heraus

```console
cd units/10-01-was-unsafe-erlaubt
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`unsafe` allows five things that otherwise do not work: reading or writing a raw
pointer, calling an `unsafe` function or method, touching a mutable `static`,
meeting an `unsafe` trait and reading a field of a `union`.

Those five are the whole list. What holds otherwise keeps holding, word for
word: the borrow checker checks, the types match, a moved value is moved, and
`Drop` runs when it would run.

The word is therefore badly chosen. `unsafe` does not mean "without checking",
it means "here I promise something the checking cannot see".

### What it is good for

The checker says no where it cannot prove anything, not only where something is
wrong. Two mutable borrows into one list are wrong where they overlap and right
where they do not. It does not see the difference and therefore always says no.

`split_at_mut` from the standard library is exactly that case. It hands out two
mutable halves of the same slice, its body is `unsafe`, and its interface is
not. That is the shape this stage is about: a safe wrapper whose condition is
checked in the body and named in the text.

The price is that the checking stops there and the person begins. That is why
the block is to be kept as small as it can be, and why over every one of them
stands a line saying which condition is being promised.

### The explanation

A raw pointer that is allowed to be built and not allowed to be read, and a safe
wrapper around two mutable halves.

```rust
use std::slice;

// Deutsch: Der Zeiger wird ohne unsafe gebaut. Gelesen wird er mit.
fn adresse(wert: &i32) -> *const i32 {
    wert
}

// Deutsch: Sicher nach außen, roh nach innen. Die Bedingung steht im Rumpf.
fn teilen(werte: &mut [i32], mitte: usize) -> (&mut [i32], &mut [i32]) {
    let laenge = werte.len();
    let zeiger = werte.as_mut_ptr();

    assert!(mitte <= laenge);

    unsafe {
        (
            slice::from_raw_parts_mut(zeiger, mitte),
            slice::from_raw_parts_mut(zeiger.add(mitte), laenge - mitte),
        )
    }
}

fn main() {
    let wert = 7;
    let zeiger = adresse(&wert);

    println!("gelesen: {}", unsafe { *zeiger });

    let mut zahlen = [1, 2, 3, 4, 5, 6];
    let (links, rechts) = teilen(&mut zahlen, 2);

    links[0] = 10;
    rechts[0] = 30;

    println!("links: {links:?}");
    println!("rechts: {rechts:?}");
    println!("zusammen: {zahlen:?}");
}
```

Compiled and started that prints:

```text
$ unsafe_beispiel.exe
gelesen: 7
links: [10, 2]
rechts: [30, 4, 5, 6]
zusammen: [10, 2, 30, 4, 5, 6]
```

`adresse` has no `unsafe` in its body, because putting a pointer down hurts
nobody. In `teilen` the block stands around exactly the two lines that need it,
and the `assert!` before it is the condition it takes for granted. Without the
`assert!` a `mitte` larger than the length would be allowed, and the second half
would point past the end.

### Common mistakes

Reading a raw pointer without a block.

```rust
fn main() {
    let wert = 7;
    let zeiger = &wert as *const i32;

    println!("{}", *zeiger);
}
```

The compiler refuses that:

```text
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> ohne_unsafe.rs:5:20
  |
5 |     println!("{}", *zeiger);
  |                    ^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

error: aborting due to 1 previous error
```

Building it was allowed, reading it was not. The two lines stand right under
each other, and only the second one is touched.

The second mistake is the more expensive one. Trying to answer a borrow error
with `unsafe`.

```rust
fn main() {
    let mut wert = 7;
    let erste = &mut wert;

    unsafe {
        let zweite = &mut wert;
        *zweite += 1;
    }

    *erste += 1;
    println!("{wert}");
}
```

The block does not help, and the compiler adds that it is superfluous:

```text
warning: unnecessary `unsafe` block
 --> unsafe_hilft_nicht.rs:5:5
  |
5 |     unsafe {
  |     ^^^^^^ unnecessary `unsafe` block

error[E0499]: cannot borrow `wert` as mutable more than once at a time
  --> unsafe_hilft_nicht.rs:6:22
   |
 3 |     let erste = &mut wert;
   |                 --------- first mutable borrow occurs here
...
 6 |         let zweite = &mut wert;
   |                      ^^^^^^^^^ second mutable borrow occurs here
...
10 |     *erste += 1;
   |     ----------- first borrow later used here
```

Two messages about one thing: the borrow checker keeps running, and in the same
breath the block unlocked nothing that would be needed here. Whoever wants a
second borrow goes through a raw pointer, the way `teilen` does, and says why
the two do not overlap.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every body needs an `unsafe` block, and every
one of them should be as small as it can be.

- `lesen` reads through a raw pointer and is `unsafe` itself
- `teilen` hands out two mutable halves of a slice
- `erstes_und_letztes` hands out both ends mutably

```console
cd units/10-01-was-unsafe-erlaubt
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
