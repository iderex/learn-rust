---
name: "Einheit / Unit"
about: "Eine neue Lerneinheit / a new learning unit"
title: "<nn-nn> <Titel deutsch> / <title english>"
labels: ["einheit / unit", "uebung / exercise"]
---

## Deutsch

### Worum es geht

<Was diese Einheit erklärt, in einem Absatz, ohne Vorwissen aus späteren Einheiten.>

### Was gebaut wird

- `units/<nn-nn-name>/` mit README, `src/lib.rs` und `tests/exercise.rs`
- `solutions/<nn-nn-name>/` mit demselben Paketnamen
- <die Aufgaben, eine Zeile je Aufgabe>

### Quelle

    Buch, Kapitel <nr> "<Kapiteltitel>", Abschnitt <nr> "<Abschnittstitel>",
    <Link>,
    geprüft gegen <version>

### Fertig, wenn

- die Einheit ist zweisprachig, Deutsch zuerst, und beide Abschnitte sagen dasselbe
- die Aufgabentests sind rot, und die gleichnamige Lösung besteht genau sie
- die Fehlermeldung im Text ist die echte Ausgabe des Übersetzers und nicht abgetippt
- der Prüflauf aus `CONTRIBUTING.md` ist gelaufen und seine Ausgabe steht im Pull Request

## English

### What it is about

<What this unit explains, in one paragraph, assuming nothing from later units.>

### What gets built

- `units/<nn-nn-name>/` with README, `src/lib.rs` and `tests/exercise.rs`
- `solutions/<nn-nn-name>/` with the same package name
- <the exercises, one line each>

### Source

    Book, chapter <no> "<chapter title>", section <no> "<section title>",
    <link>,
    checked against <version>

### Done when

- the unit is bilingual, German first, and both sections say the same thing
- the exercise tests are red, and the solution of the same name passes exactly those
- the compiler message in the text is real output and not typed out by hand
- the check run from `CONTRIBUTING.md` has been run and its output is in the pull request
