# Sicherheitsrichtlinie / Security Policy

## Deutsch

### Was dieses Repository ist, bevor irgendetwas anderes kommt

`learn-rust` lehrt Rust von der ersten Zeile an. Unter `units/` liegt je
Lerneinheit ein Crate, dessen Aufgabenrümpfe `todo!()` sind und dessen Tests rot
bleiben, bis jemand sie löst. Unter `solutions/` liegt zu jedem von ihnen ein
gleichnamiges Crate mit den ausgearbeiteten Aufgaben. `xtask/` trägt den
Prüflauf, der den Baum liest. Jeder Text steht auf Deutsch und dann auf Englisch.

Drei Messungen prägen alles Weitere.

Hier wird nichts ausgeliefert und nichts veröffentlicht. Jede `Cargo.toml`, die
ein Paket beschreibt, trägt `publish = false`; die beiden Workspace-Wurzeln
beschreiben kein Paket. Es gibt kein Paket auf einer Registry, keinen Dienst,
keinen Endpunkt, kein Konto und keine gespeicherten Daten, die jemandem gehören.

Hier hängt nichts an etwas außerhalb der Standardbibliothek. `Cargo.lock` im
Wurzelverzeichnis und `units/Cargo.lock` führen nur die lokalen Pakete dieses
Repositories auf und kein einziges Crate von außerhalb. Zusammen mit der in
`rust-toolchain.toml` gebundenen Toolchain ist das die ganze Lieferkette.

Der Code ist zum Abschreiben gedacht. Das ist der Sinn eines Lehrrepositories,
und es ist zugleich die eine Stelle, an der ein Fehler von hier über dieses
Repository hinausreicht.

### Wohin melden

Bitte die private Meldung von Schwachstellen über GitHub benutzen:

<https://github.com/iderex/learn-rust/security/advisories/new>

Diese Tür steht heute offen. Gemessen und nicht angenommen:

```console
$ gh api repos/iderex/learn-rust/private-vulnerability-reporting
{"enabled":true}
```

Die Meldung geht an mich und bleibt aus der Öffentlichkeit heraus, solange sie
offen ist. Wer das Formular lieber nicht benutzt, öffnet ein gewöhnliches Issue
mit dem Satz, dass er etwas zu melden hat, und ohne irgendetwas darüber, was es
ist; den privaten Faden öffne ich dann von meiner Seite aus.

Brauchbar in einer Meldung: der Pfad der Datei, die benutzte Toolchain und das
Kürzeste, womit sich das Problem zeigen lässt. Ein fallender Test, ein Miri-Lauf
oder acht Zeilen `main` sind mehr wert als eine Beschreibung.

### Was ich nicht verspreche

Ich verspreche keine Frist bis zur ersten Antwort. Dies ist das Lehrrepository
einer einzelnen Person, und eine Frist, die ich nicht halten kann, wäre
schlechter als gar keine: wer eine Antwort bis zu einem genannten Zeitpunkt
erwarten durfte und sie nicht bekommt, verbringt das Warten mit der Frage, ob
die Meldung überhaupt angekommen ist. Von Anfang an zu wissen, dass die Antwort
kommt, wenn sie kommt, ist die ehrlichere Lage, und es ist die, die ich
tatsächlich halten kann. Der Advisory-Faden bleibt dort, wo er eingereicht
wurde, und er ist die erste Stelle, in die ich sehe.

### Was hier wirklich eine Schwachstelle wäre

Vier Arten, in der Reihenfolge, in der sie mich kümmern.

**Unsolider Code, der lehrt.** Stufe 10 handelt von `unsafe`, rohen Zeigern,
undefiniertem Verhalten und FFI. `units/10-06-ffi-mit-extern-c` deklariert `abs`
und `strlen` hinter `extern "C"`, und die Lösung ruft sie auf; `10-01` und
`10-02` arbeiten mit dem, was `unsafe` erlaubt, und mit rohen Zeigern. Wo ein
`unsafe`-Block die Bedingung nennt, auf die er sich stützt, und diese Bedingung
in Wahrheit nicht gilt, oder wo eine Lösung unter `solutions/` undefiniertes
Verhalten erreicht, während ihr eigener Text sagt, dass sie es nicht tut, ist
das die Meldung, die ich am meisten will. Wer hier ein falsches Muster lernt,
schreibt es wieder hin, wo es zählt.

**Etwas im Baum, das sich auf dem Rechner eines Lesers danebenbenimmt.**
`CONTRIBUTING.md` zu folgen heißt, dieses Repository zu klonen und den Prüflauf
über beide Workspaces zu starten, was Code von hier übersetzt und ausführt.
`06-08-build-rs` und seine Lösung tragen ein Build-Skript, das vor jedem Test
läuft und in `OUT_DIR` schreibt. `cargo test --workspace` startet die
Lösungstests, und zwei davon, die zu `09-08` und `09-09`, binden einen echten
TCP-Listener auf `127.0.0.1:0`. All das steht in den Dateien selbst. Wenn
irgendetwas davon etwas tut, was sein eigener Text nicht sagt, außerhalb des
`OUT_DIR` seines Pakets schreibt, eine andere Adresse als Loopback bindet, ins
Netz greift oder eine Datei öffnet, die ihm nie gereicht wurde, ist das ein
Fehler, und ich will davon hören.

**Ein Weg an dem vorbei, was die CI darf.** `.github/workflows/prueflauf.yml`
läuft an jedem Pull Request und schickt `cargo run -p xtask -- ci` ab, was den
Befehlsblock aus `CONTRIBUTING.md` liest und jede Zeile mit `Command::new`
startet, ohne eine Shell dazwischen. Ein Pull Request, der diesen Block ändert,
ändert also, was die CI ausführt. Für sich genommen ist das kein Befund: der
Ablauf gewährt nur `permissions: contents: read`, das voreingestellte
Workflow-Token des Repositories ist nur lesend und kann keine Pull Requests
genehmigen, das Repository hält keine Actions-Secrets, und die CI übersetzt und
startet ohnehin schon den Rust-Code des Pull Requests selbst, der über ein
Build-Skript tun kann, was dieser Code will. Ein Befund ist ein Weg zu etwas,
das der Ablauf nicht halten soll: Schreibzugriff, ein Token, der Cache eines
anderen Branches. `xtask/src/befehle.rs` lehnt jede Befehlszeile ab, die ein
Anführungszeichen, ein rückwärtiges Anführungszeichen, ein Dollarzeichen, ein
Und-Zeichen, ein Semikolon, ein Rohr, eine spitze Klammer, eine runde Klammer,
eine geschweifte Klammer oder einen umgekehrten Schrägstrich trägt, damit eine
Zeile nicht als das eine gelesen und als etwas anderes ausgeführt werden kann.
Eine Zeile, die an dieser Absperrung vorbeikommt und es trotzdem schafft, ist
ebenfalls ein Befund.

**Die Abläufe selbst.** Es sind zwei. `codeql.yml` bindet seine Actions an eine
Commit-SHA und checkt mit `persist-credentials: false` aus. `prueflauf.yml`
benutzt `actions/checkout@v7`, eine Marke, die unter ihm verschoben werden kann.
Ich weiß von dieser Ungleichheit und habe sie noch nicht behoben; eine Meldung,
die sie oder sonst etwas in diesen beiden Dateien als erreichbar zeigt, ist
willkommen.

### Was hier keine Schwachstelle ist

Die Aufgaben fallen. Ihre Rümpfe sind `todo!()`, und ihre Tests sind rot, bis
jemand den fehlenden Code schreibt. Dieses Rot ist die Rückmeldung, auf der
dieses Repository gebaut ist, und kein kaputter Build.

Die Lösungen sind öffentlich und vollständig. Eine zu lesen, bevor man die
Aufgabe macht, kostet den Leser die Aufgabe, und `README.md` sagt das
unumwunden. Das ist eine Beschreibung dessen, was geschieht, und keine Maßnahme,
die ich durchzusetzen versuche.

Der Webserver in `09-08` und `09-09` hat kein TLS, keine Anmeldung, keine
Kopfzeilenauswertung, keine Größenbeschränkung und keine Zeitgrenze. Er
beantwortet zwei Pfade und eine 404, und der einzige Test, der über das Netz
geht, bindet `127.0.0.1:0`. Es ist der kleine blockierende Server vom Ende des
Buches, mit Absicht klein. Eine Liste dessen, was ein Produktionsserver hätte,
sagt nichts über dieses Programm. Zu zeigen, dass er etwas anderes als Loopback
bindet oder eine Datei erreicht, die ihm nie gereicht wurde, ist eine andere
Meldung, und die will ich sehr wohl.

Es gibt `unsafe`-Code, und es gibt Code, der mit Absicht undefiniert ist.
`10-03-undefiniertes-verhalten` trägt ein Enum aus acht Schritten, vier davon
undefiniertes Verhalten, und bittet den Leser, jeden einzelnen in der Reference
nachzuschlagen. `10-01` heißt "was unsafe erlaubt". Zu melden, dass dieses
Repository `unsafe` enthält, beschreibt Stufe 10 und keinen Fehler. Zu melden,
dass ein bestimmter Block unsolide ist, ist der erste Punkt weiter oben.

Scanner-Ausgabe, an der kein Argument hängt. CodeQL läuft über diesen Baum bei
jedem Push, bei jedem Pull Request und einmal die Woche mit dem Paket
`security-and-quality`, ich sehe also schon, was er sagt. Eine eingefügte
Warnung ist keine Meldung. Was jemand durch sie hindurch erreicht, wäre eine.

Alles, was eine ausgelieferte Instanz, ein veröffentlichtes Crate oder
Nutzerdaten betrifft. Keines der drei gibt es. Wenn jemand diesen Code genommen,
ausgeliefert und dabei Schaden erlitten hat, gehört der Fehler im Code hierher
gemeldet und nicht die Auslieferung.

Falsche Übersetzungen, kaputte Verweise, veraltete Kapitelnummern in
Quellenangaben oder eine Einheit, die etwas falsch erklärt, ohne dass es die
Sicherheit berührt. Der Tracker will all das haben. Es privat zu melden,
verzögert die Behebung nur.

### Nach einer Meldung

Ich lese sie, entscheide, ob es ein Fehler dieses Repositories ist, und sage,
was von beidem. Wenn ja, landet die Behebung offen auf `main`. Es gibt hier
nichts, das eine stille Veröffentlichung schützen würde: kein veröffentlichtes
Paket, keine Auslieferung und niemanden, der eine Fassung betreibt, die ich
durch eine Behebung in der Öffentlichkeit gefährden könnte. Wer in der Advisory
genannt werden will, sagt das und sagt, wie. Wer nichts sagt, wird von mir nicht
genannt.

## English

### What this repository is, before anything else

`learn-rust` teaches Rust from the first line. Under `units/` sits one crate per
lesson, whose exercise bodies are `todo!()` and whose tests stay red until
somebody solves them. Under `solutions/` sits a crate of the same name for every
one of them, with the exercises worked out. `xtask/` holds the check run that
reads the tree. Every text stands in German and then in English.

Three measurements shape everything below.

Nothing here is deployed or published. Every `Cargo.toml` that describes a
package carries `publish = false`; the two workspace roots describe no package.
There is no package on a registry, no service, no endpoint, no account, and no
stored data belonging to anybody.

Nothing here depends on anything outside the standard library. `Cargo.lock` at
the root and `units/Cargo.lock` list only the local packages of this repository
and not one crate from outside it. Together with the toolchain pinned in
`rust-toolchain.toml`, that is the whole supply chain.

The code is meant to be copied. That is the point of a teaching repository, and
it is also the one place where a fault here reaches past this repository.

### Where to report

Use GitHub's private vulnerability reporting:

<https://github.com/iderex/learn-rust/security/advisories/new>

That door opens today. Measured rather than assumed:

```console
$ gh api repos/iderex/learn-rust/private-vulnerability-reporting
{"enabled":true}
```

The report goes to me and stays out of public view while it is open. If you
would rather not use the form, open a normal issue saying that you have
something to report and nothing about what it is, and I will open the private
thread from my side.

Useful in a report: the path of the file, the toolchain you used, and the
shortest thing you can hand me that shows the problem. A failing test, a Miri
run, or eight lines of `main` are worth more than a description.

### What I do not promise

I do not promise a time to first answer. This is one person's teaching
repository, and a deadline I cannot keep would be worse than none at all: a
reporter who was told to expect a reply by some point and does not get one
spends the wait wondering whether the report arrived at all. Knowing from the
start that the answer comes when it comes is the more honest position, and it is
the one I can actually hold. The advisory thread stays where you filed it, and
it is the first place I look.

### What would actually be a vulnerability here

Four kinds, in the order I care about them.

**Unsound code that teaches.** Stage 10 is about `unsafe`, raw pointers,
undefined behaviour and FFI. `units/10-06-ffi-mit-extern-c` declares `abs` and
`strlen` behind `extern "C"` and its solution calls them; `10-01` and `10-02`
work with what `unsafe` allows and with raw pointers. Where an `unsafe` block
names the condition it relies on and that condition does not in fact hold, or
where a solution under `solutions/` reaches undefined behaviour while its own
text says it does not, that is the report I most want. A reader who learns a
wrong pattern here writes it again somewhere it matters.

**Something in the tree that misbehaves on a reader's machine.** Following
CONTRIBUTING.md means cloning this repository and running the check run over
both workspaces, which compiles and runs code from here. `06-08-build-rs` and
its solution carry a build script that runs before any test and writes into
`OUT_DIR`. `cargo test --workspace` runs the solution tests, and two of those,
for `09-08` and `09-09`, bind a real TCP listener on `127.0.0.1:0`. All of that
is described in the files themselves. If any of it does something its own text
does not say, writes outside its package's `OUT_DIR`, binds an address other
than loopback, reaches the network, or opens a file it was never handed, that is
a defect and I want to hear about it.

**A way past what CI is allowed to do.** `.github/workflows/prueflauf.yml` runs
on every pull request and sends `cargo run -p xtask -- ci`, which reads the
command block out of CONTRIBUTING.md and starts each line with `Command::new`,
with no shell in between. A pull request that edits that block therefore changes
what CI executes. By itself that is not a finding: the workflow grants only
`permissions: contents: read`, the repository's default workflow token is read
only and cannot approve pull requests, the repository holds no Actions secrets,
and CI already compiles and runs the pull request's own Rust code, which through
a build script can do whatever that code wants. A finding is a route to
something the workflow is not supposed to hold: write access, a token, another
branch's cache. `xtask/src/befehle.rs` refuses any command line carrying a
quote, a backtick, a dollar sign, an ampersand, a semicolon, a pipe, an angle
bracket, a parenthesis, a brace or a backslash, so that a line cannot read as
one thing and execute as another. A line that gets past that guard and still
manages it is a finding as well.

**The workflows themselves.** There are two. `codeql.yml` pins its actions by
commit SHA and checks out with `persist-credentials: false`. `prueflauf.yml`
uses `actions/checkout@v7`, a tag that can be moved under it. I know about that
inconsistency and have not fixed it yet; a report that shows it or anything else
in those two files being reachable is welcome.

### What is not a vulnerability here

The exercises fail. Their bodies are `todo!()` and their tests are red until
somebody writes the missing code. That red is the feedback this repository is
built on, not a broken build.

The solutions are public and complete. Reading one before doing the exercise
costs the reader the exercise, which README.md says plainly. It is a description
of what happens and not a control I am trying to enforce.

The web server in `09-08` and `09-09` has no TLS, no authentication, no header
parsing, no size limit and no timeout. It answers two paths and a 404, and the
only test that goes over the network binds `127.0.0.1:0`. It is the small
blocking server from the end of the book, deliberately small. A list of what a
production server would have says nothing about this program. Showing it binding
something other than loopback, or reaching a file it was never handed, is a
different report, and I do want that one.

There is `unsafe` code, and there is code that is undefined on purpose.
`10-03-undefiniertes-verhalten` carries an enum of eight steps, four of them
undefined behaviour, and asks the reader to look each one up in the Reference.
`10-01` is called "what unsafe allows". Reporting that this repository contains
`unsafe` describes stage 10 rather than a defect. Reporting that one specific
block is unsound is the first item above.

Scanner output with no argument attached to it. CodeQL runs over this tree on
every push, every pull request and once a week with the `security-and-quality`
pack, so I already see what it says. A pasted alert is not a report. What
somebody reaches through it would be.

Anything about a deployed instance, a published crate, or user data. None of the
three exists. If somebody has taken this code, deployed it, and come to harm,
the thing to report here is the fault in the code and not the deployment.

Wrong translations, broken links, stale chapter numbers in citations, or a unit
that explains something incorrectly with no safety consequence. The tracker
wants all of those. Filing them privately only delays the fix.

### After a report

I read it, decide whether it is a fault of this repository, and tell you which.
If it is, the fix lands on `main` in the open. There is nothing here to protect
by staging a quiet release: no published package, no deployment, and nobody
running a version I could put at risk by fixing it in public. If you want to be
named in the advisory, say so and say how. If you say nothing, I will not name
you.
