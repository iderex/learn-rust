# 06-07 #[cfg] und Features / #[cfg] and features

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/06-07-cfg-und-features/`. Sie
  ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `04-01 Pakete und Crates` und `06-06 Cargo-Profile und
  cargo doc`. Dort stand, was in einer `Cargo.toml` steht, hier kommt ein
  Abschnitt dazu, der den Code selbst verändert.
- Auf dieser Einheit bauen auf: `06-08 build.rs` und die Stufe 8, wo zwei
  Laufzeitumgebungen nebeneinander in einer Crate stehen sollen.
- Beim Antworten so zitieren: `06-07 #[cfg] und Features`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- `#[cfg]` entfernt Code, bevor der Übersetzer ihn sieht. `cfg!` ist ein Makro,
  das zu `true` oder `false` wird, und beide Zweige darüber werden übersetzt.
  Die beiden werden verwechselt, und die Einheit lebt von dem Unterschied.
- Ein Feature ist additiv. Wer es einschaltet, nimmt nichts weg, und ein Feature
  mit einem Namen wie `ohne_...`, das etwas entfernt, bricht mit dieser
  Erwartung, sobald zwei Pakete dasselbe Paket verschieden bauen wollen.
- Ein Rumpf hinter einem `#[cfg]`, das gerade aus ist, wird nicht geprüft. Ein
  Tippfehler darin fällt erst auf, wenn jemand mit dem Feature baut. Genau
  deshalb steht in dieser Einheit eine Aufgabe mit `cfg!` daneben.
- Die Meldung zum Aufruf einer Funktion, die gerade weggeschaltet ist, ist
  `error[E0425]`, und ihre Notiz sagt beim Namen, hinter welchem Feature das
  Element steht. Sie steht unter "Häufige Fehler" und ist echte Ausgabe von
  1.97.1.
- Der Prüflauf aus `CONTRIBUTING.md` baut nur die Fassung ohne das Feature. Die
  drei Tests für die andere Fassung werden von ihm nicht ausgeführt, und wie sie
  laufen, steht unter "Die Aufgaben".

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/06-07-cfg-und-features/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `04-01 Pakete und Crates` and `06-06 Cargo-Profile und
  cargo doc`. There it stood what goes into a `Cargo.toml`, here a section comes
  along that changes the code itself.
- Building on this unit: `06-08 build.rs` and stage 8, where two runtimes are
  meant to stand side by side in one crate.
- Cite like this when answering: `06-07 #[cfg] und Features`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- `#[cfg]` removes code before the compiler sees it. `cfg!` is a macro turning
  into `true` or `false`, and both branches over it get compiled. The two get
  mixed up, and the unit lives off that difference.
- A feature is additive. Whoever switches it on takes nothing away, and a feature
  with a name like `ohne_...` that removes something breaks that expectation as
  soon as two packages want the same package built differently.
- A body behind a `#[cfg]` that is currently off is not checked. A typo in it
  shows up only once somebody builds with the feature. That is exactly why an
  exercise with `cfg!` stands next to it in this unit.
- The message for calling a function that is currently switched away is
  `error[E0425]`, and its note names which feature the item is gated behind. It
  is under "Common mistakes" and is real output of 1.97.1.
- The check run from `CONTRIBUTING.md` builds only the version without the
  feature. The three tests for the other version are not run by it, and how they
  run stands under "The exercises".

</details>

## Deutsch

### Worum es geht

Ein Feature ist ein Name, der in `Cargo.toml` unter `[features]` steht. Wer das
Paket baut, kann ihn anschalten, und der Code kann fragen, ob er an ist.

Gefragt wird auf zwei Arten. `#[cfg(feature = "name")]` steht als Attribut über
einem Element und entscheidet, ob dieses Element überhaupt entsteht. Ist der
Name aus, verschwindet die Funktion, das Feld oder der ganze Block, bevor der
Übersetzer sie zu Gesicht bekommt.

`cfg!(feature = "name")` ist dagegen ein Makro, das zu `true` oder `false` wird.
Beide Zweige eines `if` darüber werden übersetzt, und genommen wird nur einer.

### Wofür das gut ist

Ein Werkzeug soll oft zwei Wege können, ohne beide immer mitzuschleppen.
Farbige Ausgabe oder nicht, eine zweite Ausgabeform oder nicht, später eine von
zwei Laufzeitumgebungen. Ein Feature ist die Stelle, an der diese Entscheidung
sichtbar getroffen wird, und `#[cfg]` ist die Stelle, an der sie im Code
ankommt.

Der Preis von `#[cfg]` steht nicht in der Rechnung, die man zuerst aufmacht: Was
gerade weggeschaltet ist, wird nicht übersetzt und also auch nicht geprüft. Ein
Tippfehler in einem Zweig, der heute aus ist, ist heute unsichtbar. Wer nur
verzweigen und nichts fernhalten will, nimmt deshalb `cfg!` und lässt beide
Zweige übersetzen.

Features sind außerdem additiv gemeint. Zwei Pakete können dasselbe Paket
brauchen, das eine mit einem Feature und das andere ohne, und gebaut wird dann
einmal mit der Vereinigung von beidem. Ein Feature, das etwas wegnimmt, macht
das zweite Paket kaputt, ohne dass jemand es angefasst hätte.

### Die Erklärung

Dieselbe Funktion steht zweimal da, und das `#[cfg]` entscheidet, welche
entsteht.

```rust
#[cfg(feature = "zusammenfassung")]
fn bericht(zeilen: &[&str]) -> String {
    format!("{}\nZeilen: {}", zeilen.join("\n"), zeilen.len())
}

#[cfg(not(feature = "zusammenfassung"))]
fn bericht(zeilen: &[&str]) -> String {
    zeilen.join("\n")
}

fn main() {
    let zeilen = ["eins", "zwei"];

    println!("{}", bericht(&zeilen));
    println!("--");
    println!("{}", cfg!(feature = "zusammenfassung"));
}
```

`cargo run` und `cargo run --features zusammenfassung` geben aus:

```text
eins
zwei
--
false

eins
zwei
Zeilen: 2
--
true
```

Oben steht der Lauf ohne das Feature, unten der mit ihm. Der Aufruf von
`bericht` ist in beiden derselbe, und das ist der Punkt: Beide Fassungen tragen
denselben Namen und dieselbe Signatur, also muss die aufrufende Seite nichts
wissen. Die letzte Zeile jedes Laufs kommt aus `cfg!` und zeigt die andere Form,
bei der beide Zweige übersetzt sind und nur einer genommen wird.

### Häufige Fehler

Eine Funktion aufrufen, die hinter einem Feature steht, das gerade aus ist.

```rust
#[cfg(feature = "zusammenfassung")]
fn summe(zeilen: &[&str]) -> usize {
    zeilen.len()
}

fn main() {
    let zeilen = ["eins", "zwei"];

    println!("{}", zeilen.join("\n"));
    println!("Zeilen: {}", summe(&zeilen));
}
```

`cargo build` sagt dazu:

```text
error[E0425]: cannot find function `summe` in this scope
  --> src\main.rs:10:28
   |
10 |     println!("Zeilen: {}", summe(&zeilen));
   |                            ^^^^^ not found in this scope
   |
note: found an item that was configured out
  --> src\main.rs:2:4
   |
 1 | #[cfg(feature = "zusammenfassung")]
   |       --------------------------- the item is gated behind the `zusammenfassung` feature
 2 | fn summe(zeilen: &[&str]) -> usize {
   |    ^^^^^
```

Die Kopfzeile allein wäre irreführend. Sie sagt, die Funktion sei nicht zu
finden, und lässt an einen Tippfehler denken. Die Notiz darunter sagt, was
wirklich los ist: Das Element ist da, es wurde weggeschaltet, und hier ist der
Name des Features, hinter dem es steht.

Zwei Wege führen heraus, und sie sind nicht dasselbe. Entweder bekommt der
Aufruf dasselbe `#[cfg]` wie die Funktion, dann verschwinden beide zusammen.
Oder es gibt eine zweite Fassung der Funktion mit `#[cfg(not(...))]`, dann ist
sie immer da. Der zweite Weg ist der aus dieser Einheit, denn er hält die
aufrufende Seite frei von der Entscheidung.

Ein zweiter Fehler macht keine Meldung und ist deshalb der teurere. Was hinter
einem ausgeschalteten `#[cfg]` steht, wird nicht übersetzt, und ein Fehler darin
schläft, bis jemand mit dem Feature baut. Wer eine Fassung anfasst, baut die
andere danach einmal mit.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `eingebaute_teile` und `zusammenfassung_an` stehen fertig da,
und der Doku-Test von `zusammenfassung_an` ist grün.

- `bericht` steht zweimal da, mit `#[cfg]` und mit `#[cfg(not(...))]`
- `beschreibung` steht einmal da und verzweigt mit `cfg!`
- `neuer_bericht` baut eine Struktur, deren Feld `anzahl` am Feature hängt

Die Testdatei hat drei Sorten. Sieben Tests gelten in beiden Übersetzungen, drei
nur ohne das Feature und drei nur mit ihm. Der Prüflauf aus `CONTRIBUTING.md`
baut die Fassung ohne das Feature, führt also zehn davon aus und die drei
anderen nicht. Wer beide Fassungen sehen will, schickt beide Befehle ab.

```console
cd units/06-07-cfg-und-features
cargo test
cargo test --features zusammenfassung
```

### Quelle

    Buch, Kapitel 14 "More about Cargo and Crates.io",
    https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 5 "Conditional compilation",
    https://doc.rust-lang.org/reference/conditional-compilation.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A feature is a name standing in `Cargo.toml` under `[features]`. Whoever builds
the package can switch it on, and the code can ask whether it is on.

The asking goes two ways. `#[cfg(feature = "name")]` stands as an attribute over
an item and decides whether that item comes into being at all. With the name off
the function, the field or the whole block disappears before the compiler ever
lays eyes on it.

`cfg!(feature = "name")`, in contrast, is a macro turning into `true` or `false`.
Both branches of an `if` over it get compiled, and only one is taken.

### What it is good for

A tool often has to manage two ways without dragging both along all the time.
Coloured output or not, a second output form or not, later one of two runtimes.
A feature is the place where that decision is made visibly, and `#[cfg]` is the
place where it arrives in the code.

The price of `#[cfg]` does not stand in the sum you draw up first: what is
currently switched away is not compiled and therefore not checked either. A typo
in a branch that is off today is invisible today. Whoever only wants to branch
and not to keep anything away therefore takes `cfg!` and lets both branches
compile.

Features are also meant to be additive. Two packages can need the same package,
one with a feature and the other without, and it is then built once with the
union of the two. A feature that takes something away breaks the second package
without anybody having touched it.

### The explanation

The same function stands there twice, and the `#[cfg]` decides which one comes
into being.

```rust
#[cfg(feature = "zusammenfassung")]
fn bericht(zeilen: &[&str]) -> String {
    format!("{}\nZeilen: {}", zeilen.join("\n"), zeilen.len())
}

#[cfg(not(feature = "zusammenfassung"))]
fn bericht(zeilen: &[&str]) -> String {
    zeilen.join("\n")
}

fn main() {
    let zeilen = ["eins", "zwei"];

    println!("{}", bericht(&zeilen));
    println!("--");
    println!("{}", cfg!(feature = "zusammenfassung"));
}
```

`cargo run` and `cargo run --features zusammenfassung` print:

```text
eins
zwei
--
false

eins
zwei
Zeilen: 2
--
true
```

Above stands the run without the feature, below the one with it. The call to
`bericht` is the same in both, and that is the point: both versions carry the
same name and the same signature, so the calling side has to know nothing. The
last line of each run comes out of `cfg!` and shows the other form, where both
branches are compiled and only one is taken.

### Common mistakes

Calling a function that stands behind a feature which is currently off.

```rust
#[cfg(feature = "zusammenfassung")]
fn summe(zeilen: &[&str]) -> usize {
    zeilen.len()
}

fn main() {
    let zeilen = ["eins", "zwei"];

    println!("{}", zeilen.join("\n"));
    println!("Zeilen: {}", summe(&zeilen));
}
```

`cargo build` answers:

```text
error[E0425]: cannot find function `summe` in this scope
  --> src\main.rs:10:28
   |
10 |     println!("Zeilen: {}", summe(&zeilen));
   |                            ^^^^^ not found in this scope
   |
note: found an item that was configured out
  --> src\main.rs:2:4
   |
 1 | #[cfg(feature = "zusammenfassung")]
   |       --------------------------- the item is gated behind the `zusammenfassung` feature
 2 | fn summe(zeilen: &[&str]) -> usize {
   |    ^^^^^
```

The headline on its own would be misleading. It says the function cannot be
found and makes you think of a typo. The note under it says what is really going
on: the item is there, it was switched away, and here is the name of the feature
it stands behind.

Two ways lead out, and they are not the same thing. Either the call gets the same
`#[cfg]` as the function, and then the two disappear together. Or there is a
second version of the function with `#[cfg(not(...))]`, and then it is always
there. The second way is the one from this unit, because it keeps the calling
side free of the decision.

A second mistake makes no message and is therefore the more expensive one. What
stands behind a switched-off `#[cfg]` is not compiled, and a fault in it sleeps
until somebody builds with the feature. Whoever touches one version builds the
other one once afterwards.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `eingebaute_teile` and `zusammenfassung_an`
stand there finished, and the doc test of `zusammenfassung_an` is green.

- `bericht` stands there twice, with `#[cfg]` and with `#[cfg(not(...))]`
- `beschreibung` stands there once and branches with `cfg!`
- `neuer_bericht` builds a struct whose field `anzahl` hangs on the feature

The test file has three kinds. Seven tests hold in both compilations, three only
without the feature and three only with it. The check run from `CONTRIBUTING.md`
builds the version without the feature, so it runs ten of them and not the other
three. Whoever wants to see both versions sends both commands.

```console
cd units/06-07-cfg-und-features
cargo test
cargo test --features zusammenfassung
```

### Source

    Book, chapter 14 "More about Cargo and Crates.io",
    https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html,
    checked against 1.97.1

    The Rust Reference, chapter 5 "Conditional compilation",
    https://doc.rust-lang.org/reference/conditional-compilation.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
