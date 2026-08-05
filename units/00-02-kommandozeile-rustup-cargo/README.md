# 00-02 Kommandozeile, rustup und cargo / The command line, rustup and cargo

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/00-02-kommandozeile-rustup-cargo/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `00-01 Was ein Programm ist und was ein Compiler tut`.
- Auf dieser Einheit bauen auf: `00-03 Das erste Projekt und eine Fehlermeldung
  lesen` und alles Weitere, denn ab hier wird jede Einheit mit cargo gestartet.
- Beim Antworten so zitieren: `00-02 Kommandozeile, rustup und cargo`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Wer hier antwortet, redet mit jemandem, der die Kommandozeile vielleicht zum
  ersten Mal öffnet. Ein Befehl wird erklärt, bevor er getippt werden soll.

</details>

## Deutsch

### Worum es geht

Die Kommandozeile ist ein Fenster, in das man einen Befehl tippt und aus dem
Text zurückkommt. Sie hat immer ein Verzeichnis, in dem sie gerade steht, und
fast jeder Befehl bezieht sich darauf.

Zwei Werkzeuge kommen hier dazu. rustup verwaltet die Übersetzer, also welche
Fassung von Rust auf dem Rechner liegt und welche gerade gilt. cargo verwaltet
die Projekte, also den Ordnerbau, das Übersetzen, das Starten und die Tests.

### Wofür das gut ist

Ohne diese beiden müsste man jede Datei einzeln an `rustc` übergeben und jede
fremde Bibliothek von Hand danebenlegen. Das geht bei einem Programm aus einer
Datei und hört danach auf.

rustup nimmt einem außerdem die Frage ab, welche Fassung gerade gilt. In diesem
Repository steht sie in `rust-toolchain.toml`, und rustup holt und benutzt genau
diese, sobald ein Befehl aus dem Repository heraus abgeschickt wird. Wer die
Zahlen in den Quellenangaben ernst nimmt, braucht das: eine Angabe ist gegen
eine Fassung geprüft und nicht gegen irgendeine.

### Die Erklärung

Drei Befehle reichen für den Anfang. Der erste sagt, welcher Übersetzer gilt,
der zweite, welches cargo dazugehört, der dritte legt ein Projekt an.

```console
rustc --version
rustc 1.97.1 (8bab26f4f 2026-07-14)

cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)

cargo new hallo
```

`cargo new hallo` legt einen Ordner `hallo` an und darin zwei Dinge:
`hallo/Cargo.toml` mit dem Namen des Projekts und seinen Abhängigkeiten, und
`hallo/src/main.rs` mit einem fertigen kleinen Programm. Mehr braucht ein
Projekt nicht, um zu laufen.

Die beiden Nummern oben sind dieselbe, und das ist kein Zufall: cargo und der
Übersetzer kommen als ein Satz.

### Häufige Fehler

Der häufigste Fehler am Anfang ist, im falschen Verzeichnis zu stehen. cargo
sucht die Datei `Cargo.toml` im aktuellen Verzeichnis und in jedem darüber. Wer
`cargo build` in einem leeren Ordner aufruft, bekommt das hier.

```console
cargo build
```

cargo sagt dazu:

```text
error: could not find `Cargo.toml` in `<das Verzeichnis, in dem der Befehl lief>` or any parent directory
```

Der Pfad in dieser Ausgabe ist ersetzt, damit hier kein Verzeichnis von einem
fremden Rechner steht. Der Rest ist die echte Ausgabe.

Zwei Dinge stehen darin. Erstens die Antwort: mit `cd hallo` in das Projekt
wechseln und den Befehl noch einmal abschicken. Zweitens etwas, das später
wichtig wird: diese Meldung hat keine Nummer. Meldungen des Übersetzers haben
eine, wie `error[E0308]`, und lassen sich damit nachschlagen. Meldungen von
cargo haben keine.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.

- `main_file` gibt den Pfad der Quelltextdatei eines frischen Projekts zurück
- `manifest_file` gibt den Pfad der Datei mit Namen und Abhängigkeiten zurück

```console
cd units/00-02-kommandozeile-rustup-cargo
cargo test
```

### Quelle

    Buch, Kapitel 1 "Getting Started", Abschnitt 1.3 "Hello, Cargo!",
    https://doc.rust-lang.org/book/ch01-03-hello-cargo.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

The command line is a window you type a command into and text comes back out
of. It always stands in some directory, and nearly every command relates to it.

Two tools join here. rustup manages the compilers, meaning which version of Rust
is on the machine and which one currently applies. cargo manages the projects,
meaning the folder layout, the compiling, the starting and the tests.

### What it is good for

Without these two you would have to hand every file to `rustc` one by one and
put every foreign library next to it by hand. That works for a program made of
one file and stops there.

rustup also takes the question of which version applies off your hands. In this
repository it stands in `rust-toolchain.toml`, and rustup fetches and uses
exactly that one as soon as a command is sent from inside the repository.
Whoever takes the numbers in the source references seriously needs this: a
reference is checked against one version and not against any version.

### The explanation

Three commands are enough to begin with. The first says which compiler applies,
the second which cargo belongs to it, the third creates a project.

```console
rustc --version
rustc 1.97.1 (8bab26f4f 2026-07-14)

cargo --version
cargo 1.97.1 (c980f4866 2026-06-30)

cargo new hallo
```

`cargo new hallo` creates a folder `hallo` and two things inside it:
`hallo/Cargo.toml` with the name of the project and its dependencies, and
`hallo/src/main.rs` with a finished small program. A project needs no more than
that to run.

The two numbers above are the same, and that is no accident: cargo and the
compiler come as one set.

### Common mistakes

The most common mistake at the start is standing in the wrong directory. cargo
looks for the file `Cargo.toml` in the current directory and in every directory
above it. Whoever calls `cargo build` in an empty folder gets this.

```console
cargo build
```

cargo answers:

```text
error: could not find `Cargo.toml` in `<the directory the command ran in>` or any parent directory
```

The path in this output is replaced, so that no directory from somebody else's
machine stands here. The rest is the real output.

Two things are in it. First the answer: change into the project with `cd hallo`
and send the command again. Second something that matters later: this message
has no number. Messages from the compiler have one, like `error[E0308]`, and can
be looked up with it. Messages from cargo have none.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are.

- `main_file` returns the path of the source file of a fresh project
- `manifest_file` returns the path of the file holding the name and the
  dependencies

```console
cd units/00-02-kommandozeile-rustup-cargo
cargo test
```

### Source

    Book, chapter 1 "Getting Started", section 1.3 "Hello, Cargo!",
    https://doc.rust-lang.org/book/ch01-03-hello-cargo.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
