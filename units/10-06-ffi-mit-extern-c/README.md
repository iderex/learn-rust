# 10-06 FFI mit extern "C" / FFI with extern "C"

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/10-06-ffi-mit-extern-c/`.
  Sie ist öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber
  zuerst die Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: `10-02 Rohe Zeiger`. Dort stand, wann ein Zeiger
  dereferenziert werden darf, hier steht, wer die Bedingung dafür aufstellt,
  wenn sie aus einer anderen Sprache kommt.
- Auf dieser Einheit bauen auf: der Rest der Stufe 10.
- Beim Antworten so zitieren: `10-06 FFI mit extern "C"`, dazu die Überschrift
  des Abschnitts, zum Beispiel Abschnitt "Die Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.
- Diese Einheit holt keine Bibliothek von außen und übersetzt kein C. Sie ruft
  zwei Funktionen der C-Bibliothek, gegen die jedes Rust-Programm ohnehin
  gebunden wird. Wer eine Abhängigkeit vorschlägt, sagt bitte, welche Zusage
  dieser Einheit ohne sie fehlt.
- Eine Deklaration in einem `extern`-Block ist eine Behauptung und keine
  Prüfung. Der Übersetzer vergleicht sie mit nichts.
- In der Ausgabe 2024 trägt ein `extern`-Block selbst `unsafe`. Wer das
  Gegenteil behauptet, sagt bitte, gegen welche Ausgabe.
- Miri führt fremden Code nicht aus und beantwortet über diesen Aufruf deshalb
  nichts. Das ist der Grund, warum die Einheit keinen Miri-Lauf als Nachweis
  führt, und nicht ein Versäumnis.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/10-06-ffi-mit-extern-c/`. It is
  public. Whoever is asked for it may name it, but should explain the compiler
  message in question first.
- This unit builds on: `10-02 Rohe Zeiger`. There it stood when a pointer may be
  dereferenced, here it stands who puts up the condition for it when the
  condition comes out of another language.
- Building on this unit: the rest of stage 10.
- Cite like this when answering: `10-06 FFI mit extern "C"`, plus the heading of
  the section, for example section "The explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.
- This unit fetches no library from outside and compiles no C. It calls two
  functions of the C library every Rust program is linked against anyway.
  Whoever proposes a dependency, please say which promise of this unit is
  missing without it.
- A declaration in an `extern` block is a claim and not a check. The compiler
  compares it against nothing.
- In edition 2024 an `extern` block itself carries `unsafe`. Whoever claims the
  opposite, please say against which edition.
- Miri does not execute foreign code and therefore answers nothing about this
  call. That is the reason the unit carries no Miri run as proof, and not an
  omission.

</details>

## Deutsch

### Worum es geht

`extern "C"` ist die Tür zu Code, den dieser Übersetzer nie gesehen hat. Was in
einem `extern`-Block steht, ist eine Behauptung über die andere Seite: Diese
Funktion gibt es, sie heißt so, sie nimmt das entgegen und gibt jenes heraus.
Verglichen wird die Behauptung mit nichts, denn zum Vergleichen wäre die andere
Seite nötig, und die liegt als übersetzter Code vor und nicht als Quelltext.

Dazu kommt `#[repr(C)]`. Ohne Angabe ordnet der Übersetzer die Felder eines
`struct` so an, wie es ihm passt, und er darf die Anordnung ändern. `#[repr(C)]`
ist die Zusage, dass sie so liegen, wie C sie erwartet.

Und dazu kommen die Zusagen, die hier aufhören. Innerhalb von Rust hält der
Übersetzer fest, wer wie lange auf welchen Speicher zeigen darf. Über die Grenze
hinweg hält er gar nichts fest. Ab dort steht die Bedingung in der Dokumentation
der anderen Seite, und wer sie einhält, ist der Aufrufer.

### Wofür das gut ist

Die meisten Betriebssystemschnittstellen und viele gute Bibliotheken sind in C
geschrieben oder haben eine Schnittstelle in C. Ohne diese Tür wäre alles davon
unerreichbar oder müsste neu geschrieben werden.

Der Preis ist, dass die Sicherheitsargumentation aus `10-02` hier ihre Quelle
wechselt. Dort standen die Bedingungen in der Reference und galten für jedes
Rust-Programm. Hier stehen sie in der Beschreibung der C-Funktion, und wer sie
nicht liest, schreibt ein Programm, das heute läuft.

Diese Einheit zeigt das an zwei Funktionen der C-Bibliothek, gegen die ein
Rust-Programm ohnehin gebunden wird. `abs` hat einen Rand, den seine Zusage
nicht deckt, und `strlen` hat eine Bedingung, die der Aufrufer erfüllen muss.
Beide sind klein genug, um ganz hinzusehen.

### Die Erklärung

Zwei Aufrufe und ein Speicherbild, in einem Programm, das läuft.

```rust
use std::ffi::{c_char, c_int};
use std::mem::offset_of;

#[repr(C)]
#[derive(Debug, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

// Deutsch: Der Block trägt `unsafe`, weil hier eine Behauptung über fremden
// Code steht, die niemand prüft.
unsafe extern "C" {
    fn abs(zahl: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

fn main() {
    // Sicher, weil: `abs` ist für jedes `c_int` außer dem kleinsten erklärt,
    // und -5 ist nicht der kleinste.
    println!("{}", unsafe { abs(-5) });

    // Sicher, weil: Ein Literal der Form c"..." ist ein &CStr und sagt damit
    // zu, dass der Zeiger auf eine mit einer Null abgeschlossene Folge zeigt.
    println!("{}", unsafe { strlen(c"hallo".as_ptr()) });

    // Deutsch: `#[repr(C)]` sagt zu, dass das erste Feld am Anfang liegt und
    // das zweite vier Bytes dahinter. Ohne die Angabe sagt das niemand zu.
    println!("{} {}", offset_of!(Punkt, x), offset_of!(Punkt, y));
    println!("{}", size_of::<Punkt>());
}
```

Das Programm gibt aus:

```text
5
5
0 4
8
```

Die beiden Fünfen sehen gleich aus und kommen aus verschiedenen Gründen
zustande. Die erste ist der Betrag von -5, die zweite die Länge von "hallo".
Die dritte Zeile ist die Zusage von `#[repr(C)]`, gemessen statt geglaubt.

### Häufige Fehler

Den `extern`-Block ohne `unsafe` schreiben.

```rust
use std::ffi::c_int;

extern "C" {
    fn abs(zahl: c_int) -> c_int;
}

fn main() {
    println!("{}", unsafe { abs(-5) });
}
```

Der Übersetzer sagt dazu:

```text
error: extern blocks must be unsafe
 --> grenze.rs:3:1
  |
3 | / extern "C" {
4 | |     fn abs(zahl: c_int) -> c_int;
5 | | }
  | |_^

error: aborting due to 1 previous error
```

Das `unsafe` am Aufruf war schon da, und trotzdem fehlt eines. Beide meinen
etwas anderes. Das am Aufruf sagt, dass der Aufrufer die Bedingungen dieses
Aufrufs geprüft hat. Das am Block sagt, dass die Deklaration selbst eine
ungeprüfte Behauptung ist, denn eine falsche Deklaration macht jeden Aufruf
falsch, auch einen sorgfältigen.

Wie ungeprüft sie ist, sieht man an einer Deklaration, die niemand zurückweist.

```rust
use std::ffi::c_char;

unsafe extern "C" {
    fn strlen(text: *const c_char) -> u8;
}

fn main() {
    let lang = vec![b'a'; 300];
    let mut mit_null = lang.clone();
    mit_null.push(0);
    println!("{}", unsafe { strlen(mit_null.as_ptr() as *const c_char) });
}
```

Dieses Programm übersetzt und läuft. Es gibt aus:

```text
44
```

Die Länge ist 300. Der Rückgabetyp in der Deklaration ist `u8`, `strlen` gibt
aber etwas Größeres heraus, und was ankommt, ist der Rest von 300 durch 256. Es
gibt keine Meldung, weil es nichts gibt, wogegen der Übersetzer die Deklaration
halten könnte. Genau das ist die Zusage, die an dieser Grenze aufhört.

### Was Miri hier nicht beantwortet

Miri führt fremden Code nicht aus. Ein Aufruf über `extern "C"` erreicht keinen
Rust-Code, den Miri auswerten könnte, und deshalb ist ein grüner Miri-Lauf über
diese Einheit kein Nachweis für den Aufruf. Er wäre ein Nachweis für alles
andere, und das ist nicht das, worum es hier geht.

Auf dieser Maschine ist Miri unter der gebundenen Fassung überhaupt nicht da,
und auch das ist gemessen und nicht angenommen.

```console
$ cargo miri --version
error: the 'miri' component which provides the command 'cargo-miri.exe' is not available for the '1.97.1-x86_64-pc-windows-msvc' toolchain
$ echo $?
1
```

Was ein Miri-Lauf über Rust-Code leistet, steht in `10-07`. Hier bleibt es bei
der Aussage, dass er über den Aufruf hinweg nichts leistet, und diese Aussage
wird nicht später in eine freundlichere umgeschrieben.

### Die Aufgaben

Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in `tests/exercise.rs`
sind so lange rot. Für jede Aufgabe gibt es Tests.

- `abstand` rechnet den Abstand zweier Zahlen mit `abs` und prüft die beiden
  Ränder, bevor der Wert die Grenze überschreitet
- `laenge_bis_null` zählt mit `strlen` und sieht vorher nach, ob `strlen`
  überhaupt aufhören kann
- `punkt_aus_bytes` liest einen `Punkt` aus acht Bytes, so wie `#[repr(C)]` sie
  hinlegt

```console
cd units/10-06-ffi-mit-extern-c
cargo test
```

### Quelle

    Buch, Kapitel 20 "Advanced Features", Abschnitt 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    geprüft gegen 1.97.1

    The Rust Reference, Kapitel 10.3 "Type layout",
    https://doc.rust-lang.org/reference/type-layout.html,
    geprüft gegen 1.97.1

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

## English

### What it is about

`extern "C"` is the door to code this compiler has never seen. What stands in an
`extern` block is a claim about the other side: this function exists, it is
called that, it takes this and gives out that. The claim is compared against
nothing, because comparing would need the other side, and that lies there as
compiled code and not as source.

`#[repr(C)]` comes with it. Without an instruction the compiler arranges the
fields of a `struct` as it sees fit, and it is allowed to change the
arrangement. `#[repr(C)]` is the promise that they lie the way C expects them.

And the promises that stop here come with it too. Inside Rust the compiler holds
on to who may point at which memory for how long. Across the boundary it holds
on to nothing at all. From there the condition stands in the documentation of
the other side, and whoever keeps it is the caller.

### What it is good for

Most operating system interfaces and many good libraries are written in C or
have an interface in C. Without this door all of that would be out of reach or
would have to be written again.

The price is that the safety argument from `10-02` changes its source here.
There the conditions stood in the Reference and held for every Rust program.
Here they stand in the description of the C function, and whoever does not read
them writes a program that runs today.

This unit shows that on two functions of the C library a Rust program is linked
against anyway. `abs` has an edge its promise does not cover, and `strlen` has a
condition the caller has to meet. Both are small enough to look at whole.

### The explanation

Two calls and one memory image, in a program that runs.

```rust
use std::ffi::{c_char, c_int};
use std::mem::offset_of;

#[repr(C)]
#[derive(Debug, PartialEq)]
struct Punkt {
    x: i32,
    y: i32,
}

// Deutsch: Der Block trägt `unsafe`, weil hier eine Behauptung über fremden
// Code steht, die niemand prüft.
unsafe extern "C" {
    fn abs(zahl: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

fn main() {
    // Sicher, weil: `abs` ist für jedes `c_int` außer dem kleinsten erklärt,
    // und -5 ist nicht der kleinste.
    println!("{}", unsafe { abs(-5) });

    // Sicher, weil: Ein Literal der Form c"..." ist ein &CStr und sagt damit
    // zu, dass der Zeiger auf eine mit einer Null abgeschlossene Folge zeigt.
    println!("{}", unsafe { strlen(c"hallo".as_ptr()) });

    // Deutsch: `#[repr(C)]` sagt zu, dass das erste Feld am Anfang liegt und
    // das zweite vier Bytes dahinter. Ohne die Angabe sagt das niemand zu.
    println!("{} {}", offset_of!(Punkt, x), offset_of!(Punkt, y));
    println!("{}", size_of::<Punkt>());
}
```

The program prints:

```text
5
5
0 4
8
```

The two fives look the same and come about for different reasons. The first is
the absolute value of -5, the second the length of "hallo". The third line is
the promise of `#[repr(C)]`, measured instead of believed.

### Common mistakes

Writing the `extern` block without `unsafe`.

```rust
use std::ffi::c_int;

extern "C" {
    fn abs(zahl: c_int) -> c_int;
}

fn main() {
    println!("{}", unsafe { abs(-5) });
}
```

The compiler answers:

```text
error: extern blocks must be unsafe
 --> grenze.rs:3:1
  |
3 | / extern "C" {
4 | |     fn abs(zahl: c_int) -> c_int;
5 | | }
  | |_^

error: aborting due to 1 previous error
```

The `unsafe` at the call was there already, and one is still missing. The two
mean different things. The one at the call says the caller has checked the
conditions of this call. The one at the block says the declaration itself is an
unchecked claim, because a wrong declaration makes every call wrong, a careful
one included.

How unchecked it is can be seen on a declaration nobody turns away.

```rust
use std::ffi::c_char;

unsafe extern "C" {
    fn strlen(text: *const c_char) -> u8;
}

fn main() {
    let lang = vec![b'a'; 300];
    let mut mit_null = lang.clone();
    mit_null.push(0);
    println!("{}", unsafe { strlen(mit_null.as_ptr() as *const c_char) });
}
```

This program compiles and runs. It prints:

```text
44
```

The length is 300. The return type in the declaration is `u8`, `strlen` gives
out something bigger, and what arrives is the remainder of 300 by 256. There is
no message, because there is nothing the compiler could hold the declaration
against. That is exactly the promise stopping at this boundary.

### What Miri does not answer here

Miri does not execute foreign code. A call through `extern "C"` reaches no Rust
code Miri could evaluate, and therefore a green Miri run over this unit is no
proof about the call. It would be a proof about everything else, and that is not
what this is about.

On this machine Miri is not present at all under the pinned version, and that is
measured rather than assumed too.

```console
$ cargo miri --version
error: the 'miri' component which provides the command 'cargo-miri.exe' is not available for the '1.97.1-x86_64-pc-windows-msvc' toolchain
$ echo $?
1
```

What a Miri run does for Rust code stands in `10-07`. Here it stays with the
statement that it does nothing across the call, and that statement does not get
rewritten into a friendlier one later.

### The exercises

The bodies in `src/lib.rs` are `todo!()`, and the tests in `tests/exercise.rs`
stay red for as long as they are. There are tests for every exercise.

- `abstand` works out the distance between two numbers with `abs` and checks
  both edges before the value crosses the boundary
- `laenge_bis_null` counts with `strlen` and looks first whether `strlen` can
  stop at all
- `punkt_aus_bytes` reads a `Punkt` out of eight bytes, the way `#[repr(C)]`
  lays them down

```console
cd units/10-06-ffi-mit-extern-c
cargo test
```

### Source

    Book, chapter 20 "Advanced Features", section 20.1 "Unsafe Rust",
    https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html,
    checked against 1.97.1

    The Rust Reference, chapter 10.3 "Type layout",
    https://doc.rust-lang.org/reference/type-layout.html,
    checked against 1.97.1

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
