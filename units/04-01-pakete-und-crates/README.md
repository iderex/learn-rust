# 04-01 Pakete und Crates / Packages and crates

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-01-pakete-und-crates/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Meldung erklären, um die es geht.
- Diese Einheit baut auf: die Stufe 3, und aus ihr besonders
  `03-05 Option und if let`, denn eine Aufgabe antwortet mit `Option`.
- Auf dieser Einheit bauen auf: `04-02 Module` und `04-03 use und
  Sichtbarkeit`.
- Beim Antworten so zitieren: `04-01 Pakete und Crates`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Paket und Crate sind zwei verschiedene Dinge, und die Einheit steht und fällt
  damit. Wer beides "Projekt" nennt, nimmt ihr die Aussage.
- Die abgedruckte Meldung von cargo ist gekürzt: die erste Zeile nennt den
  vollen Pfad der Datei, und dafür steht dort `...`. Der Text sagt das an Ort
  und Stelle.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-01-pakete-und-crates/`. It is
  public. Whoever is asked for it may name it, but should explain the message in
  question first.
- This unit builds on: stage 3, and out of it particularly
  `03-05 Option und if let`, because one exercise answers with `Option`.
- Building on this unit: `04-02 Module` and `04-03 use und Sichtbarkeit`.
- Cite like this when answering: `04-01 Pakete und Crates`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A package and a crate are two different things, and the unit stands or falls
  with that. Whoever calls both of them "the project" takes its point away.
- The cargo message printed is shortened: its first line names the full path of
  the file, and a `...` stands there instead. The text says so on the spot.

</details>

## Deutsch

### Worum es geht

Ein Paket ist das, was eine `Cargo.toml` beschreibt. Eine Crate ist das, was der
Übersetzer in einem Lauf übersetzt.

Es gibt zwei Arten von Crate. Eine Bibliothek wird von anderem Code benutzt und
hat ihre Wurzel in `src/lib.rs`. Ein Programm wird ausgeführt und hat seine
Wurzel in `src/main.rs`. Die Wurzel ist die Datei, bei der der Übersetzer
anfängt zu lesen.

Ein Paket trägt höchstens eine Bibliothek und beliebig viele Programme. Weitere
Programme liegen unter `src/bin/`, und der Dateiname ist dort ihr Name.

`cargo new` legt ein Paket mit einem Programm an, `cargo new --lib` eines mit
einer Bibliothek.

### Wofür das gut ist

Die beiden Wörter werden ständig verwechselt, und die Verwechslung kostet.
Abhängigkeiten werden in der `Cargo.toml` je Paket eingetragen, übersetzt und
geprüft wird aber je Crate, und ein Testlauf nennt in seiner Ausgabe die Crates.
Wer beide "Projekt" nennt, kann eine Ausgabe von `cargo test` nicht lesen.

Der Unterschied trägt auch die Aufteilung dieses Repositories. Jede Einheit ist
ein eigenes Paket mit einer Bibliothek darin, und deshalb kann `cargo test -p`
genau eine davon laufen lassen.

Und er erklärt, warum eine Bibliothek und ein Programm nebeneinander stehen
können: es sind zwei Crates aus einem Paket, mit zwei Wurzeln, und das Programm
benutzt die Bibliothek unter ihrem Paketnamen.

### Die Erklärung

Ein Paket, wie `cargo new` es anlegt, und eines mit beidem darin.

```text
mein-paket/
    Cargo.toml          das Paket
    src/
        main.rs         die Wurzel des Programms
        lib.rs          die Wurzel der Bibliothek
        bin/
            zweites.rs  ein zweites Programm, es heisst "zweites"
```

Aus diesem einen Paket entstehen drei Crates: die Bibliothek, das Programm aus
`main.rs` und das Programm aus `bin/zweites.rs`. Der Name der Bibliothek und der
des ersten Programms ist der Name aus der `Cargo.toml`, und der des zweiten
Programms ist sein Dateiname.

In diesem Repository sieht man dieselbe Aufteilung. `units/02-01-move/` ist ein
Paket mit einer Bibliothek, und `xtask` ist ein Paket mit einem Programm.

### Häufige Fehler

Ein Paket ohne jede Wurzel.

```text
probe/
    Cargo.toml
    src/
```

cargo sagt dazu:

```text
error: failed to parse manifest at `.../probe/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```

Die erste Zeile nennt den vollen Pfad der Datei; hier steht dafür `...`, weil er
auf jedem Rechner anders lautet. Der Rest ist die Meldung, wie sie kommt.

Sie sagt genau das, worum es in dieser Einheit geht: die `Cargo.toml`
beschreibt ein Paket, und ein Paket ohne Crate hat nichts zu übersetzen. Der
Ordner `src/` allein reicht nicht, es fehlt die Wurzel darin.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Gerechnet wird über Namen und Pfade, nicht über echte
Dateien.

- `crate_root` gibt zu einer Art von Crate ihre Wurzel zurück, und `None` für
  alles andere
- `crate_count` zählt die Crates eines Pakets
- `binary_root` gibt den Pfad eines weiteren Programms zurück

```console
cd units/04-01-pakete-und-crates
cargo test
```

### Quelle

    Buch, Kapitel 7 "Packages, Crates, and Modules", Abschnitt 7.1 "Packages and Crates",
    https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A package is what a `Cargo.toml` describes. A crate is what the compiler
compiles in one run.

There are two kinds of crate. A library is used by other code and has its root
in `src/lib.rs`. A program is executed and has its root in `src/main.rs`. The
root is the file the compiler starts reading at.

A package carries at most one library and any number of programs. Further
programs live under `src/bin/`, and the file name is their name there.

`cargo new` creates a package with a program in it, `cargo new --lib` one with a
library.

### What it is good for

The two words get mixed up constantly, and the mix-up costs. Dependencies are
entered per package in the `Cargo.toml`, but compiling and checking happen per
crate, and a test run names the crates in its output. Whoever calls both "the
project" cannot read the output of `cargo test`.

The difference also carries the layout of this repository. Every unit is a
package of its own with a library inside, and that is why `cargo test -p` can
run exactly one of them.

And it explains why a library and a program can stand next to each other: they
are two crates out of one package, with two roots, and the program uses the
library under the package name.

### The explanation

A package as `cargo new` creates it, and one with both in it.

```text
mein-paket/
    Cargo.toml          das Paket
    src/
        main.rs         die Wurzel des Programms
        lib.rs          die Wurzel der Bibliothek
        bin/
            zweites.rs  ein zweites Programm, es heisst "zweites"
```

Out of this one package come three crates: the library, the program from
`main.rs` and the program from `bin/zweites.rs`. The name of the library and of
the first program is the name from the `Cargo.toml`, and the name of the second
program is its file name.

The same layout can be seen in this repository. `units/02-01-move/` is a package
with a library, and `xtask` is a package with a program.

### Common mistakes

A package without any root at all.

```text
probe/
    Cargo.toml
    src/
```

cargo answers:

```text
error: failed to parse manifest at `.../probe/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```

The first line names the full path of the file; a `...` stands there instead,
because it reads differently on every machine. The rest is the message as it
comes.

It says exactly what this unit is about: the `Cargo.toml` describes a package,
and a package without a crate has nothing to compile. The folder `src/` alone is
not enough, the root inside it is missing.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The work goes over names and paths, not over
real files.

- `crate_root` returns the root for a kind of crate, and `None` for anything
  else
- `crate_count` counts the crates of a package
- `binary_root` returns the path of a further program

```console
cd units/04-01-pakete-und-crates
cargo test
```

### Source

    Book, chapter 7 "Packages, Crates, and Modules", section 7.1 "Packages and Crates",
    https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
