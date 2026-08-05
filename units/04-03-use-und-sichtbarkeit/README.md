# 04-03 use und Sichtbarkeit / use and visibility

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-03-use-und-sichtbarkeit/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `04-02 Module`.
- Auf dieser Einheit bauen auf: `04-04 Vec`, `04-05 String` und `04-06 HashMap`,
  die alle mit einem `use` aus der Standardbibliothek anfangen.
- Beim Antworten so zitieren: `04-03 use und Sichtbarkeit`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `use` ändert nichts an der Sichtbarkeit. Es kürzt einen Pfad, und was privat
  ist, bleibt privat. Wer beides vermischt, erklärt `E0603` aus `04-02` falsch.
- `pub use` ist etwas anderes als `use`: es gibt den Namen nach außen weiter.
  Die Einheit zeigt genau das an einer Stelle, und ein Test ruft denselben Wert
  über beide Pfade auf.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-03-use-und-sichtbarkeit/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `04-02 Module`.
- Building on this unit: `04-04 Vec`, `04-05 String` and `04-06 HashMap`, which
  all start with a `use` from the standard library.
- Cite like this when answering: `04-03 use und Sichtbarkeit`, plus the heading
  of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `use` changes nothing about visibility. It shortens a path, and what is
  private stays private. Whoever mixes the two explains `E0603` from `04-02`
  wrongly.
- `pub use` is something else than `use`: it hands the name on outwards. The
  unit shows exactly that in one place, and a test calls the same value through
  both paths.

</details>

## Deutsch

### Worum es geht

`use` holt einen Pfad in den Blick. Danach reicht der letzte Teil des Namens,
und der lange Pfad steht einmal oben statt zehnmal im Text.

Es gibt ein paar Formen davon. Mehrere Namen aus einem Ast stehen in
geschweiften Klammern. `as` gibt einem Namen einen zweiten, wenn der erste schon
vergeben ist. Und `pub use` kürzt nicht nur, sondern gibt den Namen nach außen
weiter.

Die Sichtbarkeit ist davon unberührt. `use` darf nur, was auch ohne `use` ginge:
was privat ist, bleibt privat, und der Pfad wird davon nicht länger oder kürzer.

### Wofür das gut ist

Der übliche Nutzen ist die Lesbarkeit. Wer `HashMap::new()` schreibt, will nicht
jedes Mal `std::collections::HashMap::new()` schreiben, und der eine `use`
oben sagt einmal, woher der Name kommt.

Wie weit man kürzt, ist eine Entscheidung. Ein Modul zu holen und den letzten
Schritt am Aufruf stehen zu lassen, also `celsius::to_fahrenheit(20)`, sagt
beim Lesen noch, wo die Funktion herkommt. Die Funktion selbst zu holen ist
kürzer und sagt es nicht mehr.

`pub use` ist das Mittel, mit dem eine Bibliothek eine kurze Oberfläche über
einem tiefen Baum anbietet. Innen liegt `messwerte::celsius::to_fahrenheit`,
außen steht `to_fahrenheit`, und wer den Baum umbaut, ändert eine Zeile.

### Die Erklärung

Drei Formen von `use` nebeneinander.

```rust
pub mod messwerte {
    pub mod celsius {
        pub fn to_fahrenheit(grad: i32) -> i32 {
            grad * 9 / 5 + 32
        }
    }

    pub mod kelvin {
        pub fn from_celsius(grad: i32) -> i32 {
            grad + 273
        }
    }
}

// Deutsch: `use` holt einen Pfad in den Blick. Üblich ist es, das Modul zu
// holen und den letzten Schritt am Aufruf stehen zu lassen.
use messwerte::celsius;

// Deutsch: Zwei Namen aus einem Ast, in geschweiften Klammern.
use messwerte::{celsius::to_fahrenheit, kelvin};

// Deutsch: `as` gibt einen zweiten Namen, wenn der erste schon vergeben ist
// oder zu allgemein klingt.
use messwerte::kelvin::from_celsius as kelvin_aus_celsius;

fn main() {
    println!("{}", celsius::to_fahrenheit(100));
    println!("{}", to_fahrenheit(0));
    println!("{}", kelvin::from_celsius(0));
    println!("{}", kelvin_aus_celsius(100));
}
```

Alle vier Aufrufe meinen dieselben zwei Funktionen. Was sich ändert, ist nur,
wie viel vom Weg dorthin noch im Aufruf steht.

`pub use` sieht genauso aus, mit `pub` davor, und wirkt nach außen. In dieser
Einheit steht eine solche Zeile in `src/lib.rs`, und ein Test ruft denselben
Wert einmal über den kurzen und einmal über den langen Pfad auf.

### Häufige Fehler

Einen Pfad benutzen, ohne ihn zu holen.

```rust
pub mod messwerte {
    pub mod celsius {
        pub fn to_fahrenheit(grad: i32) -> i32 {
            grad * 9 / 5 + 32
        }
    }
}

fn main() {
    println!("{}", celsius::to_fahrenheit(100));
}
```

Der Übersetzer sagt dazu:

```text
error[E0433]: cannot find module or crate `celsius` in this scope
  --> pfad.rs:10:20
   |
10 |     println!("{}", celsius::to_fahrenheit(100));
   |                    ^^^^^^^ use of unresolved module or unlinked crate `celsius`
   |
   = help: you might be missing a crate named `celsius`
help: consider importing this module
   |
 1 + use crate::messwerte::celsius;
   |

error: aborting due to 1 previous error
```

Der Name `celsius` steht im Baum unter `messwerte`, und im Blick ist er
deshalb nicht. Die Meldung schlägt genau die fehlende Zeile vor.

Das ist ein anderer Fall als `E0603` aus `04-02`. Dort war der Weg versperrt,
hier ist er nur nicht abgekürzt: `messwerte::celsius::to_fahrenheit(100)` geht
auch ohne jedes `use`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Baum aus `celsius` und `kelvin` steht schon da, und
`src/lib.rs` trägt ein `pub use`.

- `boiling_in_fahrenheit` gibt den Siedepunkt in Fahrenheit zurück
- `round_trip` rechnet nach Fahrenheit und wieder zurück
- `in_kelvin` rechnet von Celsius nach Kelvin

```console
cd units/04-03-use-und-sichtbarkeit
cargo test
```

### Quelle

    Buch, Kapitel 7 "Packages, Crates, and Modules", Abschnitt 7.4 "Bringing Paths Into Scope with the use Keyword",
    https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`use` brings a path into view. Afterwards the last part of the name is enough,
and the long path stands once at the top instead of ten times in the text.

There are a few forms of it. Several names out of one branch stand in curly
braces. `as` gives a name a second one where the first is taken already. And
`pub use` does not only shorten but hands the name on outwards.

Visibility is untouched by all of it. `use` may only do what would work without
`use` as well: what is private stays private, and the path does not grow longer
or shorter for it.

### What it is good for

The usual use is readability. Whoever writes `HashMap::new()` does not want to
write `std::collections::HashMap::new()` every time, and the one `use` at the
top says once where the name comes from.

How far to shorten is a decision. Bringing in the module and leaving the last
step at the call, so `celsius::to_fahrenheit(20)`, still says while reading
where the function comes from. Bringing in the function itself is shorter and no
longer says it.

`pub use` is the means by which a library offers a short surface over a deep
tree. Inside lies `messwerte::celsius::to_fahrenheit`, outside stands
`to_fahrenheit`, and whoever rebuilds the tree changes one line.

### The explanation

Three forms of `use` next to each other.

```rust
pub mod messwerte {
    pub mod celsius {
        pub fn to_fahrenheit(grad: i32) -> i32 {
            grad * 9 / 5 + 32
        }
    }

    pub mod kelvin {
        pub fn from_celsius(grad: i32) -> i32 {
            grad + 273
        }
    }
}

// Deutsch: `use` holt einen Pfad in den Blick. Üblich ist es, das Modul zu
// holen und den letzten Schritt am Aufruf stehen zu lassen.
use messwerte::celsius;

// Deutsch: Zwei Namen aus einem Ast, in geschweiften Klammern.
use messwerte::{celsius::to_fahrenheit, kelvin};

// Deutsch: `as` gibt einen zweiten Namen, wenn der erste schon vergeben ist
// oder zu allgemein klingt.
use messwerte::kelvin::from_celsius as kelvin_aus_celsius;

fn main() {
    println!("{}", celsius::to_fahrenheit(100));
    println!("{}", to_fahrenheit(0));
    println!("{}", kelvin::from_celsius(0));
    println!("{}", kelvin_aus_celsius(100));
}
```

All four calls mean the same two functions. What changes is only how much of the
way there still stands in the call.

`pub use` looks the same with `pub` in front and works outwards. In this unit
such a line stands in `src/lib.rs`, and a test calls the same value once through
the short and once through the long path.

### Common mistakes

Using a path without bringing it in.

```rust
pub mod messwerte {
    pub mod celsius {
        pub fn to_fahrenheit(grad: i32) -> i32 {
            grad * 9 / 5 + 32
        }
    }
}

fn main() {
    println!("{}", celsius::to_fahrenheit(100));
}
```

The compiler answers:

```text
error[E0433]: cannot find module or crate `celsius` in this scope
  --> pfad.rs:10:20
   |
10 |     println!("{}", celsius::to_fahrenheit(100));
   |                    ^^^^^^^ use of unresolved module or unlinked crate `celsius`
   |
   = help: you might be missing a crate named `celsius`
help: consider importing this module
   |
 1 + use crate::messwerte::celsius;
   |

error: aborting due to 1 previous error
```

The name `celsius` stands in the tree under `messwerte`, and it is therefore not
in view. The message suggests exactly the missing line.

That is a different case from `E0603` in `04-02`. There the way was blocked,
here it is only not shortened: `messwerte::celsius::to_fahrenheit(100)` works
without any `use` at all.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The tree of `celsius` and `kelvin` is already
there, and `src/lib.rs` carries a `pub use`.

- `boiling_in_fahrenheit` returns the boiling point in Fahrenheit
- `round_trip` converts to Fahrenheit and back again
- `in_kelvin` converts from Celsius to Kelvin

```console
cd units/04-03-use-und-sichtbarkeit
cargo test
```

### Source

    Book, chapter 7 "Packages, Crates, and Modules", section 7.4 "Bringing Paths Into Scope with the use Keyword",
    https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
