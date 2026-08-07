# 06-01 Argumente von der Kommandozeile / Command line arguments

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/06-01-argumente-von-der-kommandozeile/`. Sie ist öffentlich. Wer
  nach ihr gefragt wird, kann sie nennen, sollte aber zuerst erklären, woran der
  Aufruf gescheitert ist.
- Diese Einheit baut auf: `04-07 panic und Result` und `05-05 Tests und ihr
  Aufbau`. Der Fehlerweg kommt aus der Stufe 4, die Tests aus der Stufe 5.
- Auf dieser Einheit bauen auf: die übrige Stufe 6, denn dort liest und schreibt
  dasselbe Werkzeug Dateien und Umgebungsvariablen.
- Beim Antworten so zitieren: `06-01 Argumente von der Kommandozeile`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Das erste Argument ist nicht das erste Argument des Aufrufers. Es ist der Weg,
  unter dem das Programm gestartet wurde, und die Angaben des Aufrufers fangen
  erst danach an.
- Ein fehlendes Argument ist kein Fehler im Programm. Wer hier `panic!` oder
  einen Zugriff über den Rand vorschlägt, sagt bitte, was der Aufrufer aus der
  Meldung ablesen soll.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/06-01-argumente-von-der-kommandozeile/`. It is public. Whoever is
  asked for it may name it, but should first explain what the call failed on.
- This unit builds on: `04-07 panic und Result` and `05-05 Tests und ihr
  Aufbau`. The error path comes from stage 4, the tests from stage 5.
- Building on this unit: the rest of stage 6, because there the same tool reads
  and writes files and environment variables.
- Cite like this when answering: `06-01 Argumente von der Kommandozeile`, plus
  the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- The first argument is not the caller's first argument. It is the way the
  program was started under, and what the caller gave begins after it.
- A missing argument is not a fault in the program. Whoever suggests `panic!`
  or an access past the end here, please say what the caller is meant to read
  off the message.

</details>

## Deutsch

### Worum es geht

Ein Programm wird mit Wörtern hinter seinem Namen aufgerufen. `std::env::args()`
gibt genau diese Wörter, eines nach dem anderen, und `collect()` sammelt sie in
einen `Vec<String>`.

Das erste davon gehört nicht dem Aufrufer. Es ist der Weg, unter dem das
Programm gestartet wurde, oft mit Ordnern davor. Was der Aufrufer geschrieben
hat, fängt beim zweiten an.

Damit steht die Frage im Raum, die diese Einheit beantwortet: was tut ein
Programm, wenn dort weniger steht, als es braucht? Die Antwort ist nicht
abbrechen, sondern sagen, was fehlt.

### Wofür das gut ist

Eine Liste, die zu kurz ist, merkt man erst beim Zugriff. `args[1]` auf einer
Liste mit einem Element bricht das Programm ab, und der Aufrufer liest eine
Meldung über einen Index, den er nie geschrieben hat.

Er hat aber einen Fehler gemacht, den er beheben kann, und dafür braucht er zwei
Angaben: was fehlt, und wie der Aufruf aussehen müsste. Beides kann das Programm
sagen, denn es weiß beides.

Deshalb wird die Länge geprüft, bevor zugegriffen wird, und die Antwort auf
einen falschen Aufruf geht nach `stderr` und nicht nach `stdout`. Was das
Programm ausgibt, wenn es tut, wozu es da ist, und was es meldet, wenn es das
nicht kann, sind zwei verschiedene Ströme.

### Die Erklärung

Ein Programm, das zwei Angaben braucht, sie zählt, und einen zu kurzen Aufruf
beantwortet.

```rust
use std::env;

// Deutsch: Der Name, unter dem gestartet wurde, ohne den Pfad davor.
fn name(weg: &str) -> &str {
    match weg.rfind(['/', '\\']) {
        Some(stelle) => &weg[stelle + 1..],
        None => weg,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("es fehlt etwas");
        eprintln!("Aufruf: {} <Muster> <Datei>", name(&args[0]));
        std::process::exit(2);
    }

    println!("Muster: {}", args[1]);
    println!("Datei: {}", args[2]);
}
```

Zweimal gestartet, einmal richtig und einmal zu kurz, gibt das aus:

```text
$ suchen.exe wort buch.txt
Muster: wort
Datei: buch.txt

$ suchen.exe wort
es fehlt etwas
Aufruf: suchen.exe <Muster> <Datei>
```

`args.len() != 3` heißt: der Name des Programms und zwei Angaben. Die Zahl 3
steht deshalb da, wo die meisten 2 erwarten, und das ist der Grund, warum das
erste Element im Text immer wieder vorkommt. `name` schneidet den Pfad ab, damit
die Zeile den Namen zeigt und nicht den halben Ordnerbaum.

### Häufige Fehler

Auf `args[1]` zugreifen, ohne vorher zu zählen.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Muster: {}", args[1]);
}
```

Das übersetzt. Ohne Argument gestartet, sagt das Programm:

```text

thread 'main' (30932) panicked at ohne_pruefung.rs:6:32:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Die Meldung stimmt und hilft trotzdem niemandem: `the len is 1` ist der Name des
Programms allein, und `the index is 1` ist die Angabe, die der Aufrufer nicht
geschrieben hat. Nichts davon sagt ihm, was er tippen soll. Die Zahl in Klammern
ist die Nummer des laufenden Vorgangs und bei jedem Lauf eine andere.

Die Antwort ist nicht, `args.get(1)` zu nehmen und bei `None` still nichts zu
tun. Ein Programm, das ohne Meldung nichts tut, sieht aus wie eines, das fertig
ist.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Geprüft wird gegen Listen, die im Test gebaut werden, damit
die Tests nicht davon abhängen, wie der Testlauf selbst aufgerufen wurde.

- `parse` liest Muster und Datei heraus und sagt sonst, was fehlt oder zu viel
  ist
- `usage` schreibt die Zeile mit dem richtigen Aufruf
- `answer` setzt beides zu der Antwort zusammen, die ein falscher Aufruf bekommt

```console
cd units/06-01-argumente-von-der-kommandozeile
cargo test
```

### Quelle

    Buch, Kapitel 12 "An I/O Project: Building a Command Line Program", Abschnitt 12.1 "Accepting Command Line Arguments",
    https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A program is called with words behind its name. `std::env::args()` gives exactly
those words, one after another, and `collect()` gathers them into a
`Vec<String>`.

The first of them does not belong to the caller. It is the way the program was
started under, often with folders in front of it. What the caller wrote begins
at the second.

That puts the question this unit answers on the table: what does a program do
when there is less there than it needs? The answer is not to abort, but to say
what is missing.

### What it is good for

A list that is too short only shows up at the access. `args[1]` on a list with
one element aborts the program, and the caller reads a message about an index
they never wrote.

They did make a mistake they can fix, though, and for that they need two things:
what is missing, and what the call would have to look like. The program can say
both, because it knows both.

That is why the length is checked before the access, and why the answer to a
wrong call goes to `stderr` and not to `stdout`. What a program prints when it
does what it is there for, and what it reports when it cannot, are two different
streams.

### The explanation

A program that needs two things, counts them, and answers a call that is too
short.

```rust
use std::env;

// Deutsch: Der Name, unter dem gestartet wurde, ohne den Pfad davor.
fn name(weg: &str) -> &str {
    match weg.rfind(['/', '\\']) {
        Some(stelle) => &weg[stelle + 1..],
        None => weg,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("es fehlt etwas");
        eprintln!("Aufruf: {} <Muster> <Datei>", name(&args[0]));
        std::process::exit(2);
    }

    println!("Muster: {}", args[1]);
    println!("Datei: {}", args[2]);
}
```

Started twice, once right and once too short, that prints:

```text
$ suchen.exe wort buch.txt
Muster: wort
Datei: buch.txt

$ suchen.exe wort
es fehlt etwas
Aufruf: suchen.exe <Muster> <Datei>
```

`args.len() != 3` means: the name of the program and two things. The number 3
therefore stands where most people expect 2, and that is why the first element
keeps turning up in the text. `name` cuts the path off, so that the line shows
the name and not half the folder tree.

### Common mistakes

Reaching for `args[1]` without counting first.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Muster: {}", args[1]);
}
```

That compiles. Started with no argument, the program says:

```text

thread 'main' (30932) panicked at ohne_pruefung.rs:6:32:
index out of bounds: the len is 1 but the index is 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The message is right and helps nobody all the same: `the len is 1` is the name
of the program on its own, and `the index is 1` is the thing the caller did not
write. None of it tells them what to type. The number in brackets is the number
of the running process and a different one on every run.

The answer is not to take `args.get(1)` and quietly do nothing on `None`. A
program that does nothing without a word looks like one that is finished.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. The checks run against lists built inside the
test, so that the tests do not depend on how the test run itself was called.

- `parse` reads pattern and file out and otherwise says what is missing or too
  much
- `usage` writes the line with the right call
- `answer` puts both together into the answer a wrong call gets

```console
cd units/06-01-argumente-von-der-kommandozeile
cargo test
```

### Source

    Book, chapter 12 "An I/O Project: Building a Command Line Program", section 12.1 "Accepting Command Line Arguments",
    https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
