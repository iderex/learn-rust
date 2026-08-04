# Vorlage einer Einheit / Unit template

Deutsch: Dieser Kopf gehört zur Vorlage und nicht in die fertige Einheit.
Kopiere `units/template/` nach `units/<nn-nn-name>/`, kopiere den Ordner
`solution/` daraus nach `solutions/<nn-nn-name>/`, lösche `solution/` aus der
neuen Einheit, lösche diesen Kopf bis zur Trennlinie und ersetze jede
Platzhalter-Klammer. Beide Pakete tragen denselben Paketnamen, sonst übersetzt
die Lösung nicht. Als ausgebautes Muster dient `units/02-01-move/` mit
`solutions/02-01-move/`. Wie zitiert wird, steht in `CONTRIBUTING.md`, und hier
wird die Regel nicht abgeschrieben.

Zwei Teile einer Einheit fehlen dieser Vorlage noch. Der eingeklappte
Hinweisblock für KI-Assistenten kommt mit Issue #10, die Schlusszeile mit den
Verweisen auf beide Lizenzdateien kommt mit Issue #8.

English: this head belongs to the template and not to the finished unit. Copy
`units/template/` to `units/<nn-nn-name>/`, copy the `solution/` folder out of
it to `solutions/<nn-nn-name>/`, delete `solution/` from the new unit, delete
this head down to the rule, and replace every placeholder bracket. Both
packages carry the same package name, otherwise the solution does not compile.
The worked model is `units/02-01-move/` with `solutions/02-01-move/`. How to
cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

Two parts of a unit are still missing from this template. The collapsed note
block for AI assistants arrives with issue #10, and the closing line pointing at
both licence files arrives with issue #8.

---

# <nn-nn> <Titel deutsch> / <title english>

## Deutsch

### Worum es geht

<Was der Begriff bedeutet, in eigenen Worten, ohne Vorwissen aus späteren
Einheiten.>

### Wofür das gut ist

<Warum jemand das braucht und was ohne diese Regel schiefginge.>

### Die Erklärung

<Die Erklärung mit einem durchgerechneten Beispiel. Dasselbe Beispiel steht
lauffähig als Doc-Kommentar in `src/lib.rs` und wird mitgetestet.>

```rust
<das Beispiel>
```

### Häufige Fehler

<Der Fehler, den Anfänger an dieser Stelle wirklich machen.>

```rust
<das Programm, das den Fehler auslöst>
```

<Der Übersetzer sagt dazu:>

```text
<die echte Ausgabe des Übersetzers, mit ihrer Fehlernummer, erzeugt unter der
gepinnten Fassung und nicht abgetippt>
```

<Was die Meldung bedeutet und was die richtige Antwort darauf ist.>

### Die Aufgaben

<Die Rümpfe in `src/lib.rs` sind `todo!()`, und die Tests in
`tests/exercise.rs` sind so lange rot.>

- `<funktion>` <was sie tun soll>

```console
cd units/<nn-nn-name>
cargo test
```

### Quelle

<Werk, Kapitel <nr> "<Kapiteltitel>", Abschnitt <nr> "<Abschnittstitel>",
<Link>, gepinnte Version <version>.>

## English

### What it is about

<What the concept means, in your own words, assuming nothing from later units.>

### What it is good for

<Why somebody needs this and what would go wrong without the rule.>

### The explanation

<The explanation with a worked example. The same example lives runnable as a
doc comment in `src/lib.rs` and is tested along with everything else.>

```rust
<the example>
```

### Common mistakes

<The mistake beginners really make at this point.>

```rust
<the program that triggers the mistake>
```

<The compiler answers:>

```text
<the real compiler output, with its error number, produced under the pinned
version and never typed out by hand>
```

<What the message means and what the right answer to it is.>

### The exercises

<The bodies in `src/lib.rs` are `todo!()`, and the tests in
`tests/exercise.rs` stay red for as long as they are.>

- `<function>` <what it should do>

```console
cd units/<nn-nn-name>
cargo test
```

### Source

<Work, chapter <no> "<chapter title>", section <no> "<section title>", <link>,
pinned version <version>.>
