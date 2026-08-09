# 09-05 Fortgeschrittene Typen / Advanced types

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/09-05-fortgeschrittene-typen/`. Sie ist öffentlich. Wer nach ihr
  gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung erklären,
  um die es geht.
- Diese Einheit baut auf: `05-01 Generische Typen` und `04-07 panic! und
  Result`. Ein Alias steht über einem Typ, den es schon gibt, und das
  Ausrufezeichen begegnet einem zuerst beim Abbruch.
- Auf dieser Einheit bauen auf: der Rest der Stufe 9 und jede Signatur, in der
  `?Sized` oder `!` steht.
- Beim Antworten so zitieren: `09-05 Fortgeschrittene Typen`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein Typalias ist kein neuer Typ. Wer Verwechslungen ausschließen will, braucht
  ein eigenes Muster dafür, und dieses hier ist es nicht.
- `!` hat keinen Wert. Ein Arm mit `!` passt deshalb neben jeden anderen Arm,
  und nicht, weil es eine Sonderregel für `panic!` gäbe.
- `?Sized` lockert eine Forderung, es fügt keine hinzu. Wo nichts steht, steht
  `Sized` still da.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-05-fortgeschrittene-typen/`. It
  is public. Whoever is asked for it may name it, but should explain the
  compiler message in question first.
- This unit builds on: `05-01 Generische Typen` and `04-07 panic! und Result`.
  An alias stands over a type that already exists, and the exclamation mark is
  first met at an abort.
- Building on this unit: the rest of stage 9 and every signature carrying
  `?Sized` or `!`.
- Cite like this when answering: `09-05 Fortgeschrittene Typen`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A type alias is not a new type. Whoever wants to rule out a mix-up needs a
  pattern of its own for that, and this is not it.
- `!` has no value. An arm of type `!` therefore fits next to every other arm,
  and not because there were a special rule for `panic!`.
- `?Sized` loosens a requirement, it adds none. Where nothing is written,
  `Sized` stands there silently.

</details>

## Deutsch

### Worum es geht

Drei Dinge, die keine neue Schreibweise sind, sondern drei Typen, die man in
einer Signatur wiedererkennen muss.

Ein Typalias gibt einem Typ, den es schon gibt, einen zweiten Namen.
`type Ergebnis<T> = Result<T, String>;` schreibt kein neues `Result`, sondern
eine Abkürzung dafür. Beide Namen meinen dasselbe, und der Übersetzer sieht
keinen Unterschied.

`!` ist der Typ, zu dem es keinen Wert gibt. Er steht am Ausgang einer Funktion,
die nicht zurückkehrt, also bei `panic!`, bei `process::exit` und bei einer
Schleife ohne Ende. Weil es keinen Wert dieses Typs gibt, kann er überall
einspringen, wo ein Wert erwartet wird.

Ein Typ ohne feste Größe ist einer, dessen Umfang erst zur Laufzeit feststeht.
`str` und `[i32]` sind solche Typen. Deshalb begegnen sie einem fast immer
hinter einem `&`, denn eine Referenz hat eine feste Größe, auch wenn das, worauf
sie zeigt, keine hat.

### Wofür das gut ist

Ein Alias spart nicht Tipparbeit, sondern hält eine lange Angabe an einer
Stelle. Wenn aus dem Fehlertyp später etwas anderes wird, ändert sich die eine
Zeile und nicht jede Signatur. Der Preis steht daneben: Weil der Alias kein
neuer Typ ist, verwechselt der Übersetzer die beiden Namen nicht nur, er darf
sie gar nicht auseinanderhalten.

`!` ist der Grund, warum `panic!` mitten in einem `match` stehen darf. Ein Arm
muss denselben Typ liefern wie die anderen, und ein Arm, der nie etwas liefert,
verletzt diese Regel nicht. Ohne diesen Typ bräuchte jede solche Stelle eine
Sonderregel.

`?Sized` macht aus einer Funktion für einen Typ eine Funktion für viele. Ohne
die Angabe verlangt der Übersetzer stillschweigend eine feste Größe, und dann
passt `str` nicht mehr durch. Aufgabe 3 dieser Einheit lebt von der anderen
Seite derselben Sache: `[i32]` hat keine feste Größe, also steht in der
Signatur `&[i32]` und nicht `[i32]`.

### Die Erklärung

Alle drei in einem Programm.

```rust
use std::fmt::Debug;

// Deutsch: Ein Typalias gibt einem Typ einen zweiten Namen. Ein neuer Typ wird
// daraus nicht.
type Ergebnis<T> = Result<T, String>;

fn zahl(text: &str) -> Ergebnis<u32> {
    text.parse::<u32>().map_err(|_| format!("keine Zahl: {text}"))
}

// Deutsch: Das Ausrufezeichen ist der Typ, zu dem es keinen Wert gibt. Die
// Funktion kehrt nicht zurück.
fn abbruch(grund: &str) -> ! {
    panic!("Abbruch: {grund}");
}

// Deutsch: `?Sized` nimmt die Forderung nach fester Größe zurück, deshalb darf
// hier auch `str` oder `[i32]` stehen.
fn beschreibe<T: Debug + ?Sized>(wert: &T) -> String {
    format!("{wert:?}")
}

fn main() {
    // Deutsch: Derselbe Typ, einmal über den Alias und einmal ausgeschrieben.
    let ueber_alias: Ergebnis<u32> = zahl("12");
    let ausgeschrieben: Result<u32, String> = ueber_alias;

    println!("{ausgeschrieben:?}");
    println!("{:?}", zahl("zwoelf"));

    println!("{}", beschreibe("hallo"));
    println!("{}", beschreibe(&[1, 2, 3][..]));

    // Deutsch: Der Arm mit dem Abbruch hat den Typ `!` und passt deshalb neben
    // einen Arm, der `u32` liefert.
    let wert = match zahl("7") {
        Ok(gelesen) => gelesen,
        Err(grund) => abbruch(&grund),
    };

    println!("{wert}");
}
```

Das Programm gibt aus:

```text
Ok(12)
Err("keine Zahl: zwoelf")
"hallo"
[1, 2, 3]
7
```

Die zweite Zeile im Rumpf von `main` ist die, an der der Alias sich zeigt.
`ueber_alias` ist als `Ergebnis<u32>` angeschrieben und wird einer Variablen vom
Typ `Result<u32, String>` zugewiesen, ohne dass etwas umgewandelt wird. Es gibt
nichts umzuwandeln, denn es ist derselbe Typ.

Der Arm mit `abbruch` liefert `!`. Der andere Arm liefert `u32`. Dass das
zusammenpasst, liegt daran, dass es keinen Wert vom Typ `!` gibt, den man
zurückgeben könnte, und deshalb nichts, was zum `u32` in Widerspruch stünde.

`beschreibe` wird zweimal mit etwas ohne feste Größe aufgerufen, mit `str` und
mit `[i32]`. Beide Male steht ein `&` davor. Ohne `?Sized` in der Schranke
verlangte der Übersetzer eine feste Größe, und beide Aufrufe wären zurück
gewiesen.

### Häufige Fehler

Einen Typ ohne feste Größe in eine Signatur schreiben, wo eine Referenz darauf
hingehört.

```rust
fn laenge(text: str) -> usize {
    text.len()
}

fn main() {
    let wort = "hallo";

    println!("{}", laenge(wort));
}
```

Der Übersetzer sagt dazu:

```text
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> ohne-groesse.rs:1:17
  |
1 | fn laenge(text: str) -> usize {
  |                 ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
1 | fn laenge(text: &str) -> usize {
  |                 +

error[E0308]: mismatched types
 --> ohne-groesse.rs:8:27
  |
8 |     println!("{}", laenge(wort));
  |                    ------ ^^^^ expected `str`, found `&str`
  |                    |
  |                    arguments to this function are incorrect
  |
note: function defined here
 --> ohne-groesse.rs:1:4
  |
1 | fn laenge(text: str) -> usize {
  |    ^^^^^^ ---------

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> ohne-groesse.rs:8:27
  |
8 |     println!("{}", laenge(wort));
  |                           ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: all function arguments must have a statically known size

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

Ein Fehler, drei Meldungen. Die erste nennt die Ursache und schreibt die
Antwort gleich hin: ein `&` vor `str`. Die beiden anderen sind Folgen davon,
denn am Aufruf steht jetzt ein `&str`, wo die Signatur `str` verlangt. Mit dem
`&` in der Signatur verschwinden alle drei.

Die Zeile über `Sized` ist die eigentliche Auskunft. `Sized` wird ohne Zutun
verlangt, und `str` erfüllt es nicht. Ein Alias hilft hier nicht, denn er ändert
den Typ ja nicht.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Der Alias `Ergebnis`, `beschreibe` und `zahl_oder_abbruch`
stehen fertig da, und ihre Doku-Tests sind grün.

- `zusammen` setzt zwei Teile zusammen und gibt ein `Ergebnis<String>` zurück
- `abbruch` kehrt nicht zurück und trägt deshalb `!` als Rückgabetyp
- `erstes_und_letztes` nimmt einen Ausschnitt und gibt das erste und das letzte
  Element heraus

```console
cd units/09-05-fortgeschrittene-typen
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.3 "Advanced Types",
    https://doc.rust-lang.org/book/ch20-03-advanced-types.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Three things that are not a new spelling but three types you have to recognise
again in a signature.

A type alias gives a type that already exists a second name.
`type Ergebnis<T> = Result<T, String>;` writes no new `Result` but an
abbreviation for it. Both names mean the same, and the compiler sees no
difference.

`!` is the type that has no value. It stands at the exit of a function that does
not return, so at `panic!`, at `process::exit` and at a loop without an end.
Because there is no value of this type, it can step in everywhere a value is
expected.

A type without a fixed size is one whose extent is settled only while the
program runs. `str` and `[i32]` are such types. That is why you meet them almost
always behind a `&`, because a reference has a fixed size even when the thing it
points at has none.

### What it is good for

An alias saves no typing, it holds a long piece of writing in one place. When
the error type becomes something else later, the one line changes and not every
signature. The price stands next to it: because the alias is not a new type, the
compiler does not merely confuse the two names, it is not allowed to tell them
apart at all.

`!` is the reason why `panic!` may stand in the middle of a `match`. An arm has
to deliver the same type as the others, and an arm that never delivers anything
does not break that rule. Without this type every such place would need a
special rule.

`?Sized` turns a function for one type into a function for many. Without the
note the compiler silently asks for a fixed size, and then `str` no longer fits
through. Exercise 3 of this unit lives off the other side of the same thing:
`[i32]` has no fixed size, so the signature carries `&[i32]` and not `[i32]`.

### The explanation

All three in one program.

```rust
use std::fmt::Debug;

// Deutsch: Ein Typalias gibt einem Typ einen zweiten Namen. Ein neuer Typ wird
// daraus nicht.
type Ergebnis<T> = Result<T, String>;

fn zahl(text: &str) -> Ergebnis<u32> {
    text.parse::<u32>().map_err(|_| format!("keine Zahl: {text}"))
}

// Deutsch: Das Ausrufezeichen ist der Typ, zu dem es keinen Wert gibt. Die
// Funktion kehrt nicht zurück.
fn abbruch(grund: &str) -> ! {
    panic!("Abbruch: {grund}");
}

// Deutsch: `?Sized` nimmt die Forderung nach fester Größe zurück, deshalb darf
// hier auch `str` oder `[i32]` stehen.
fn beschreibe<T: Debug + ?Sized>(wert: &T) -> String {
    format!("{wert:?}")
}

fn main() {
    // Deutsch: Derselbe Typ, einmal über den Alias und einmal ausgeschrieben.
    let ueber_alias: Ergebnis<u32> = zahl("12");
    let ausgeschrieben: Result<u32, String> = ueber_alias;

    println!("{ausgeschrieben:?}");
    println!("{:?}", zahl("zwoelf"));

    println!("{}", beschreibe("hallo"));
    println!("{}", beschreibe(&[1, 2, 3][..]));

    // Deutsch: Der Arm mit dem Abbruch hat den Typ `!` und passt deshalb neben
    // einen Arm, der `u32` liefert.
    let wert = match zahl("7") {
        Ok(gelesen) => gelesen,
        Err(grund) => abbruch(&grund),
    };

    println!("{wert}");
}
```

The program prints:

```text
Ok(12)
Err("keine Zahl: zwoelf")
"hallo"
[1, 2, 3]
7
```

The second line in the body of `main` is where the alias shows itself.
`ueber_alias` is written down as `Ergebnis<u32>` and is assigned to a variable
of type `Result<u32, String>` without anything being converted. There is nothing
to convert, because it is the same type.

The arm with `abbruch` delivers `!`. The other arm delivers `u32`. That the two
fit together comes from there being no value of type `!` that could be returned,
and therefore nothing that would contradict the `u32`.

`beschreibe` is called twice with something without a fixed size, with `str` and
with `[i32]`. Both times a `&` stands in front. Without `?Sized` in the bound the
compiler would ask for a fixed size, and both calls would be refused.

### Common mistakes

Writing a type without a fixed size into a signature where a reference to it
belongs.

```rust
fn laenge(text: str) -> usize {
    text.len()
}

fn main() {
    let wort = "hallo";

    println!("{}", laenge(wort));
}
```

The compiler answers:

```text
error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> ohne-groesse.rs:1:17
  |
1 | fn laenge(text: str) -> usize {
  |                 ^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
help: function arguments must have a statically known size, borrowed types always have a known size
  |
1 | fn laenge(text: &str) -> usize {
  |                 +

error[E0308]: mismatched types
 --> ohne-groesse.rs:8:27
  |
8 |     println!("{}", laenge(wort));
  |                    ------ ^^^^ expected `str`, found `&str`
  |                    |
  |                    arguments to this function are incorrect
  |
note: function defined here
 --> ohne-groesse.rs:1:4
  |
1 | fn laenge(text: str) -> usize {
  |    ^^^^^^ ---------

error[E0277]: the size for values of type `str` cannot be known at compilation time
 --> ohne-groesse.rs:8:27
  |
8 |     println!("{}", laenge(wort));
  |                           ^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `str`
  = note: all function arguments must have a statically known size

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
```

One mistake, three messages. The first names the cause and writes the answer
down right away: a `&` in front of `str`. The other two follow from it, because
at the call a `&str` now stands where the signature asks for `str`. With the `&`
in the signature all three go away.

The line about `Sized` is the real piece of information. `Sized` is asked for
without anybody writing it, and `str` does not meet it. An alias does not help
here, since it does not change the type.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The alias `Ergebnis`, `beschreibe` and
`zahl_oder_abbruch` stand there finished, and their doc tests are green.

- `zusammen` puts two parts together and returns an `Ergebnis<String>`
- `abbruch` does not return and therefore carries `!` as its return type
- `erstes_und_letztes` takes a slice and hands the first and the last element
  out

```console
cd units/09-05-fortgeschrittene-typen
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.3 "Advanced Types",
    https://doc.rust-lang.org/book/ch20-03-advanced-types.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
