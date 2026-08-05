# 03-03 enum / enum

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-03-enum/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-01 struct` und `03-02 Methoden`.
- Auf dieser Einheit bauen auf: `03-04 match`, `03-05 Option und if let` und
  später `04-07 panic! und Result`.
- Beim Antworten so zitieren: `03-03 enum`, dazu die Überschrift des Abschnitts,
  zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `match` wird hier benutzt und nicht erklärt. Es steht in `03-04`. Die
  Aufgaben legen deshalb Werte an, statt sie auseinanderzunehmen, und der Text
  sagt das an Ort und Stelle.
- Die Aussage der Einheit ist, dass ein unmöglicher Zustand nicht mehr
  hinschreibbar ist. Wer das mit "ordentlicher" oder "schöner" begründet, hat
  die Aussage verfehlt: der Unterschied ist, was der Übersetzer noch annimmt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-03-enum/`. It is public. Whoever
  is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `03-01 struct` and `03-02 Methoden`.
- Building on this unit: `03-04 match`, `03-05 Option und if let` and later
  `04-07 panic! und Result`.
- Cite like this when answering: `03-03 enum`, plus the heading of the section,
  for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `match` is used here and not explained. It stands in `03-04`. That is why the
  exercises create values instead of taking them apart, and the text says so on
  the spot.
- The point of the unit is that an impossible state can no longer be written
  down. Whoever argues that with "tidier" or "nicer" has missed it: the
  difference is what the compiler still accepts.

</details>

## Deutsch

### Worum es geht

Ein `enum` zählt die Fälle auf, die es gibt. Ein Wert dieses Typs ist genau
einer davon, nie zwei und nie keiner.

Jede Variante darf eigene Daten tragen, und zwar in denselben drei Formen wie
ein `struct`: gar keine, ein Tupel oder benannte Felder. Die Varianten eines
`enum` müssen dabei nicht dieselbe Form haben.

Eine Variante wird mit dem Namen des Typs angesprochen, also
`Reading::Temperature(17)`. Ohne den Namen davor kennt der Übersetzer sie nicht.

### Wofür das gut ist

Der übliche Ersatz ist ein `struct` mit einem Schalter darin: ein `bool`, das
sagt, ob der Wert gilt, und daneben der Wert. Diese Form erlaubt einen Zustand,
den es nicht geben darf, nämlich Schalter aus und trotzdem ein Wert daneben.
Beim Lesen fällt das niemandem auf, denn beide Felder sind für sich genommen in
Ordnung.

Mit einem `enum` gibt es diesen Zustand nicht mehr. Es gibt keine Stelle, an der
man ihn hinschreiben könnte, und deshalb muss auch niemand ihn abfangen. Das ist
der Unterschied zwischen einer Vereinbarung und einer Sache, die der Übersetzer
zurückweist.

Dass die Varianten verschiedene Daten tragen dürfen, macht den Unterschied erst
nützlich. Ein fehlender Wert braucht kein Feld, ein einzelner Messwert eine
Zahl, ein Bereich zwei. Alle drei sind derselbe Typ und passen in dieselbe
Funktion.

### Die Erklärung

Ein `enum` mit drei Varianten und daneben die Form, die zu viel erlaubt.

```rust
// Deutsch: Ein `enum` mit drei Varianten. Jede trägt andere Daten, und mehr
// Fälle als diese drei gibt es nicht.
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

// Deutsch: Dasselbe als struct mit einem Schalter. Diese Form erlaubt Werte,
// die es nicht geben darf.
struct ReadingStruct {
    hat_wert: bool,
    grad: i32,
}

fn main() {
    let ohne = Reading::Missing;
    let einzeln = Reading::Temperature(17);
    let bereich = Reading::Range { von: 3, bis: 9 };

    for messwert in [ohne, einzeln, bereich] {
        match messwert {
            Reading::Missing => println!("kein Wert"),
            Reading::Temperature(grad) => println!("{grad} Grad"),
            Reading::Range { von, bis } => println!("von {von} bis {bis} Grad"),
        }
    }

    // Deutsch: Kein Wert da, und trotzdem stehen 17 Grad daneben. Der Übersetzer
    // nimmt es an, denn beide Felder sind für sich genommen in Ordnung.
    let unmoeglich = ReadingStruct {
        hat_wert: false,
        grad: 17,
    };

    println!("{} {}", unmoeglich.hat_wert, unmoeglich.grad);
}
```

Das `match` in der Mitte nimmt die Varianten wieder auseinander. Es wird hier
benutzt und in `03-04` erklärt; diese Einheit legt Werte an. Die Funktion
`as_text` in `src/lib.rs` steht schon fertig da und benutzt dasselbe `match`,
damit die Tests etwas ansehen können.

### Häufige Fehler

Eine Variante ohne den Namen ihres Typs.

```rust
enum Reading {
    Missing,
    Temperature(i32),
}

fn main() {
    let messwert = Missing;

    match messwert {
        Reading::Missing => println!("kein Wert"),
        Reading::Temperature(grad) => println!("{grad} Grad"),
    }
}
```

Der Übersetzer sagt dazu:

```text
error[E0425]: cannot find value `Missing` in this scope
 --> variante.rs:7:20
  |
7 |     let messwert = Missing;
  |                    ^^^^^^^ not found in this scope
  |
help: consider importing this unit variant
  |
1 + use crate::Reading::Missing;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

Eine Variante gehört zu ihrem Typ und heißt deshalb `Reading::Missing`. Der
Vorschlag in der Meldung, sie mit `use` hereinzuholen, geht auch; üblich ist er
für lange Namen und nicht als Regel.

Dass hier `E0425` steht und nicht eine Meldung über Varianten, hat einen Grund:
für den Übersetzer ist `Missing` an dieser Stelle einfach ein Name, den es nicht
gibt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Typ `Reading` und die Funktion `as_text` stehen schon da.

- `missing` gibt die Variante ohne Daten zurück
- `single` gibt eine Variante mit einer Zahl zurück
- `range` gibt eine Variante mit zwei benannten Feldern zurück, wobei `von` nie
  größer als `bis` ist

```console
cd units/03-03-enum
cargo test
```

### Quelle

    Buch, Kapitel 6 "Enums and Pattern Matching", Abschnitt 6.1 "Defining an Enum",
    https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

An `enum` lists the cases there are. A value of that type is exactly one of
them, never two and never none.

Every variant may carry data of its own, in the same three forms as a `struct`:
none at all, a tuple, or named fields. The variants of one `enum` do not have to
share a form.

A variant is addressed through the name of its type, so
`Reading::Temperature(17)`. Without the name in front the compiler does not know
it.

### What it is good for

The usual substitute is a `struct` with a switch inside: a `bool` saying whether
the value counts, and the value beside it. That form allows a state that must
not exist, namely the switch off and a value beside it all the same. Nobody
notices while reading, because each field on its own is in order.

With an `enum` that state is gone. There is no place where it could be written
down, and therefore nobody has to catch it either. That is the difference
between an agreement and a thing the compiler refuses.

That the variants may carry different data is what makes the difference useful
in the first place. A missing value needs no field, a single reading one number,
a range two. All three are the same type and fit into the same function.

### The explanation

An `enum` with three variants and beside it the form that allows too much.

```rust
// Deutsch: Ein `enum` mit drei Varianten. Jede trägt andere Daten, und mehr
// Fälle als diese drei gibt es nicht.
enum Reading {
    Missing,
    Temperature(i32),
    Range { von: i32, bis: i32 },
}

// Deutsch: Dasselbe als struct mit einem Schalter. Diese Form erlaubt Werte,
// die es nicht geben darf.
struct ReadingStruct {
    hat_wert: bool,
    grad: i32,
}

fn main() {
    let ohne = Reading::Missing;
    let einzeln = Reading::Temperature(17);
    let bereich = Reading::Range { von: 3, bis: 9 };

    for messwert in [ohne, einzeln, bereich] {
        match messwert {
            Reading::Missing => println!("kein Wert"),
            Reading::Temperature(grad) => println!("{grad} Grad"),
            Reading::Range { von, bis } => println!("von {von} bis {bis} Grad"),
        }
    }

    // Deutsch: Kein Wert da, und trotzdem stehen 17 Grad daneben. Der Übersetzer
    // nimmt es an, denn beide Felder sind für sich genommen in Ordnung.
    let unmoeglich = ReadingStruct {
        hat_wert: false,
        grad: 17,
    };

    println!("{} {}", unmoeglich.hat_wert, unmoeglich.grad);
}
```

The `match` in the middle takes the variants apart again. It is used here and
explained in `03-04`; this unit creates values. The function `as_text` in
`src/lib.rs` already stands there finished and uses the same `match`, so that
the tests have something to look at.

### Common mistakes

A variant without the name of its type.

```rust
enum Reading {
    Missing,
    Temperature(i32),
}

fn main() {
    let messwert = Missing;

    match messwert {
        Reading::Missing => println!("kein Wert"),
        Reading::Temperature(grad) => println!("{grad} Grad"),
    }
}
```

The compiler answers:

```text
error[E0425]: cannot find value `Missing` in this scope
 --> variante.rs:7:20
  |
7 |     let messwert = Missing;
  |                    ^^^^^^^ not found in this scope
  |
help: consider importing this unit variant
  |
1 + use crate::Reading::Missing;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

A variant belongs to its type and is therefore called `Reading::Missing`. The
suggestion in the message, to bring it in with `use`, works too; it is customary
for long names rather than as a rule.

That `E0425` stands here and not a message about variants has a reason: to the
compiler `Missing` in that place is simply a name that does not exist.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The type `Reading` and the function `as_text`
are already there.

- `missing` returns the variant without data
- `single` returns a variant carrying one number
- `range` returns a variant with two named fields, where `von` is never bigger
  than `bis`

```console
cd units/03-03-enum
cargo test
```

### Source

    Book, chapter 6 "Enums and Pattern Matching", section 6.1 "Defining an Enum",
    https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
