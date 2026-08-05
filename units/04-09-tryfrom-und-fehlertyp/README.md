# 04-09 TryFrom und ein eigener Fehlertyp / TryFrom and an error type of your own

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/04-09-tryfrom-und-fehlertyp/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `03-07 Display selbst schreiben` und `04-08 From, Into
  und der Operator ?`.
- Auf dieser Einheit bauen auf: `04-10 std::error::Error und Box<dyn Error>`,
  und später `09-04 Das Newtype-Muster`.
- Beim Antworten so zitieren: `04-09 TryFrom und ein eigener Fehlertyp`, dazu
  die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `TryFrom` ist `From` für Umwandlungen, die scheitern können. Wer `From` für so
  eine Umwandlung vorschlägt, muss den schlechten Fall irgendwo verstecken, und
  genau das ist der Fehler.
- `Display` am Fehlertyp ist die Meldung für Menschen und nicht `Debug`. Der
  Unterschied steht in `03-07` und wird hier gebraucht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-09-tryfrom-und-fehlertyp/`. It
  is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `03-07 Display selbst schreiben` and `04-08 From, Into
  und der Operator ?`.
- Building on this unit: `04-10 std::error::Error und Box<dyn Error>`, and later
  `09-04 Das Newtype-Muster`.
- Cite like this when answering: `04-09 TryFrom und ein eigener Fehlertyp`, plus
  the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `TryFrom` is `From` for conversions that can fail. Whoever suggests `From` for
  such a conversion has to hide the bad case somewhere, and that is exactly the
  mistake.
- `Display` on the error type is the message for people and not `Debug`. The
  difference stands in `03-07` and is needed here.

</details>

## Deutsch

### Worum es geht

`From` sagt, wie aus einem Wert ein anderer wird, und es kann nicht scheitern.
Viele Umwandlungen können aber scheitern: aus jeder Zahl wird kein Alter, aus
jedem Text keine Zahl.

Dafür gibt es `TryFrom`. Es sieht aus wie `From`, hat aber einen Fehlertyp
daneben, der als `type Error` im `impl` steht, und `try_from` gibt ein `Result`
zurück.

Wie `into` zu `From` gehört, gehört `try_into` zu `TryFrom`. Wer `TryFrom`
schreibt, bekommt es geschenkt.

Der Fehlertyp ist dabei ein eigener Typ, und er trägt `Display`, damit die
Meldung für Menschen an ihm steht und nicht an jeder Aufrufstelle neu.

### Wofür das gut ist

Ein Typ, der nur gültige Werte enthält, ist mehr wert als eine Prüfung, die
irgendwo steht. Wer ein `Alter` in der Hand hat, muss nicht mehr fragen, ob die
Zahl darin ein Alter sein kann, denn sonst gäbe es den Wert nicht.

Die Prüfung steht damit an einer Stelle, nämlich im `try_from`. Das ist dieselbe
Bewegung wie beim `enum` in `03-03`: einen unmöglichen Zustand nicht mehr
hinschreibbar machen, statt ihn überall abzufangen.

Und `Display` am Fehlertyp trennt die beiden Leser. `Debug` sagt
`Err(ZuGross)`, `Display` sagt "ein Alter über 130 gibt es nicht", und beide
stehen an einer Stelle statt in jeder Meldung.

### Die Erklärung

Ein Typ mit Prüfung, ein Fehlertyp mit Meldung.

```rust
use std::fmt;

// Deutsch: Ein eigener Typ, der nur gültige Werte enthalten soll.
#[derive(Debug, PartialEq)]
struct Alter(u32);

#[derive(Debug, PartialEq)]
enum AlterFehler {
    Negativ,
    ZuGross,
}

// Deutsch: Die Meldung für Menschen, wie in `03-07`.
impl fmt::Display for AlterFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlterFehler::Negativ => write!(f, "ein Alter ist nicht negativ"),
            AlterFehler::ZuGross => write!(f, "ein Alter über 130 gibt es nicht"),
        }
    }
}

// Deutsch: `TryFrom` ist `From` für Umwandlungen, die scheitern können. Der
// zugehörige Fehlertyp steht als `type Error` daneben.
impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        if zahl < 0 {
            return Err(AlterFehler::Negativ);
        }

        if zahl > 130 {
            return Err(AlterFehler::ZuGross);
        }

        Ok(Alter(zahl as u32))
    }
}

fn main() {
    // Deutsch: `try_into` kommt mit `TryFrom`, so wie `into` mit `From`. Das
    // Ergebnis ist ein `Result` und wird behandelt.
    let gut: Result<Alter, AlterFehler> = 42.try_into();
    let schlecht: Result<Alter, AlterFehler> = 200.try_into();

    println!("{gut:?}");
    println!("{schlecht:?}");

    match schlecht {
        Ok(alter) => println!("{}", alter.0),
        Err(fehler) => println!("{fehler}"),
    }
}
```

Das Programm gibt aus:

```text
Ok(Alter(42))
Err(ZuGross)
ein Alter über 130 gibt es nicht
```

Die zweite Zeile ist `Debug`, die dritte `Display`. Derselbe Fehler, zwei Leser,
und das ist der Unterschied aus `03-07`.

### Häufige Fehler

Das Ergebnis für den Wert halten.

```rust
struct Alter(u32);

#[derive(Debug)]
enum AlterFehler {
    ZuGross,
}

impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        if zahl > 130 {
            return Err(AlterFehler::ZuGross);
        }

        Ok(Alter(zahl as u32))
    }
}

fn main() {
    let alter: Alter = 42.try_into();

    println!("{}", alter.0);
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
  --> versuch.rs:21:24
   |
21 |     let alter: Alter = 42.try_into();
   |                -----   ^^^^^^^^^^^^^ expected `Alter`, found `Result<_, _>`
   |                |
   |                expected due to this
   |
   = note: expected struct `Alter`
                found enum `Result<_, _>`
help: consider using `Result::expect` to unwrap the `Result<_, _>` value, panicking if the value is a `Result::Err`
   |
21 |     let alter: Alter = 42.try_into().expect("REASON");
   |                                     +++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`try_into` gibt ein `Result`, und das ist der ganze Punkt: die Umwandlung kann
scheitern, und der schlechte Fall steht im Typ.

Der Vorschlag am Ende ist bequem und in einem kurzen Versuch in Ordnung.
`expect` hält an, wenn der Fall doch eintritt, und dann ist von der Prüfung
nichts geblieben außer einem Absturz mit dem Wort "REASON" darin.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Typ `Alter` und der Fehlertyp stehen schon da.

- `TryFrom<i32> for Alter` prüft die Zahl und gibt sonst einen Fehler zurück
- `Display for AlterFehler` schreibt die Meldung für Menschen
- `age_from` benutzt `try_into` statt `Alter::try_from`

```console
cd units/04-09-tryfrom-und-fehlertyp
cargo test
```

### Quelle

    Buch, Kapitel 9 "Error Handling", Abschnitt 9.3 "To panic! or Not to panic!",
    https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html,
    geprüft gegen 1.97.1

    Standardbibliothek, "TryFrom in std::convert",
    https://doc.rust-lang.org/std/convert/trait.TryFrom.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`From` says how one value becomes another, and it cannot fail. Many conversions
can fail though: not every number is an age, not every text is a number.

For those there is `TryFrom`. It looks like `From` but has an error type beside
it, standing as `type Error` in the `impl`, and `try_from` returns a `Result`.

The way `into` belongs to `From`, `try_into` belongs to `TryFrom`. Whoever
writes `TryFrom` gets it as a gift.

The error type is a type of your own, and it carries `Display`, so that the
message for people stands on it and not anew at every call site.

### What it is good for

A type holding only valid values is worth more than a check standing somewhere.
Whoever holds an `Alter` no longer has to ask whether the number in it can be an
age, because otherwise the value would not exist.

The check therefore stands in one place, namely in `try_from`. That is the same
move as with the `enum` in `03-03`: make an impossible state impossible to write
down instead of catching it everywhere.

And `Display` on the error type separates the two readers. `Debug` says
`Err(ZuGross)`, `Display` says "ein Alter über 130 gibt es nicht", and both
stand in one place instead of in every message.

### The explanation

A type with a check, an error type with a message.

```rust
use std::fmt;

// Deutsch: Ein eigener Typ, der nur gültige Werte enthalten soll.
#[derive(Debug, PartialEq)]
struct Alter(u32);

#[derive(Debug, PartialEq)]
enum AlterFehler {
    Negativ,
    ZuGross,
}

// Deutsch: Die Meldung für Menschen, wie in `03-07`.
impl fmt::Display for AlterFehler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlterFehler::Negativ => write!(f, "ein Alter ist nicht negativ"),
            AlterFehler::ZuGross => write!(f, "ein Alter über 130 gibt es nicht"),
        }
    }
}

// Deutsch: `TryFrom` ist `From` für Umwandlungen, die scheitern können. Der
// zugehörige Fehlertyp steht als `type Error` daneben.
impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        if zahl < 0 {
            return Err(AlterFehler::Negativ);
        }

        if zahl > 130 {
            return Err(AlterFehler::ZuGross);
        }

        Ok(Alter(zahl as u32))
    }
}

fn main() {
    // Deutsch: `try_into` kommt mit `TryFrom`, so wie `into` mit `From`. Das
    // Ergebnis ist ein `Result` und wird behandelt.
    let gut: Result<Alter, AlterFehler> = 42.try_into();
    let schlecht: Result<Alter, AlterFehler> = 200.try_into();

    println!("{gut:?}");
    println!("{schlecht:?}");

    match schlecht {
        Ok(alter) => println!("{}", alter.0),
        Err(fehler) => println!("{fehler}"),
    }
}
```

The program prints:

```text
Ok(Alter(42))
Err(ZuGross)
ein Alter über 130 gibt es nicht
```

The second line is `Debug`, the third is `Display`. The same error, two readers,
and that is the difference from `03-07`.

### Common mistakes

Taking the result for the value.

```rust
struct Alter(u32);

#[derive(Debug)]
enum AlterFehler {
    ZuGross,
}

impl TryFrom<i32> for Alter {
    type Error = AlterFehler;

    fn try_from(zahl: i32) -> Result<Self, Self::Error> {
        if zahl > 130 {
            return Err(AlterFehler::ZuGross);
        }

        Ok(Alter(zahl as u32))
    }
}

fn main() {
    let alter: Alter = 42.try_into();

    println!("{}", alter.0);
}
```

The compiler answers:

```text
error[E0308]: mismatched types
  --> versuch.rs:21:24
   |
21 |     let alter: Alter = 42.try_into();
   |                -----   ^^^^^^^^^^^^^ expected `Alter`, found `Result<_, _>`
   |                |
   |                expected due to this
   |
   = note: expected struct `Alter`
                found enum `Result<_, _>`
help: consider using `Result::expect` to unwrap the `Result<_, _>` value, panicking if the value is a `Result::Err`
   |
21 |     let alter: Alter = 42.try_into().expect("REASON");
   |                                     +++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

`try_into` gives a `Result`, and that is the whole point: the conversion can
fail, and the bad case stands in the type.

The suggestion at the end is convenient and in order in a short try. `expect`
stops when the case does occur after all, and then nothing is left of the check
but a break with the word "REASON" in it.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The type `Alter` and the error type are
already there.

- `TryFrom<i32> for Alter` checks the number and otherwise returns an error
- `Display for AlterFehler` writes the message for people
- `age_from` uses `try_into` instead of `Alter::try_from`

```console
cd units/04-09-tryfrom-und-fehlertyp
cargo test
```

### Source

    Book, chapter 9 "Error Handling", section 9.3 "To panic! or Not to panic!",
    https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html,
    checked against 1.97.1

    Standard library, "TryFrom in std::convert",
    https://doc.rust-lang.org/std/convert/trait.TryFrom.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
