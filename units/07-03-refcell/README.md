# 07-03 RefCell / RefCell

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/07-03-refcell/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-04 Veränderbares Ausleihen`, `07-01 Box` und
  `07-02 Rc`. Die Ausleihregel kommt von dort, hier wechselt nur der Zeitpunkt,
  zu dem sie geprüft wird.
- Auf dieser Einheit bauen auf: `07-07 Mutex und Arc`, wo dieselbe Verschiebung
  noch einmal auftritt, diesmal über Threads hinweg.
- Beim Antworten so zitieren: `07-03 RefCell`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `RefCell` hebt die Ausleihregel nicht auf. Sie zählt sie zur Laufzeit, und der
  Bruch ist ein Abbruch und keine Meldung. Wer sagt, `RefCell` erlaube zwei
  veränderbare Ausleihen, sagt das Gegenteil dessen, was die Zelle tut.
- Der Text der Meldung stammt aus der gebundenen Fassung und lautet dort
  `RefCell already borrowed`. Frühere Fassungen schrieben etwas anderes. Wer
  eine Aussage über den Wortlaut braucht, misst sie an der Fassung aus
  `rust-toolchain.toml` nach.
- `zwei_veraenderbare_ausleihen` ist mit Absicht falsch und steht mit Absicht
  fertig da. Sie ist keine Aufgabe und wird nicht repariert; ein Test hält den
  Abbruch fest.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/07-03-refcell/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: `02-04 Veränderbares Ausleihen`, `07-01 Box` and
  `07-02 Rc`. The borrowing rule comes from there, what changes here is only the
  moment it is checked at.
- Building on this unit: `07-07 Mutex und Arc`, where the same shift turns up
  once more, this time across threads.
- Cite like this when answering: `07-03 RefCell`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `RefCell` does not lift the borrowing rule. It counts it at run time, and the
  break is an abort and not a message. Whoever says `RefCell` allows two mutable
  borrows says the opposite of what the cell does.
- The text of the message comes from the pinned version and reads
  `RefCell already borrowed` there. Earlier versions wrote something else.
  Whoever needs a statement about the wording measures it against the version in
  `rust-toolchain.toml`.
- `zwei_veraenderbare_ausleihen` is wrong on purpose and stands there finished
  on purpose. It is not an exercise and does not get repaired; a test holds the
  abort down.

</details>

## Deutsch

### Worum es geht

Bis hierher hat der Übersetzer die Ausleihregel durchgesetzt: entweder beliebig
viele `&` oder genau ein `&mut`, und nie beides zugleich. Er entscheidet das,
bevor das Programm läuft, und er entscheidet konservativ. Was er nicht
durchschaut, lehnt er ab.

`RefCell<T>` verschiebt dieselbe Regel in die Laufzeit. Die Zelle hält einen
Zähler. `borrow` gibt eine Ausleihe zum Lesen und erhöht ihn, `borrow_mut` gibt
eine zum Schreiben und will ihn allein haben. Beide geben einen Wert zurück, der
die Ausleihe darstellt, und erst wenn dieser Wert stirbt, ist die Zelle wieder
frei.

Der sichtbare Unterschied steht an den Methoden. Eine `RefCell` lässt sich
hinter einem `&` verändern, also nimmt eine Methode, die schreibt, trotzdem
`&self`. Das ist der Grund, aus dem die Zelle gebraucht wird, und es ist auch
der Grund, aus dem der Übersetzer den Fehler nicht mehr findet.

### Wofür das gut ist

Gebraucht wird das dort, wo mehrere Stellen auf denselben Wert zeigen und eine
davon schreiben soll. `Rc<T>` gibt aus jedem Zeiger nur ein `&`, denn mehrere
Besitzer und ein `&mut` schließen sich aus. `Rc<RefCell<T>>` ist die Antwort
darauf: der `Rc` teilt, die `RefCell` erlaubt das Schreiben.

Der Preis ist der Zeitpunkt. Ein Programm mit einem Ausleihfehler übersetzt
jetzt und bricht später ab, und zwar nur, wenn die Stelle wirklich erreicht wird.
Ein Fehler, der beim Übersetzen eine Meldung mit Zeilennummer war, ist jetzt ein
Absturz beim Benutzer.

Deshalb ist eine Ausleihe so kurz wie möglich zu halten. Wer den Rückgabewert von
`borrow_mut` in einer Variablen ablegt und danach eine andere Methode desselben
Werts aufruft, hat die Zelle noch in der Hand, wenn die andere Methode sie
braucht. Genau daran hängt Aufgabe 3.

### Die Erklärung

Ein Programm, das die Zelle richtig benutzt und am Ende doch abbricht.

```rust
use std::cell::RefCell;

struct Protokoll {
    zeilen: RefCell<Vec<String>>,
}

impl Protokoll {
    fn neu() -> Self {
        Protokoll {
            zeilen: RefCell::new(Vec::new()),
        }
    }

    // Deutsch: `&self` genügt, die Veränderung geht durch die Zelle.
    fn notieren(&self, zeile: &str) {
        self.zeilen.borrow_mut().push(zeile.to_string());
    }

    fn anzahl(&self) -> usize {
        self.zeilen.borrow().len()
    }
}

fn main() {
    let protokoll = Protokoll::neu();

    protokoll.notieren("erste");
    protokoll.notieren("zweite");
    println!("{}", protokoll.anzahl());

    let geliehen = protokoll.zeilen.borrow_mut();
    println!("{}", geliehen.len());

    protokoll.notieren("dritte");
}
```

Übersetzt läuft es an und bricht in der Mitte ab:

```console
$ ./zelle2
2
2

thread 'main' (55252) panicked at zelle2.rs:16:21:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101
```

Zwei Dinge daran sind wichtig. Erstens steht die abgebrochene Zeile nicht dort,
wo der Fehler gemacht wurde: gemeldet wird `notieren`, verursacht hat es die
Zeile mit `let geliehen`, die die Ausleihe bis zum Ende von `main` festhält.
Zweitens hat das Programm vorher zwei Zeilen ausgegeben. Ein Ausleihfehler ist
hier nichts, was vor dem Start auffällt.

### Häufige Fehler

Die Zelle weglassen und trotzdem hinter einem `&` schreiben wollen.

```rust
struct Protokoll {
    zeilen: Vec<String>,
}

impl Protokoll {
    fn notieren(&self, zeile: &str) {
        self.zeilen.push(zeile.to_string());
    }
}

fn main() {
    let protokoll = Protokoll { zeilen: Vec::new() };

    protokoll.notieren("erste");
}
```

Der Übersetzer sagt dazu:

```text
error[E0596]: cannot borrow `self.zeilen` as mutable, as it is behind a `&` reference
 --> zelle.rs:7:9
  |
7 |         self.zeilen.push(zeile.to_string());
  |         ^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
6 |     fn notieren(&mut self, zeile: &str) {
  |                  +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

Der Vorschlag ist oft der richtige Weg, und dann ist `RefCell` nicht nötig. Er
trägt aber nicht mehr, sobald mehrere Besitzer auf denselben Wert zeigen, denn
aus einem `Rc` kommt kein `&mut`. Erst dann ist die Zelle die Antwort, und
danach findet der Übersetzer diesen Fehler nicht mehr.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `neu`, `notieren`, `zeilen_geliehen` und
`zwei_veraenderbare_ausleihen` stehen fertig da, und der Doku-Test von
`notieren` ist grün.

- `anzahl` gibt zurück, wie viele Zeilen im Protokoll stehen
- `letzte` gibt die zuletzt geschriebene Zeile zurück, als Kopie
- `notieren_und_zaehlen` schreibt eine Zeile und zählt danach, ohne die Ausleihe
  dabei festzuhalten

```console
cd units/07-03-refcell
cargo test
```

### Quelle

    Buch, Kapitel 15 "Smart Pointers",
    Abschnitt 15.5 "RefCell<T> and the Interior Mutability Pattern",
    https://doc.rust-lang.org/book/ch15-05-interior-mutability.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Up to here the compiler enforced the borrowing rule: either any number of `&` or
exactly one `&mut`, and never both at once. It decides that before the program
runs, and it decides conservatively. What it cannot see through, it refuses.

`RefCell<T>` moves the same rule into run time. The cell holds a counter.
`borrow` gives a borrow for reading and raises it, `borrow_mut` gives one for
writing and wants it to itself. Both return a value standing for the borrow, and
only once that value dies is the cell free again.

The visible difference stands at the methods. A `RefCell` can be changed behind
a `&`, so a method that writes takes `&self` all the same. That is the reason
the cell is needed, and it is also the reason the compiler no longer finds the
mistake.

### What it is good for

It is needed where several places point at the same value and one of them should
write. `Rc<T>` gives only a `&` out of every pointer, because several owners and
one `&mut` rule each other out. `Rc<RefCell<T>>` is the answer to that: the `Rc`
shares, the `RefCell` allows the writing.

The price is the moment. A program with a borrowing mistake now compiles and
aborts later, and only if the place is really reached. A mistake that used to be
a compiler message with a line number is now a crash at the user.

That is why a borrow is to be kept as short as possible. Whoever puts the
returned value of `borrow_mut` into a variable and afterwards calls another
method of the same value still holds the cell when the other method needs it.
Exercise 3 hangs on exactly that.

### The explanation

A program that uses the cell correctly and still aborts at the end.

```rust
use std::cell::RefCell;

struct Protokoll {
    zeilen: RefCell<Vec<String>>,
}

impl Protokoll {
    fn neu() -> Self {
        Protokoll {
            zeilen: RefCell::new(Vec::new()),
        }
    }

    // Deutsch: `&self` genügt, die Veränderung geht durch die Zelle.
    fn notieren(&self, zeile: &str) {
        self.zeilen.borrow_mut().push(zeile.to_string());
    }

    fn anzahl(&self) -> usize {
        self.zeilen.borrow().len()
    }
}

fn main() {
    let protokoll = Protokoll::neu();

    protokoll.notieren("erste");
    protokoll.notieren("zweite");
    println!("{}", protokoll.anzahl());

    let geliehen = protokoll.zeilen.borrow_mut();
    println!("{}", geliehen.len());

    protokoll.notieren("dritte");
}
```

Compiled, it starts up and aborts in the middle:

```console
$ ./zelle2
2
2

thread 'main' (55252) panicked at zelle2.rs:16:21:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ echo $?
101
```

Two things about it matter. First, the aborted line is not where the mistake was
made: what gets reported is `notieren`, what caused it is the line with
`let geliehen`, which holds the borrow down until the end of `main`. Second, the
program printed two lines before that. A borrowing mistake is nothing that shows
up before the start here.

### Common mistakes

Leaving the cell out and wanting to write behind a `&` all the same.

```rust
struct Protokoll {
    zeilen: Vec<String>,
}

impl Protokoll {
    fn notieren(&self, zeile: &str) {
        self.zeilen.push(zeile.to_string());
    }
}

fn main() {
    let protokoll = Protokoll { zeilen: Vec::new() };

    protokoll.notieren("erste");
}
```

The compiler answers:

```text
error[E0596]: cannot borrow `self.zeilen` as mutable, as it is behind a `&` reference
 --> zelle.rs:7:9
  |
7 |         self.zeilen.push(zeile.to_string());
  |         ^^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
6 |     fn notieren(&mut self, zeile: &str) {
  |                  +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0596`.
```

The suggestion is often the right way, and then `RefCell` is not needed. It no
longer carries as soon as several owners point at the same value, though,
because no `&mut` comes out of an `Rc`. Only then is the cell the answer, and
after that the compiler no longer finds this mistake.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `neu`, `notieren`, `zeilen_geliehen` and
`zwei_veraenderbare_ausleihen` stand there finished, and the doc test of
`notieren` is green.

- `anzahl` returns how many lines stand in the log
- `letzte` returns the line written last, as a copy
- `notieren_und_zaehlen` writes a line and counts afterwards, without holding
  the borrow down while doing so

```console
cd units/07-03-refcell
cargo test
```

### Source

    Book, chapter 15 "Smart Pointers",
    section 15.5 "RefCell<T> and the Interior Mutability Pattern",
    https://doc.rust-lang.org/book/ch15-05-interior-mutability.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
