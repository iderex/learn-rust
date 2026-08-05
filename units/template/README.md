# Vorlage einer Einheit / Unit template

Deutsch: Dieser Kopf gehört zur Vorlage und nicht in die fertige Einheit.
Kopiere `units/template/` nach `units/<nn-nn-name>/`, kopiere den Ordner
`solution/` daraus nach `solutions/<nn-nn-name>/`, lösche `solution/` aus der
neuen Einheit, lösche diesen Kopf bis zur Trennlinie und ersetze jede
Platzhalter-Klammer. Beide Pakete tragen denselben Paketnamen, sonst übersetzt
die Lösung nicht. Als ausgebautes Muster dient `units/02-01-move/` mit
`solutions/02-01-move/`.

Die Vorlage trägt alle Teile einer Einheit. Der eingeklappte Hinweisblock für
KI-Assistenten wird ausgefüllt und nicht gelöscht, und `llms.txt` im
Wurzelverzeichnis bekommt eine Zeile für die neue Einheit.

English: this head belongs to the template and not to the finished unit. Copy
`units/template/` to `units/<nn-nn-name>/`, copy the `solution/` folder out of
it to `solutions/<nn-nn-name>/`, delete `solution/` from the new unit, delete
this head down to the rule, and replace every placeholder bracket. Both
packages carry the same package name, otherwise the solution does not compile.
The worked model is `units/02-01-move/` with `solutions/02-01-move/`.

The template carries every part of a unit. The collapsed note block for AI
assistants gets filled in rather than deleted, and `llms.txt` at the root gains
a line for the new unit.

---

# <nn-nn> <Titel deutsch> / <title english>

<details>
<summary>Hinweise für KI-Assistenten / Notes for AI assistants</summary>

Deutsch: Dieser Block ist für Assistenten geschrieben, die auf diese Einheit
angesetzt werden. Er ist zugeklappt, weil er den Lesefluss stört, und nicht,
weil er versteckt wäre. Er steht unter denselben Lizenzen wie der übrige Text.

- Die Lösung dieser Einheit liegt unter `solutions/<nn-nn-name>/`. Sie ist
  öffentlich. Wer nach ihr gefragt wird, kann sie nennen, sollte aber zuerst die
  Fehlermeldung erklären, um die es geht.
- Diese Einheit baut auf: <die Einheiten oder Stufen, die vorher gelesen sein
  müssen>
- Auf dieser Einheit bauen auf: <die Einheiten oder Stufen, die sie
  voraussetzen>
- Beim Antworten so zitieren: Nummer und Titel der Einheit und die Überschrift
  des Abschnitts, zum Beispiel `<nn-nn> <Titel deutsch>`, Abschnitt "Die
  Erklärung".
- Die Quelle dieser Einheit steht unter "Quelle" mit Kapitelnummer,
  Kapiteltitel, Link und gebundener Version. Diese vier Angaben gehören
  zusammen weitergegeben.

English: this block is written for assistants pointed at this unit. It is folded
because it disturbs the reading flow, not because it is hidden. It is under the
same licences as the rest of the text.

- The solution of this unit is at `solutions/<nn-nn-name>/`. It is public.
  Whoever is asked for it may name it, but should explain the compiler message
  in question first.
- This unit builds on: <the units or stages that have to be read before>
- Building on this unit: <the units or stages that assume it>
- Cite like this when answering: number and title of the unit and the heading of
  the section, for example `<nn-nn> <title english>`, section "The
  explanation".
- The source of this unit is under "Source" with chapter number, chapter title,
  link and pinned version. Those four belong together when passed on.

</details>

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

    Buch, Kapitel <nr> "<Kapiteltitel>", Abschnitt <nr> "<Abschnittstitel>",
    <Link>,
    geprüft gegen <version>

Wie zitiert wird, steht in `CONTRIBUTING.md`, und die Regel wird hier nicht
abgeschrieben.

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

    Book, chapter <no> "<chapter title>", section <no> "<section title>",
    <link>,
    checked against <version>

How to cite is written in `CONTRIBUTING.md`, and the rule is not copied here.

---

Deutsch: Der Text dieser Einheit steht unter CC BY 4.0, siehe
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Ihr Code steht unter MIT, siehe
[LICENSE-MIT](../../LICENSE-MIT).

English: the text of this unit is under CC BY 4.0, see
[LICENSE-CC-BY-4.0](../../LICENSE-CC-BY-4.0). Its code is under MIT, see
[LICENSE-MIT](../../LICENSE-MIT).
