# 02-04 Veränderbares Ausleihen / Mutable borrowing

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/02-04-veraenderbares-ausleihen/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `02-03 Ausleihen`.
- Auf dieser Einheit bauen auf: `02-05 Slices`, und später `07-03 RefCell`, wo
  dieselbe Regel erst beim Laufen geprüft wird.
- Beim Antworten so zitieren: `02-04 Veränderbares Ausleihen`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Text zeigt zwei Auflösungen des Stoßes und behauptet nicht, es gebe nur
  diese zwei. Wer eine dritte nennt, sagt bitte dazu, was sie kostet.
- `E0502` und `E0499` sind verschiedene Meldungen. Hier steht `E0502`, also der
  Stoß zwischen einer geteilten und einer veränderbaren Ausleihe.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/02-04-veraenderbares-ausleihen/`.
  It is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `02-03 Ausleihen`.
- Building on this unit: `02-05 Slices`, and later `07-03 RefCell`, where the
  same rule is checked while the program runs instead.
- Cite like this when answering: `02-04 Veränderbares Ausleihen`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The text shows two ways out of the collision and does not claim there are only
  those two. Whoever names a third, please say what it costs.
- `E0502` and `E0499` are different messages. What stands here is `E0502`, the
  collision between a shared and a mutable loan.

</details>

## Deutsch

### Worum es geht

`&mut wert` ist die veränderbare Ausleihe. Sie darf lesen und schreiben, und der
Wert gehört weiter dem Aufrufer. Damit sie überhaupt entstehen kann, muss die
Bindung selbst `mut` sein.

Dazu gehört eine Regel, und sie ist die halbe Sprache: zu einem Wert darf zur
selben Zeit entweder eine veränderbare Ausleihe bestehen oder beliebig viele
geteilte, aber nie beides. Wer dagegen verstößt, bekommt `E0502`.

Wie in `02-03` gilt eine Ausleihe bis zu ihrer letzten Benutzung. Das ist der
Grund, warum zwei Ausleihen sich in einem Programm oft doch nicht in die Quere
kommen.

### Wofür das gut ist

Die Regel verhindert eine Klasse von Fehlern, die in anderen Sprachen erst beim
Laufen auffällt oder gar nicht. Wer einen Wert liest, während jemand anderes ihn
verändert, liest etwas, das es so nie gab. Hier wird das beim Übersetzen
abgelehnt.

Sie ist außerdem der Grund, warum in Rust kein Datenwettlauf zwischen zwei
Strängen entstehen kann, ohne dass jemand ausdrücklich danach fragt. Das gehört
zur Stufe 7 und wird hier nur genannt.

Der Preis ist, dass Programme, die überall gleichzeitig hineinschreiben wollen,
umgebaut werden müssen. Der Umbau ist meistens klein: die Ausleihe früher
beenden oder den gelesenen Wert kopieren statt festzuhalten.

### Die Erklärung

Verändern durch eine Ausleihe.

```rust
fn ausrufen(text: &mut String) {
    // Deutsch: Eine veränderbare Ausleihe darf lesen und schreiben. Der Wert
    // gehört weiter dem Aufrufer.
    text.push('!');
}

fn main() {
    let mut text = String::from("hallo");

    ausrufen(&mut text);

    // Deutsch: Der Aufrufer hat seinen Wert noch, und er ist verändert.
    println!("{text}");

    let mut zahl = 21;
    let zeiger = &mut zahl;

    // Deutsch: `*` schreibt durch die Referenz hindurch.
    *zeiger *= 2;

    println!("{zahl}");
}
```

Am Aufruf steht `&mut text` und nicht nur `text`, und das ist Absicht: an der
aufrufenden Stelle soll zu sehen sein, dass hier etwas verändert wird.

### Häufige Fehler

Eine geteilte Ausleihe halten und daneben verändern.

```rust
fn main() {
    let mut text = String::from("hallo");

    let geliehen = &text;

    text.push('!');

    println!("{geliehen}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable
 --> stoss.rs:6:5
  |
4 |     let geliehen = &text;
  |                    ----- immutable borrow occurs here
5 |
6 |     text.push('!');
  |     ^^^^^^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{geliehen}");
  |                -------- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

Die Meldung zeigt alle drei Stellen: wo geliehen wird, wo verändert wird, und wo
die Ausleihe später noch gebraucht wird. Die letzte ist die wichtige, denn ohne
sie wäre die Ausleihe längst zu Ende und es gäbe keinen Stoß.

Der erste Weg heraus ist, die Ausleihe vor der Änderung zu Ende zu bringen.

```rust
fn main() {
    let mut text = String::from("hallo");

    let geliehen = &text;
    println!("{geliehen}");

    // Deutsch: Die Ausleihe ist mit ihrer letzten Benutzung zu Ende, und
    // danach darf verändert werden.
    text.push('!');

    println!("{text}");
}
```

Der zweite ist, gar nichts festzuhalten, sondern den gelesenen Wert zu kopieren.

```rust
fn main() {
    let mut text = String::from("hallo");

    // Deutsch: Statt der Ausleihe wird nur die Länge gemerkt. Eine Zahl ist
    // `Copy` und hält nichts fest.
    let laenge = text.len();

    text.push('!');

    println!("{laenge} {text}");
}
```

Welcher der beiden passt, hängt davon ab, ob der ganze Wert später noch gebraucht
wird oder nur eine Angabe daraus.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Keine Aufgabe gibt einen Wert zurück, den der Aufrufer schon
hat.

- `double_in_place` verdoppelt eine Zahl durch eine veränderbare Ausleihe
- `append_twice` hängt einen Zusatz zweimal an einen geliehenen Text
- `add_into` addiert eine geliehene Zahl auf eine veränderbar geliehene

```console
cd units/02-04-veraenderbares-ausleihen
cargo test
```

### Quelle

    Buch, Kapitel 4 "Understanding Ownership", Abschnitt 4.2 "References and Borrowing",
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`&mut wert` is the mutable loan. It may read and write, and the value still
belongs to the caller. For it to come into being at all the binding itself has
to be `mut`.

A rule comes with it, and it is half the language: for one value there may at
the same time be either one mutable loan or any number of shared ones, but never
both. Whoever breaks that gets `E0502`.

As in `02-03` a loan holds until its last use. That is the reason why two loans
in one program often do not get in each other's way after all.

### What it is good for

The rule prevents a class of fault that in other languages surfaces only while
the program runs, or not at all. Whoever reads a value while somebody else
changes it reads something that never was. Here that is refused at compile time.

It is also the reason why no data race between two threads can arise in Rust
without somebody asking for it in so many words. That belongs to stage 7 and is
only named here.

The price is that programs which want to write everywhere at once have to be
rebuilt. The rebuild is mostly small: end the loan earlier, or copy the value
read instead of holding on to it.

### The explanation

Changing something through a loan.

```rust
fn ausrufen(text: &mut String) {
    // Deutsch: Eine veränderbare Ausleihe darf lesen und schreiben. Der Wert
    // gehört weiter dem Aufrufer.
    text.push('!');
}

fn main() {
    let mut text = String::from("hallo");

    ausrufen(&mut text);

    // Deutsch: Der Aufrufer hat seinen Wert noch, und er ist verändert.
    println!("{text}");

    let mut zahl = 21;
    let zeiger = &mut zahl;

    // Deutsch: `*` schreibt durch die Referenz hindurch.
    *zeiger *= 2;

    println!("{zahl}");
}
```

At the call site `&mut text` stands and not only `text`, and that is on purpose:
at the calling place it should be visible that something gets changed here.

### Common mistakes

Holding a shared loan and changing the value beside it.

```rust
fn main() {
    let mut text = String::from("hallo");

    let geliehen = &text;

    text.push('!');

    println!("{geliehen}");
}
```

The compiler answers:

```text
error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable
 --> stoss.rs:6:5
  |
4 |     let geliehen = &text;
  |                    ----- immutable borrow occurs here
5 |
6 |     text.push('!');
  |     ^^^^^^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{geliehen}");
  |                -------- immutable borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0502`.
```

The message shows all three places: where the loan is taken, where the change
happens, and where the loan is needed later. The last one is the important one,
because without it the loan would long be over and there would be no collision.

The first way out is to finish the loan before the change.

```rust
fn main() {
    let mut text = String::from("hallo");

    let geliehen = &text;
    println!("{geliehen}");

    // Deutsch: Die Ausleihe ist mit ihrer letzten Benutzung zu Ende, und
    // danach darf verändert werden.
    text.push('!');

    println!("{text}");
}
```

The second is to hold nothing at all and copy the value that was read instead.

```rust
fn main() {
    let mut text = String::from("hallo");

    // Deutsch: Statt der Ausleihe wird nur die Länge gemerkt. Eine Zahl ist
    // `Copy` und hält nichts fest.
    let laenge = text.len();

    text.push('!');

    println!("{laenge} {text}");
}
```

Which of the two fits depends on whether the whole value is needed later or only
one piece of information out of it.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. No exercise returns a value the caller already
has.

- `double_in_place` doubles a number through a mutable loan
- `append_twice` appends an addition twice to a borrowed text
- `add_into` adds a borrowed number onto a mutably borrowed one

```console
cd units/02-04-veraenderbares-ausleihen
cargo test
```

### Source

    Book, chapter 4 "Understanding Ownership", section 4.2 "References and Borrowing",
    https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
