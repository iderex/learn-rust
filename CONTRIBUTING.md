# Beitragen / Contributing

Diese Datei ist noch nicht vollständig. Sie trägt heute die Zitierregel und die
Befehle. Es fehlen der Einstieg mit rustup, die Zweisprachigkeitsregel, der
Aufbau einer Einheit, die Lizenz, der DCO mit `git commit -s`, der Ablauf für
Beiträge und der Umgangston. Diese Teile gehören zu Issue #9. Dieser Absatz wird
entfernt, sobald sie hier stehen.

This file is not complete yet. Today it carries the citation rule and the
commands. Missing are how to start with rustup, the bilingual rule, how a unit is
built, the licence, the DCO with `git commit -s`, the process for contributions
and the tone. Those parts belong to issue #9. This paragraph goes as soon as they
are here.

## Deutsch

### Quellen angeben

Eine Quellenangabe nennt vier Dinge.

1. Die Kapitelnummer.
2. Den Kapiteltitel in der Schreibweise der gebundenen Fassung.
3. Den Link.
4. Die Version, gegen die geprüft wurde.

Welche Fassung gebunden ist, steht in `rust-toolchain.toml`. Dieselbe Fassung
des Buchs liegt offline neben der Toolchain und öffnet sich mit
`rustup doc --book`.

So sieht eine Angabe aus.

    Buch, Kapitel 7 "Packages, Crates, and Modules",
    https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html,
    geprüft gegen 1.97.1

Der Titel allein trägt nicht, weil Titel und Seitenadresse auseinanderlaufen.
In der gebundenen Fassung heißt Kapitel 7 "Packages, Crates, and Modules",
während die Adresse den älteren Namen weiterführt. Nachzusehen im offline
abgelegten Buch.

    book=$(dirname "$(rustup doc --book --path)")
    grep -o '<title>[^<]*</title>' "$book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html"
    <title>Packages, Crates, and Modules - The Rust Programming Language</title>

Die Kapitelnummer allein trägt ebenso wenig, denn Nummern verschieben sich,
wenn das Buch umgebaut wird. Deshalb stehen alle vier Teile zusammen, und der
Link führt auf die Seite, die der Titel meint.

Wer die gebundene Version später anhebt, sieht die Titel neu nach und zieht die
Angaben nach, die sich geändert haben.

Automatisch geprüft wird davon heute nichts. Eine Behauptung ohne Quelle wird
im Review nachgefragt. Ob eine genannte Quelle die Behauptung wirklich trägt,
entscheidet ein Mensch beim Lesen. Issue #5 plant eine Prüfung, die nachsieht,
ob jede Einheit eine Quelle mit Kapiteltitel und gebundener Version nennt. Ob
eine Quelle stimmt, wird auch die nicht beantworten.

## English

### Citing sources

A source reference names four things.

1. The chapter number.
2. The chapter title as spelled in the pinned version.
3. The link.
4. The version it was checked against.

Which version is pinned is in `rust-toolchain.toml`. The same version of the
book sits offline next to the toolchain and opens with `rustup doc --book`.

A reference looks like this.

    Book, chapter 7 "Packages, Crates, and Modules",
    https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html,
    checked against 1.97.1

The title on its own does not carry, because the title and the page address
drift apart. In the pinned version chapter 7 is spelled "Packages, Crates, and
Modules" while the address keeps the older name. Look it up in the offline copy.

    book=$(dirname "$(rustup doc --book --path)")
    grep -o '<title>[^<]*</title>' "$book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html"
    <title>Packages, Crates, and Modules - The Rust Programming Language</title>

The chapter number on its own does not carry either, because numbers move when
the book is rearranged. That is why all four parts stay together, and why the
link points at the page the title means.

Whoever raises the pinned version later checks the titles again and carries
over the references that changed.

None of this is checked automatically today. A claim without a source gets
asked about in review. Whether a named source really carries the claim is
decided by a person reading it. Issue #5 plans a check that looks at whether
every unit names a source with chapter title and pinned version. Whether a
source is right is not something that check will answer either.

## Die Befehle / The commands

Deutsch: Zwei Arten von Befehlen kommen vor. Die einen braucht, wer eine Einheit
löst. Die anderen prüfen das Repository als Ganzes.

Beide Blöcke stehen genau einmal im Repository, und dieser Abschnitt ist diese
eine Stelle. Sie sind nicht nach Sprachen getrennt, denn ein zweiter Abdruck
unter `## English` wäre schon der zweite Ort, und genau davor soll die Regel
schützen. Wie viele Befehle der Prüflauf hat, steht nirgends als Zahl. Wer sie
zählen will, liest den Block.

English: two kinds of command appear. One kind is what somebody solving a unit
needs. The other checks the repository as a whole.

Both blocks stand exactly once in the repository, and this section is that one
place. They are not split by language, because a second printing under
`## English` would already be the second place, and that is what the rule guards
against. How many commands the check run has is written nowhere as a number.
Whoever wants to count them reads the block.

### Beim Lösen einer Einheit / While solving a unit

Deutsch: `<nn-nn-name>` ist der Ordner der Einheit, zum Beispiel `02-01-move`.
Der Lauf ist rot, bis die Aufgaben gelöst sind. Das ist so gewollt und kein
Fehler im Repository.

English: `<nn-nn-name>` is the folder of the unit, for example `02-01-move`. The
run is red until the exercises are solved. That is intended and not a fault in
the repository.

```console
cd units/<nn-nn-name>
cargo test
```

### Der Prüflauf / The check run

Deutsch: Der Prüflauf geht über beide Workspaces, und alle Befehle werden aus dem
Wurzelverzeichnis heraus abgeschickt. Die ersten drei gehen über Workspace A, also
über die Lösungen, und der muss grün sein. Die letzten beiden gehen über Workspace
B unter `units/`. Die Einheiten müssen übersetzen und formatiert sein, ihre
Aufgabentests sind aber absichtlich rot, und deshalb steht für sie kein
`cargo test` im Block.

English: the check run goes over both workspaces, and every command is sent from
the root directory. The first three go over workspace A, meaning the solutions,
and that one has to be green. The last two go over workspace B under `units/`.
The units have to compile and be formatted, but their exercise tests are red on
purpose, and that is why no `cargo test` for them is in the block.

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo fmt --manifest-path units/Cargo.toml --all --check
cargo clippy --manifest-path units/Cargo.toml --workspace --all-targets -- -D warnings
```

Deutsch: Dass `--manifest-path` die Einheiten wirklich erreicht, ist gemessen und
nicht angenommen. Mit einer absichtlich falsch formatierten Zeile in
`units/02-01-move/src/lib.rs` gibt der vierte Befehl 1 zurück und nennt die Datei
und die Zeile, während `cargo fmt --all --check` im Wurzelverzeichnis 0 zurückgibt
und nichts meldet. Zwei Läufe sind es also deshalb, weil der eine den anderen
nicht erreicht.

English: that `--manifest-path` really reaches the units is measured rather than
assumed. With a deliberately misformatted line in `units/02-01-move/src/lib.rs`
the fourth command returns 1 and names the file and the line, while
`cargo fmt --all --check` at the root returns 0 and reports nothing. Two runs
therefore exist because one does not reach the other.

### Was der Lauf nicht erreicht / What the run does not reach

Deutsch: `units/template/` liegt in keinem der beiden Workspaces. Im
Wurzelverzeichnis führt `members` nur `solutions/*`, und `units/Cargo.toml` führt
`template` unter `exclude`. Kein Befehl des Blocks sieht die Vorlage an. Ein
Fehler in der Vorlage fällt heute nur beim Lesen auf.

Automatisch läuft nichts davon. Es gibt keinen Ablauf, der den Prüflauf bei einem
Pull Request anstößt. Solange das so ist, hängt die Prüfung eines fremden
Beitrags an einem Menschen, und eine fehlende `Signed-off-by`-Zeile fällt nur
beim Lesen auf.

`cargo run -p xtask -- check` steht noch nicht im Block, weil es `xtask` noch
nicht gibt. Es kommt mit Issue #5 dazu, und zwar hier und an keiner zweiten
Stelle.

English: `units/template/` is in neither workspace. At the root `members` lists
only `solutions/*`, and `units/Cargo.toml` lists `template` under `exclude`. No
command in the block looks at the template. A fault in the template is caught
today only by reading.

None of this runs automatically. There is no route that starts the check run on a
pull request. While that holds, the review of an outside contribution hangs on a
person, and a missing `Signed-off-by` line is caught only by reading.

`cargo run -p xtask -- check` is not in the block yet, because `xtask` does not
exist yet. It joins with issue #5, here and at no second place.
