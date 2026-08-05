# Beitragen / Contributing

Deutsch: Dieses Repository ist zum Rust-Lernen da, von der ersten Zeile an und
ohne vorausgesetztes Vorwissen. Fragen von Anfängern sind sein Zweck und nicht
seine Störung. Wer beiträgt, findet hier, wie das geht.

English: this repository exists for learning Rust, from the first line on and
with no prior experience assumed. Questions from beginners are its point and not
an interruption to it. Whoever contributes finds here how that works.

## Deutsch

### Anfangen

Geklont wird wie üblich.

    git clone https://github.com/iderex/learn-rust.git
    cd learn-rust

Die Version des Übersetzers wird nicht ausgesucht. Sie steht in
`rust-toolchain.toml`, und rustup holt und benutzt sie beim ersten Befehl in
diesem Verzeichnis von selbst. Wer rustup noch nicht hat, holt es von
https://rustup.rs. Ob die richtige Fassung greift, zeigt der Übersetzer selbst.

    rustc --version

Steht dort eine andere Nummer als in `rust-toolchain.toml`, wurde der Befehl
außerhalb des Repositories abgeschickt.

Die Befehle für die tägliche Arbeit stehen weiter unten unter
"Die Befehle / The commands" und werden hier nicht abgeschrieben.

### Zweisprachig, Deutsch zuerst

Jeder Text in diesem Repository steht auf Deutsch und auf Englisch, und Deutsch
steht vorn. Das gilt für die README jeder Einheit, für die Doku-Kommentare und
für die Rümpfe von Issues und Pull Requests.

Die beiden Abschnitte sollen dasselbe sagen. Sie sind keine Wort-für-Wort
Übersetzung voneinander, aber was der eine erklärt, erklärt der andere auch,
mit denselben Beispielen und denselben Fehlernummern. Ein englischer Einzeiler
unter einem langen deutschen Abschnitt ist kein zweisprachiger Text.

Ob die beiden Fassungen wirklich dasselbe sagen, entscheidet ein Mensch beim
Lesen. Keine Prüfung beantwortet das.

### Wie eine Einheit aufgebaut ist

Eine Einheit liegt zweimal im Repository. Unter `units/<nn-nn-name>/` liegt die
Aufgabe: die README mit der Erklärung, `src/lib.rs` mit den Rümpfen, die
`todo!()` sind, und `tests/exercise.rs` mit den Tests, die deshalb rot sind.
Unter `solutions/<nn-nn-name>/` liegt die Lösung. Sie hat keine eigene
Testdatei, sondern bindet die der Einheit ein, damit beide Seiten gegen genau
dieselben Tests laufen.

Beide Pakete tragen denselben Paketnamen, sonst übersetzt die Lösung nicht. Sie
stören einander trotzdem nicht, weil sie in verschiedenen Workspaces liegen.

Das ausgebaute Muster ist `units/02-01-move/` mit `solutions/02-01-move/`. Wer
eine neue Einheit anlegt, sieht dort nach und kopiert `units/template/`. Wie das
geht, steht im Kopf der Vorlage.

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

### Lizenz und die Zeile Signed-off-by

Die Lerntexte stehen unter CC BY 4.0, der Code unter MIT. Lerntexte sind die
README im Wurzelverzeichnis und in jeder Einheit und der erklärende Fließtext in
den Doku-Kommentaren. Code ist alles unter `src/`, `tests/`, `solutions/` und
`xtask/`, dazu jede `Cargo.toml` und die Beispiele in den Doku-Kommentaren. Wer
einen ganzen Doku-Kommentar übernimmt, hält beide Bedingungen ein.

Die beiden Lizenzdateien im Wurzelverzeichnis, `LICENSE-CC-BY-4.0` und
`LICENSE-MIT`, kommen mit Issue #8. Bis dahin ist dieser Abschnitt die
Zuordnung und nicht der Lizenztext.

Wer einen Text unter CC BY 4.0 weiterverwendet, nennt dieses Repository als
Quelle, verlinkt es und sagt, ob er etwas geändert hat. Für den Code unter MIT
gilt das nicht.

Jeder Commit trägt eine `Signed-off-by`-Zeile. Sie entsteht von selbst mit
`git commit -s` und ist die Zustimmung zum Developer Certificate of Origin. Mit
ihr sagst du, dass du das Recht hast, diesen Beitrag unter der Lizenz des
Projekts einzureichen.

Der folgende Text steht nur auf Englisch, weil es keine verbindliche deutsche
Fassung von ihm gibt und eine selbst übersetzte etwas anderes bedeuten könnte.

    Developer's Certificate of Origin 1.1

    By making a contribution to this project, I certify that:

    (a) The contribution was created in whole or in part by me and I
        have the right to submit it under the open source license
        indicated in the file; or

    (b) The contribution is based upon previous work that, to the best
        of my knowledge, is covered under an appropriate open source
        license and I have the right under that license to submit that
        work with modifications, whether created in whole or in part
        by me, under the same open source license (unless I am
        permitted to submit under a different license), as indicated
        in the file; or

    (c) The contribution was provided directly to me by some other
        person who certified (a), (b) or (c) and I have not modified
        it.

    (d) I understand and agree that this project and the contribution
        are public and that a record of the contribution (including all
        personal information I submit with it, including my sign-off) is
        maintained indefinitely and may be redistributed consistent with
        this project or the open source license(s) involved.

Der vollständige Text mit seinem Urheberrechtsvermerk steht unter
https://developercertificate.org.

Ob eine `Signed-off-by`-Zeile da ist, prüft heute nichts von selbst. Sie fällt
nur beim Lesen auf, und das steht hier, damit ihr Fehlen nicht für unmöglich
gehalten wird.

### Der Ablauf

Zuerst ein Issue, dann ein Pull Request. Ausgenommen sind Tippfehler und kaputte
Links; die dürfen direkt als Pull Request kommen.

Ein Pull Request trägt eine Einheit oder einen Text, nicht zwei. Wer zwei
Einheiten in einen packt, bekommt ihn mit der Bitte zurück, ihn zu teilen. Der
Grund ist nicht Ordnungsliebe: zwei Einheiten in einem Rumpf bedeuten eine
Beschreibung, die eine von beiden meint.

Der Rumpf nennt sein Issue und trägt die Ausgabe des Prüflaufs. Lief ein Befehl
nicht, steht dort, welcher und warum. Ein leerer Abschnitt heißt nicht, dass
alles grün war.

Zusammengeführt wird von jemandem, der den Beitrag nicht geschrieben hat.
Solange nur eine Person dieses Repository pflegt, gilt das für Beiträge von
außen und nicht für die eigenen. Ein eigener Beitrag wird also ohne zweiten
Leser zusammengeführt, und der Rumpf sagt das dann auch.

### Einsprachige Beiträge

Ein Beitrag in nur einer Sprache wird angenommen. Der Pull Request bekommt
`text-de` oder `text-en`, je nachdem welche Sprache fehlt, und es wird ein
Übersetzungs-Issue geöffnet, das offen bleibt. Fertig ist eine Einheit erst mit
beiden Sprachen.

Zwei Dinge an diesem Weg sind nicht in Ordnung, und sie stehen hier, statt beim
Bauen aufzutauchen.

Erstens kann der Beitragende den Weg über seinen Branch zumachen. In den Fork
einer anderen Person schreiben geht nur, wenn dort "Allow edits by maintainers"
gesetzt ist, und bei Forks im Besitz einer Organisation geht es gar nicht. Ist
dieser Weg zu, öffnet das Projekt einen eigenen Pull Request, der die Commits mit
ihrer Urheberschaft und ihren `Signed-off-by`-Zeilen übernimmt, und der
ursprüngliche wird mit dem Grund in seinem Rumpf geschlossen.

Zweitens läuft genau dieser Weg gegen die Regel, dass zusammenführt, wer den
Beitrag nicht geschrieben hat, denn die ergänzte Hälfte schreibt dann die Person,
die auch zusammenführt. Bei einer Person lässt sich das nicht auflösen.
Stattdessen ist die ergänzte Hälfte ein eigener Commit, und der Rumpf nennt, wer
welche Hälfte geschrieben hat. Nichts weist den Fall zurück, er wird nur sichtbar
gemacht.

### Umgangston

Fragen von Anfängern sind der Zweck dieses Repositories. Eine Frage, die jemand
für dumm hält, ist meistens die Stelle, an der ein Text zu viel vorausgesetzt
hat, und dann ist der Text schuld und nicht die Frage.

Der Verhaltenskodex steht in `CODE_OF_CONDUCT.md` und gilt für alles hier.

## English

### Getting started

Cloning is as usual.

    git clone https://github.com/iderex/learn-rust.git
    cd learn-rust

The compiler version is not chosen by hand. It stands in `rust-toolchain.toml`,
and rustup fetches and uses it on the first command inside this directory by
itself. Whoever does not have rustup yet gets it from https://rustup.rs. Whether
the right version took hold is answered by the compiler itself.

    rustc --version

A number different from the one in `rust-toolchain.toml` means the command was
sent from outside the repository.

The commands for daily work are further down under
"Die Befehle / The commands" and are not copied here.

### Bilingual, German first

Every text in this repository stands in German and in English, and German comes
first. That holds for the README of every unit, for the doc comments, and for the
bodies of issues and pull requests.

The two sections should say the same thing. They are not a word for word
translation of each other, but what one explains the other explains too, with the
same examples and the same error numbers. A one line English section under a long
German one is not a bilingual text.

Whether the two versions really say the same thing is decided by a person
reading them. No check answers that.

### How a unit is built

A unit lives twice in the repository. Under `units/<nn-nn-name>/` lives the
exercise: the README with the explanation, `src/lib.rs` with the bodies that are
`todo!()`, and `tests/exercise.rs` with the tests that are therefore red. Under
`solutions/<nn-nn-name>/` lives the solution. It has no test file of its own but
includes the unit's, so that both sides run against exactly the same tests.

Both packages carry the same package name, otherwise the solution does not
compile. They still do not clash, because they live in different workspaces.

The worked model is `units/02-01-move/` with `solutions/02-01-move/`. Whoever
starts a new unit looks there and copies `units/template/`. How that goes is
written in the head of the template.

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

### Licence and the Signed-off-by line

The learning texts go under CC BY 4.0, the code under MIT. Learning texts are the
README at the root and in every unit and the explanatory prose in doc comments.
Code is everything under `src/`, `tests/`, `solutions/` and `xtask/`, plus every
`Cargo.toml` and the examples inside doc comments. Whoever takes a whole doc
comment meets both conditions.

The two licence files at the root, `LICENSE-CC-BY-4.0` and `LICENSE-MIT`, arrive
with issue #8. Until then this section is the assignment and not the licence
text.

Whoever reuses a text under CC BY 4.0 names this repository as the source, links
it, and says whether they changed anything. For the code under MIT that does not
apply.

Every commit carries a `Signed-off-by` line. It appears by itself with
`git commit -s` and is the agreement to the Developer Certificate of Origin. With
it you say that you have the right to submit this contribution under the licence
of the project. The wording is quoted in the German section above and is not
printed a second time here, because it is the same English text in both places.

Whether a `Signed-off-by` line is present is checked by nothing today. It is
caught only by reading, and that stands here so its absence is not taken to be
impossible.

### The process

An issue first, then a pull request. Typos and broken links are the exception and
may come straight as a pull request.

A pull request carries one unit or one text, not two. Whoever packs two units
into one gets it back with the request to split it. The reason is not tidiness:
two units in one body mean a description that means one of them.

The body names its issue and carries the output of the check run. If a command
did not run, it says which and why. An empty section does not mean everything was
green.

A contribution is merged by somebody who did not write it. While one person
maintains this repository, that holds for outside contributions and not for their
own. A contribution of their own is therefore merged without a second reader, and
the body says so when it happens.

### Single language contributions

A contribution in one language only is accepted. The pull request gets `text-de`
or `text-en`, whichever language is missing, and a translation issue is opened
and stays open. A unit counts as done only with both languages.

Two things about this route are not in order, and they stand here rather than
surfacing during the build.

First, the contributor can close the route through their branch. Writing into
somebody else's fork works only where "Allow edits by maintainers" is set, and
for forks owned by an organisation it does not work at all. If that route is
closed, the project opens a pull request of its own taking over the commits with
their authorship and their `Signed-off-by` lines, and the original is closed with
the reason in its body.

Second, that same route runs against the rule that a contribution is merged by
somebody who did not write it, because the added half is then written by the
person who also merges. With one person that cannot be resolved. Instead the
added half is a commit of its own, and the body names who wrote which half.
Nothing rejects the case, it is only made visible.

### Tone

Questions from beginners are the point of this repository. A question somebody
thinks is stupid is usually the place where a text assumed too much, and then the
text is at fault and not the question.

The code of conduct is in `CODE_OF_CONDUCT.md` and holds for everything here.

## Die Befehle / The commands

Deutsch: Zwei Arten von Befehlen kommen vor. Die einen braucht, wer eine Einheit
löst. Die anderen prüfen das Repository als Ganzes.

Der Prüflauf steht genau einmal im Repository, und dieser Abschnitt ist diese
eine Stelle. Deshalb sind die Blöcke hier auch nicht nach Sprachen getrennt: ein
zweiter Abdruck unter `## English` wäre schon der zweite Ort. Wie viele Befehle
der Prüflauf hat, steht nirgends als Zahl. Wer sie zählen will, liest den Block.

Für die beiden Befehle des Lernenden gilt das nicht. Sie stehen zusätzlich in
der README jeder Einheit, weil dort der Ordner genau dieser Einheit dasteht und
weil man sie beim Lösen dort liest und nicht hier. Das ist gewollt.

English: two kinds of command appear. One kind is what somebody solving a unit
needs. The other checks the repository as a whole.

The check run stands exactly once in the repository, and this section is that one
place. That is also why the blocks here are not split by language: a second
printing under `## English` would already be the second place. How many commands
the check run has is written nowhere as a number. Whoever wants to count them
reads the block.

For the learner's two commands that does not hold. They also stand in the README
of every unit, because there the folder of that very unit is written out and
because somebody solving reads them there and not here. That is intended.

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
