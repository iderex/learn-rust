# 06-06 Cargo-Profile und cargo doc / Cargo profiles and cargo doc

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/06-06-cargo-profile-und-cargo-doc/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `01-02 Zahlen und andere einfache Typen` und
  `05-06 Doku-Tests`. Der Überlauf gehört zu den Zahlen, der Doku-Test zur
  Doku.
- Auf dieser Einheit bauen auf: alles, was gebaut und ausgeliefert wird, und
  jeder Doku-Kommentar, der eine eigene Überschrift bekommt.
- Beim Antworten so zitieren: `06-06 Cargo-Profile und cargo doc`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Der Unterschied zwischen den Profilen ist nicht nur Geschwindigkeit. Im
  `dev`-Profil bricht eine Rechnung mit Überlauf ab, im `release`-Profil läuft
  sie weiter. Das ist gemessen und steht unter "Die Erklärung" mit beiden
  Läufen.
- Ein Doku-Test wird nicht mit dem Profil des Laufs gebaut. Unter
  `cargo test --release` liefert `cfg!(debug_assertions)` im Doku-Test `true`,
  während die Bibliothek daneben mit `release` gebaut ist. Wer das Gegenteil
  behauptet, sagt bitte, mit welchem Lauf.
- Was `cargo doc` aus einer Überschrift wie `# Panics` macht, ist eine Frage der
  Darstellung. Ob eine solche Überschrift da ist, prüft in diesem Repository
  nichts.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/06-06-cargo-profile-und-cargo-doc/`. It is public. Whoever is asked
  for it may name it, but should explain the compiler message in question first.
- This unit builds on: `01-02 Zahlen und andere einfache Typen` and
  `05-06 Doku-Tests`. The overflow belongs to the numbers, the doc test to the
  documentation.
- Building on this unit: everything that gets built and shipped, and every doc
  comment that gains a heading of its own.
- Cite like this when answering: `06-06 Cargo-Profile und cargo doc`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The difference between the profiles is not only speed. In the `dev` profile a
  calculation with an overflow aborts, in the `release` profile it carries on.
  That is measured and stands under "The explanation" with both runs.
- A doc test is not built with the profile of the run. Under
  `cargo test --release`, `cfg!(debug_assertions)` inside the doc test gives
  `true` while the library next to it is built with `release`. Whoever claims
  the opposite, please say with which run.
- What `cargo doc` makes out of a heading like `# Panics` is a question of
  presentation. Whether such a heading is there is checked by nothing in this
  repository.

</details>

## Deutsch

### Worum es geht

Cargo baut denselben Quelltext nach zwei Voreinstellungen. `cargo build` nimmt
das Profil `dev`, `cargo build --release` nimmt `release`. Das eine baut schnell
und läuft langsam, das andere umgekehrt.

Beide Profile stehen in der `Cargo.toml` und lassen sich dort ändern, zum
Beispiel mit `[profile.release]` und `opt-level = 3`. Steht nichts da, gelten
die Voreinstellungen, und die sind es, um die es hier geht.

Der zweite Teil der Einheit ist `cargo doc`. Es liest die Doku-Kommentare aus
dem Quelltext und macht daraus dieselben Seiten, auf denen man die
Standardbibliothek nachschlägt.

### Wofür das gut ist

Der Unterschied zwischen den Profilen ist nicht nur Geschwindigkeit. Im
`dev`-Profil ist die Überlaufprüfung an, und eine Addition, die nicht in ihren
Typ passt, bricht ab. Im `release`-Profil ist sie aus, und dieselbe Addition
fängt vorn wieder an.

Das heißt, dass ein Programm im Test etwas anderes tut als beim Ausliefern, und
zwar an einer Stelle, an der niemand hinsieht. Wer eine Rechnung schreibt, die
überlaufen kann, sagt deshalb ausdrücklich, was dann passieren soll, mit
`checked_add`, `wrapping_add` oder `saturating_add`. Dann steht es im Quelltext
statt in einer Einstellung.

`cargo doc` wiederum kostet nichts und beantwortet die Frage, wie die eigene
Schnittstelle von außen aussieht. Ein Doku-Kommentar, der beim Schreiben klar
wirkt, ist auf der erzeugten Seite oft die Stelle, an der etwas fehlt.

### Die Erklärung

Ein Programm, das mit Absicht überläuft. Die Addition steht in einer eigenen
Funktion, sonst rechnet der Übersetzer sie schon beim Übersetzen aus und weist
sie zurück, bevor ein Profil überhaupt zum Zug kommt.

```rust
// Deutsch: Die Addition steht in einer eigenen Funktion, damit der Übersetzer
// sie nicht schon beim Übersetzen ausrechnet und den Überlauf meldet.
fn addiere(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    println!("{}", addiere(250, 10));
}
```

`cargo run` gibt aus:

```text
thread 'main' (49248) panicked at src\main.rs:4:5:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`cargo run --release` gibt aus:

```text
4
```

Derselbe Quelltext, zwei Antworten. 250 plus 10 sind 260, und 260 passt nicht in
ein `u8`, das bis 255 reicht. Im `dev`-Profil bricht der Lauf ab und gibt 101
zurück, im `release`-Profil kommt 4 heraus, denn 260 minus 256 ist 4, und der
Lauf gibt 0 zurück.

`cargo doc` erzeugt aus denselben Dateien die Seiten der eigenen Doku.

```console
$ cargo doc -p unit-06-06-cargo-profile-und-cargo-doc --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
   Generated <Wurzel>/target/doc/unit_06_06_cargo_profile_und_cargo_doc/index.html
```

An `<Wurzel>` stand der Pfad des Rechners, auf dem der Befehl lief. Sonst ist
die Ausgabe unverändert. `--no-deps` lässt die Abhängigkeiten weg, sonst wird
auch deren Doku gebaut.

Eine Sache dazu, die beim Bauen dieser Einheit herauskam. Ein Doku-Test wird
nicht mit dem Profil des Laufs gebaut. Unter `cargo test --release` gibt die
Bibliothek `release` zurück, während `cfg!(debug_assertions)` im Doku-Test
daneben `true` ist und `debug` erwartet. Der Doku-Test von `profile_name`
vergleicht deshalb nicht die beiden, sondern prüft nur, dass eine der zwei
Antworten kommt.

### Häufige Fehler

Sich in einem Test auf das Profil verlassen, in dem er gerade läuft.

```rust
/// ```
/// use unit_06_06_cargo_profile_und_cargo_doc::profile_name;
///
/// let erwartet = if cfg!(debug_assertions) { "debug" } else { "release" };
///
/// assert_eq!(profile_name(), erwartet);
/// ```
pub fn profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
```

Unter `cargo test --release` sagt der Lauf dazu:

```text
---- src\lib.rs - profile_name (line 17) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' (25060) panicked at doctest_bundle_2024.rs:10:1:
assertion `left == right` failed
  left: "release"
 right: "debug"
```

Zwei Pfade in dieser Ausgabe sind gekürzt, weil beide auf den Rechner zeigen,
auf dem der Lauf stattfand: der vor `src\lib.rs` und der vor
`doctest_bundle_2024.rs`. Alles andere steht so da, wie es kam.

Links steht, was die Bibliothek sagt, rechts, was der Doku-Test erwartet hat.
Unter `cargo test` ohne `--release` ist derselbe Doku-Test grün, und deshalb
fällt der Fehler im täglichen Lauf nicht auf. Der Weg heraus ist, im Beispiel
nichts über das Profil zu behaupten.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `profile_name` steht fertig da, und sein Doku-Test ist grün.

- `sum_checked` zählt zusammen und sagt `None`, sobald es nicht mehr passt
- `sum_wrapping` zählt zusammen und lässt es absichtlich überlaufen
- `half_even` halbiert und bricht bei einer ungeraden Zahl ab

```console
cd units/06-06-cargo-profile-und-cargo-doc
cargo test
```

### Quelle

    Buch, Kapitel 14 "More about Cargo and Crates.io", Abschnitt 14.1
    "Customizing Builds with Release Profiles",
    https://doc.rust-lang.org/book/ch14-01-release-profiles.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Cargo builds the same source according to two presets. `cargo build` takes the
`dev` profile, `cargo build --release` takes `release`. One builds fast and runs
slowly, the other one the other way round.

Both profiles stand in the `Cargo.toml` and can be changed there, for example
with `[profile.release]` and `opt-level = 3`. If nothing stands there, the
presets hold, and those are the ones this is about.

The second part of the unit is `cargo doc`. It reads the doc comments out of the
source and makes the same pages out of them on which one looks up the standard
library.

### What it is good for

The difference between the profiles is not only speed. In the `dev` profile the
overflow check is on, and an addition that does not fit into its type aborts. In
the `release` profile it is off, and the same addition starts from the front
again.

That means a program does something else in a test than it does when shipped, in
a place where nobody looks. Whoever writes a calculation that can overflow
therefore says explicitly what should happen then, with `checked_add`,
`wrapping_add` or `saturating_add`. Then it stands in the source instead of in a
setting.

`cargo doc` in turn costs nothing and answers the question of what your own
interface looks like from outside. A doc comment that seems clear while it is
written is often, on the generated page, the place where something is missing.

### The explanation

A program that overflows on purpose. The addition stands in a function of its
own, otherwise the compiler works it out at compile time and refuses it before a
profile ever comes into play.

```rust
// Deutsch: Die Addition steht in einer eigenen Funktion, damit der Übersetzer
// sie nicht schon beim Übersetzen ausrechnet und den Überlauf meldet.
fn addiere(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    println!("{}", addiere(250, 10));
}
```

`cargo run` prints:

```text
thread 'main' (49248) panicked at src\main.rs:4:5:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`cargo run --release` prints:

```text
4
```

The same source, two answers. 250 plus 10 is 260, and 260 does not fit into a
`u8`, which reaches up to 255. In the `dev` profile the run aborts and returns
101, in the `release` profile 4 comes out, because 260 minus 256 is 4, and the
run returns 0.

`cargo doc` makes the pages of your own documentation out of the same files.

```console
$ cargo doc -p unit-06-06-cargo-profile-und-cargo-doc --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
   Generated <Wurzel>/target/doc/unit_06_06_cargo_profile_und_cargo_doc/index.html
```

Where `<Wurzel>` stands, the path of the machine the command ran on stood.
Otherwise the output is unchanged. `--no-deps` leaves the dependencies out,
otherwise their documentation is built as well.

One thing on top of that, which came out while this unit was built. A doc test is
not built with the profile of the run. Under `cargo test --release` the library
returns `release`, while `cfg!(debug_assertions)` in the doc test next to it is
`true` and expects `debug`. The doc test of `profile_name` therefore does not
compare the two but only checks that one of the two answers comes.

### Common mistakes

Relying, inside a test, on the profile it happens to be running in.

```rust
/// ```
/// use unit_06_06_cargo_profile_und_cargo_doc::profile_name;
///
/// let erwartet = if cfg!(debug_assertions) { "debug" } else { "release" };
///
/// assert_eq!(profile_name(), erwartet);
/// ```
pub fn profile_name() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
```

Under `cargo test --release` the run says:

```text
---- src\lib.rs - profile_name (line 17) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' (25060) panicked at doctest_bundle_2024.rs:10:1:
assertion `left == right` failed
  left: "release"
 right: "debug"
```

Two paths in that output are shortened, because both point at the machine the
run happened on: the one before `src\lib.rs` and the one before
`doctest_bundle_2024.rs`. Everything else stands as it came.

On the left stands what the library says, on the right what the doc test
expected. Under `cargo test` without `--release` the same doc test is green, and
that is why the mistake does not show up in the daily run. The way out is to
claim nothing about the profile inside the example.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `profile_name` stands there finished, and its
doc test is green.

- `sum_checked` adds up and says `None` as soon as it no longer fits
- `sum_wrapping` adds up and lets it overflow on purpose
- `half_even` halves and aborts on an odd number

```console
cd units/06-06-cargo-profile-und-cargo-doc
cargo test
```

### Source

    Book, chapter 14 "More about Cargo and Crates.io", section 14.1 "Customizing
    Builds with Release Profiles",
    https://doc.rust-lang.org/book/ch14-01-release-profiles.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
