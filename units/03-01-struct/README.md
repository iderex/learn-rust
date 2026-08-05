# 03-01 struct / struct

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/03-01-struct/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: die Stufe 2, vor allem `02-03 Ausleihen`, denn die
  Aufgaben lesen Felder durch eine Ausleihe.
- Auf dieser Einheit bauen auf: `03-02 Methoden`, `03-06 derive mit Debug` und
  alles Weitere der Stufe 3.
- Beim Antworten so zitieren: `03-01 struct`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Hier steht kein `impl` und kein `derive`. Methoden sind `03-02`, und
  `#[derive(Debug, PartialEq)]` ist `03-06`. Deshalb lesen die Tests einzelne
  Felder, statt ganze Werte zu vergleichen, und das ist Absicht und kein
  Versehen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/03-01-struct/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: stage 2, above all `02-03 Ausleihen`, because the
  exercises read fields through a loan.
- Building on this unit: `03-02 Methoden`, `03-06 derive mit Debug` and
  everything else in stage 3.
- Cite like this when answering: `03-01 struct`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- No `impl` and no `derive` stand here. Methods are `03-02`, and
  `#[derive(Debug, PartialEq)]` is `03-06`. That is why the tests read single
  fields instead of comparing whole values, and it is deliberate rather than an
  oversight.

</details>

## Deutsch

### Worum es geht

Ein `struct` fasst mehrere Werte zu einem zusammen und gibt jedem einen Namen.
Statt Breite und Höhe getrennt durch das Programm zu reichen, gibt es ein
Rechteck, das beides trägt.

Es gibt drei Formen. Die übliche hat benannte Felder. Ein Tupel-Struct hat
Felder ohne Namen, die über ihre Nummer angesprochen werden. Ein Struct ohne
Feld trägt gar keine Daten.

Ein Feld wird mit dem Punkt gelesen, also `rechteck.breite`. Beim Anlegen müssen
alle Felder dastehen; es gibt keinen halb gefüllten Wert.

### Wofür das gut ist

Zwei Zahlen mit Namen sind etwas anderes als zwei Zahlen. Wer `flaeche(3, 4)`
liest, muss wissen, welche Zahl welche ist. Wer `flaeche(&rechteck)` liest, sieht
es am Typ, und wer die beiden vertauscht, bekommt es vom Übersetzer gesagt.

Das Tupel-Struct ist für den Fall, dass ein einzelner Wert eine Bedeutung
bekommen soll. `Meter(1200)` ist keine beliebige Zahl mehr, und eine Funktion,
die Meter erwartet, nimmt keine Sekunden entgegen. Diese Form heißt Newtype und
kommt in `09-04` ausführlich vor.

Ein Struct ohne Feld belegt keinen Platz. Es ist dann nützlich, wenn nicht die
Daten zählen, sondern nur, dass es diesen Typ gibt; solche Fälle stehen in
Stufe 5.

### Die Erklärung

Alle drei Formen und wie ihre Werte entstehen.

```rust
// Deutsch: Ein struct mit benannten Feldern.
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

// Deutsch: Ein Tupel-Struct. Die Felder haben keine Namen, sondern Nummern.
struct Meter(u32);

// Deutsch: Ein struct ohne Feld. Es belegt keinen Platz und steht für sich
// selbst.
struct Marker;

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    println!("{} {}", rechteck.breite, rechteck.hoehe);

    let strecke = Meter(1200);

    println!("{}", strecke.0);

    let _marker = Marker;
}
```

Ein `struct` lässt sich nicht ohne Weiteres mit `{}` ausgeben und auch nicht mit
`==` vergleichen. Beides kommt erst mit `#[derive(...)]` in `03-06` dazu, und
deshalb liest diese Einheit einzelne Felder.

### Häufige Fehler

Ein Feld beim Anlegen vergessen.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

fn main() {
    let rechteck = Rectangle { breite: 3 };

    println!("{}", rechteck.hoehe);
}
```

Der Übersetzer sagt dazu:

```text
error[E0063]: missing field `hoehe` in initializer of `Rectangle`
 --> feld.rs:7:20
  |
7 |     let rechteck = Rectangle { breite: 3 };
  |                    ^^^^^^^^^ missing `hoehe`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0063`.
```

Die Meldung nennt das fehlende Feld beim Namen. Ein Wert ist entweder ganz da
oder gar nicht, und ein Feld, das später gefüllt werden soll, gibt es nicht.
Wenn ein Feld wirklich fehlen darf, ist sein Typ `Option`, und das steht in
`03-05`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Die Typen `Rectangle`, `Meter` und `Marker` stehen schon da.

- `new_rectangle` legt ein `Rectangle` aus Breite und Höhe an
- `area_of` gibt die Fläche eines geliehenen Rechtecks zurück
- `in_meters` gibt die Zahl aus einem `Meter` zurück

```console
cd units/03-01-struct
cargo test
```

### Quelle

    Buch, Kapitel 5 "Using Structs to Structure Related Data", Abschnitt 5.1 "Defining and Instantiating Structs",
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A `struct` gathers several values into one and gives each of them a name.
Instead of passing width and height through the program separately there is a
rectangle carrying both.

There are three forms. The usual one has named fields. A tuple struct has fields
without names, addressed by their number. A struct without a field carries no
data at all.

A field is read with the dot, so `rechteck.breite`. When a value is created
every field has to be there; there is no half filled value.

### What it is good for

Two numbers with names are something else than two numbers. Whoever reads
`flaeche(3, 4)` has to know which number is which. Whoever reads
`flaeche(&rechteck)` sees it from the type, and whoever swaps the two is told so
by the compiler.

The tuple struct is for the case where a single value is to carry a meaning.
`Meter(1200)` is no longer just any number, and a function expecting metres does
not accept seconds. That form is called a newtype and appears in full in
`09-04`.

A struct without a field takes up no space. It is useful where the data do not
matter and only the existence of the type does; such cases stand in stage 5.

### The explanation

All three forms and how their values come about.

```rust
// Deutsch: Ein struct mit benannten Feldern.
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

// Deutsch: Ein Tupel-Struct. Die Felder haben keine Namen, sondern Nummern.
struct Meter(u32);

// Deutsch: Ein struct ohne Feld. Es belegt keinen Platz und steht für sich
// selbst.
struct Marker;

fn main() {
    let rechteck = Rectangle {
        breite: 3,
        hoehe: 4,
    };

    println!("{} {}", rechteck.breite, rechteck.hoehe);

    let strecke = Meter(1200);

    println!("{}", strecke.0);

    let _marker = Marker;
}
```

A `struct` cannot simply be printed with `{}` and cannot be compared with `==`
either. Both only arrive with `#[derive(...)]` in `03-06`, and that is why this
unit reads single fields.

### Common mistakes

Forgetting a field when creating a value.

```rust
struct Rectangle {
    breite: u32,
    hoehe: u32,
}

fn main() {
    let rechteck = Rectangle { breite: 3 };

    println!("{}", rechteck.hoehe);
}
```

The compiler answers:

```text
error[E0063]: missing field `hoehe` in initializer of `Rectangle`
 --> feld.rs:7:20
  |
7 |     let rechteck = Rectangle { breite: 3 };
  |                    ^^^^^^^^^ missing `hoehe`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0063`.
```

The message names the missing field. A value is either wholly there or not at
all, and a field to be filled in later does not exist. Where a field really may
be missing its type is `Option`, and that stands in `03-05`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The types `Rectangle`, `Meter` and `Marker`
are already there.

- `new_rectangle` creates a `Rectangle` from a width and a height
- `area_of` returns the area of a borrowed rectangle
- `in_meters` returns the number out of a `Meter`

```console
cd units/03-01-struct
cargo test
```

### Source

    Book, chapter 5 "Using Structs to Structure Related Data", section 5.1 "Defining and Instantiating Structs",
    https://doc.rust-lang.org/book/ch05-01-defining-structs.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
