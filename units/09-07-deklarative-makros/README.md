# 09-07 Deklarative Makros mit macro_rules! / Declarative macros with macro_rules!

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/09-07-deklarative-makros/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `01-03 Funktionen` und `05-01 Generische Typen`. Ein
  Makro wird erst da interessant, wo eine Funktion und ein generischer Typ nicht
  mehr reichen.
- Auf dieser Einheit bauen auf: der Rest der Stufe 9 und jede Stelle, an der
  `vec!`, `println!` oder `assert_eq!` nicht mehr als gegeben hingenommen werden.
- Beim Antworten so zitieren: `09-07 Deklarative Makros mit macro_rules!`, dazu
  die Überschrift des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Ein `macro_rules!`-Makro gilt erst ab der Zeile, in der es steht. Das ist
  anders als bei einer Funktion, und die Meldung dazu steht unter "Häufige
  Fehler".
- Ein Fragment `expr` ist ein fertiger Ausdruck. `quadrat!(1 + 2)` ist deshalb
  9 und nicht 5. Wer das Gegenteil behauptet, sagt bitte, an welcher Stelle die
  Ersetzung als Text stattfinden soll.
- Hygiene deckt die Namen, die das Makro selbst einführt. Sie deckt nicht, was
  hineingereicht wird, und keine Typen und Funktionen.
- Innerhalb eines exportierten Makros heißt der eigene Name `$crate::name!`.
  Ohne das findet ein anderes Paket das Makro beim Rekursionsschritt nicht.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/09-07-deklarative-makros/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `01-03 Funktionen` and `05-01 Generische Typen`. A macro
  becomes interesting only where a function and a generic type no longer reach.
- Building on this unit: the rest of stage 9 and every place where `vec!`,
  `println!` or `assert_eq!` stop being taken as given.
- Cite like this when answering: `09-07 Deklarative Makros mit macro_rules!`,
  plus the heading of the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- A `macro_rules!` macro holds only from the line it stands on. That is
  different from a function, and the message for it is under "Common mistakes".
- A fragment `expr` is a finished expression. `quadrat!(1 + 2)` is therefore 9
  and not 5. Whoever claims the opposite, please say at which point the
  substitution is meant to happen as text.
- Hygiene covers the names the macro introduces itself. It does not cover what
  is handed in, and no types and functions.
- Inside an exported macro its own name is `$crate::name!`. Without that another
  package does not find the macro at the recursion step.

</details>

## Deutsch

### Worum es geht

`macro_rules!` schreibt ein Makro, das aus einem Muster eine Ersetzung macht.
Aufgerufen wird es mit einem Ausrufezeichen, und der Übersetzer setzt an der
Stelle des Aufrufs den Rumpf ein, bevor er den Rest der Arbeit macht.

Ein Muster besteht aus Fragmenten. `$x:expr` fängt einen Ausdruck ein,
`$name:ident` einen Namen. Eine Wiederholung schreibt sich `$( ... ),*` für
beliebig viele und `$( ... ),+` für mindestens eines.

Ein Makro darf mehrere Regeln haben. Genommen wird die erste, deren Muster
passt, und deshalb steht die engste Regel oben.

### Wofür das gut ist

Eine Funktion in Rust nimmt eine feste Zahl von Argumenten mit festen Typen.
`vec![1, 2, 3]` und `vec![]` sind derselbe Aufruf mit verschiedenen Längen, und
`println!("{a} {b}")` liest seine Platzhalter, bevor irgendetwas läuft. Beides
kann eine Funktion nicht, und beides ist ein Makro.

Der zweite Grund ist die Stelle, an der ein Makro einsetzt. Es kann Dinge
erzeugen, die es sonst nicht gäbe, zum Beispiel eine ganze Funktion. Aufgabe 3
dieser Einheit tut genau das: Aus einem Namen und einer Grenze entsteht eine
Funktion, die es vorher nicht gab.

Der Preis steht daneben. Ein Makro wird an jeder Aufrufstelle eingesetzt, seine
Fehlermeldungen zeigen auf den erzeugten Code und nicht auf das, was jemand
geschrieben hat, und es gilt erst ab seiner eigenen Zeile. Wo eine Funktion
reicht, ist die Funktion die bessere Wahl.

### Die Erklärung

Zwei Makros in einem Programm, eines mit einer Regel und eines mit zweien.

```rust
// Deutsch: Ein Fragment `expr` ist ein fertiger Ausdruck und kein Text, der
// noch einmal gelesen wird.
macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}

// Deutsch: Zwei Regeln. Die erste greift bei einem Argument, die zweite ruft
// das Makro mit einem Argument weniger noch einmal auf.
macro_rules! groesster {
    ($einziger:expr) => {
        $einziger
    };
    ($erster:expr, $($weitere:expr),+ $(,)?) => {{
        let rest = groesster!($($weitere),+);

        if $erster > rest { $erster } else { rest }
    }};
}

fn main() {
    // Deutsch: 9 und nicht 5, denn `1 + 2` ist ein Ausdruck und wird nicht
    // zwischen die Sterne geschrieben.
    println!("{}", quadrat!(1 + 2));

    println!("{}", groesster!(3));
    println!("{}", groesster!(3, 9, 4));
    println!("{}", groesster!("aal", "zebra", "kuh"));

    // Deutsch: Hygiene. Das `rest` im Makro und das hier sind zwei
    // verschiedene Dinge, obwohl sie gleich heißen.
    let rest = 100;

    println!("{}", groesster!(1, 2));
    println!("{}", rest);
}
```

Das Programm gibt aus:

```text
9
3
9
zebra
2
100
```

Die erste Zeile ist die, die aus anderen Sprachen falsch erwartet wird. Wäre die
Ersetzung eine Ersetzung von Text, stünde dort `1 + 2 * 1 + 2` und damit 5. Ein
Fragment `expr` ist aber ein fertiger Ausdruck, und der bleibt einer.

Die dritte Zeile zeigt, warum hier keine Funktion genügt. `groesster!` nimmt
eine Zahl von Argumenten, die erst am Aufruf feststeht, und dazu Werte, deren
Typ nur `>` beherrschen muss. Die vierte Zeile ist dieselbe Sache mit Texten.

Die letzten beiden Zeilen sind die Hygiene. Im Makro steht ein `let rest`, und
daneben steht ein `let rest = 100` im Programm. Sie stören einander nicht, weil
der Name aus dem Makro ein anderer ist als der gleich geschriebene daneben. Was
die Hygiene nicht deckt: alles, was hineingereicht wird, und Namen von Typen und
Funktionen.

### Häufige Fehler

Ein Makro aufrufen, bevor es dasteht.

```rust
fn main() {
    println!("{}", quadrat!(1 + 2));
}

macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}
```

Der Übersetzer sagt dazu:

```text
error: cannot find macro `quadrat` in this scope
 --> zu-frueh.rs:2:20
  |
2 |     println!("{}", quadrat!(1 + 2));
  |                    ^^^^^^^ consider moving the definition of `quadrat` before this call
  |
note: a macro with the same name exists, but it appears later
 --> zu-frueh.rs:5:14
  |
5 | macro_rules! quadrat {
  |              ^^^^^^^

warning: unused macro definition: `quadrat`
 --> zu-frueh.rs:5:14
  |
5 | macro_rules! quadrat {
  |              ^^^^^^^
  |
  = note: `#[warn(unused_macros)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted
```

Die Meldung trägt keine Fehlernummer und dafür den Rat gleich mit: Die Definition
gehört vor den Aufruf. Der Hinweis darunter sagt sogar, dass es das Makro gibt
und wo es steht.

Bei einer Funktion ist die Reihenfolge im Modul gleichgültig. Bei
`macro_rules!` ist sie es nicht, denn das Makro gilt ab seiner eigenen Zeile.
Die Warnung darunter kommt aus derselben Sache: Weil der Aufruf das Makro nicht
gefunden hat, ist es aus Sicht des Übersetzers unbenutzt.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind offen und brechen ab, und die Tests in
`tests/exercise.rs` sind so lange rot. Bei den beiden Makros, die einen Wert
liefern, steht dort kein nacktes `todo!()`, sondern ein Aufruf von `offen`. Der
Grund steht am Doku-Kommentar dieser Funktion: `todo!()` hat den Typ `!`, und
ein Makro hat keinen angeschriebenen Rückgabetyp, an dem sich das aufheben
ließe. Gelöst wird eine Aufgabe, indem die ganze Regel ersetzt wird. `quadrat!`
steht fertig da, und sein Doku-Test ist grün.

- `groesster!` gibt von beliebig vielen Werten den größten heraus
- `vec_von!` baut aus beliebig vielen Werten ein `Vec`, auch aus keinem
- `mach_pruefer!` erzeugt aus einem Namen und einer Grenze eine Funktion

```console
cd units/09-07-deklarative-makros
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.5 "Macros",
    https://doc.rust-lang.org/book/ch20-05-macros.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`macro_rules!` writes a macro that turns a pattern into a substitution. It is
called with an exclamation mark, and the compiler puts the body in at the place
of the call before doing the rest of its work.

A pattern is made of fragments. `$x:expr` catches an expression, `$name:ident` a
name. A repetition is written `$( ... ),*` for any number and `$( ... ),+` for at
least one.

A macro may have several rules. The first one whose pattern fits is taken, which
is why the narrowest rule stands on top.

### What it is good for

A function in Rust takes a fixed number of arguments with fixed types.
`vec![1, 2, 3]` and `vec![]` are the same call with different lengths, and
`println!("{a} {b}")` reads its placeholders before anything runs. A function
can do neither, and both are a macro.

The second reason is the point at which a macro steps in. It can bring things
into being that would otherwise not exist, a whole function for instance.
Exercise 3 of this unit does exactly that: out of a name and a limit comes a
function that was not there before.

The price stands next to it. A macro is put in at every call site, its error
messages point at the generated code and not at what somebody wrote, and it
holds only from its own line on. Where a function reaches, the function is the
better choice.

### The explanation

Two macros in one program, one with a single rule and one with two.

```rust
// Deutsch: Ein Fragment `expr` ist ein fertiger Ausdruck und kein Text, der
// noch einmal gelesen wird.
macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}

// Deutsch: Zwei Regeln. Die erste greift bei einem Argument, die zweite ruft
// das Makro mit einem Argument weniger noch einmal auf.
macro_rules! groesster {
    ($einziger:expr) => {
        $einziger
    };
    ($erster:expr, $($weitere:expr),+ $(,)?) => {{
        let rest = groesster!($($weitere),+);

        if $erster > rest { $erster } else { rest }
    }};
}

fn main() {
    // Deutsch: 9 und nicht 5, denn `1 + 2` ist ein Ausdruck und wird nicht
    // zwischen die Sterne geschrieben.
    println!("{}", quadrat!(1 + 2));

    println!("{}", groesster!(3));
    println!("{}", groesster!(3, 9, 4));
    println!("{}", groesster!("aal", "zebra", "kuh"));

    // Deutsch: Hygiene. Das `rest` im Makro und das hier sind zwei
    // verschiedene Dinge, obwohl sie gleich heißen.
    let rest = 100;

    println!("{}", groesster!(1, 2));
    println!("{}", rest);
}
```

The program prints:

```text
9
3
9
zebra
2
100
```

The first line is the one expected wrongly from other languages. If the
substitution were a substitution of text, `1 + 2 * 1 + 2` would stand there and
with it 5. A fragment `expr` is a finished expression, though, and it stays one.

The third line shows why a function does not reach here. `groesster!` takes a
number of arguments that is settled at the call, and values whose type only has
to master `>`. The fourth line is the same thing with texts.

The last two lines are the hygiene. Inside the macro stands a `let rest`, and
next to it stands a `let rest = 100` in the program. They do not disturb each
other, because the name out of the macro is a different one from the identically
written name next to it. What hygiene does not cover: everything handed in, and
names of types and functions.

### Common mistakes

Calling a macro before it stands there.

```rust
fn main() {
    println!("{}", quadrat!(1 + 2));
}

macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}
```

The compiler answers:

```text
error: cannot find macro `quadrat` in this scope
 --> zu-frueh.rs:2:20
  |
2 |     println!("{}", quadrat!(1 + 2));
  |                    ^^^^^^^ consider moving the definition of `quadrat` before this call
  |
note: a macro with the same name exists, but it appears later
 --> zu-frueh.rs:5:14
  |
5 | macro_rules! quadrat {
  |              ^^^^^^^

warning: unused macro definition: `quadrat`
 --> zu-frueh.rs:5:14
  |
5 | macro_rules! quadrat {
  |              ^^^^^^^
  |
  = note: `#[warn(unused_macros)]` (part of `#[warn(unused)]`) on by default

error: aborting due to 1 previous error; 1 warning emitted
```

The message carries no error number and carries the advice instead: the
definition belongs before the call. The note below it even says that the macro
exists and where it stands.

For a function the order inside a module makes no difference. For
`macro_rules!` it does, because the macro holds from its own line on. The
warning below comes out of the same thing: because the call did not find the
macro, the macro is unused as far as the compiler can tell.

### The exercises

The bodies in `src/lib.rs` are open and abort, and the tests in
`tests/exercise.rs` stay red for as long as they are. At the two macros
delivering a value there is no bare `todo!()` but a call of `offen`. The reason
stands at the doc comment of that function: `todo!()` has the type `!`, and a
macro has no written-down return type at which that could be taken up. An
exercise is solved by replacing the whole rule. `quadrat!` stands there
finished, and its doc test is green.

- `groesster!` hands the largest of any number of values out
- `vec_von!` builds a `Vec` out of any number of values, out of none as well
- `mach_pruefer!` makes a function out of a name and a limit

```console
cd units/09-07-deklarative-makros
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.5 "Macros",
    https://doc.rust-lang.org/book/ch20-05-macros.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
