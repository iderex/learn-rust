# Beitragen / Contributing

Diese Datei ist noch nicht vollständig. Sie trägt heute nur die Zitierregel.
Der Prüflauf und die Anleitung zu den lokalen Befehlen gehören zu Issue #7.
Alles Übrige, also der Einstieg mit rustup, die Zweisprachigkeitsregel, der
Aufbau einer Einheit, die Lizenz, der DCO mit `git commit -s`, der Ablauf für
Beiträge und der Umgangston, gehört zu Issue #9. Dieser Absatz wird entfernt,
sobald diese Teile hier stehen.

This file is not complete yet. Today it carries the citation rule and nothing
else. The check run and the guide to the local commands belong to issue #7.
Everything else, meaning how to start with rustup, the bilingual rule, how a
unit is built, the licence, the DCO with `git commit -s`, the process for
contributions and the tone, belongs to issue #9. This paragraph goes as soon as
those parts are here.

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
