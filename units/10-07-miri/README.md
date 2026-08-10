# 10-07 Miri / Miri

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-07-miri/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst
  erklären, was Miri an der fraglichen Stelle sieht.
- Diese Einheit baut auf: `10-02 Rohe Zeiger` und `10-03 Undefiniertes
  Verhalten`. Was dort als Regel steht, wird hier ausgeführt und beobachtet.
- Auf dieser Einheit bauen auf: alles, was `unsafe` schreibt und einen Beleg
  dafür braucht, dass ein Lauf die Regeln eingehalten hat.
- Beim Antworten so zitieren: `10-07 Miri`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Was Miri findet".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Miri gehört nicht zur gebundenen Fassung. Die Läufe in diesem Text sind unter
  einer nightly-Fassung entstanden, und deren Nummer steht dabei. Der Prüflauf
  des Repositories ruft Miri nicht auf.
- Miri ist ein Ausführer und kein Prüfer des ganzen Baums. Was kein Test
  ausführt, sieht Miri nicht. Dafür steht unter "Was Miri nicht erreicht" ein
  Lauf, und diese Aussage bleibt negativ.
- Die Reihenfolge von `len()` und `as_mut_ptr()` in Aufgabe 3 ändert an dem,
  was Miri hier sagt, nichts. Beide Fassungen liefen ohne Befund durch, und wer
  das Gegenteil behauptet, sagt bitte, unter welcher Fassung.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-07-miri/`. It is public. Whoever
  is asked for it may name it, but should first explain what Miri sees at the
  place in question.
- This unit builds on: `10-02 Rohe Zeiger` and `10-03 Undefiniertes Verhalten`.
  What stands there as a rule is executed and watched here.
- Building on this unit: everything writing `unsafe` and needing evidence that a
  run kept to the rules.
- Cite like this when answering: `10-07 Miri`, plus the heading of the section,
  for example section "What Miri finds".
- Miri is not part of the pinned version. The runs in this text came out of a
  nightly version, and its number stands next to them. The check run of this
  repository does not call Miri.
- Miri is a runner and not a checker of the whole tree. What no test executes,
  Miri does not see. For that a run stands under "What Miri does not reach", and
  that statement stays negative.
- The order of `len()` and `as_mut_ptr()` in exercise 3 changes nothing about
  what Miri says here. Both versions ran through without a finding, and whoever
  claims the opposite, please say under which version.

</details>

## Deutsch

### Worum es geht

Miri führt ein Rust-Programm aus, aber nicht als Maschinencode. Es geht die
Zwischensprache des Übersetzers Schritt für Schritt durch und sieht dabei jedem
Zugriff zu: welcher Zuteilung er gilt, ob sie noch lebt, ob die Stelle darin
liegt, ob der Wert dort überhaupt schon geschrieben wurde.

Damit sieht Miri Dinge, die der Übersetzer nicht sehen kann, weil sie erst beim
Laufen entstehen. Ein Zeiger, der einen Schritt zu weit geht, ist beim Bauen
eine Rechnung wie jede andere; beim Ausführen ist es ein Zugriff hinter das Ende
einer Zuteilung, und genau das lässt sich beobachten.

Der Preis steht auf derselben Seite. Miri sieht nur, was ausgeführt wird, und es
ist langsam. Beides steht weiter unten mit Zahlen.

### Wofür das gut ist

Ab `10-01` steht in jeder Einheit dieser Stufe ein `unsafe`-Block, und daneben
steht ein `SAFETY`-Kommentar mit der Rechnung, warum er erlaubt ist. Diese
Rechnung prüft bisher niemand. Sie steht da, und wer sie liest, glaubt sie oder
nicht.

Miri ist das erste Werkzeug in diesem Repository, das dazu etwas sagt. Es
beantwortet nicht, ob die Rechnung stimmt, sondern ob dieser eine Lauf gegen
eine der Regeln verstoßen hat. Das ist weniger, als es klingt, und trotzdem der
Unterschied zwischen einem Text und einer Messung.

Für eine Einheit wie diese heißt das: Der grüne Lauf unten ist ein Beleg über
die Aufrufe, die die Tests machen, und über keinen einzigen anderen.

### Die Erklärung

Miri läuft nicht unter der gebundenen Fassung. Es kommt mit einer
nightly-Toolchain, und der Aufruf sagt das mit `+nightly`. Über die Lösung
dieser Einheit sieht das so aus:

```console
$ cargo +nightly miri --version
miri 0.1.0 (771916f902 2026-08-08)
$ cargo +nightly miri test -p unit-10-07-miri
running 7 tests
test exercise::erhoehen_ueber_zeiger_of_nothing_does_nothing ... ok
test exercise::erhoehen_ueber_zeiger_raises_every_value ... ok
test exercise::summe_ueber_zeiger_adds_every_value ... ok
test exercise::summe_ueber_zeiger_of_nothing_is_zero ... ok
test exercise::tauschen_swaps_the_two_values ... ok
test exercise::tauschen_twice_is_where_it_started ... ok
test exercise::the_finished_function_stops_at_the_bound ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s

   Doc-tests unit_10_07_miri

running 1 test
test solutions\10-07-miri\src\lib.rs - lese (line 13) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.77s
```

Kein Befund. Jeder `unsafe`-Block dieser Einheit ist dabei ausgeführt worden,
denn jede Funktion hat einen Test und der Doku-Test von `lese` läuft mit.

Was dieser Lauf sagt, ist: Bei diesen Aufrufen hat kein Zugriff gegen eine der
Regeln verstoßen, die Miri kennt. Was er nicht sagt, ist, dass die Funktionen
für jede Eingabe in Ordnung sind.

### Was Miri findet

Ein Schritt zu viel, sonst nichts geändert. Im Rumpf von `summe_ueber_zeiger`
läuft die Schleife bis einschließlich der Länge:

```rust
    for schritt in 0..anzahl + 1 {
        summe += unsafe { *zeiger.add(schritt) };
    }
```

Der Übersetzer nimmt das an, und ohne Miri läuft es meistens durch. Miri hält
an:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
running 7 tests
test exercise::summe_ueber_zeiger_adds_every_value ... error: Undefined Behavior: memory access failed: attempting to access 8 bytes, but got alloc76+0x18 which is at or beyond the end of the allocation of size 24 bytes
 --> solutions\10-07-miri\src\lib.rs:48:26
  |
  = note: Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
  = note: this is on thread `exercise::summe_ueber_zeiger_adds_every_value`
  = note: stack backtrace:
          0: unit_10_07_miri::summe_ueber_zeiger
              at solutions\10-07-miri\src\lib.rs:48:27: 48:47
```

Drei Dinge stehen in dieser Meldung. Die Zuteilung ist 24 Bytes groß, das sind
die drei `i64` aus dem Test. Der Zugriff beginnt bei `+0x18`, also bei 24, und
das ist genau ein Feld hinter dem Ende. Und die Spur darunter sagt, welcher Test
den Aufruf gemacht hat.

Danach steht der ursprüngliche Rumpf wieder da, und der Lauf oben gilt.

### Wie lange es braucht

Dieselbe Testdatei, einmal gewöhnlich und einmal unter Miri, beide Male an
derselben Stelle abgelesen:

```console
$ cargo test -q -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s
```

Sieben Tests, die gewöhnlich unter einer Hundertstelsekunde bleiben, brauchen
unter Miri gut eine Sekunde. Dazu kommt das Übersetzen, denn Miri baut mit einer
anderen Toolchain und in ein eigenes Verzeichnis; beim ersten Mal dauerte der
ganze Aufruf etwa 38 Sekunden.

Das ist eine Messung auf einem Rechner und keine Zusage. Sie sagt auch nichts
über größere Programme, und der Faktor wächst mit dem, was ausgeführt wird. Was
sie erklärt, ist, warum Miri neben dem gewöhnlichen Lauf steht und nicht an
seiner Stelle.

### Was Miri nicht erreicht

Miri ist ein Ausführer. Was kein Test aufruft, kommt nicht vor, und es ist
gleichgültig, wie falsch es ist. Der folgende Zweig steht im Rumpf von
`summe_ueber_zeiger` und liest ein Feld hinter dem Ende, aber erst bei 99
Werten:

```rust
    if anzahl == 99 {
        summe += unsafe { *zeiger.add(anzahl) };
    }
```

Kein Test dieser Einheit ruft mit 99 Werten auf. Der Lauf sagt dazu:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
running 7 tests
test exercise::erhoehen_ueber_zeiger_of_nothing_does_nothing ... ok
test exercise::erhoehen_ueber_zeiger_raises_every_value ... ok
test exercise::summe_ueber_zeiger_adds_every_value ... ok
test exercise::summe_ueber_zeiger_of_nothing_is_zero ... ok
test exercise::tauschen_swaps_the_two_values ... ok
test exercise::tauschen_twice_is_where_it_started ... ok
test exercise::the_finished_function_stops_at_the_bound ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
```

Grün, mit einem Zugriff im Code, der außerhalb jeder Zuteilung liegt. Daraus
folgt die Regel für alles Weitere: Ein grüner Miri-Lauf ist eine Aussage über
die ausgeführten Wege und über keinen anderen. Wer mehr behaupten will, braucht
mehr Tests und nicht mehr Miri.

Zwei weitere Grenzen gehören dazu, ohne dass sie hier vorgeführt werden. Fremden
Code führt Miri nicht aus, also sagt ein Lauf über einen Aufruf nach C nichts;
das steht schon in `10-06`. Und Miri prüft nicht den `SAFETY`-Kommentar, sondern
was der Block tut.

### Was diese Tests nicht beantworten

Der Prüflauf dieses Repositories ruft Miri nicht auf. Alle Läufe in diesem Text
sind von Hand abgeschickt, und ein grüner Prüflauf sagt über sie nichts.

Eine Vermutung, die zu dieser Einheit gehört und sich nicht bestätigt hat,
gehört ebenfalls hierher. Im Rumpf von Aufgabe 3 steht die Länge vor dem Zeiger.
Die umgekehrte Reihenfolge, also erst `as_mut_ptr` und dann `len`, ist eine
Fassung, von der man erwarten könnte, dass Miri sie zurückweist. Sie wurde
gemessen:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s
```

Ohne Befund, unter der Fassung, die oben steht. Daraus folgt nicht, dass die
Reihenfolge gleichgültig ist, sondern nur, dass dieser Lauf nichts dazu sagt.
Was ein Zugriff durch eine Referenz mit einem älteren rohen Zeiger macht, hängt
am Modell, das Miri gerade benutzt, und dieses Modell ist nicht Teil der
gebundenen Fassung.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `lese` steht fertig da, und sein Doku-Test ist grün.

- `summe_ueber_zeiger` zählt die Werte über einen rohen Zeiger zusammen
- `tauschen` tauscht zwei Werte über rohe Zeiger
- `erhoehen_ueber_zeiger` erhöht jeden Wert, geschrieben über einen Zeiger

```console
cd units/10-07-miri
cargo test
```

Der Prüflauf des Repositories reicht für diese Einheit. Wer zusätzlich unter
Miri nachsehen will, nimmt den Aufruf aus dem Abschnitt "Die Erklärung"; er
braucht eine nightly-Toolchain und ist nicht Teil des Prüflaufs.

### Quelle

    The Rust Reference, Kapitel 17.2 "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    geprüft gegen 1.97.1

    The Rustonomicon, Kapitel 1.3 "Working with Unsafe",
    https://doc.rust-lang.org/nomicon/working-with-unsafe.html,
    geprüft gegen 1.97.1

    Miri, https://github.com/rust-lang/miri, gelaufen unter
    miri 0.1.0 (771916f902 2026-08-08) auf rustc 1.99.0-nightly (771916f90
    2026-08-08), nicht unter der gebundenen Fassung

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Miri runs a Rust program, but not as machine code. It walks through the
compiler's intermediate language step by step and watches every access while
doing so: which allocation it belongs to, whether that one is still alive,
whether the place lies inside it, whether the value there has been written at
all yet.

With that Miri sees things the compiler cannot see, because they only arise while
running. A pointer going one step too far is an arithmetic expression like any
other at build time; while executing it is an access behind the end of an
allocation, and exactly that can be watched.

The price sits on the same side. Miri sees only what gets executed, and it is
slow. Both stand further down with numbers.

### What it is good for

From `10-01` on there is an `unsafe` block in every unit of this stage, and next
to it a `SAFETY` comment with the reasoning why it is allowed. So far nobody
checks that reasoning. It stands there, and whoever reads it believes it or does
not.

Miri is the first tool in this repository that says anything about it. It does
not answer whether the reasoning is right but whether this one run broke one of
the rules. That is less than it sounds, and it is still the difference between a
text and a measurement.

For a unit like this one that means: the green run below is evidence about the
calls the tests make, and about no other one.

### The explanation

Miri does not run under the pinned version. It comes with a nightly toolchain,
and the call says so with `+nightly`. Over the solution of this unit it looks
like this:

```console
$ cargo +nightly miri --version
miri 0.1.0 (771916f902 2026-08-08)
$ cargo +nightly miri test -p unit-10-07-miri
running 7 tests
test exercise::erhoehen_ueber_zeiger_of_nothing_does_nothing ... ok
test exercise::erhoehen_ueber_zeiger_raises_every_value ... ok
test exercise::summe_ueber_zeiger_adds_every_value ... ok
test exercise::summe_ueber_zeiger_of_nothing_is_zero ... ok
test exercise::tauschen_swaps_the_two_values ... ok
test exercise::tauschen_twice_is_where_it_started ... ok
test exercise::the_finished_function_stops_at_the_bound ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s

   Doc-tests unit_10_07_miri

running 1 test
test solutions\10-07-miri\src\lib.rs - lese (line 13) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.77s
```

No finding. Every `unsafe` block of this unit was executed while doing it,
because every function has a test and the doc test of `lese` runs along.

What this run says is: with these calls no access broke one of the rules Miri
knows. What it does not say is that the functions are in order for every input.

### What Miri finds

One step too many, nothing else changed. In the body of `summe_ueber_zeiger` the
loop runs up to and including the length:

```rust
    for schritt in 0..anzahl + 1 {
        summe += unsafe { *zeiger.add(schritt) };
    }
```

The compiler accepts it, and without Miri it mostly runs through. Miri stops:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
running 7 tests
test exercise::summe_ueber_zeiger_adds_every_value ... error: Undefined Behavior: memory access failed: attempting to access 8 bytes, but got alloc76+0x18 which is at or beyond the end of the allocation of size 24 bytes
 --> solutions\10-07-miri\src\lib.rs:48:26
  |
  = note: Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
  = note: this is on thread `exercise::summe_ueber_zeiger_adds_every_value`
  = note: stack backtrace:
          0: unit_10_07_miri::summe_ueber_zeiger
              at solutions\10-07-miri\src\lib.rs:48:27: 48:47
```

Three things stand in that message. The allocation is 24 bytes, which is the
three `i64` out of the test. The access begins at `+0x18`, meaning at 24, and
that is exactly one slot behind the end. And the trace under it says which test
made the call.

Afterwards the original body stands there again, and the run above holds.

### How long it takes

The same test file, once ordinarily and once under Miri, read off at the same
place both times:

```console
$ cargo test -q -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s
```

Seven tests staying under a hundredth of a second ordinarily need a good second
under Miri. On top comes the compiling, because Miri builds with another
toolchain and into a directory of its own; the first time the whole call took
about 38 seconds.

That is a measurement on one machine and not a promise. It says nothing about
larger programs either, and the factor grows with what gets executed. What it
explains is why Miri stands next to the ordinary run and not in its place.

### What Miri does not reach

Miri is a runner. What no test calls does not occur, and how wrong it is makes no
difference. The following branch stands in the body of `summe_ueber_zeiger` and
reads a slot behind the end, but only at 99 values:

```rust
    if anzahl == 99 {
        summe += unsafe { *zeiger.add(anzahl) };
    }
```

No test of this unit calls with 99 values. The run says this about it:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
running 7 tests
test exercise::erhoehen_ueber_zeiger_of_nothing_does_nothing ... ok
test exercise::erhoehen_ueber_zeiger_raises_every_value ... ok
test exercise::summe_ueber_zeiger_adds_every_value ... ok
test exercise::summe_ueber_zeiger_of_nothing_is_zero ... ok
test exercise::tauschen_swaps_the_two_values ... ok
test exercise::tauschen_twice_is_where_it_started ... ok
test exercise::the_finished_function_stops_at_the_bound ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
```

Green, with an access in the code lying outside every allocation. From that
follows the rule for everything else: a green Miri run is a statement about the
executed paths and about no other. Whoever wants to claim more needs more tests
and not more Miri.

Two further limits belong here without being shown. Foreign code Miri does not
execute, so a run says nothing about a call into C; that already stands in
`10-06`. And Miri does not check the `SAFETY` comment but what the block does.

### What these tests do not answer

The check run of this repository does not call Miri. Every run in this text was
sent by hand, and a green check run says nothing about them.

A supposition belonging to this unit that did not hold up belongs here as well.
In the body of exercise 3 the length stands before the pointer. The other order,
meaning `as_mut_ptr` first and `len` afterwards, is a version one might expect
Miri to refuse. It was measured:

```console
$ cargo +nightly miri test -p unit-10-07-miri --test exercise
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s
```

No finding, under the version standing above. From that it does not follow that
the order makes no difference, only that this run says nothing about it. What an
access through a reference does to an older raw pointer hangs on the model Miri
is currently using, and that model is not part of the pinned version.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `lese` stands there finished, and its doc test
is green.

- `summe_ueber_zeiger` adds the values up over a raw pointer
- `tauschen` swaps two values over raw pointers
- `erhoehen_ueber_zeiger` raises every value, written through a pointer

```console
cd units/10-07-miri
cargo test
```

The check run of the repository is enough for this unit. Whoever additionally
wants to look under Miri takes the call from the section "The explanation"; it
needs a nightly toolchain and is not part of the check run.

### Source

    The Rust Reference, chapter 17.2 "Behavior considered undefined",
    https://doc.rust-lang.org/reference/behavior-considered-undefined.html,
    checked against 1.97.1

    The Rustonomicon, chapter 1.3 "Working with Unsafe",
    https://doc.rust-lang.org/nomicon/working-with-unsafe.html,
    checked against 1.97.1

    Miri, https://github.com/rust-lang/miri, run under
    miri 0.1.0 (771916f902 2026-08-08) on rustc 1.99.0-nightly (771916f90
    2026-08-08), not under the pinned version

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
