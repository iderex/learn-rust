# 06-08 build.rs / build.rs

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/06-08-build-rs/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `04-01 Pakete und Crates` und
  `06-02 Dateien lesen und schreiben`. Das Skript ist ein eigenes Programm im
  Paket, und es liest eine Datei.
- Auf dieser Einheit bauen auf: alles, was etwas erzeugt, statt es von Hand zu
  schreiben.
- Beim Antworten so zitieren: `06-08 build.rs`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein Build-Skript gehört zu genau einem Paket. Deshalb liegen `build.rs` und
  `daten/farben.txt` in dieser Einheit und in ihrer Lösung, und nicht nur an
  einer Stelle.
- Die erzeugte Datei liegt unter `OUT_DIR` und damit unter `target/`. Sie steht
  nicht im Repository und wird bei jedem Bau, der das Skript ausführt, neu
  geschrieben.
- Ob das Skript erneut läuft, entscheidet die Zeile mit `rerun-if-changed`. Dass
  sie greift, ist nachgemessen und steht unter "Die Erklärung".

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/06-08-build-rs/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `04-01 Pakete und Crates` and
  `06-02 Dateien lesen und schreiben`. The script is a program of its own inside
  the package, and it reads a file.
- Building on this unit: everything that generates something instead of writing
  it by hand.
- Cite like this when answering: `06-08 build.rs`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A build script belongs to exactly one package. That is why `build.rs` and
  `daten/farben.txt` lie in this unit and in its solution, and not only in one
  place.
- The generated file lies under `OUT_DIR` and therefore under `target/`. It is
  not in the repository and is written anew on every build that runs the script.
- Whether the script runs again is decided by the line with `rerun-if-changed`.
  That it takes hold is measured and stands under "The explanation".

</details>

## Deutsch

### Worum es geht

`build.rs` liegt neben der `Cargo.toml` und ist ein Rust-Programm mit einem
eigenen `main`. Cargo übersetzt es und führt es aus, bevor es das Paket selbst
übersetzt.

Es bekommt seine Angaben über Umgebungsvariablen. `CARGO_MANIFEST_DIR` ist der
Ordner des Pakets, `OUT_DIR` ist ein Ordner unter `target/`, in den es schreiben
darf. Was es dorthin schreibt, holt sich der Code danach mit `include!`.

Und es redet mit Cargo, indem es auf die Standardausgabe schreibt. Zeilen, die
mit `cargo::` anfangen, sind Anweisungen. Die wichtigste davon ist
`cargo::rerun-if-changed`.

### Wofür das gut ist

Manches steht am besten gar nicht im Quelltext. Eine Liste, die aus einer
Datendatei kommt, wird von Hand abgeschrieben falsch, und zwar spätestens beim
zweiten Mal. Ein Skript, das sie erzeugt, kann das nicht falsch abschreiben.

Der Preis dafür ist Bauzeit, und deshalb gibt es `rerun-if-changed`. Ohne diese
Zeile nimmt Cargo an, dass jede Änderung am Paket das Skript betrifft, und führt
es wieder aus. Mit ihr läuft es nur, wenn genau die genannte Datei sich ändert.

Das ist auch der Grund, warum die erzeugte Datei nicht ins Repository gehört.
Läge sie dort, gäbe es zwei Wahrheiten, die Datendatei und das Erzeugnis, und
irgendwann sagen sie Verschiedenes.

### Die Erklärung

Das Skript dieser Einheit liest `daten/farben.txt` und schreibt daraus eine
Konstante.

```rust
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Deutsch: Ohne diese Zeile läuft das Skript bei jeder Änderung am Paket
    // erneut. Mit ihr läuft es nur, wenn die genannte Datei sich ändert.
    println!("cargo::rerun-if-changed=daten/farben.txt");

    let quelle = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .join("daten")
        .join("farben.txt");
    let text = fs::read_to_string(&quelle).expect("daten/farben.txt laesst sich lesen");

    let farben: Vec<&str> = text.lines().map(str::trim).filter(|z| !z.is_empty()).collect();

    let mut erzeugt = String::new();
    erzeugt.push_str(&format!("pub const FARBEN: [&str; {}] = [\n", farben.len()));
    for farbe in &farben {
        erzeugt.push_str(&format!("    {farbe:?},\n"));
    }
    erzeugt.push_str("];\n");

    let ziel = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("farben.rs");
    fs::write(&ziel, erzeugt).expect("die erzeugte Datei laesst sich schreiben");
}
```

Auf der anderen Seite steht in `src/lib.rs` eine einzige Zeile, die das
Erzeugnis hereinholt:

```text
include!(concat!(env!("OUT_DIR"), "/farben.rs"));
```

Danach ist `FARBEN` eine Konstante wie jede andere. Der Code, der sie benutzt,
merkt nicht, dass sie erzeugt wurde.

Dass `rerun-if-changed` wirklich greift, ist nachgemessen. Ein zweiter Bau ohne
Änderung führt das Skript nicht aus, ein Bau nach einer Änderung an der
Datendatei führt es aus:

```console
$ cargo build -p unit-06-08-build-rs
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s

$ printf 'lila\n' >> solutions/06-08-build-rs/daten/farben.txt
$ cargo build -p unit-06-08-build-rs
   Compiling unit-06-08-build-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
```

Der erste Lauf sagt nur "Finished". Der zweite sagt "Compiling", obwohl keine
`.rs`-Datei angefasst wurde, und danach kennt `contains("lila")` die neue Farbe.
Aus der zweiten Ausgabe ist der Pfad hinter der Versionsnummer weggelassen, weil
er auf den Rechner zeigt, auf dem der Lauf stattfand.

### Häufige Fehler

Die erzeugte Datei im Paketordner suchen statt in `OUT_DIR`.

```rust
// Deutsch: So nicht. `farben.rs` liegt nicht neben `lib.rs`, sondern in dem
// Ordner, den Cargo dem Skript zugewiesen hat.
include!("farben.rs");
```

Der Übersetzer sagt dazu:

```text
error: couldn't read `src\farben.rs`: Das System kann die angegebene Datei nicht finden. (os error 2)
 --> src\lib.rs:1:1
  |
1 | include!("farben.rs");
  | ^^^^^^^^^^^^^^^^^^^^^

error: could not compile `inc` (lib) due to 1 previous error
```

Der Lauf fand in einem kleinen eigenen Paket namens `inc` statt, deshalb steht
dieser Name in der letzten Zeile. Der Satz nach dem Doppelpunkt kommt vom
Betriebssystem und steht deshalb in dessen Sprache. Auf einem anders eingestellten Rechner steht dort ein anderer
Satz und dieselbe Nummer.

Die Meldung nennt den Pfad, unter dem gesucht wurde, und der ist der Ordner der
Datei mit dem `include!`. Das Skript hat aber nach `OUT_DIR` geschrieben, und
dieser Ordner steht nur zur Bauzeit fest. Deshalb `concat!` mit `env!("OUT_DIR")`
statt eines Namens, den man hinschreiben kann.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `colours` steht fertig da, und sein Doku-Test ist grün.
`build.rs` und `daten/farben.txt` sind fertig und gehören nicht zur Aufgabe.

- `contains` sagt, ob ein Name in der erzeugten Liste steht
- `longest` gibt den längsten Namen zurück
- `as_line` baut eine Zeile aus allen Namen

```console
cd units/06-08-build-rs
cargo test
```

### Quelle

    Cargo Book, Kapitel 3 "Cargo Reference", Abschnitt 3.8 "Build Scripts",
    https://doc.rust-lang.org/cargo/reference/build-scripts.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`build.rs` lies next to the `Cargo.toml` and is a Rust program with a `main` of
its own. Cargo compiles it and runs it before it compiles the package itself.

It gets its information through environment variables. `CARGO_MANIFEST_DIR` is
the folder of the package, `OUT_DIR` is a folder under `target/` it is allowed to
write into. What it writes there is fetched by the code afterwards with
`include!`.

And it talks to Cargo by writing to standard output. Lines starting with
`cargo::` are instructions. The most important of them is
`cargo::rerun-if-changed`.

### What it is good for

Some things are best not in the source at all. A list that comes out of a data
file is copied wrongly by hand, at the latest on the second try. A script that
generates it cannot copy it wrongly.

The price for that is build time, and that is why `rerun-if-changed` exists.
Without that line Cargo assumes every change to the package concerns the script
and runs it again. With it, it runs only when exactly the named file changes.

That is also the reason the generated file does not belong in the repository. If
it lay there, there would be two truths, the data file and the product, and at
some point they say different things.

### The explanation

The script of this unit reads `daten/farben.txt` and writes a constant out of it.

```rust
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Deutsch: Ohne diese Zeile läuft das Skript bei jeder Änderung am Paket
    // erneut. Mit ihr läuft es nur, wenn die genannte Datei sich ändert.
    println!("cargo::rerun-if-changed=daten/farben.txt");

    let quelle = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .join("daten")
        .join("farben.txt");
    let text = fs::read_to_string(&quelle).expect("daten/farben.txt laesst sich lesen");

    let farben: Vec<&str> = text.lines().map(str::trim).filter(|z| !z.is_empty()).collect();

    let mut erzeugt = String::new();
    erzeugt.push_str(&format!("pub const FARBEN: [&str; {}] = [\n", farben.len()));
    for farbe in &farben {
        erzeugt.push_str(&format!("    {farbe:?},\n"));
    }
    erzeugt.push_str("];\n");

    let ziel = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("farben.rs");
    fs::write(&ziel, erzeugt).expect("die erzeugte Datei laesst sich schreiben");
}
```

On the other side one single line in `src/lib.rs` fetches the product in:

```text
include!(concat!(env!("OUT_DIR"), "/farben.rs"));
```

After that `FARBEN` is a constant like any other. The code using it does not
notice that it was generated.

That `rerun-if-changed` really takes hold is measured. A second build without a
change does not run the script, a build after a change to the data file runs it:

```console
$ cargo build -p unit-06-08-build-rs
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s

$ printf 'lila\n' >> solutions/06-08-build-rs/daten/farben.txt
$ cargo build -p unit-06-08-build-rs
   Compiling unit-06-08-build-rs v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
```

The first run says only "Finished". The second says "Compiling" although no
`.rs` file was touched, and afterwards `contains("lila")` knows the new colour.
Out of the second output the path behind the version number is left away, because
it points at the machine the run happened on.

### Common mistakes

Looking for the generated file in the package folder instead of in `OUT_DIR`.

```rust
// Deutsch: So nicht. `farben.rs` liegt nicht neben `lib.rs`, sondern in dem
// Ordner, den Cargo dem Skript zugewiesen hat.
include!("farben.rs");
```

The compiler answers:

```text
error: couldn't read `src\farben.rs`: Das System kann die angegebene Datei nicht finden. (os error 2)
 --> src\lib.rs:1:1
  |
1 | include!("farben.rs");
  | ^^^^^^^^^^^^^^^^^^^^^

error: could not compile `inc` (lib) due to 1 previous error
```

The run happened in a small package of its own called `inc`, which is why that
name stands in the last line. The sentence after the colon comes from the
operating system and therefore stands in its language. On a machine set up differently another sentence stands
there, with the same number.

The message names the path it looked under, and that is the folder of the file
carrying the `include!`. The script, however, wrote to `OUT_DIR`, and that folder
is only fixed at build time. Hence `concat!` with `env!("OUT_DIR")` instead of a
name that can be written down.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `colours` stands there finished, and its doc
test is green. `build.rs` and `daten/farben.txt` are finished and are not part of
the exercise.

- `contains` says whether a name is in the generated list
- `longest` returns the longest name
- `as_line` builds one line out of all names

```console
cd units/06-08-build-rs
cargo test
```

### Source

    Cargo Book, chapter 3 "Cargo Reference", section 3.8 "Build Scripts",
    https://doc.rust-lang.org/cargo/reference/build-scripts.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
