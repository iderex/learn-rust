# 06-02 Dateien lesen und schreiben / Reading and writing files

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter
  `solutions/06-02-dateien-lesen-und-schreiben/`. Sie ist öffentlich. Wer nach
  ihr gefragt wird, kann sie nennen, sollte aber zuerst die Fehlermeldung
  erklären, um die es geht.
- Diese Einheit baut auf: `04-07 panic! und Result` und
  `04-08 From, Into und der Operator ?`. Der Umgang mit `Result` kommt von dort,
  hier kommt nur die Welt außerhalb des Programms dazu.
- Auf dieser Einheit bauen auf: der Rest der Stufe 6 und alles, was etwas
  einliest, statt es im Quelltext stehen zu haben.
- Beim Antworten so zitieren: `06-02 Dateien lesen und schreiben`, dazu die
  Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- "Die Datei gibt es nicht" ist ein `io::ErrorKind` unter vielen. Wer jeden
  Fehler gleich behandelt, verschluckt die anderen, und der Test mit dem
  Nullbyte im Pfad ist genau dagegen da.
- Welchen `ErrorKind` ein Zugriff liefert, hängt am Betriebssystem. Auf dem
  Rechner, auf dem diese Einheit gebaut wurde, gibt `read_to_string` auf einen
  Ordner `NotFound` und nicht `IsADirectory`. Wer eine Aussage darüber
  braucht, misst sie nach, statt sie anzunehmen.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at
  `solutions/06-02-dateien-lesen-und-schreiben/`. It is public. Whoever is asked
  for it may name it, but should explain the compiler message in question first.
- This unit builds on: `04-07 panic! und Result` and
  `04-08 From, Into und der Operator ?`. Handling `Result` comes from there,
  what is added here is only the world outside the program.
- Building on this unit: the rest of stage 6 and everything that reads something
  in instead of having it in the source.
- Cite like this when answering: `06-02 Dateien lesen und schreiben`, plus the
  heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- "The file is not there" is one `io::ErrorKind` among many. Whoever treats
  every error alike swallows the others, and the test with the NUL byte in the
  path exists against exactly that.
- Which `ErrorKind` an access yields hangs on the operating system. On the
  machine this unit was built on, `read_to_string` on a folder gives `NotFound`
  and not `IsADirectory`. Whoever needs a statement about that measures it
  rather than assuming it.

</details>

## Deutsch

### Worum es geht

Eine Datei liegt außerhalb des Programms. Ob es sie gibt, ob sie lesbar ist und
ob sie es gleich noch sein wird, entscheidet nicht der Quelltext, sondern das
Betriebssystem. Deshalb gibt in `std::fs` fast alles ein `io::Result` zurück.

`fs::read_to_string` liest eine ganze Datei in einen `String`. `fs::write`
schreibt einen Text in eine Datei und ersetzt dabei, was vorher darin stand. Wer
etwas anhängen will statt zu ersetzen, geht über `fs::OpenOptions` und sagt dort,
was er vorhat.

Ein Pfad ist ein `Path` und kein `String`. Er sieht auf jedem Betriebssystem
anders aus und ist nicht überall gültiger Text, und deshalb gibt es dafür einen
eigenen Typ.

### Wofür das gut ist

Ein `io::Error` sagt nicht nur, dass etwas schiefging, sondern über `kind()`
auch, was. `NotFound` heißt, dass es die Datei nicht gibt.
`PermissionDenied` heißt, dass es sie gibt und man nicht darf. Das sind zwei
verschiedene Aussagen, und ein Programm, das beide gleich behandelt, sagt dem
Benutzer etwas Falsches.

Eine fehlende Datei ist oft gar kein Fehler. Eine Einstellungsdatei, die es beim
ersten Start noch nicht gibt, ist ein normaler Fall mit einer normalen Antwort,
nämlich den Voreinstellungen. Genau dann wird `NotFound` beantwortet und alles
andere weitergereicht.

Das Weiterreichen ist der Teil, den man leicht wegkürzt. Ein `unwrap_or_else`
über den ganzen Fehler sieht sparsam aus und macht aus jedem Grund denselben.
Danach ist die Meldung "keine Berechtigung" verschwunden, und niemand erfährt
mehr, warum die Voreinstellungen greifen.

### Die Erklärung

Ein Programm, das schreibt, liest, die Datei wegräumt und danach noch einmal
liest, ohne abzubrechen.

```rust
use std::fs;
use std::io;
use std::path::Path;

// Deutsch: Ein fehlender Pfad ist ein Fall und kein Absturz. `NotFound` wird
// beantwortet, jeder andere Fehler wird weitergereicht.
fn read_or(pfad: &Path, ersatz: &str) -> io::Result<String> {
    match fs::read_to_string(pfad) {
        Ok(inhalt) => Ok(inhalt),
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(ersatz.to_string()),
        Err(fehler) => Err(fehler),
    }
}

fn main() -> io::Result<()> {
    let pfad = Path::new("notizen.txt");

    fs::write(pfad, "erste Zeile\nzweite Zeile\n")?;
    println!("{}", read_or(pfad, "nichts da")?.lines().count());

    fs::remove_file(pfad)?;
    println!("{}", read_or(pfad, "nichts da")?);

    Ok(())
}
```

Das Programm gibt aus:

```text
2
nichts da
```

Der dritte Zweig ist der wichtige. Er sieht überflüssig aus, denn er gibt den
Fehler unverändert weiter. Ohne ihn wäre der zweite Zweig kein Zweig mehr,
sondern die Antwort auf jeden Fehler, und `main` bekäme nie einen zu sehen.

### Häufige Fehler

Einen Pfad als `&str` übergeben, wo ein `&Path` steht.

```rust
use std::fs;
use std::io;
use std::path::Path;

fn read_text(pfad: &Path) -> io::Result<String> {
    fs::read_to_string(pfad)
}

fn main() -> io::Result<()> {
    let inhalt = read_text("notizen.txt")?;

    println!("{inhalt}");
    Ok(())
}
```

Der Übersetzer sagt dazu:

```text
error[E0308]: mismatched types
  --> lesen.rs:10:28
   |
10 |     let inhalt = read_text("notizen.txt")?;
   |                  --------- ^^^^^^^^^^^^^ expected `&Path`, found `&str`
   |                  |
   |                  arguments to this function are incorrect
   |
   = note: expected reference `&Path`
              found reference `&'static str`
note: function defined here
  --> lesen.rs:5:4
   |
 5 | fn read_text(pfad: &Path) -> io::Result<String> {
   |    ^^^^^^^^^ -----------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Verwirrend daran ist, dass `fs::read_to_string("notizen.txt")` durchgeht. Das
liegt daran, dass diese Funktion nicht `&Path` verlangt, sondern alles, was sich
in einen Pfad verwandeln lässt. `Path::new("notizen.txt")` macht aus dem Text
einen Pfad und behebt es.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. `read_text` steht fertig da, und sein Doku-Test ist grün.

- `read_or` liest eine Datei und antwortet auf die fehlende mit einem Ersatz
- `append_line` hängt eine Zeile an, statt zu ersetzen
- `copy_lines` schreibt die passenden Zeilen einer Datei in eine zweite

```console
cd units/06-02-dateien-lesen-und-schreiben
cargo test
```

### Quelle

    Buch, Kapitel 12 "An I/O Project: Building a Command Line Program",
    Abschnitt 12.2 "Reading a File",
    https://doc.rust-lang.org/book/ch12-02-reading-a-file.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

A file lies outside the program. Whether it exists, whether it is readable and
whether it will still be readable a moment later is not decided by the source but
by the operating system. That is why almost everything in `std::fs` returns an
`io::Result`.

`fs::read_to_string` reads a whole file into a `String`. `fs::write` writes a
text into a file and replaces whatever stood in it before. Whoever wants to
append instead of replacing goes through `fs::OpenOptions` and says there what
they are up to.

A path is a `Path` and not a `String`. It looks different on every operating
system and is not valid text everywhere, and that is why it has a type of its
own.

### What it is good for

An `io::Error` says not only that something went wrong but, through `kind()`,
also what. `NotFound` means the file is not there. `PermissionDenied` means it is
there and you are not allowed. Those are two different statements, and a program
treating both alike tells the user something false.

A missing file is often not an error at all. A settings file that is not there
yet on the first start is a normal case with a normal answer, namely the
defaults. That is exactly when `NotFound` gets answered and everything else gets
passed on.

Passing on is the part that is easily cut away. An `unwrap_or_else` over the
whole error looks frugal and makes every reason into the same one. After that the
message "no permission" has disappeared, and nobody finds out any more why the
defaults took hold.

### The explanation

A program that writes, reads, clears the file away and then reads once more,
without aborting.

```rust
use std::fs;
use std::io;
use std::path::Path;

// Deutsch: Ein fehlender Pfad ist ein Fall und kein Absturz. `NotFound` wird
// beantwortet, jeder andere Fehler wird weitergereicht.
fn read_or(pfad: &Path, ersatz: &str) -> io::Result<String> {
    match fs::read_to_string(pfad) {
        Ok(inhalt) => Ok(inhalt),
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(ersatz.to_string()),
        Err(fehler) => Err(fehler),
    }
}

fn main() -> io::Result<()> {
    let pfad = Path::new("notizen.txt");

    fs::write(pfad, "erste Zeile\nzweite Zeile\n")?;
    println!("{}", read_or(pfad, "nichts da")?.lines().count());

    fs::remove_file(pfad)?;
    println!("{}", read_or(pfad, "nichts da")?);

    Ok(())
}
```

The program prints:

```text
2
nichts da
```

The third arm is the important one. It looks superfluous, because it passes the
error on unchanged. Without it the second arm would no longer be an arm but the
answer to every error, and `main` would never get to see one.

### Common mistakes

Passing a path as a `&str` where a `&Path` stands.

```rust
use std::fs;
use std::io;
use std::path::Path;

fn read_text(pfad: &Path) -> io::Result<String> {
    fs::read_to_string(pfad)
}

fn main() -> io::Result<()> {
    let inhalt = read_text("notizen.txt")?;

    println!("{inhalt}");
    Ok(())
}
```

The compiler answers:

```text
error[E0308]: mismatched types
  --> lesen.rs:10:28
   |
10 |     let inhalt = read_text("notizen.txt")?;
   |                  --------- ^^^^^^^^^^^^^ expected `&Path`, found `&str`
   |                  |
   |                  arguments to this function are incorrect
   |
   = note: expected reference `&Path`
              found reference `&'static str`
note: function defined here
  --> lesen.rs:5:4
   |
 5 | fn read_text(pfad: &Path) -> io::Result<String> {
   |    ^^^^^^^^^ -----------

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

What is confusing about it is that `fs::read_to_string("notizen.txt")` goes
through. That is because this function does not ask for a `&Path` but for
anything that can be turned into a path. `Path::new("notizen.txt")` turns the
text into a path and settles it.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. `read_text` stands there finished, and its doc
test is green.

- `read_or` reads a file and answers the missing one with a substitute
- `append_line` appends a line instead of replacing
- `copy_lines` writes the matching lines of one file into a second one

```console
cd units/06-02-dateien-lesen-und-schreiben
cargo test
```

### Source

    Book, chapter 12 "An I/O Project: Building a Command Line Program",
    section 12.2 "Reading a File",
    https://doc.rust-lang.org/book/ch12-02-reading-a-file.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
