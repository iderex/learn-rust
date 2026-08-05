# 05-04 Lifetimes / Lifetimes

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-04-lifetimes/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `02-03 Ausleihen` und `05-03 Trait Bounds`. Die
  Referenz kommt aus der Stufe 2, der Name im Kopf aus dieser Stufe.
- Auf dieser Einheit bauen auf: `05-05 Tests und ihr Aufbau` und alles, was eine
  Referenz zurückgibt oder in einem struct hält.
- Beim Antworten so zitieren: `05-04 Lifetimes`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Eine Lebensdauer ist ein Name und keine Angabe, wie lange etwas lebt. Sie
  verlängert nichts, sie schreibt nur auf, was ohnehin schon gilt.
- Dass der Übersetzer die Anmerkung oft selbst einsetzt, heißt nicht, dass es
  sie dort nicht gibt. Wer einen Unterschied zwischen beiden Fällen behauptet,
  sagt bitte, welchen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-04-lifetimes/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `02-03 Ausleihen` and `05-03 Trait Bounds`. The reference
  comes from stage 2, the name in the head from this stage.
- Building on this unit: `05-05 Tests und ihr Aufbau` and everything that
  returns a reference or holds one in a struct.
- Cite like this when answering: `05-04 Lifetimes`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A lifetime is a name and not a statement of how long something lives. It
  lengthens nothing, it only writes down what already holds anyway.
- That the compiler often fills the annotation in itself does not mean it is not
  there. Whoever claims a difference between the two cases, please say which
  one.

</details>

## Deutsch

### Worum es geht

Eine Referenz zeigt auf etwas, das jemand anderem gehört. Damit sie nie auf
etwas zeigt, das es nicht mehr gibt, muss der Übersetzer wissen, woher sie
kommt.

Bei einer Funktion, die eine Referenz zurückgibt, ist das nicht immer aus der
Signatur ablesbar. `'a` ist ein Name für diese Herkunft. Steht er an zwei
Eingaben und an der Ausgabe, sagt der Kopf: das Ergebnis stammt aus einer der
beiden und lebt nicht länger als die kürzere.

Der Name ist eine Behauptung über das, was ohnehin gilt, und keine Anweisung.
Er verlängert nichts. Er schreibt nur auf, was der Rumpf sowieso tut, damit die
Aufrufstelle es lesen kann.

### Wofür das gut ist

Ohne den Namen bleibt eine Frage offen, und der Übersetzer stellt sie mit
`E0106`: gehört die zurückgegebene Referenz zur ersten Eingabe oder zur
zweiten? Raten darf er nicht, denn an der Antwort hängt, wie lange der Aufrufer
das Ergebnis halten darf.

In den einfachen Fällen fragt er gar nicht erst. Gibt es genau eine
Eingabereferenz, bekommt die Ausgabe deren Lebensdauer, und die Anmerkung
entfällt. Deshalb sind in den Einheiten davor kaum welche vorgekommen.

Ein struct, das eine Referenz hält, braucht den Namen immer. Sonst könnte der
Auszug den Text überleben, aus dem er stammt, und genau das ist die Sorte
Fehler, gegen die Rust gebaut ist.

### Die Erklärung

Ein Name an zwei Eingaben, ein Fall ohne Anmerkung, und ein struct, das eine
Referenz hält.

```rust
// Deutsch: `'a` ist ein Name für eine Lebensdauer. Der Kopf sagt damit: das
// Ergebnis lebt so lange wie die kürzere der beiden Eingaben.
fn laengster<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if links.len() > rechts.len() {
        links
    } else {
        rechts
    }
}

// Deutsch: Hier steht keine Anmerkung, und der Übersetzer setzt sie selbst
// ein. Bei genau einer Eingabereferenz bekommt die Ausgabe deren Lebensdauer.
fn erstes_wort(satz: &str) -> &str {
    match satz.find(' ') {
        Some(stelle) => &satz[..stelle],
        None => satz,
    }
}

// Deutsch: Ein struct, das eine Referenz hält, nennt ihre Lebensdauer. Damit
// kann ein Auszug den Text nicht überleben, aus dem er stammt.
struct Auszug<'a> {
    teil: &'a str,
}

impl<'a> Auszug<'a> {
    fn erster_satz(text: &'a str) -> Auszug<'a> {
        let ende = text.find('.').unwrap_or(text.len());
        Auszug {
            teil: &text[..ende],
        }
    }
}

fn main() {
    let satz = String::from("Lebensdauern sind Namen. Mehr sind sie nicht.");

    println!("{}", laengster("kurz", "laenger"));
    println!("{}", erstes_wort(&satz));
    println!("{}", Auszug::erster_satz(&satz).teil);
}
```

Das Programm gibt aus:

```text
laenger
Lebensdauern
Lebensdauern sind Namen
```

Die zweite Funktion zeigt, warum die Anmerkung selten dasteht. Der Übersetzer
setzt sie in den einfachen Fällen selbst ein, und der einfachste ist genau
dieser: eine Eingabereferenz, eine Ausgabe.

### Häufige Fehler

Eine Referenz zurückgeben und nicht sagen, aus welcher Eingabe sie stammt.

```rust
fn laengster(links: &str, rechts: &str) -> &str {
    if links.len() > rechts.len() {
        links
    } else {
        rechts
    }
}

fn main() {
    println!("{}", laengster("kurz", "laenger"));
}
```

Der Übersetzer sagt dazu:

```text
error[E0106]: missing lifetime specifier
 --> laengster.rs:1:44
  |
1 | fn laengster(links: &str, rechts: &str) -> &str {
  |                     ----          ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `links` or `rechts`
help: consider introducing a named lifetime parameter
  |
1 | fn laengster<'a>(links: &'a str, rechts: &'a str) -> &'a str {
  |             ++++         ++               ++          ++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0106`.
```

Die Meldung sagt genau, was fehlt: die Signatur sagt nicht, ob die Rückgabe von
`links` oder von `rechts` geliehen ist. Der Vorschlag darunter ist die Antwort,
und er ist es fast immer.

Die Antwort ist nicht, den Rückgabetyp auf `String` zu ändern und damit zu
kopieren. Das ginge, kostet aber eine Kopie für eine Frage, die eine Anmerkung
beantwortet.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Die Namen stehen schon in den Köpfen, wo sie hingehören.

- `longest` gibt den längeren von zwei Texten zurück, mit `'a` an beiden Seiten
- `first_word` gibt das erste Wort zurück, ganz ohne Anmerkung
- `Excerpt::first_sentence` nimmt den ersten Satz als Auszug, mit `'a` am struct

```console
cd units/05-04-lifetimes
cargo test
```

### Quelle

    Buch, Kapitel 10 "Generic Types, Traits, and Lifetimes", Abschnitt 10.3 "Validating References with Lifetimes",
    https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A reference points at something that belongs to somebody else. So that it never
points at something that no longer exists, the compiler has to know where it
comes from.

For a function returning a reference that cannot always be read off the
signature. `'a` is a name for that origin. Standing on two inputs and on the
output, the head says: the result comes from one of the two and lives no longer
than the shorter one.

The name is a claim about what holds anyway, and not an instruction. It
lengthens nothing. It only writes down what the body does regardless, so that
the call site can read it.

### What it is good for

Without the name a question stays open, and the compiler asks it with `E0106`:
does the returned reference belong to the first input or to the second? It may
not guess, because how long the caller may hold the result hangs on the answer.

In the simple cases it does not ask at all. Where there is exactly one input
reference the output gets its lifetime, and the annotation falls away. That is
why hardly any have turned up in the units before this one.

A struct holding a reference always needs the name. Otherwise the excerpt could
outlive the text it comes from, and that is exactly the kind of mistake Rust is
built against.

### The explanation

One name on two inputs, one case without an annotation, and a struct holding a
reference.

```rust
// Deutsch: `'a` ist ein Name für eine Lebensdauer. Der Kopf sagt damit: das
// Ergebnis lebt so lange wie die kürzere der beiden Eingaben.
fn laengster<'a>(links: &'a str, rechts: &'a str) -> &'a str {
    if links.len() > rechts.len() {
        links
    } else {
        rechts
    }
}

// Deutsch: Hier steht keine Anmerkung, und der Übersetzer setzt sie selbst
// ein. Bei genau einer Eingabereferenz bekommt die Ausgabe deren Lebensdauer.
fn erstes_wort(satz: &str) -> &str {
    match satz.find(' ') {
        Some(stelle) => &satz[..stelle],
        None => satz,
    }
}

// Deutsch: Ein struct, das eine Referenz hält, nennt ihre Lebensdauer. Damit
// kann ein Auszug den Text nicht überleben, aus dem er stammt.
struct Auszug<'a> {
    teil: &'a str,
}

impl<'a> Auszug<'a> {
    fn erster_satz(text: &'a str) -> Auszug<'a> {
        let ende = text.find('.').unwrap_or(text.len());
        Auszug {
            teil: &text[..ende],
        }
    }
}

fn main() {
    let satz = String::from("Lebensdauern sind Namen. Mehr sind sie nicht.");

    println!("{}", laengster("kurz", "laenger"));
    println!("{}", erstes_wort(&satz));
    println!("{}", Auszug::erster_satz(&satz).teil);
}
```

The program prints:

```text
laenger
Lebensdauern
Lebensdauern sind Namen
```

The second function shows why the annotation is rarely there. The compiler
fills it in itself in the simple cases, and the simplest one is exactly this:
one input reference, one output.

### Common mistakes

Returning a reference and not saying which input it comes from.

```rust
fn laengster(links: &str, rechts: &str) -> &str {
    if links.len() > rechts.len() {
        links
    } else {
        rechts
    }
}

fn main() {
    println!("{}", laengster("kurz", "laenger"));
}
```

The compiler answers:

```text
error[E0106]: missing lifetime specifier
 --> laengster.rs:1:44
  |
1 | fn laengster(links: &str, rechts: &str) -> &str {
  |                     ----          ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `links` or `rechts`
help: consider introducing a named lifetime parameter
  |
1 | fn laengster<'a>(links: &'a str, rechts: &'a str) -> &'a str {
  |             ++++         ++               ++          ++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0106`.
```

The message says exactly what is missing: the signature does not say whether the
return is borrowed from `links` or from `rechts`. The suggestion below it is the
answer, and it almost always is.

The answer is not to change the return type to `String` and copy instead. That
would work, but it costs a copy for a question that an annotation answers.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The names already stand in the heads, where
they belong.

- `longest` returns the longer of two texts, with `'a` on both sides
- `first_word` returns the first word, with no annotation at all
- `Excerpt::first_sentence` takes the first sentence as an excerpt, with `'a` on
  the struct

```console
cd units/05-04-lifetimes
cargo test
```

### Source

    Book, chapter 10 "Generic Types, Traits, and Lifetimes", section 10.3 "Validating References with Lifetimes",
    https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
