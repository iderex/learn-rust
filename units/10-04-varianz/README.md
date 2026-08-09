# 10-04 Varianz / Variance

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-04-varianz/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `05-04 Lifetimes` und `10-02 Rohe Zeiger`. Dort stand,
  wie lange eine Referenz gilt, hier steht, wann eine Lebenszeit sich einer
  anderen anpassen darf.
- Auf dieser Einheit bauen auf: `10-05 Drop check` und alles, was einen eigenen
  Behälter über einer Referenz baut.
- Beim Antworten so zitieren: `10-04 Varianz`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Varianz ist eine Eigenschaft des Typs und nicht der Stelle, an der er steht.
  Der Übersetzer rechnet sie aus den Feldern aus, und niemand schreibt sie hin.
- Kovariant heißt, dass eine lange Lebenszeit als kurze durchgeht. Invariant
  heißt, dass gar nichts durchgeht, auch nicht in die andere Richtung.
- `&mut T` ist invariant über `T`, `&T` ist kovariant über `T`, und der
  Parameter einer Funktion ist kontravariant. Wer das Gegenteil behauptet, sagt
  bitte, an welchem Beispiel.
- Die Meldung zur Invarianz trägt keine Fehlernummer. Wer nach `error[E....]`
  sucht, findet sie nicht und hält sie deshalb nicht für erfunden.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-04-varianz/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: `05-04 Lifetimes` and `10-02 Rohe Zeiger`. There it stood
  how long a reference is valid, here it stands when one lifetime may adapt to
  another.
- Building on this unit: `10-05 Drop check` and everything that builds a
  container of its own over a reference.
- Cite like this when answering: `10-04 Varianz`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Variance is a property of the type and not of the place it stands in. The
  compiler works it out from the fields, and nobody writes it down.
- Covariant means a long lifetime passes for a short one. Invariant means
  nothing passes at all, in either direction.
- `&mut T` is invariant over `T`, `&T` is covariant over `T`, and the parameter
  of a function is contravariant. Whoever claims the opposite, please say on
  which example.
- The message about invariance carries no error number. Whoever searches for
  `error[E....]` does not find it and therefore does not take it for invented.

</details>

## Deutsch

### Worum es geht

Eine Referenz, die ewig gilt, darf überall dort stehen, wo eine gebraucht wird,
die nur kurz gilt. Das ist der Satz aus `05-04`, und er stimmt für `&str`. Er
stimmt nicht für jeden Typ, der ein `&str` enthält, und welche Typen ihn erben,
entscheidet die Varianz.

Drei Antworten gibt es. Kovariant heißt, die lange Lebenszeit geht als kurze
durch. Kontravariant heißt, es geht andersherum. Invariant heißt, es geht gar
nicht, und die beiden Lebenszeiten müssen genau dieselbe sein.

Hinschreiben kann man die Varianz nicht. Der Übersetzer liest sie an den Feldern
ab: Was nur gelesen wird, ist kovariant, was als Parameter weitergereicht wird,
ist kontravariant, und was gelesen und geschrieben wird, ist invariant, weil
beide Richtungen zusammen keine Anpassung übrig lassen.

### Wofür das gut ist

Ohne Kovarianz wäre fast jede Signatur mit zwei Referenzen unbrauchbar. Wer
`laengere(a, b)` mit einer Zeile aus dem Programmtext und einer aus einer
eingelesenen Datei aufruft, gibt zwei verschiedene Lebenszeiten hinein, und nur
weil beide sich auf eine gemeinsame verkürzen lassen, passt der Aufruf.

Mit der Invarianz kommt man in Berührung, wenn ein eigener Behälter gebaut wird.
Solange er nur herausgibt, erbt er die Kovarianz seiner Felder. Sobald er auch
entgegennimmt, ist er invariant, und dann lehnt der Übersetzer Aufrufe ab, die
vorher gingen.

Die Meldung zu dieser Ablehnung nennt die Varianz beim Namen. Wer weiß, dass es
sie gibt, liest die Meldung als Antwort. Wer es nicht weiß, liest sie als
Rätsel über Lebenszeiten und fängt an, `'static` an Stellen zu schreiben, an
die es nicht gehört.

### Die Erklärung

Alle drei Richtungen in einem Programm, das läuft.

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Notiz<'a> {
    text: &'a str,
}

// Deutsch: `Notiz` hält ihren Text nur zum Lesen und ist deshalb kovariant
// über `'a`. Eine ewig lebende Notiz geht als kurzlebige durch.
fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

// Deutsch: Der Parameter einer Funktion ist kontravariant. Ein Zeiger auf eine
// Funktion, die eine kurzlebige Referenz annimmt, darf dort stehen, wo eine
// für eine ewig lebende erwartet wird, denn er verlangt weniger.
fn laenge_unter<'kurz>(f: fn(&'kurz str) -> usize) -> usize {
    let fuer_ewig: fn(&'static str) -> usize = f;
    fuer_ewig("ewig")
}

fn main() {
    let notiz = Notiz { text: "ewig" };
    let kurz = kuerzer(notiz);
    println!("{}", kurz.text);

    println!("{}", laenge_unter(str::len));

    // Deutsch: `Cell` gibt ihren Wert heraus und nimmt einen entgegen. Beide
    // Richtungen zusammen machen sie invariant über ihren Inhalt, und deshalb
    // bleibt diese Zelle bis zuletzt eine Zelle für `&'static str`.
    let zelle: Cell<&'static str> = Cell::new("ewig");
    zelle.set("auch ewig");
    println!("{}", zelle.get());
}
```

Das Programm gibt aus:

```text
ewig
4
auch ewig
```

Die erste Zeile entsteht, weil `Notiz` kovariant ist. Die zweite entsteht, weil
der Parameter kontravariant ist. Die dritte entsteht, ohne dass sich eine
Lebenszeit angepasst hätte, und genau das ist die Invarianz.

### Häufige Fehler

Dieselbe Verkürzung hinter einer veränderbaren Referenz versuchen.

```rust
#[derive(Debug)]
struct Notiz<'a> {
    text: &'a str,
}

fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

fn kuerzer_hinter_mut<'a, 'kurz>(notiz: &'a mut Notiz<'static>) -> &'a mut Notiz<'kurz> {
    notiz
}

fn main() {
    let notiz = Notiz { text: "ewig" };
    println!("{:?}", kuerzer(notiz));
}
```

Der Übersetzer sagt dazu:

```text
error: lifetime may not live long enough
  --> varianz.rs:11:5
   |
10 | fn kuerzer_hinter_mut<'a, 'kurz>(notiz: &'a mut Notiz<'static>) -> &'a mut Notiz<'kurz> {
   |                           ----- lifetime `'kurz` defined here
11 |     notiz
   |     ^^^^^ returning this value requires that `'kurz` must outlive `'static`
   |
   = note: requirement occurs because of a mutable reference to `Notiz<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error: aborting due to 1 previous error
```

Die erste Funktion steht daneben und wird angenommen, die zweite nicht, und der
einzige Unterschied ist das `&mut` davor. Die Notiz ist dieselbe, ihre Kovarianz
ist dieselbe, und trotzdem reicht sie nicht durch die veränderbare Referenz
hindurch. Die zweite Notiz der Meldung sagt genau das und ist die Stelle, die
man sich merkt. Eine Fehlernummer trägt diese Meldung nicht, denn sie kommt
nicht aus der Typprüfung, sondern aus dem Ausleihprüfer.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Für jede Aufgabe gibt es Tests.

- `laengere` gibt den längeren von zwei Texten heraus, mit einer Signatur, die
  von der Kovarianz lebt
- `ersetzen` setzt einen neuen Text in eine Notiz und gibt den alten heraus,
  durch eine veränderbare Referenz und damit invariant
- `kuerzeste` findet den kürzesten Text unter den Notizen, und die Referenz gilt
  weiter, wenn die Liste schon fallen gelassen ist

```console
cd units/10-04-varianz
cargo test
```

### Quelle

    The Rustonomicon, Kapitel 3.8 "Subtyping and Variance",
    https://doc.rust-lang.org/nomicon/subtyping.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 10.5 "Subtyping and variance",
    https://doc.rust-lang.org/reference/subtyping.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A reference that is valid forever may stand everywhere one is needed that is
valid only briefly. That is the sentence from `05-04`, and it holds for `&str`.
It does not hold for every type containing a `&str`, and which types inherit it
is decided by variance.

There are three answers. Covariant means the long lifetime passes for the short
one. Contravariant means it goes the other way round. Invariant means it does
not go at all, and the two lifetimes have to be exactly the same one.

Variance cannot be written down. The compiler reads it off the fields: what is
only read is covariant, what is passed on as a parameter is contravariant, and
what is read and written is invariant, because both directions together leave no
room for an adjustment.

### What it is good for

Without covariance nearly every signature with two references would be unusable.
Whoever calls `laengere(a, b)` with one line out of the program text and one out
of a file that was read in puts two different lifetimes in, and only because both
can be shortened onto a shared one does the call fit.

Invariance is met when a container of your own is built. As long as it only gives
out, it inherits the covariance of its fields. As soon as it also takes in, it is
invariant, and then the compiler refuses calls that went through before.

The message about that refusal names the variance. Whoever knows it exists reads
the message as an answer. Whoever does not know reads it as a riddle about
lifetimes and starts writing `'static` in places it does not belong.

### The explanation

All three directions in one program that runs.

```rust
use std::cell::Cell;

#[derive(Debug)]
struct Notiz<'a> {
    text: &'a str,
}

// Deutsch: `Notiz` hält ihren Text nur zum Lesen und ist deshalb kovariant
// über `'a`. Eine ewig lebende Notiz geht als kurzlebige durch.
fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

// Deutsch: Der Parameter einer Funktion ist kontravariant. Ein Zeiger auf eine
// Funktion, die eine kurzlebige Referenz annimmt, darf dort stehen, wo eine
// für eine ewig lebende erwartet wird, denn er verlangt weniger.
fn laenge_unter<'kurz>(f: fn(&'kurz str) -> usize) -> usize {
    let fuer_ewig: fn(&'static str) -> usize = f;
    fuer_ewig("ewig")
}

fn main() {
    let notiz = Notiz { text: "ewig" };
    let kurz = kuerzer(notiz);
    println!("{}", kurz.text);

    println!("{}", laenge_unter(str::len));

    // Deutsch: `Cell` gibt ihren Wert heraus und nimmt einen entgegen. Beide
    // Richtungen zusammen machen sie invariant über ihren Inhalt, und deshalb
    // bleibt diese Zelle bis zuletzt eine Zelle für `&'static str`.
    let zelle: Cell<&'static str> = Cell::new("ewig");
    zelle.set("auch ewig");
    println!("{}", zelle.get());
}
```

The program prints:

```text
ewig
4
auch ewig
```

The first line comes about because `Notiz` is covariant. The second comes about
because the parameter is contravariant. The third comes about without any
lifetime having adapted, and that is exactly the invariance.

### Common mistakes

Trying the same shortening behind a mutable reference.

```rust
#[derive(Debug)]
struct Notiz<'a> {
    text: &'a str,
}

fn kuerzer<'kurz>(notiz: Notiz<'static>) -> Notiz<'kurz> {
    notiz
}

fn kuerzer_hinter_mut<'a, 'kurz>(notiz: &'a mut Notiz<'static>) -> &'a mut Notiz<'kurz> {
    notiz
}

fn main() {
    let notiz = Notiz { text: "ewig" };
    println!("{:?}", kuerzer(notiz));
}
```

The compiler answers:

```text
error: lifetime may not live long enough
  --> varianz.rs:11:5
   |
10 | fn kuerzer_hinter_mut<'a, 'kurz>(notiz: &'a mut Notiz<'static>) -> &'a mut Notiz<'kurz> {
   |                           ----- lifetime `'kurz` defined here
11 |     notiz
   |     ^^^^^ returning this value requires that `'kurz` must outlive `'static`
   |
   = note: requirement occurs because of a mutable reference to `Notiz<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error: aborting due to 1 previous error
```

The first function stands next to it and is accepted, the second is not, and the
only difference is the `&mut` in front. The note is the same one, its covariance
is the same, and it still does not reach through the mutable reference. The
second note of the message says exactly that and is the place worth remembering.
This message carries no error number, because it does not come out of the type
check but out of the borrow checker.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. There are tests for every exercise.

- `laengere` gives out the longer of two texts, with a signature that lives off
  covariance
- `ersetzen` puts a new text into a note and gives out the old one, through a
  mutable reference and therefore invariant
- `kuerzeste` finds the shortest text among the notes, and the reference stays
  valid once the list has already been dropped

```console
cd units/10-04-varianz
cargo test
```

### Source

    The Rustonomicon, chapter 3.8 "Subtyping and Variance",
    https://doc.rust-lang.org/nomicon/subtyping.html,
    checked against 1.97.1

    The Rust Reference, chapter 10.5 "Subtyping and variance",
    https://doc.rust-lang.org/reference/subtyping.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
