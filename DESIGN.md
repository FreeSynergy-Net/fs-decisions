# fs-decisions – Design

Decisions, votes and finding a date, as a program of its own (ADR 0087, supplement of 2026-11-16: fs-decisions is a program; date finding is a facet; the input fields are embedded from fs-forms; the whole flow with calendar and invitations is a suite whose name is still open). Contract: `surveys.capnp` in `fs-interfaces` v0.0.27 (draft 0.1, provisional; its header is binding). Plan: steps D0 to D8 of the plan "decisions" (about 23.5 agent days); feature list: `features/decisions.md`. Program docs: `fs-documentation/docs/en/programs/apps/decisions/` (not written yet).

## State

Version 0.0.1: **the basics only** (window, server, client, starter). The repository was empty before. Nothing is moved: the ten counting methods and the ballot box stay in `fs-forms-methods` (fs-forms) until plan steps D1 (move to `fs-tally`, `fs-ballot`) and D2 (the contract `decisions_v1`).

| Area | State |
|---|---|
| Window, server, client, starter | done in 0.0.1; the server only reports what it offers |
| Main view (date poll grid, list of decisions) | preview on example data |
| Counting methods, ballot box | in fs-forms (library), not moved, not connected |
| Everything else in the contract | not built, reported unsupported |
| Tests | the tests of the basics (real sockets, the real binary) and the rule of the suggestion |

## Decisions (the owner may change them; all provisional)

* **Method fixed before the vote.** The creator sets the method; it is visible before voting; a group may prescribe a method (reference fs-groups). Owner decision 2026-11-17.
* **Date windows without weighting.** Yes, If needed and No stay apart. Suggested window: most commitments (Yes plus If needed); on a tie fewer "If needed"; then the earliest (the last step is ours, to stay deterministic). The rule is shown next to the suggestion. A person fixes the date. The rule is a pure function (`fs-decisions-core`, `count.rs`) with tests (`crates/fs-decisions-core/tests/count.rs`); sorted in from the frontend, not compiled yet.
* **Delegation of votes (`fs-delegate`):** later. Automation through workflows, not macros: commands and objects of Decisions become workflow blocks with the rights of whoever starts them.
* **Same contract as fs-forms.** `surveys.capnp` is owned by fs-forms in `fs-interfaces`; Decisions serves the same contract under its own program name `fs-decisions` and its own socket (`decisions.sock`). The capability key stays `surveys` (ADR 0068); the display name in `capabilities.txt` is already "Decisions". The split into `decisions_v1` is plan step D2 and is not done here.
* **No data folder yet.** `serve` accepts `--data` like the other servers but does not create or use it; a folder is made only through `create_private` or `prepare_data_dir` of fs-rpc when the first data path exists.

## The basics (0.0.1)

| Crate | What |
|---|---|
| `fs-decisions-server` | the contract over a local socket through `fs-rpc` v0.0.2 (`serve`); no lease: the server keeps running after the window closes until it is stopped or replaced |
| `fs-decisions-client` | handshake (`connect_matching`), `Hello::fetch` with a limit of 15 s for the whole first conversation (a hanging instance ends with an error code, never a hang; test), contract check (same main level, minor level not lower); errors carry a code and a detail, never a sentence for the window |
| `fs-decisions-simple` | the frontend `simple` (fs-components v0.0.25; only the client, a test checks `cargo tree`); pages Start, Decisions (the main view) and What comes; words in MF2, de and en |
| `fs-decisions-bin` | the command `fs-decisions`: starter with fallback (pure `decide`), `serve`, `--version`, `--help` before any window or socket |

`fs-decisions-core` (sorted in, NOT compiled): the counting rule of the date poll (answers, count per window, suggested window), pure code. Until the open talk is settled the preview of the frontend still calls it, which `tests/deps.rs` forbids (see CHANGELOG, Unreleased).

### The main view: a date poll grid in preview
Model: Doodle (closed; only feature names and the flow are used: a host offers time windows, participants mark Yes, If needed, No, the host picks) and Loomio (a list of decisions with state and deadline); concepts only, nothing copied. The page has a banner "Preview", the grid (`list_view`: participants by windows, words in the cells, four rows of counts: Yes, If needed, No, commitments), a card with the suggestion (no badge per row or card: the preview mark stands once), a left navigation with symbols (date polls, open, decided; not yet working), large headings, rows of 40 px, the rule, the zone note (all times UTC), and the list "my decisions" (title, state, deadline).
Example data live only in `preview.rs` of the frontend crate (typed, `preview:` ids, names "Example A" to "Example D" as words of the window, never real people; never stored, exported or sent). `mode(capabilities)` is `Preview` exactly while the server does not offer `decision.read`; then `MainView` holds no sample at all and shows an empty state (a test with a mutant check: `mode` always `Preview` makes the test red).

### Gaps (components that are missing; stand-ins used, to be replaced)
* Grid: `list_view` shows text cells only; no cell that can be chosen, no coloured answer chips (the three answers are words, not only colours); a grid with cell selection is planned in fs-components (data plan G1 to G3).
* Answer input: no time-window field (from fs-forms, number 57 of its feature list); the window cannot be answered yet.
* Time zones: no block that shows a time in the viewer's zone with the origin zone as a label; all times UTC (ADR 0085).
* Preview mark: `states::Alert` Info stands in for the planned `preview-notice` block (fs-components, after M3); no button "Hide example" yet.
* Menus: entries are locked placeholders; no command model (M1) and no shortcuts (M2), no context menu (M3); no person picker (M6) for the electorate.
* Filters Open and Decided, search, a bar chart for results (chart exists, not used yet).
* The picture of the empty state (`main-empty`) is not made.
* Blinc desktop has no screen-reader bridge (git-bug ec4693d); a list and a grid are read through the list component only.
* Contract: draft (`surveys.capnp`, not locked). The basics are released with this draft contract.

## Tests
Real sockets: handshake tests start the real server in a thread under one mutex (`Control.stop` ends every server of the process) and run the real binaries; the pure `decide` of the starter; the rule of the suggestion (`fs-decisions-core`, `tests/count.rs`), the suggestion in the sample (`tests/grid.rs`); the preview mode and the sample (`tests/main_view.rs`); `tests/deps.rs` (the frontend reaches neither the server nor a core in `cargo tree -e normal`).
