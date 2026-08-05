# 09-01 Muster im Detail / Patterns in detail

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-01-muster-im-detail/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-04 match` und `03-05 Option und if let`. Dort
  standen die ersten Muster, hier steht der ganze Vorrat.
- Auf dieser Einheit bauen auf: der Rest der Stufe 9 und alles, was einen
  verschachtelten Wert auseinandernimmt.
- Beim Antworten so zitieren: `09-01 Muster im Detail`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Widerlegbar und unwiderlegbar sind Eigenschaften des Musters und nicht der
  Stelle. Die Stelle entscheidet nur, welche von beiden dort erlaubt ist.
- Ein Wächter gehört zum Zweig und nicht zum Muster. Der Übersetzer rechnet ihn
  deshalb nicht in die Vollständigkeitsprüfung ein. Wer das Gegenteil behauptet,
  sagt bitte, an welchem Beispiel.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-01-muster-im-detail/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `03-04 match` and `03-05 Option und if let`. The first
  patterns stood there, the whole supply stands here.
- Building on this unit: the rest of stage 9 and everything that takes a nested
  value apart.
- Cite like this when answering: `09-01 Muster im Detail`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Refutable and irrefutable are properties of the pattern and not of the place.
  The place only decides which of the two is allowed there.
- A guard belongs to the arm and not to the pattern. That is why the compiler
  does not count it towards the exhaustiveness check. Whoever claims the
  opposite, please say on which example.

</details>

## Deutsch

### Worum es geht

Ein Muster beschreibt eine Form und benennt die Teile darin. Bekannt ist es aus
`match`, aber es steht an mehr Stellen: in `let`, in `if let`, in `while let`,
im Kopf einer `for`-Schleife und in den Parametern einer Funktion.

Ein Muster ist widerlegbar, wenn es scheitern kann, und unwiderlegbar, wenn
nicht. `(breite, hoehe)` passt auf jedes Paar und ist unwiderlegbar.
`Some(wert)` passt nicht auf `None` und ist widerlegbar. Welche Sorte erlaubt
ist, hängt an der Stelle: `let` will eine unwiderlegbare, `if let` und
`while let` wollen eine widerlegbare, weil sonst nichts zu entscheiden wäre.

Dazu kommen zwei Werkzeuge. Ein Wächter ist ein `if` hinter dem Muster und engt
den Zweig weiter ein. Eine Bindung mit `@` hält den Wert fest, den das Muster
gerade geprüft hat, statt ihn nur zu prüfen.

### Wofür das gut ist

Ein verschachtelter Wert lässt sich in einem Zug auseinandernehmen, statt Ebene
für Ebene. Das spart nicht nur Zeilen, es hält auch die Bedingung und den Namen
für den Wert an einer Stelle zusammen.

Ein Wächter nimmt eine Bedingung auf, die kein Muster ausdrücken kann. `x == y`
vergleicht zwei Teile desselben Wertes miteinander, und dafür gibt es keine
Form, die man hinschreiben könnte.

Und `@` löst den Fall, in dem beides gebraucht wird: prüfen, dass ein Zeichen
eine Ziffer ist, und dieses Zeichen danach benutzen. Ohne die Bindung ist es
entweder geprüft oder greifbar, nicht beides.

### Die Erklärung

Muster an drei Stellen, ein Wächter, eine Bindung mit `@` und ein
verschachtelter Wert.

```rust
#[derive(Debug)]
struct Punkt {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Ereignis {
    Klick { punkt: Punkt, taste: char },
    Taste(char),
    Nichts,
}

// Deutsch: Ein match nimmt den verschachtelten Wert in einem Zug auseinander.
// Der Wächter hinter `if` gehört zum Zweig und nicht zum Muster, und `@` hält
// fest, was das Muster gerade geprüft hat.
fn beschreibung(ereignis: &Ereignis) -> String {
    match ereignis {
        Ereignis::Klick {
            punkt: Punkt { x, y },
            taste,
        } if x == y => format!("{taste} auf der Diagonalen bei {x}"),
        Ereignis::Klick {
            punkt: Punkt { x: 0, y },
            ..
        } => format!("am linken Rand, {y} tief"),
        Ereignis::Klick {
            punkt: Punkt { x, y },
            ..
        } => format!("bei {x} und {y}"),
        Ereignis::Taste(zeichen @ '0'..='9') => format!("Ziffer {zeichen}"),
        Ereignis::Taste(zeichen) => format!("Taste {zeichen}"),
        Ereignis::Nichts => String::from("nichts"),
    }
}

fn main() {
    // Deutsch: `let` nimmt ein Paar auseinander. Das Muster kann nicht
    // scheitern, es ist unwiderlegbar.
    let (breite, hoehe) = (3, 4);
    println!("{}", breite * hoehe);

    // Deutsch: `while let` läuft, solange das Muster passt. Hier darf es
    // scheitern, denn genau das beendet die Schleife.
    let mut stapel = vec![1, 2, 3];
    while let Some(oben) = stapel.pop() {
        print!("{oben} ");
    }
    println!();

    let klick = Ereignis::Klick {
        punkt: Punkt { x: 2, y: 2 },
        taste: 'L',
    };
    println!("{}", beschreibung(&klick));
    println!(
        "{}",
        beschreibung(&Ereignis::Klick {
            punkt: Punkt { x: 0, y: 5 },
            taste: 'R',
        })
    );
    println!("{}", beschreibung(&Ereignis::Taste('7')));
    println!("{}", beschreibung(&Ereignis::Nichts));
}
```

Das Programm gibt aus:

```text
12
3 2 1 
L auf der Diagonalen bei 2
am linken Rand, 5 tief
Ziffer 7
nichts
```

Die Reihenfolge der Zweige entscheidet, denn der erste passende gewinnt. Der
Klick auf 0 und 0 landet im ersten Zweig und nicht im zweiten, obwohl beide
passen würden.

### Häufige Fehler

Ein widerlegbares Muster in ein `let` schreiben.

```rust
fn main() {
    let zahlen = vec![1, 2, 3];

    let Some(erste) = zahlen.first();

    println!("{erste}");
}
```

Der Übersetzer sagt dazu:

```text
error[E0005]: refutable pattern in local binding
 --> muster.rs:4:9
  |
4 |     let Some(erste) = zahlen.first();
  |         ^^^^^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-02-refutability.html
  = note: the matched value is of type `Option<&i32>`
help: you might want to use `let...else` to handle the variant that isn't matched
  |
4 |     let Some(erste) = zahlen.first() else { todo!() };
  |                                      ++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0005`.
```

Die Meldung nennt den Fall, der nicht abgedeckt ist, und schlägt `let ... else`
vor, das aus `03-05` bekannt ist. `if let` täte es auch. Beides ist eine Stelle,
an der ein Muster scheitern darf, und genau das fehlte dem `let`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Für jeden Zweig gibt es einen Test.

- `describe` beschreibt ein Ereignis, mit Wächter, Bindung und verschachteltem
  Muster
- `drain_stack` räumt einen Stapel ab, mit `while let`
- `first_click` sucht den ersten Klick, mit `if let` in einer Schleife

```console
cd units/09-01-muster-im-detail
cargo test
```

### Quelle

    Buch, Kapitel 19 "Patterns and Matching", Abschnitt 19.3 "Pattern Syntax",
    https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A pattern describes a shape and names the parts inside it. It is known from
`match`, but it stands at more places: in `let`, in `if let`, in `while let`, in
the head of a `for` loop and in the parameters of a function.

A pattern is refutable when it can fail, and irrefutable when it cannot.
`(breite, hoehe)` fits every pair and is irrefutable. `Some(wert)` does not fit
`None` and is refutable. Which sort is allowed hangs on the place: `let` wants
an irrefutable one, `if let` and `while let` want a refutable one, because
otherwise there would be nothing to decide.

Two tools come with it. A guard is an `if` behind the pattern and narrows the
arm further. A binding with `@` holds on to the value the pattern has just
checked, instead of only checking it.

### What it is good for

A nested value can be taken apart in one go instead of level by level. That does
not only save lines, it also keeps the condition and the name for the value
together in one place.

A guard takes up a condition that no pattern can express. `x == y` compares two
parts of the same value with each other, and there is no shape for that which
could be written down.

And `@` solves the case where both are needed: check that a character is a
digit, and then use that character. Without the binding it is either checked or
within reach, not both.

### The explanation

Patterns at three places, a guard, a binding with `@` and a nested value.

```rust
#[derive(Debug)]
struct Punkt {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Ereignis {
    Klick { punkt: Punkt, taste: char },
    Taste(char),
    Nichts,
}

// Deutsch: Ein match nimmt den verschachtelten Wert in einem Zug auseinander.
// Der Wächter hinter `if` gehört zum Zweig und nicht zum Muster, und `@` hält
// fest, was das Muster gerade geprüft hat.
fn beschreibung(ereignis: &Ereignis) -> String {
    match ereignis {
        Ereignis::Klick {
            punkt: Punkt { x, y },
            taste,
        } if x == y => format!("{taste} auf der Diagonalen bei {x}"),
        Ereignis::Klick {
            punkt: Punkt { x: 0, y },
            ..
        } => format!("am linken Rand, {y} tief"),
        Ereignis::Klick {
            punkt: Punkt { x, y },
            ..
        } => format!("bei {x} und {y}"),
        Ereignis::Taste(zeichen @ '0'..='9') => format!("Ziffer {zeichen}"),
        Ereignis::Taste(zeichen) => format!("Taste {zeichen}"),
        Ereignis::Nichts => String::from("nichts"),
    }
}

fn main() {
    // Deutsch: `let` nimmt ein Paar auseinander. Das Muster kann nicht
    // scheitern, es ist unwiderlegbar.
    let (breite, hoehe) = (3, 4);
    println!("{}", breite * hoehe);

    // Deutsch: `while let` läuft, solange das Muster passt. Hier darf es
    // scheitern, denn genau das beendet die Schleife.
    let mut stapel = vec![1, 2, 3];
    while let Some(oben) = stapel.pop() {
        print!("{oben} ");
    }
    println!();

    let klick = Ereignis::Klick {
        punkt: Punkt { x: 2, y: 2 },
        taste: 'L',
    };
    println!("{}", beschreibung(&klick));
    println!(
        "{}",
        beschreibung(&Ereignis::Klick {
            punkt: Punkt { x: 0, y: 5 },
            taste: 'R',
        })
    );
    println!("{}", beschreibung(&Ereignis::Taste('7')));
    println!("{}", beschreibung(&Ereignis::Nichts));
}
```

The program prints:

```text
12
3 2 1 
L auf der Diagonalen bei 2
am linken Rand, 5 tief
Ziffer 7
nichts
```

The order of the arms decides, because the first matching one wins. The click on
0 and 0 lands in the first arm and not in the second, although both would fit.

### Common mistakes

Writing a refutable pattern into a `let`.

```rust
fn main() {
    let zahlen = vec![1, 2, 3];

    let Some(erste) = zahlen.first();

    println!("{erste}");
}
```

The compiler answers:

```text
error[E0005]: refutable pattern in local binding
 --> muster.rs:4:9
  |
4 |     let Some(erste) = zahlen.first();
  |         ^^^^^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-02-refutability.html
  = note: the matched value is of type `Option<&i32>`
help: you might want to use `let...else` to handle the variant that isn't matched
  |
4 |     let Some(erste) = zahlen.first() else { todo!() };
  |                                      ++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0005`.
```

The message names the case that is not covered and suggests `let ... else`,
which is known from `03-05`. `if let` would do as well. Both are a place where a
pattern may fail, and that is exactly what the `let` was missing.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. There is a test for every arm.

- `describe` describes an event, with a guard, a binding and a nested pattern
- `drain_stack` clears a stack, with `while let`
- `first_click` looks for the first click, with `if let` inside a loop

```console
cd units/09-01-muster-im-detail
cargo test
```

### Source

    Book, chapter 19 "Patterns and Matching", section 19.3 "Pattern Syntax",
    https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
