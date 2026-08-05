# 04-08 From, Into und der Operator ? / From, Into and the ? operator

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/04-08-from-into-und-fragezeichen/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `04-07 panic! und Result`.
- Auf dieser Einheit bauen auf: `04-09 TryFrom und ein eigener Fehlertyp` und
  `04-10 std::error::Error und Box<dyn Error>`.
- Beim Antworten so zitieren: `04-08 From, Into und der Operator ?`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `?` ist kein Werfen. Es gibt sofort zurück, und der Rückgabetyp der Funktion
  steht unverändert im Kopf. Wer es als Ausnahme beschreibt, beschreibt eine
  andere Sprache.
- `Into` wird hier nicht von Hand geschrieben. Wer `From` schreibt, bekommt
  `into` dazu, und ein eigenes `impl Into` ist der Fehler, den die Einheit
  vermeidet.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/04-08-from-into-und-fragezeichen/`. It is public. Whoever is asked
  for it may name it, but should explain the compiler message in question first.
- This unit builds on: `04-07 panic! und Result`.
- Building on this unit: `04-09 TryFrom und ein eigener Fehlertyp` and
  `04-10 std::error::Error und Box<dyn Error>`.
- Cite like this when answering: `04-08 From, Into und der Operator ?`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `?` is not a throw. It returns at once, and the return type of the function
  stands unchanged in its head. Whoever describes it as an exception describes a
  different language.
- `Into` is not written by hand here. Whoever writes `From` gets `into` along
  with it, and an `impl Into` of one's own is the mistake this unit avoids.

</details>

## Deutsch

### Worum es geht

`?` steht hinter einem `Result`. Ist es `Ok`, geht es mit dem Wert weiter. Ist
es `Err`, gibt die Funktion sofort zurück, und zwar mit diesem Fehler.

Dabei wandelt `?` den Fehler um. Es ruft `From` auf, damit der Fehler der
aufgerufenen Funktion zum Fehlertyp der eigenen wird.

`From` schreibt man selbst: ein `impl From<A> for B` mit einer Funktion `from`.
`Into` schreibt man nicht, man bekommt es geschenkt, sobald `From` dasteht.

### Wofür das gut ist

Ohne `?` steht in jeder Funktion, die zwei fehlbare Aufrufe macht, zweimal
dasselbe `match`, dessen Fehlerzweig den Fehler nur weiterreicht. Das ist
Buchführung, und sie verdeckt, was die Funktion eigentlich tut.

Die Umwandlung über `From` ist der Teil, der es zusammenhält. Eine Funktion darf
einen eigenen Fehlertyp haben, obwohl sie fremde Funktionen mit fremden
Fehlertypen ruft, und die Übersetzung steht an einer Stelle statt an jeder
Aufrufstelle.

Dass `Into` von `From` kommt, spart die zweite Hälfte. Wer beide schreibt,
schreibt dieselbe Umwandlung zweimal, und die zweite kann veralten.

### Die Erklärung

Zwei Fehlertypen, eine Umwandlung, und `?` an zwei Stellen.

```rust
#[derive(Debug)]
enum EingabeFehler {
    KeineZahl,
}

#[derive(Debug)]
enum Fehler {
    Eingabe(EingabeFehler),
    DurchNull,
}

// Deutsch: `From` sagt, wie aus dem einen Fehler der andere wird. Wer sie
// schreibt, bekommt `into` dazu.
impl From<EingabeFehler> for Fehler {
    fn from(fehler: EingabeFehler) -> Self {
        Fehler::Eingabe(fehler)
    }
}

fn gelesen(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

fn geteilt(links: &str, rechts: &str) -> Result<i32, Fehler> {
    // Deutsch: `?` gibt den Fehler sofort zurück und wandelt ihn dabei mit
    // `From` um. Ohne das Fragezeichen stünde hier ein `match`.
    let a = gelesen(links)?;
    let b = gelesen(rechts)?;

    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a / b)
}

fn main() {
    println!("{:?}", geteilt("10", "2"));
    println!("{:?}", geteilt("zehn", "2"));
    println!("{:?}", geteilt("10", "0"));

    // Deutsch: `into` ist dieselbe Umwandlung von der anderen Seite gelesen.
    let umgewandelt: Fehler = EingabeFehler::KeineZahl.into();

    println!("{umgewandelt:?}");
}
```

Das Programm gibt aus:

```text
Ok(5)
Err(Eingabe(KeineZahl))
Err(DurchNull)
Eingabe(KeineZahl)
```

Die zweite Zeile ist der umgewandelte Fehler: hereingekommen ist ein
`EingabeFehler`, herausgekommen ist ein `Fehler`, und geschrieben hat das
niemand an der Aufrufstelle.

### Häufige Fehler

`?` benutzen, ohne die Umwandlung zu haben.

```rust
#[derive(Debug)]
enum EingabeFehler {
    KeineZahl,
}

#[derive(Debug)]
enum Fehler {
    DurchNull,
}

fn gelesen(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

fn verdoppelt(text: &str) -> Result<i32, Fehler> {
    let zahl = gelesen(text)?;

    if zahl == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(zahl * 2)
}

fn main() {
    println!("{:?}", verdoppelt("10"));
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: `?` couldn't convert the error to `Fehler`
  --> frage.rs:19:29
   |
18 | fn verdoppelt(text: &str) -> Result<i32, Fehler> {
   |                              ------------------- expected `Fehler` because of this
19 |     let zahl = gelesen(text)?;
   |                -------------^ the trait `From<EingabeFehler>` is not implemented for `Fehler`
   |                |
   |                this can't be annotated with `?` because it has type `Result<_, EingabeFehler>`
   |
note: `Fehler` needs to implement `From<EingabeFehler>`
  --> frage.rs:7:1
   |
 7 | enum Fehler {
   | ^^^^^^^^^^^
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

Die letzte Zeile der Meldung sagt genau, was `?` tut: es wandelt den Fehler mit
`From` um. Fehlt diese Umwandlung, fehlt dem Fragezeichen der Weg, und die
Meldung nennt das fehlende `impl` beim Namen.

Es ist wieder `E0277` wie in `03-06` und `03-07`, und wieder heißt es: dem Typ
fehlt etwas, das hier verlangt wird.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Die beiden Fehlertypen und `parsed` stehen schon da.

- `From<EingabeFehler> for Fehler` sagt, wie aus dem einen der andere wird
- `divided_text` liest zwei Texte und teilt sie, mit `?`
- `as_fehler` wandelt mit `into` um, ohne ein zweites `impl`

```console
cd units/04-08-from-into-und-fragezeichen
cargo test
```

### Quelle

    Buch, Kapitel 9 "Error Handling", Abschnitt 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    geprüft gegen 1.97.1

    Standardbibliothek, "From in std::convert",
    https://doc.rust-lang.org/std/convert/trait.From.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`?` stands behind a `Result`. If it is `Ok`, things carry on with the value. If
it is `Err`, the function returns at once, with that error.

While doing so `?` converts the error. It calls `From`, so that the error of the
function called becomes the error type of the calling one.

`From` is written by yourself: an `impl From<A> for B` with a function `from`.
`Into` is not written, it comes as a gift as soon as `From` stands there.

### What it is good for

Without `?` every function making two fallible calls holds the same `match`
twice, whose error arm only passes the error on. That is bookkeeping, and it
hides what the function actually does.

The conversion through `From` is the part that holds it together. A function may
have an error type of its own although it calls foreign functions with foreign
error types, and the translation stands in one place instead of at every call
site.

That `Into` comes from `From` saves the second half. Whoever writes both writes
the same conversion twice, and the second one can go stale.

### The explanation

Two error types, one conversion, and `?` in two places.

```rust
#[derive(Debug)]
enum EingabeFehler {
    KeineZahl,
}

#[derive(Debug)]
enum Fehler {
    Eingabe(EingabeFehler),
    DurchNull,
}

// Deutsch: `From` sagt, wie aus dem einen Fehler der andere wird. Wer sie
// schreibt, bekommt `into` dazu.
impl From<EingabeFehler> for Fehler {
    fn from(fehler: EingabeFehler) -> Self {
        Fehler::Eingabe(fehler)
    }
}

fn gelesen(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

fn geteilt(links: &str, rechts: &str) -> Result<i32, Fehler> {
    // Deutsch: `?` gibt den Fehler sofort zurück und wandelt ihn dabei mit
    // `From` um. Ohne das Fragezeichen stünde hier ein `match`.
    let a = gelesen(links)?;
    let b = gelesen(rechts)?;

    if b == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(a / b)
}

fn main() {
    println!("{:?}", geteilt("10", "2"));
    println!("{:?}", geteilt("zehn", "2"));
    println!("{:?}", geteilt("10", "0"));

    // Deutsch: `into` ist dieselbe Umwandlung von der anderen Seite gelesen.
    let umgewandelt: Fehler = EingabeFehler::KeineZahl.into();

    println!("{umgewandelt:?}");
}
```

The program prints:

```text
Ok(5)
Err(Eingabe(KeineZahl))
Err(DurchNull)
Eingabe(KeineZahl)
```

The second line is the converted error: what came in was an `EingabeFehler`,
what came out is a `Fehler`, and nobody wrote that at the call site.

### Common mistakes

Using `?` without having the conversion.

```rust
#[derive(Debug)]
enum EingabeFehler {
    KeineZahl,
}

#[derive(Debug)]
enum Fehler {
    DurchNull,
}

fn gelesen(text: &str) -> Result<i32, EingabeFehler> {
    match text.trim().parse::<i32>() {
        Ok(zahl) => Ok(zahl),
        Err(_) => Err(EingabeFehler::KeineZahl),
    }
}

fn verdoppelt(text: &str) -> Result<i32, Fehler> {
    let zahl = gelesen(text)?;

    if zahl == 0 {
        return Err(Fehler::DurchNull);
    }

    Ok(zahl * 2)
}

fn main() {
    println!("{:?}", verdoppelt("10"));
}
```

The compiler answers:

```text
error[E0277]: `?` couldn't convert the error to `Fehler`
  --> frage.rs:19:29
   |
18 | fn verdoppelt(text: &str) -> Result<i32, Fehler> {
   |                              ------------------- expected `Fehler` because of this
19 |     let zahl = gelesen(text)?;
   |                -------------^ the trait `From<EingabeFehler>` is not implemented for `Fehler`
   |                |
   |                this can't be annotated with `?` because it has type `Result<_, EingabeFehler>`
   |
note: `Fehler` needs to implement `From<EingabeFehler>`
  --> frage.rs:7:1
   |
 7 | enum Fehler {
   | ^^^^^^^^^^^
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0277`.
```

The last line of the message says exactly what `?` does: it converts the error
with `From`. Where that conversion is missing the question mark has no way, and
the message names the missing `impl`.

It is `E0277` again, as in `03-06` and `03-07`, and again it says: the type is
missing something that is asked of it here.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The two error types and `parsed` are already
there.

- `From<EingabeFehler> for Fehler` says how the one becomes the other
- `divided_text` reads two texts and divides them, with `?`
- `as_fehler` converts with `into`, without a second `impl`

```console
cd units/04-08-from-into-und-fragezeichen
cargo test
```

### Source

    Book, chapter 9 "Error Handling", section 9.2 "Recoverable Errors with Result",
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html,
    checked against 1.97.1

    Standard library, "From in std::convert",
    https://doc.rust-lang.org/std/convert/trait.From.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
