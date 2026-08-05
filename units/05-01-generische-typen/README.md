# 05-01 Generische Typen / Generic types

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-01-generische-typen/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `03-01 struct`, `03-02 Methoden` und
  `03-05 Option und if let`.
- Auf dieser Einheit bauen auf: `05-02 Traits` und `05-03 Trait Bounds`, wo aus
  dem freien `T` ein gebundenes wird.
- Beim Antworten so zitieren: `05-01 Generische Typen`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein freies `T` kann fast nichts. Wer hier eine Aufgabe mit `>` oder `+` löst,
  braucht eine Schranke, und die steht in `05-03`. Die Aufgaben dieser Einheit
  kommen ohne aus, und das ist Absicht.
- Es gibt keine Kosten zur Laufzeit. Der Übersetzer schreibt je benutztem Typ
  eine eigene Fassung; wer von einer Prüfung beim Laufen spricht, beschreibt
  eine andere Sprache.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-01-generische-typen/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `03-01 struct`, `03-02 Methoden` and `03-05 Option und if
  let`.
- Building on this unit: `05-02 Traits` and `05-03 Trait Bounds`, where the free
  `T` becomes a bounded one.
- Cite like this when answering: `05-01 Generische Typen`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A free `T` can do almost nothing. Whoever solves an exercise here with `>` or
  `+` needs a bound, and that stands in `05-03`. The exercises of this unit do
  without one, and that is deliberate.
- There is no cost at run time. The compiler writes a version of its own per
  type used; whoever talks about a check while running describes a different
  language.

</details>

## Deutsch

### Worum es geht

Ein Typparameter ist ein Platzhalter für einen Typ. `fn erstes<T>(werte: &[T])`
sagt: diese Funktion arbeitet auf einer Liste von irgendetwas, und welches
Irgendetwas es ist, steht beim Aufruf fest.

Dasselbe geht an einem `struct` und an einem `enum`. `Option<T>` aus `03-05` ist
genau das, und `Result<T, E>` aus `04-07` hat zwei davon.

Beim Übersetzen bleibt davon nichts übrig. Der Übersetzer schreibt für jeden
Typ, mit dem die Funktion wirklich benutzt wird, eine eigene Fassung. Das
Programm läuft danach so schnell wie eines, in dem beide Fassungen von Hand
dastünden.

### Wofür das gut ist

Ohne Typparameter steht dieselbe Funktion mehrmals da, einmal je Typ, und wer
sie ändert, muss alle finden. Mit ihnen steht sie einmal da.

Der zweite Nutzen ist, dass der Übersetzer trotzdem alles prüft. Ein
generischer Code ist kein ungeprüfter Code: was in einer Fassung nicht passt,
fällt beim Übersetzen auf, und zwar an der Stelle, an der die Fassung entsteht.

Der Preis ist, dass ein freies `T` fast nichts kann. Es lässt sich weitergeben,
ausleihen und zurückgeben, aber nicht vergleichen und nicht addieren, denn nicht
jeder Typ kann das. Was fehlt, sind Schranken, und die stehen in `05-03`.

### Die Erklärung

Ein Typparameter an einer Funktion und einer an einem `struct`.

```rust
// Deutsch: Ein Typparameter an einer Funktion. `T` steht für einen Typ, der
// erst beim Aufruf feststeht.
fn erstes<T>(werte: &[T]) -> Option<&T> {
    werte.first()
}

// Deutsch: Dasselbe an einem struct. Beide Felder haben denselben Typ.
struct Paar<T> {
    links: T,
    rechts: T,
}

impl<T> Paar<T> {
    fn neu(links: T, rechts: T) -> Self {
        Paar { links, rechts }
    }
}

fn main() {
    // Deutsch: Eine Funktion, zwei Typen. Der Übersetzer schreibt sich für
    // jeden benutzten Typ eine eigene Fassung.
    println!("{:?}", erstes(&[3, 9, 4]));
    println!("{:?}", erstes(&["drei", "neun"]));

    let zahlen = Paar::neu(3, 9);
    let texte = Paar::neu(String::from("links"), String::from("rechts"));

    println!("{} {}", zahlen.links, zahlen.rechts);
    println!("{} {}", texte.links, texte.rechts);
}
```

Das Programm gibt aus:

```text
Some(3)
Some("drei")
3 9
links rechts
```

Am `impl` steht das `T` zweimal: einmal hinter `impl`, wo es eingeführt wird,
und einmal hinter dem Typnamen, wo es benutzt wird. Ohne das erste wäre `T` ein
Typ, den es gar nicht gibt.

### Häufige Fehler

Mit einem freien `T` rechnen wollen.

```rust
fn groesster<T>(werte: &[T]) -> &T {
    let mut groesster = &werte[0];

    for wert in werte {
        if wert > groesster {
            groesster = wert;
        }
    }

    groesster
}

fn main() {
    println!("{}", groesster(&[3, 9, 4]));
}
```

Der Übersetzer sagt dazu:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> generisch.rs:5:17
  |
5 |         if wert > groesster {
  |            ---- ^ --------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn groesster<T: std::cmp::PartialOrd>(werte: &[T]) -> &T {
  |               ++++++++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

`T` steht für jeden Typ, und nicht jeder Typ lässt sich vergleichen. Der
Übersetzer weiß beim Lesen der Funktion noch nicht, welcher Typ kommt, und
deshalb lehnt er den Vergleich ab, statt ihn erst beim Aufruf zu prüfen.

Der Vorschlag in der Meldung ist die Antwort und heißt Trait Bound. Was das
genau ist, steht in `05-02` und `05-03`; hier reicht der Satz, dass `T` erst
etwas können muss, bevor man es benutzen darf.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Jede Aufgabe wird mit mindestens zwei verschiedenen Typen
geprüft, und keine braucht eine Schranke.

- `last_of` gibt den letzten Wert einer Liste zurück
- `Paar::new` legt ein Paar an
- `swapped` vertauscht die beiden Seiten eines Paars

```console
cd units/05-01-generische-typen
cargo test
```

### Quelle

    Buch, Kapitel 10 "Generic Types, Traits, and Lifetimes", Abschnitt 10.1 "Generic Data Types",
    https://doc.rust-lang.org/book/ch10-01-syntax.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A type parameter is a placeholder for a type. `fn erstes<T>(werte: &[T])` says:
this function works on a list of something, and which something it is stands
fixed at the call.

The same works on a `struct` and on an `enum`. `Option<T>` from `03-05` is
exactly that, and `Result<T, E>` from `04-07` has two of them.

At compile time nothing of it is left. The compiler writes a version of its own
for every type the function is really used with. The program afterwards runs as
fast as one in which both versions stood by hand.

### What it is good for

Without type parameters the same function stands there several times, once per
type, and whoever changes it has to find them all. With them it stands there
once.

The second use is that the compiler checks everything all the same. Generic code
is not unchecked code: what does not fit in one version shows up at compile
time, at the place where that version comes into being.

The price is that a free `T` can do almost nothing. It can be handed on, lent
and returned, but not compared and not added, because not every type can do
that. What is missing are bounds, and those stand in `05-03`.

### The explanation

One type parameter on a function and one on a `struct`.

```rust
// Deutsch: Ein Typparameter an einer Funktion. `T` steht für einen Typ, der
// erst beim Aufruf feststeht.
fn erstes<T>(werte: &[T]) -> Option<&T> {
    werte.first()
}

// Deutsch: Dasselbe an einem struct. Beide Felder haben denselben Typ.
struct Paar<T> {
    links: T,
    rechts: T,
}

impl<T> Paar<T> {
    fn neu(links: T, rechts: T) -> Self {
        Paar { links, rechts }
    }
}

fn main() {
    // Deutsch: Eine Funktion, zwei Typen. Der Übersetzer schreibt sich für
    // jeden benutzten Typ eine eigene Fassung.
    println!("{:?}", erstes(&[3, 9, 4]));
    println!("{:?}", erstes(&["drei", "neun"]));

    let zahlen = Paar::neu(3, 9);
    let texte = Paar::neu(String::from("links"), String::from("rechts"));

    println!("{} {}", zahlen.links, zahlen.rechts);
    println!("{} {}", texte.links, texte.rechts);
}
```

The program prints:

```text
Some(3)
Some("drei")
3 9
links rechts
```

On the `impl` the `T` stands twice: once behind `impl`, where it is introduced,
and once behind the type name, where it is used. Without the first one `T` would
be a type that does not exist at all.

### Common mistakes

Wanting to compute with a free `T`.

```rust
fn groesster<T>(werte: &[T]) -> &T {
    let mut groesster = &werte[0];

    for wert in werte {
        if wert > groesster {
            groesster = wert;
        }
    }

    groesster
}

fn main() {
    println!("{}", groesster(&[3, 9, 4]));
}
```

The compiler answers:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> generisch.rs:5:17
  |
5 |         if wert > groesster {
  |            ---- ^ --------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn groesster<T: std::cmp::PartialOrd>(werte: &[T]) -> &T {
  |               ++++++++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

`T` stands for every type, and not every type can be compared. While reading the
function the compiler does not yet know which type will come, and that is why it
refuses the comparison instead of checking it only at the call.

The suggestion in the message is the answer and is called a trait bound. What
that is exactly stands in `05-02` and `05-03`; here the sentence is enough that
`T` first has to be able to do something before it may be used.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Every exercise is checked with at least two
different types, and none of them needs a bound.

- `last_of` returns the last value of a list
- `Paar::new` creates a pair
- `swapped` swaps the two sides of a pair

```console
cd units/05-01-generische-typen
cargo test
```

### Source

    Book, chapter 10 "Generic Types, Traits, and Lifetimes", section 10.1 "Generic Data Types",
    https://doc.rust-lang.org/book/ch10-01-syntax.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
