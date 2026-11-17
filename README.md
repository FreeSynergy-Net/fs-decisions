# FreeSynergy Decisions — decide together, and find a date

**Proposals, polls and finding a date, where the method is fixed before anyone votes and a person records the result.**

FreeSynergy Decisions is for the decisions a group makes together: a proposal with options, a vote with a method everyone can see beforehand, and the question "which day suits us?" answered the way Doodle does, with Yes, If needed and No. Nobody counts for you in secret: the rule is shown, the result is recorded by a person, and a machine never decides.

What is different here: the creator fixes the method before the vote and everyone sees it; "If needed" is not weighted, the three answers stay apart, and the suggestion says by which rule it was made. Anonymous or public is chosen when a decision is created and never switched. A decision can also be yours alone, as a diary with reasons.

## What it can do today

Version `0.0.1` (first version of this repository; the tag is set by the project owner). Nothing is released yet: the release waits until the signing environment of this new repository is set up.

- **A window and a server.** `fs-decisions` makes sure the server runs and opens the frontend `simple`: name, icon, what works today, what comes, and a first look at the main view with **example data and it says so** ("Preview", "Example"): a date poll grid (participants by time windows, Yes / If needed / No, counts apart, a suggested window with its rule) and the list "my decisions". Nothing in it is real, saved or sent. Everything that does not work yet says "coming soon". A missing, hanging or unfitting server is said in the window, never a grey window. Try `fs-decisions --version`, `fs-decisions --help`.
- The counting of votes (ten methods, an anonymous ballot box) exists as a library in fs-forms; it is not moved here yet and not connected to the window.

## What is not there yet

- Nothing can be created, answered or counted: the server answers only what it offers (nothing yet) and the contract version. The server keeps running after the window closes until it is stopped or replaced (no lease).
- The date poll, the decision with methods, results, invitations: see the page "What comes" in the window and `CHANGELOG.md`.
- Platform: Linux (Windows and macOS follow; the transport uses Unix sockets).
- Contract: draft (`surveys.capnp`, not locked). The basics are released with this draft contract. This frontend and server are its first consumers here.
- Gaps in the main view, named: no grid component with a chosen cell, no time-zone display block, no answer buttons in cells (the list shows words), no command model; see DESIGN.md "Gaps".
- The names `fs-tally` (counting) and `fs-ballot` (ballot box) are reserved as repositories; the name of the whole date-finding suite is still open.

## Part of FreeSynergy

FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>. Start with the [manifesto](https://freesynergy.github.io/fs-documentation/manifesto/) if you want to know why we do this.

## For developers

- Workspace crates: `fs-decisions-server` (the contract over a local socket), `fs-decisions-client` (the contract as a client), `fs-decisions-simple` (the frontend `simple`, a process of its own, depends only on the client and fs-components), `fs-decisions-bin` (the command `fs-decisions`: starter and `serve`). `fs-decisions-core` (the counting rule of the date poll; sorted in, not compiled yet).
- Engine and frontend are separate processes (ADR 0021, 0094 draft); the manifest of the frontend is `packaging/frontends/simple.metainfo.xml`.
- The contract is `surveys.capnp` in [fs-interfaces](https://github.com/FreeSynergy/fs-interfaces) (draft); fs-forms serves the same contract. The capability key stays `surveys` (display name "Decisions", ADR 0068).
- Build and test: `cargo build --workspace` and `cargo test --workspace`.
- Design and decisions: [DESIGN.md](DESIGN.md).
- Licence: MIT OR Apache-2.0.

## Auf Deutsch

**Vorschläge, Abstimmungen und Terminfindung, bei denen das Verfahren vor der Abstimmung feststeht und ein Mensch das Ergebnis festhält.** FreeSynergy Decisions ist für Entscheidungen, die eine Gruppe gemeinsam trifft: ein Vorschlag mit Optionen, eine Abstimmung mit einem Verfahren, das alle vorher sehen, und die Frage „Welcher Tag passt?“ so beantwortet, wie Doodle es tut, mit Ja, Wenn nötig und Nein.

**Was es heute kann.** Version `0.0.1` (erste Version dieses Repositories; den Tag setzt der Projektinhaber). Noch nichts ist veröffentlicht: das Release wartet, bis die Signier-Umgebung dieses neuen Repositories eingerichtet ist. `fs-decisions` startet den Server und das Frontend `simple` mit Name, Symbol, „Was es heute kann“, „Was kommt“ und einem ersten Blick auf die Hauptansicht mit **Beispieldaten, und sie sagt es** („Vorschau“, „Beispiel“): ein Termin-Raster (Teilnehmende mal Zeitfenster, Ja / Wenn nötig / Nein, Zahlen getrennt, vorgeschlagenes Fenster mit der Regel) und die Liste „Meine Entscheidungen“. Nichts darin ist echt, gespeichert oder gesendet. Was noch nicht geht, sagt „kommt noch“. Fehlt der Server, hängt er oder passt er nicht, steht das im Fenster. Das Zählen der Stimmen (zehn Verfahren, eine anonyme Urne) liegt als Bibliothek in fs-forms vor und ist noch nicht hierher verschoben.

**Plattform.** Linux (Windows und macOS folgen).

**Für Entwickler.** Frontend und Engine sind getrennte Prozesse; das Frontend hängt nur am Client und an fs-components. Bauen: `cargo build --workspace`, `cargo test --workspace`. Vertrag: Entwurf (`surveys.capnp`, nicht gesperrt). Die Grundfassung erscheint mit diesem Entwurf. Lücken der Hauptansicht (Ersatz-Komponenten, vorläufig): DESIGN.md „Gaps“. Lizenz: MIT OR Apache-2.0.
