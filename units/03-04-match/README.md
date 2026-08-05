# 03-04 match / match

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-04-match/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-03 enum`, dessen Typ `Reading` hier weiterbenutzt
  wird.
- Auf dieser Einheit bauen auf: `03-05 Option und if let`, `04-07 panic! und
  Result` und später `09-01 Muster im Detail`.
- Beim Antworten so zitieren: `03-04 match`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die Vollständigkeit ist die Aussage der Einheit. Wer `_` als allgemeine
  Empfehlung hinstellt, dreht sie um: `_` nimmt genau die Prüfung weg, um die es
  hier geht, und der Text sagt, wann das trotzdem richtig ist.
- Muster mit Bedingung, Bereichen oder `@` stehen in `09-01` und nicht hier.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-04-match/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `03-03 enum`, whose type `Reading` is used on here.
- Building on this unit: `03-05 Option und if let`, `04-07 panic! und Result`
  and later `09-01 Muster im Detail`.
- Cite like this when answering: `03-04 match`, plus the heading of the section,
  for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- Exhaustiveness is the point of the unit. Whoever presents `_` as general
  advice turns it around: `_` removes exactly the check this is about, and the
  text says when it is right all the same.
- Patterns with a condition, with ranges or with `@` stand in `09-01` and not
  here.

</details>

## Deutsch

### Worum es geht

`match` vergleicht einen Wert der Reihe nach mit Mustern. Der erste Zweig, dessen
Muster passt, gilt, und alle Zweige liefern denselben Typ.

Ein Muster kann die Daten einer Variante gleich mit herausholen. Aus
`Reading::Temperature(grad)` wird `grad` zu einer Bindung, die im Zweig
danebensteht.

Der Übersetzer verlangt, dass jeder mögliche Fall behandelt wird. Fehlt einer,
übersetzt das Programm nicht, und die Meldung nennt den Fall, der fehlt.

### Wofür das gut ist

Die Vollständigkeit ist der Grund, warum ein `enum` und `match` zusammengehören.
Wer später eine Variante hinzufügt, bekommt jede Stelle genannt, die sie noch
nicht behandelt. Das ist eine Liste von Fundstellen, die kein Mensch von Hand
zusammensucht.

`match` ist außerdem ein Ausdruck wie `if`. Sein Wert kann rechts von einem `let`
stehen oder der Rückgabewert einer Funktion sein, und deshalb müssen alle Zweige
denselben Typ liefern.

`_` fängt alles ab, was vorher nicht dastand. Das ist genau dann richtig, wenn
die übrigen Fälle wirklich gleich behandelt werden sollen, und es ist genau dann
falsch, wenn man sich nur die Arbeit sparen will: dann schweigt der Übersetzer
auch bei der nächsten neuen Variante.

### Die Erklärung

Ein vollständiges `match` und eines mit `_`.

```rust
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

fn as_text(messwert: &Reading) -> String {
    // Deutsch: Jeder Zweig nennt ein Muster und dahinter, was dann gilt. Alle
    // Zweige liefern denselben Typ, hier `String`.
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
        Reading::Range { von, bis } => format!("von {von} bis {bis} Grad"),
    }
}

fn carried_values(messwert: &Reading) -> u32 {
    // Deutsch: `_` fängt alles ab, was vorher nicht dastand. Hier sind das die
    // Bereiche mit ihren zwei Zahlen.
    match messwert {
        Reading::Missing => 0,
        Reading::Temperature(_) => 1,
        _ => 2,
    }
}

fn main() {
    let werte = [
        Reading::Missing,
        Reading::Temperature(17),
        Reading::Range { von: 3, bis: 9 },
    ];

    for messwert in &werte {
        println!("{} {}", as_text(messwert), carried_values(messwert));
    }
}
```

In `Reading::Temperature(_)` steht `_` an der Stelle einer Zahl, die niemanden
interessiert. Als ganzer Zweig steht `_` für jeden Fall, der noch offen ist.

### Häufige Fehler

Einen Zweig weglassen.

```rust
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

fn as_text(messwert: &Reading) -> String {
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
    }
}

fn main() {
    println!("{}", as_text(&Reading::Range { von: 3, bis: 9 }));
}
```

Der Übersetzer sagt dazu:

```text
error[E0004]: non-exhaustive patterns: `&Reading::Range { .. }` not covered
  --> vergessen.rs:8:11
   |
 8 |     match messwert {
   |           ^^^^^^^^ pattern `&Reading::Range { .. }` not covered
   |
note: `Reading` defined here
  --> vergessen.rs:1:6
   |
 1 | enum Reading {
   |      ^^^^^^^
...
 4 |     Range { von: i32, bis: i32 },
   |     ----- not covered
   = note: the matched value is of type `&Reading`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
10 ~         Reading::Temperature(grad) => format!("{grad} Grad"),
11 ~         &Reading::Range { .. } => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

Die Meldung nennt den fehlenden Fall beim Namen und zeigt die Zeile, an der er
im `enum` steht. Das ist der Dienst, den die Vollständigkeit leistet, und
deshalb ist ein `_`, das man aus Bequemlichkeit setzt, teuer: es macht genau
diese Meldung unmöglich.

Der Vorschlag am Ende der Meldung setzt `todo!()` in den neuen Zweig. Das ist ein
Platzhalter, der beim Laufen anhält, und keine Behandlung des Falls.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Typ `Reading` und die Funktion `as_text` stehen schon da.

- `highest` gibt die höchste Zahl eines Messwerts zurück, bei `Missing` die Null
- `label` gibt zu jedem Fall ein Wort zurück
- `carried_values` sagt, wie viele Zahlen ein Fall trägt, und benutzt dabei `_`

```console
cd units/03-04-match
cargo test
```

### Quelle

    Buch, Kapitel 6 "Enums and Pattern Matching", Abschnitt 6.2 "The match Control Flow Construct",
    https://doc.rust-lang.org/book/ch06-02-match.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`match` compares a value against patterns one after the other. The first arm
whose pattern fits holds, and all arms deliver the same type.

A pattern can pull the data of a variant out at the same time. Out of
`Reading::Temperature(grad)` comes `grad` as a binding standing beside the arm.

The compiler demands that every possible case is handled. If one is missing the
program does not compile, and the message names the case that is missing.

### What it is good for

Exhaustiveness is the reason why an `enum` and `match` belong together. Whoever
adds a variant later is told every place that does not handle it yet. That is a
list of sites no person puts together by hand.

`match` is also an expression like `if`. Its value can stand on the right of a
`let` or be the return value of a function, and that is why all arms have to
deliver the same type.

`_` catches everything that did not stand there before. It is right exactly when
the remaining cases really should be treated the same, and it is wrong exactly
when somebody only wants to save the work: then the compiler stays silent for
the next new variant as well.

### The explanation

One exhaustive `match` and one with `_`.

```rust
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

fn as_text(messwert: &Reading) -> String {
    // Deutsch: Jeder Zweig nennt ein Muster und dahinter, was dann gilt. Alle
    // Zweige liefern denselben Typ, hier `String`.
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
        Reading::Range { von, bis } => format!("von {von} bis {bis} Grad"),
    }
}

fn carried_values(messwert: &Reading) -> u32 {
    // Deutsch: `_` fängt alles ab, was vorher nicht dastand. Hier sind das die
    // Bereiche mit ihren zwei Zahlen.
    match messwert {
        Reading::Missing => 0,
        Reading::Temperature(_) => 1,
        _ => 2,
    }
}

fn main() {
    let werte = [
        Reading::Missing,
        Reading::Temperature(17),
        Reading::Range { von: 3, bis: 9 },
    ];

    for messwert in &werte {
        println!("{} {}", as_text(messwert), carried_values(messwert));
    }
}
```

In `Reading::Temperature(_)` the `_` stands in the place of a number nobody is
interested in. As a whole arm `_` stands for every case still open.

### Common mistakes

Leaving an arm out.

```rust
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

fn as_text(messwert: &Reading) -> String {
    match messwert {
        Reading::Missing => String::from("kein Wert"),
        Reading::Temperature(grad) => format!("{grad} Grad"),
    }
}

fn main() {
    println!("{}", as_text(&Reading::Range { von: 3, bis: 9 }));
}
```

The compiler answers:

```text
error[E0004]: non-exhaustive patterns: `&Reading::Range { .. }` not covered
  --> vergessen.rs:8:11
   |
 8 |     match messwert {
   |           ^^^^^^^^ pattern `&Reading::Range { .. }` not covered
   |
note: `Reading` defined here
  --> vergessen.rs:1:6
   |
 1 | enum Reading {
   |      ^^^^^^^
...
 4 |     Range { von: i32, bis: i32 },
   |     ----- not covered
   = note: the matched value is of type `&Reading`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
10 ~         Reading::Temperature(grad) => format!("{grad} Grad"),
11 ~         &Reading::Range { .. } => todo!(),
   |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

The message names the missing case and shows the line where it stands in the
`enum`. That is the service exhaustiveness performs, and it is why a `_` set out
of convenience is expensive: it makes exactly this message impossible.

The suggestion at the end of the message puts `todo!()` into the new arm. That
is a placeholder stopping the program while it runs, and not a treatment of the
case.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The type `Reading` and the function `as_text`
are already there.

- `highest` returns the highest number of a reading, zero for `Missing`
- `label` returns one word for each case
- `carried_values` says how many numbers a case carries, using `_` on the way

```console
cd units/03-04-match
cargo test
```

### Source

    Book, chapter 6 "Enums and Pattern Matching", section 6.2 "The match Control Flow Construct",
    https://doc.rust-lang.org/book/ch06-02-match.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
