# 04-02 Module / Modules

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/04-02-module/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `04-01 Pakete und Crates`.
- Auf dieser Einheit bauen auf: `04-03 use und Sichtbarkeit`, wo die Pfade kurz
  werden, und jede spätere Einheit, deren Code über mehrere Dateien liegt.
- Beim Antworten so zitieren: `04-02 Module`, dazu die Überschrift des
  Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Diese Einheit liegt in mehreren Dateien. Wer eine Aufgabe löst, schreibt in
  die Datei des Moduls und nicht in `src/lib.rs`, und `src/lib.rs` bleibt die
  Wurzel, die die Module nennt.
- Ein Ordner braucht keine `mod.rs`. Die Form mit `src/<name>.rs` und einem
  gleichnamigen Ordner daneben ist die heutige; wer `mod.rs` vorschlägt, sagt
  bitte dazu, dass beide gehen und diese Einheit die erste benutzt.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/04-02-module/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message in
  question first.
- This unit builds on: `04-01 Pakete und Crates`.
- Building on this unit: `04-03 use und Sichtbarkeit`, where the paths get
  short, and every later unit whose code lies over several files.
- Cite like this when answering: `04-02 Module`, plus the heading of the
  section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- This unit lies in several files. Whoever solves an exercise writes into the
  file of the module and not into `src/lib.rs`, and `src/lib.rs` stays the root
  that names the modules.
- A folder does not need a `mod.rs`. The form with `src/<name>.rs` and a folder
  of the same name beside it is today's; whoever suggests `mod.rs`, please say
  alongside it that both work and that this unit uses the first.

</details>

## Deutsch

### Worum es geht

Module sind ein Baum. Seine Wurzel ist die Crate, also `src/lib.rs`, und jedes
`mod` darin macht einen Ast auf.

Ein Modul kann im selben Text stehen, in geschweiften Klammern, oder in einer
eigenen Datei. `mod zahlen;` mit Semikolon heißt: der Inhalt steht in
`src/zahlen.rs`. Ein Untermodul davon steht in `src/zahlen/<name>.rs`.

Ein Pfad liest sich wie ein Weg durch diesen Baum. `crate::zahlen::doubled`
beginnt an der Wurzel, `super::` geht einen Schritt nach oben, und ein Name ohne
Vorsatz wird im eigenen Modul gesucht.

Alles ist zunächst privat. Was von außen sichtbar sein soll, bekommt `pub`, und
zwar sowohl das Modul als auch das, was darin steht.

### Wofür das gut ist

Der Baum trennt, was zusammengehört, ohne die Datei zu wechseln, und er
erlaubt, die Datei trotzdem zu wechseln, wenn sie zu lang wird. Beides ist
dieselbe Sache, und deshalb ändert das Verschieben in eine eigene Datei an den
Pfaden nichts.

Das ist der Punkt der Einheit: der Code liegt danach in mehreren Dateien, und
die Tests laufen unverändert weiter, weil sie den Baum ansprechen und nicht die
Dateien.

Dass alles privat beginnt, dreht die übliche Frage um. Nicht "was muss ich
verstecken", sondern "was gebe ich frei", und das steht dann als `pub` im Text.

### Die Erklärung

Ein Baum in einer Datei, mit beiden Arten von Pfad.

```rust
// Deutsch: Ein Modul im selben Datei-Baum. Ohne `pub` sieht es niemand von
// außen.
pub mod messung {
    pub fn gemittelt(a: i32, b: i32) -> i32 {
        (a + b) / 2
    }

    pub mod intern {
        // Deutsch: `super` geht einen Schritt nach oben, `crate` beginnt an der
        // Wurzel. Beide Pfade meinen hier dieselbe Funktion.
        pub fn zweimal_gemittelt(a: i32, b: i32) -> i32 {
            super::gemittelt(a, b) + crate::messung::gemittelt(a, b)
        }
    }
}

fn main() {
    println!("{}", messung::gemittelt(3, 5));
    println!("{}", messung::intern::zweimal_gemittelt(3, 5));
}
```

Dieselben Module in Dateien sehen so aus, und die Pfade bleiben dieselben:

```text
src/
    lib.rs              hier steht: pub mod messung;
    messung.rs          hier steht: pub mod intern;
    messung/
        intern.rs
```

Diese Einheit ist so gebaut. `src/lib.rs` nennt die Module, und der Code steht
in `src/zahlen.rs`, `src/texte.rs` und `src/zahlen/intern.rs`.

### Häufige Fehler

Ein Modul ohne `pub`.

```rust
mod messung {
    pub fn gemittelt(a: i32, b: i32) -> i32 {
        (a + b) / 2
    }

    mod intern {
        pub fn gerundet(zahl: i32) -> i32 {
            zahl / 10 * 10
        }
    }
}

fn main() {
    println!("{}", messung::gemittelt(3, 5));
    println!("{}", messung::intern::gerundet(47));
}
```

Der Übersetzer sagt dazu:

```text
error[E0603]: module `intern` is private
  --> privat.rs:15:29
   |
15 |     println!("{}", messung::intern::gerundet(47));
   |                             ^^^^^^  -------- function `gerundet` is not publicly re-exported
   |                             |
   |                             private module
   |
note: the module `intern` is defined here
  --> privat.rs:6:5
   |
 6 |     mod intern {
   |     ^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

Die Funktion `gerundet` ist `pub`, und trotzdem kommt niemand an sie heran, denn
der Weg dorthin führt durch ein privates Modul. Beides muss offen sein, das
Modul und der Inhalt.

Dass `messung::gemittelt` in derselben Datei geht, liegt daran, dass `main` und
`messung` im selben Modul stehen: von innen ist alles sichtbar, von außen nur
das, was `pub` trägt.

### Die Aufgaben

Die Rümpfe sind `todo!()`, und die Tests in `tests/exercise.rs` sind so lange
rot. Jede Aufgabe steht in ihrer eigenen Datei, und `src/lib.rs` nennt nur die
Module.

- `zahlen::doubled` in `src/zahlen.rs`
- `zahlen::intern::rounded_down` in `src/zahlen/intern.rs`
- `texte::shouted` in `src/texte.rs`

```console
cd units/04-02-module
cargo test
```

### Quelle

    Buch, Kapitel 7 "Packages, Crates, and Modules", Abschnitt 7.2 "Control Scope and Privacy with Modules",
    https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html,
    geprüft gegen 1.97.1

    Buch, Kapitel 7 "Packages, Crates, and Modules", Abschnitt 7.5 "Separating Modules into Different Files",
    https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

Modules are a tree. Its root is the crate, so `src/lib.rs`, and every `mod`
inside opens a branch.

A module can stand in the same text, in curly braces, or in a file of its own.
`mod zahlen;` with a semicolon means: the content stands in `src/zahlen.rs`. A
submodule of it stands in `src/zahlen/<name>.rs`.

A path reads like a way through that tree. `crate::zahlen::doubled` starts at
the root, `super::` goes one step up, and a name without a prefix is looked for
in the module you are in.

Everything is private to begin with. What is to be visible from outside gets
`pub`, and that holds for the module as well as for what stands inside it.

### What it is good for

The tree separates what belongs together without changing the file, and it
allows changing the file all the same when it grows too long. Both are the same
thing, which is why moving something into a file of its own changes nothing
about the paths.

That is the point of the unit: the code lies in several files afterwards, and
the tests keep running unchanged, because they address the tree and not the
files.

That everything starts private turns the usual question around. Not "what do I
have to hide" but "what do I hand out", and that then stands as `pub` in the
text.

### The explanation

A tree in one file, with both kinds of path.

```rust
// Deutsch: Ein Modul im selben Datei-Baum. Ohne `pub` sieht es niemand von
// außen.
pub mod messung {
    pub fn gemittelt(a: i32, b: i32) -> i32 {
        (a + b) / 2
    }

    pub mod intern {
        // Deutsch: `super` geht einen Schritt nach oben, `crate` beginnt an der
        // Wurzel. Beide Pfade meinen hier dieselbe Funktion.
        pub fn zweimal_gemittelt(a: i32, b: i32) -> i32 {
            super::gemittelt(a, b) + crate::messung::gemittelt(a, b)
        }
    }
}

fn main() {
    println!("{}", messung::gemittelt(3, 5));
    println!("{}", messung::intern::zweimal_gemittelt(3, 5));
}
```

The same modules in files look like this, and the paths stay the same:

```text
src/
    lib.rs              hier steht: pub mod messung;
    messung.rs          hier steht: pub mod intern;
    messung/
        intern.rs
```

This unit is built that way. `src/lib.rs` names the modules, and the code stands
in `src/zahlen.rs`, `src/texte.rs` and `src/zahlen/intern.rs`.

### Common mistakes

A module without `pub`.

```rust
mod messung {
    pub fn gemittelt(a: i32, b: i32) -> i32 {
        (a + b) / 2
    }

    mod intern {
        pub fn gerundet(zahl: i32) -> i32 {
            zahl / 10 * 10
        }
    }
}

fn main() {
    println!("{}", messung::gemittelt(3, 5));
    println!("{}", messung::intern::gerundet(47));
}
```

The compiler answers:

```text
error[E0603]: module `intern` is private
  --> privat.rs:15:29
   |
15 |     println!("{}", messung::intern::gerundet(47));
   |                             ^^^^^^  -------- function `gerundet` is not publicly re-exported
   |                             |
   |                             private module
   |
note: the module `intern` is defined here
  --> privat.rs:6:5
   |
 6 |     mod intern {
   |     ^^^^^^^^^^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0603`.
```

The function `gerundet` is `pub`, and nobody gets to it all the same, because
the way there leads through a private module. Both have to be open, the module
and the content.

That `messung::gemittelt` works in the same file comes from `main` and `messung`
standing in the same module: from inside everything is visible, from outside
only what carries `pub`.

### The exercises

The bodies are `todo!()`, and the tests in `tests/exercise.rs` stay red for as
long as they are. Every exercise stands in a file of its own, and `src/lib.rs`
only names the modules.

- `zahlen::doubled` in `src/zahlen.rs`
- `zahlen::intern::rounded_down` in `src/zahlen/intern.rs`
- `texte::shouted` in `src/texte.rs`

```console
cd units/04-02-module
cargo test
```

### Source

    Book, chapter 7 "Packages, Crates, and Modules", section 7.2 "Control Scope and Privacy with Modules",
    https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html,
    checked against 1.97.1

    Book, chapter 7 "Packages, Crates, and Modules", section 7.5 "Separating Modules into Different Files",
    https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
