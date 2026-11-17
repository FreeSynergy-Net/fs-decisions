//! The sample of the main view: the date poll grid and the list "my decisions" with every word resolved, made only from
//! `preview.rs` and the rule of `grid.rs`.
//!
//! Pattern: Builder: one function from the words to typed rows and columns; no element is built here except the suggestion card.

use fs_components::blinc::layout::div::{Div, div};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Column, ColumnWidth, Row, badge_count};
use fs_components_words::{Args, Words};

use crate::grid::{Answer, Tally, best, tally};
use crate::main_view::paragraph;
use crate::preview::{DECISIONS, PEOPLE, STARTS, State, WINDOWS};

fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

fn plain(texts: &Mf2Texts, id: &str) -> String {
    say(texts, id, &Args::new())
}

fn answer_word(texts: &Mf2Texts, answer: Answer) -> String {
    plain(
        texts,
        match answer {
            Answer::Yes => "m-yes",
            Answer::IfNeeded => "m-if-needed",
            Answer::No => "m-no",
        },
    )
}

/// The sample: the grid and the list, with every word resolved. It exists only in [`Mode::Preview`].
#[derive(Clone, Debug)]
pub(crate) struct Sample {
    pub(crate) poll_title: String,
    pub(crate) grid_columns: Vec<Column>,
    pub(crate) grid_rows: Vec<Row>,
    pub(crate) suggestion: Option<(String, u32, String)>,
    pub(crate) rule: String,
    pub(crate) zone: String,
    pub(crate) list_title: String,
    pub(crate) list_columns: Vec<Column>,
    pub(crate) list_rows: Vec<Row>,
}

impl Sample {
    pub(crate) fn new(texts: &Mf2Texts) -> Self {
        let columns: Vec<Tally> = (0..WINDOWS)
            .map(|w| tally(PEOPLE.iter().map(|p| p.answers[w])))
            .collect();
        let suggested = best(&columns);
        let mut grid_columns = vec![Column::new(
            plain(texts, "m-col-people"),
            ColumnWidth::Share(1.2),
        )];
        for (w, start) in STARTS.iter().enumerate() {
            let title = if suggested == Some(w) {
                format!("{start} ({})", plain(texts, "m-suggested"))
            } else {
                (*start).to_owned()
            };
            grid_columns.push(Column::new(title, ColumnWidth::Share(1.0)));
        }
        let mut grid_rows: Vec<Row> = PEOPLE
            .iter()
            .map(|p| {
                let mut cells = vec![plain(texts, p.name)];
                cells.extend(p.answers.iter().map(|a| answer_word(texts, *a)));
                Row::new(p.key, cells)
            })
            .collect();
        for (key, id, pick) in [
            (
                "preview:sum-yes",
                "m-sum-yes",
                (|t: &Tally| t.yes) as fn(&Tally) -> u32,
            ),
            ("preview:sum-if", "m-sum-if-needed", |t: &Tally| t.if_needed),
            ("preview:sum-no", "m-sum-no", |t: &Tally| t.no),
            ("preview:sum-all", "m-sum-commitments", |t: &Tally| {
                t.commitments()
            }),
        ] {
            let mut cells = vec![plain(texts, id)];
            cells.extend(columns.iter().map(|t| pick(t).to_string()));
            grid_rows.push(Row::new(key, cells));
        }
        let suggestion = suggested.map(|w| {
            let t = columns[w];
            let said = say(
                texts,
                "m-suggestion",
                &Args::new()
                    .text("window", STARTS[w])
                    .text("yes", t.yes.to_string())
                    .text("if", t.if_needed.to_string()),
            );
            let spoken = say(
                texts,
                "m-commitments-spoken",
                &Args::new().text("count", t.commitments().to_string()),
            );
            (said, t.commitments(), spoken)
        });
        let list_rows = DECISIONS
            .iter()
            .map(|d| {
                let state = plain(
                    texts,
                    match d.state {
                        State::Open => "m-state-open",
                        State::Decided => "m-state-decided",
                    },
                );
                Row::new(
                    d.key,
                    vec![
                        plain(texts, d.title),
                        state,
                        d.deadline.unwrap_or("-").to_owned(),
                    ],
                )
            })
            .collect();
        Self {
            poll_title: plain(texts, "m-poll-title"),
            grid_columns,
            grid_rows,
            suggestion,
            rule: plain(texts, "m-rule"),
            zone: plain(texts, "m-zone"),
            list_title: plain(texts, "m-list-title"),
            list_columns: vec![
                Column::new(plain(texts, "m-col-title"), ColumnWidth::Share(2.0)),
                Column::new(plain(texts, "m-col-state"), ColumnWidth::Share(1.0)),
                Column::new(plain(texts, "m-col-deadline"), ColumnWidth::Share(1.0)),
            ],
            list_rows,
        }
    }
}

impl Sample {
    /// The suggestion as a card; the number of commitments is a badge on its corner (the words for a screen reader are part of it).
    pub(crate) fn suggestion_card(&self) -> Div {
        let card = div().relative().class("fs-dc-card");
        match &self.suggestion {
            Some((said, count, spoken)) => {
                let card = card.child(paragraph(said.clone()).build());
                match badge_count(*count, spoken) {
                    Some(badge) => card.child(badge),
                    None => card,
                }
            }
            None => card,
        }
    }
}
