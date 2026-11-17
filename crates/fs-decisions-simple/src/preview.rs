//! The sample data of the preview: typed, neutral names, never stored, never sent, never taken from the server.
//!
//! Pattern: Null object: stands in for the real data while the server offers nothing to read. The ids carry the prefix
//! `preview:`; the names are words of the window ("Example A"), never real people (rule AUSSEHEN-SOFORT).

use crate::grid::Answer;

/// How many time windows the sample poll has.
pub const WINDOWS: usize = 3;

/// The start of each window, in UTC (the zone the poll is made in).
pub const STARTS: [&str; WINDOWS] = ["2026-12-03 17:00", "2026-12-04 09:00", "2026-12-05 14:00"];

/// One sample participant: the id, the id of the word that names it, the answers per window.
pub struct Person {
    /// The key of the row.
    pub key: &'static str,
    /// The id of the word ("Example A").
    pub name: &'static str,
    /// The answers in the order of [`STARTS`].
    pub answers: [Answer; WINDOWS],
}

/// Four sample participants; the second window has the same number of commitments as the third but fewer "If needed".
pub const PEOPLE: [Person; 4] = [
    Person {
        key: "preview:person-a",
        name: "m-sample-a",
        answers: [Answer::Yes, Answer::Yes, Answer::IfNeeded],
    },
    Person {
        key: "preview:person-b",
        name: "m-sample-b",
        answers: [Answer::Yes, Answer::IfNeeded, Answer::Yes],
    },
    Person {
        key: "preview:person-c",
        name: "m-sample-c",
        answers: [Answer::IfNeeded, Answer::Yes, Answer::IfNeeded],
    },
    Person {
        key: "preview:person-d",
        name: "m-sample-d",
        answers: [Answer::No, Answer::Yes, Answer::Yes],
    },
];

/// The state of a sample decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Voting is open.
    Open,
    /// A person recorded the result.
    Decided,
}

/// One row of the sample list "my decisions".
pub struct Decision {
    /// The key of the row.
    pub key: &'static str,
    /// The id of the word that names it.
    pub title: &'static str,
    /// Its state.
    pub state: State,
    /// The deadline, in UTC, if there is one.
    pub deadline: Option<&'static str>,
}

/// Three sample decisions.
pub const DECISIONS: [Decision; 3] = [
    Decision {
        key: "preview:decision-1",
        title: "m-sample-poll",
        state: State::Open,
        deadline: Some("2026-12-01"),
    },
    Decision {
        key: "preview:decision-2",
        title: "m-sample-motion",
        state: State::Open,
        deadline: Some("2026-12-10"),
    },
    Decision {
        key: "preview:decision-3",
        title: "m-sample-diary",
        state: State::Decided,
        deadline: None,
    },
];
