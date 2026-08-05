# learn-rust

Rust lernen, von der ersten Zeile an / Learning Rust, from the first line on

## Deutsch

### Was das hier ist

Ein Lern-Repository für Rust. Es setzt kein Vorwissen voraus, weder in Rust noch
in einer anderen Sprache. Wer noch nie programmiert hat, fängt bei Stufe 0 an und
kommt von dort bis zu den fortgeschrittenen Themen.

Der Stoff steht in Einheiten. Eine Einheit erklärt einen Begriff, zeigt ihn an
einem Beispiel, zeigt den Fehler, den Anfänger an dieser Stelle wirklich machen,
und stellt Aufgaben. Die Aufgaben liegen als Rümpfe mit `todo!()` da, und die
Tests der Einheit sind so lange rot. Grün werden sie, wenn die Aufgabe gelöst
ist. Das ist die ganze Rückmeldung, und sie kommt vom Übersetzer und nicht von
einer Meinung.

Jeder Text steht auf Deutsch und auf Englisch, und Deutsch steht vorn.

### Der Weg durch die Stufen

Elf Stufen, in dieser Reihenfolge. Jede Stufe ist ein Meilenstein auf dem
Tracker, und die Einheiten der Stufe sind seine Issues. Was eine Stufe am Ende
können soll, steht in der Beschreibung ihres Meilensteins.

- [Stufe 0, vor der ersten Zeile](https://github.com/iderex/learn-rust/milestone/2)
- [Stufe 1, Grundbausteine](https://github.com/iderex/learn-rust/milestone/3)
- [Stufe 2, Ownership](https://github.com/iderex/learn-rust/milestone/4)
- [Stufe 3, eigene Datentypen](https://github.com/iderex/learn-rust/milestone/5)
- [Stufe 4, Programme, die wachsen](https://github.com/iderex/learn-rust/milestone/6)
- [Stufe 5, Generics, Traits, Lifetimes, Tests](https://github.com/iderex/learn-rust/milestone/7)
- [Stufe 6, ein Werkzeug bauen](https://github.com/iderex/learn-rust/milestone/8)
- [Stufe 7, Smart Pointer, Nebenläufigkeit, Trait-Objekte](https://github.com/iderex/learn-rust/milestone/9)
- [Stufe 8, warten, ohne zu blockieren](https://github.com/iderex/learn-rust/milestone/10)
- [Stufe 9, Muster, fortgeschrittene Sprachmittel, Makros](https://github.com/iderex/learn-rust/milestone/11)
- [Stufe 10, unter der Oberfläche](https://github.com/iderex/learn-rust/milestone/12)

Die Einheiten liegen unter `units/`, benannt nach Stufe und Nummer, zum Beispiel
`units/02-01-move/` für die erste Einheit der Stufe 2. Wer eine Einheit lösen
will, wechselt in ihren Ordner und lässt ihre Tests laufen. Die beiden Befehle
dafür stehen in [CONTRIBUTING.md](CONTRIBUTING.md) und werden hier nicht
abgeschrieben, damit sie nicht an zwei Stellen auseinanderlaufen.

Nicht jede Stufe steht schon. Was gebaut ist, sieht man an den geschlossenen
Issues des jeweiligen Meilensteins.

### Die Lösungen

Zu jeder Einheit liegt unter `solutions/` ein Ordner gleichen Namens mit einer
Lösung. Sie ist öffentlich, und nichts hindert daran, sie zu lesen.

Nachsehen hilft, wenn die Aufgabe gelöst ist und man wissen will, wie es jemand
anders geschrieben hätte. Es hilft auch, wenn man nach ehrlichem Versuchen an
einer Stelle feststeckt, an der nicht der Stoff der Einheit das Problem ist,
sondern etwas Handwerkliches.

Nachsehen zerstört die Übung, wenn es passiert, bevor die Fehlermeldung des
Übersetzers gelesen und verstanden wurde. Der Nutzen dieser Aufgaben liegt nicht
in der fertigen Funktion, sondern in den Minuten davor, in denen man eine Meldung
wie `error[E0382]` liest und begreift, wovon sie redet. Wer die überspringt,
bekommt eine grüne Ausgabe und behält die Lücke.

Das ist keine Regel, die etwas verbietet. Es ist die Beschreibung dessen, was
passiert.

### Mitmachen

Wie beigetragen wird, steht in [CONTRIBUTING.md](CONTRIBUTING.md). Fragen von
Anfängern sind der Zweck dieses Repositories und keine Störung. Der
Verhaltenskodex steht in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Die Planung läuft auf dem Tracker. Was noch niemand entschieden hat, trägt dort
das Label `zu-entscheiden / to be decided`.

## English

### What this is

A learning repository for Rust. It assumes no prior experience, neither in Rust
nor in another language. Somebody who has never programmed starts at stage 0 and
gets from there to the advanced topics.

The material stands in units. A unit explains a concept, shows it in an example,
shows the mistake beginners really make at that point, and sets exercises. The
exercises sit there as bodies with `todo!()`, and the unit's tests are red for as
long as they do. They go green when the exercise is solved. That is the whole
feedback, and it comes from the compiler rather than from an opinion.

Every text stands in German and in English, and German comes first.

### The way through the stages

Eleven stages, in this order. Each stage is a milestone on the tracker, and the
units of the stage are its issues. What a stage should leave you able to do is in
the description of its milestone.

- [Stage 0, before the first line](https://github.com/iderex/learn-rust/milestone/2)
- [Stage 1, building blocks](https://github.com/iderex/learn-rust/milestone/3)
- [Stage 2, ownership](https://github.com/iderex/learn-rust/milestone/4)
- [Stage 3, your own data types](https://github.com/iderex/learn-rust/milestone/5)
- [Stage 4, programs that grow](https://github.com/iderex/learn-rust/milestone/6)
- [Stage 5, generics, traits, lifetimes, tests](https://github.com/iderex/learn-rust/milestone/7)
- [Stage 6, building a tool](https://github.com/iderex/learn-rust/milestone/8)
- [Stage 7, smart pointers, concurrency, trait objects](https://github.com/iderex/learn-rust/milestone/9)
- [Stage 8, waiting without blocking](https://github.com/iderex/learn-rust/milestone/10)
- [Stage 9, patterns, advanced features, macros](https://github.com/iderex/learn-rust/milestone/11)
- [Stage 10, under the surface](https://github.com/iderex/learn-rust/milestone/12)

The units live under `units/`, named by stage and number, for example
`units/02-01-move/` for the first unit of stage 2. Whoever wants to solve a unit
changes into its folder and runs its tests. The two commands for that are in
[CONTRIBUTING.md](CONTRIBUTING.md) and are not copied here, so they cannot drift
apart in two places.

Not every stage stands yet. What is built shows in the closed issues of the
milestone in question.

### The solutions

For every unit there is a folder of the same name under `solutions/` holding a
solution. It is public, and nothing prevents reading it.

Looking helps once the exercise is solved and you want to know how somebody else
would have written it. It also helps when, after honestly trying, you are stuck
at a point where the problem is not the material of the unit but something
mechanical.

Looking destroys the exercise when it happens before the compiler's message has
been read and understood. The value of these exercises is not the finished
function but the minutes before it, in which you read a message like
`error[E0382]` and work out what it is talking about. Whoever skips those gets
green output and keeps the gap.

This is not a rule that forbids anything. It is a description of what happens.

### Contributing

How to contribute is in [CONTRIBUTING.md](CONTRIBUTING.md). Questions from
beginners are the point of this repository and not an interruption to it. The
code of conduct is in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Planning runs on the tracker. Anything nobody has decided yet carries the label
`zu-entscheiden / to be decided` there.

## Lizenz / License

Deutsch: Die Lerntexte stehen unter CC BY 4.0, der Code unter MIT. Lerntexte sind
diese README, die README jeder Einheit und der erklärende Fließtext in den
Doku-Kommentaren. Code ist alles unter `src/`, `tests/`, `solutions/` und
`xtask/`, dazu jede `Cargo.toml` und die Beispiele in den Doku-Kommentaren. Wer
einen ganzen Doku-Kommentar übernimmt, hält beide Bedingungen ein.

Wer einen Text unter CC BY 4.0 weiterverwendet, nennt dieses Repository als
Quelle, verlinkt es und sagt, ob er etwas geändert hat. Für den Code unter MIT
gilt das nicht.

Der volle Text steht in den beiden Dateien im Wurzelverzeichnis:
[LICENSE-CC-BY-4.0](LICENSE-CC-BY-4.0) und [LICENSE-MIT](LICENSE-MIT). Dieser
Abschnitt ist die Zuordnung und nicht der Lizenztext.

English: the learning texts go under CC BY 4.0, the code under MIT. Learning
texts are this README, the README of every unit and the explanatory prose in doc
comments. Code is everything under `src/`, `tests/`, `solutions/` and `xtask/`,
plus every `Cargo.toml` and the examples inside doc comments. Whoever takes a
whole doc comment meets both conditions.

Whoever reuses a text under CC BY 4.0 names this repository as the source, links
it, and says whether they changed anything. For the code under MIT that does not
apply.

The full text is in the two files at the root:
[LICENSE-CC-BY-4.0](LICENSE-CC-BY-4.0) and [LICENSE-MIT](LICENSE-MIT). This
section is the assignment and not the licence text.

Siehe NOTICE.md für den Hinweis zur bestimmungsgemäßen Nutzung.

See NOTICE.md for the intended-use notice.
