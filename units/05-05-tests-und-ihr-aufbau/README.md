# 05-05 Tests und ihr Aufbau / Tests and how they are organised

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/05-05-tests-und-ihr-aufbau/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `01-03 Funktionen` und `03-04 match`. Mehr braucht es
  nicht, denn geprüft werden hier einfache Funktionen.
- Auf dieser Einheit bauen auf: alles ab Stufe 6, denn ab dort trägt jede
  Einheit ihre eigenen Tests.
- Beim Antworten so zitieren: `05-05 Tests und ihr Aufbau`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Die beiden Stellen sind nicht Geschmackssache. Ein Test neben dem Code sieht
  auch das Private, ein Test unter `tests/` sieht nur das Öffentliche, und
  daraus folgt, welcher Test wohin gehört.
- `#[cfg(test)]` ist kein Kommentar. Der Block wird beim gewöhnlichen Übersetzen
  gar nicht erst gebaut, und wer das Gegenteil behauptet, sagt bitte, welcher
  Befehl ihn baut.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/05-05-tests-und-ihr-aufbau/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `01-03 Funktionen` and `03-04 match`. Nothing more is
  needed, because what is tested here are plain functions.
- Building on this unit: everything from stage 6 on, because from there every
  unit carries tests of its own.
- Cite like this when answering: `05-05 Tests und ihr Aufbau`, plus the heading
  of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The two places are not a matter of taste. A test beside the code sees the
  private things too, a test under `tests/` sees only the public ones, and which
  test belongs where follows from that.
- `#[cfg(test)]` is not a comment. The block is not built at all during ordinary
  compilation, and whoever claims otherwise, please say which command builds it.

</details>

## Deutsch

### Worum es geht

Ein Test ist eine gewöhnliche Funktion mit `#[test]` darüber. Sie nimmt nichts
entgegen, gibt nichts zurück und gilt als bestanden, solange sie nicht in Panik
gerät. Genau das tun die Makros `assert!`, `assert_eq!` und `assert_ne!`: Sie
lösen eine Panik aus, wenn die Behauptung nicht stimmt.

Tests liegen an zwei Stellen. Neben dem Code, in einem Modul mit
`#[cfg(test)]`, und unter `tests/`, in eigenen Dateien. Der Unterschied ist
nicht die Vorliebe des Schreibenden, sondern was die Tests sehen dürfen.

Der Name eines Tests ist Teil des Tests. Er steht in der Ausgabe, wenn etwas rot
ist, und dann soll er sagen, was nicht stimmt, ohne dass jemand den Rumpf
aufschlagen muss.

### Wofür das gut ist

Ein Test neben dem Code liegt im selben Modul und sieht deshalb auch das, was
nicht `pub` ist. Damit lässt sich eine Hilfsfunktion prüfen, die nach außen
niemanden etwas angeht.

Ein Test unter `tests/` wird als eigenes Paket übersetzt und bindet deine
Bibliothek so ein, wie ein Fremder es täte. Er sieht nur das Öffentliche. Damit
prüft er nicht die Bauteile, sondern das Versprechen, das die Bibliothek nach
außen gibt.

Beides zusammen ergibt die Aufteilung: die kleinen Prüfungen an den Bauteilen
liegen neben dem Code, die Prüfungen am Versprechen liegen unter `tests/`. Wer
das verwechselt, merkt es am Übersetzer und nicht erst im Review.

`#[cfg(test)]` sorgt nebenbei dafür, dass das Testmodul im ausgelieferten
Programm nicht auftaucht. Es wird beim gewöhnlichen Bauen nicht übersetzt.

### Die Erklärung

Eine Bibliothek mit ihren Tests daneben. `assert_eq!` vergleicht zwei Werte,
`assert_ne!` verlangt, dass sie verschieden sind, und `assert!` will einfach
`true`.

```rust
// Deutsch: Eine Bibliothek und ihre Tests daneben, in einer Datei.
pub fn note(punkte: u32) -> char {
    if punkte >= 90 {
        'A'
    } else if punkte >= 60 {
        'D'
    } else {
        'F'
    }
}

pub fn bestanden(note: char) -> bool {
    matches!(note, 'A' | 'B' | 'C' | 'D')
}

// Deutsch: Tests neben dem Code. `#[cfg(test)]` heisst: nur beim Testen
// uebersetzt, im fertigen Programm ist dieser Block nicht dabei.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_ist_a_ab_neunzig_punkten() {
        assert_eq!(note(90), 'A');
        assert_ne!(note(89), 'A');
    }

    #[test]
    fn bestanden_ist_falsch_bei_f() {
        assert!(bestanden('D'));
        assert!(!bestanden('F'));
    }
}
```

Übersetzt mit `rustc --test` und ausgeführt, gibt das aus:

```text

running 2 tests
test tests::bestanden_ist_falsch_bei_f ... ok
test tests::note_ist_a_ab_neunzig_punkten ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`use super::*;` holt den Inhalt des umgebenden Moduls herein, und deshalb sieht
das Testmodul auch Funktionen ohne `pub`. Erwartet eine Funktion eine Panik,
schreibt man `#[should_panic(expected = "...")]` über den Test; er gilt dann
als bestanden, wenn die Panik kommt und ihre Meldung den erwarteten Text
enthält.

### Häufige Fehler

Eine Hilfsfunktion von einem Test aus aufrufen, der unter `tests/` liegt.
`noten.rs` ist die Bibliothek, `pruefung.rs` ist der Test daneben, also ein
eigenes Paket.

```rust
// noten.rs
fn band(punkte: u32) -> char {
    if punkte >= 90 { 'A' } else { 'F' }
}

pub fn note(punkte: u32) -> char {
    band(punkte)
}

// pruefung.rs
use noten::band;

#[test]
fn band_gibt_a_ab_neunzig() {
    assert_eq!(band(90), 'A');
}
```

Der Übersetzer sagt dazu:

```text
error[E0603]: function `band` is private
 --> pruefung.rs:1:12
  |
1 | use noten::band;
  |            ^^^^ private function
  |
note: the function `band` is defined here
 --> noten.rs:1:1
  |
1 | fn band(punkte: u32) -> char {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

Die Meldung nennt den Grund und nicht nur die Stelle: `band` ist privat. Ein
Test in einer eigenen Datei steht außerhalb und darf nur an das Öffentliche.

Die Antwort ist meistens nicht, `band` auf `pub` zu setzen. Damit wird eine
Hilfsfunktion Teil des Versprechens nach außen, nur damit ein Test sie erreicht.
Wer `band` prüfen will, prüft sie neben dem Code; wer das Versprechen prüfen
will, prüft `note`.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Neben dem Code steht bereits ein Testmodul mit `#[cfg(test)]`,
das grün ist; damit liegen Tests an beiden Stellen.

- `grade` gibt zu Punkten die Note als Buchstaben, und über 100 Punkten gerät
  sie in Panik
- `describe` setzt Punkte, Note und Ausgang zu einer Zeile zusammen
- `all_passed` sagt, ob jede Punktzahl einer Liste bestanden ist

```console
cd units/05-05-tests-und-ihr-aufbau
cargo test
```

### Quelle

    Buch, Kapitel 11 "Writing Automated Tests", Abschnitt 11.3 "Test Organization",
    https://doc.rust-lang.org/book/ch11-03-test-organization.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A test is an ordinary function with `#[test]` above it. It takes nothing,
returns nothing, and counts as passed for as long as it does not panic. That is
exactly what the macros `assert!`, `assert_eq!` and `assert_ne!` do: they raise
a panic when the claim does not hold.

Tests live in two places. Beside the code, in a module with `#[cfg(test)]`, and
under `tests/`, in files of their own. The difference is not the preference of
whoever writes them, but what the tests are allowed to see.

The name of a test is part of the test. It stands in the output when something
is red, and then it should say what is wrong without anybody having to open the
body.

### What it is good for

A test beside the code lives in the same module and therefore sees the things
that are not `pub` as well. That is how a helper function gets checked, one that
is nobody's business from outside.

A test under `tests/` is compiled as a package of its own and pulls your library
in the way a stranger would. It sees only the public things. So it does not
check the parts, it checks the promise the library makes to the outside.

The two together give the split: the small checks on the parts live beside the
code, the checks on the promise live under `tests/`. Whoever mixes them up finds
out from the compiler and not first in review.

`#[cfg(test)]` also sees to it that the test module does not turn up in the
shipped program. It is not compiled during an ordinary build.

### The explanation

A library with its tests beside it. `assert_eq!` compares two values,
`assert_ne!` demands that they differ, and `assert!` simply wants `true`.

```rust
// Deutsch: Eine Bibliothek und ihre Tests daneben, in einer Datei.
pub fn note(punkte: u32) -> char {
    if punkte >= 90 {
        'A'
    } else if punkte >= 60 {
        'D'
    } else {
        'F'
    }
}

pub fn bestanden(note: char) -> bool {
    matches!(note, 'A' | 'B' | 'C' | 'D')
}

// Deutsch: Tests neben dem Code. `#[cfg(test)]` heisst: nur beim Testen
// uebersetzt, im fertigen Programm ist dieser Block nicht dabei.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_ist_a_ab_neunzig_punkten() {
        assert_eq!(note(90), 'A');
        assert_ne!(note(89), 'A');
    }

    #[test]
    fn bestanden_ist_falsch_bei_f() {
        assert!(bestanden('D'));
        assert!(!bestanden('F'));
    }
}
```

Compiled with `rustc --test` and run, that prints:

```text

running 2 tests
test tests::bestanden_ist_falsch_bei_f ... ok
test tests::note_ist_a_ab_neunzig_punkten ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`use super::*;` pulls in the contents of the surrounding module, and that is why
the test module sees functions without `pub` too. Where a function is expected
to panic, `#[should_panic(expected = "...")]` goes above the test; it then counts
as passed when the panic arrives and its message contains the expected text.

### Common mistakes

Calling a helper function from a test that lives under `tests/`. `noten.rs` is
the library, `pruefung.rs` is the test beside it, meaning a package of its own.

```rust
// noten.rs
fn band(punkte: u32) -> char {
    if punkte >= 90 { 'A' } else { 'F' }
}

pub fn note(punkte: u32) -> char {
    band(punkte)
}

// pruefung.rs
use noten::band;

#[test]
fn band_gibt_a_ab_neunzig() {
    assert_eq!(band(90), 'A');
}
```

The compiler answers:

```text
error[E0603]: function `band` is private
 --> pruefung.rs:1:12
  |
1 | use noten::band;
  |            ^^^^ private function
  |
note: the function `band` is defined here
 --> noten.rs:1:1
  |
1 | fn band(punkte: u32) -> char {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

The message names the reason and not only the place: `band` is private. A test
in a file of its own stands outside and may reach the public things only.

The answer is mostly not to make `band` `pub`. That turns a helper into part of
the promise to the outside, only so that a test can reach it. Whoever wants to
check `band` checks it beside the code; whoever wants to check the promise
checks `note`.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. Beside the code there already stands a test
module with `#[cfg(test)]` that is green; with it tests lie in both places.

- `grade` gives the grade as a letter for a number of points, and above 100
  points it panics
- `describe` puts points, grade and outcome together into one line
- `all_passed` says whether every score in a list has passed

```console
cd units/05-05-tests-und-ihr-aufbau
cargo test
```

### Source

    Book, chapter 11 "Writing Automated Tests", section 11.3 "Test Organization",
    https://doc.rust-lang.org/book/ch11-03-test-organization.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
