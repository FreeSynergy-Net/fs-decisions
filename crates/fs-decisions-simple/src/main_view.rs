//! The main view "Decisions": the date poll grid (participants by time windows, as in Doodle) and the list "my decisions", on
//! sample data while the server offers nothing to read. The sample says it is a sample ("Preview", "Example" in words).
//!
//! Pattern: Builder + Null object: [`Mode`] decides once whether the sample shows ([`mode`], from what the server offers); [`MainView`]
//! holds every word already resolved, so building the elements needs no texts. Built only from fs-components blocks.

use std::sync::Arc;

use fs_components::blinc::layout::div::{Div, div};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::states::{Alert, EmptyState, Severity};
use fs_components_widgets::{Block, Column, Glyph, Inline, ListView, Row, TextView, VecRows};
use fs_components_words::{Args, Words};

use crate::sample::Sample;

/// The capability id that stands for "the server can list decisions": when it is offered, the sample goes away.
pub const READ_ID: &str = "decision.read";

/// The height of a row of the lists, in pixels, and of the heading above them.
const ROW: f32 = 28.0;
const HEAD: f32 = 28.0;

/// What the main view shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Sample data, marked as a preview: the server offers nothing to read yet.
    Preview,
    /// The server offers decisions to read: no sample, only what it delivers (today: the empty state).
    Live,
}

/// The mode for the capability ids the server offered: [`Mode::Live`] exactly when it offers [`READ_ID`].
#[must_use]
pub fn mode(ids: &[String]) -> Mode {
    if ids.iter().any(|id| id == READ_ID) {
        Mode::Live
    } else {
        Mode::Preview
    }
}

fn say(texts: &Mf2Texts, id: &str, args: &Args) -> String {
    texts.say(&id.into(), args).text().to_owned()
}

fn plain(texts: &Mf2Texts, id: &str) -> String {
    say(texts, id, &Args::new())
}

fn heading(text: String) -> TextView {
    TextView::new(vec![Block::Heading {
        level: 3,
        text: Inline::plain(text),
    }])
}

pub(crate) fn paragraph(text: String) -> TextView {
    TextView::new(vec![Block::Paragraph(Inline::plain(text))])
}

/// The main view with every word already resolved.
#[derive(Clone, Debug)]
pub struct MainView {
    banner: (String, String),
    empty: (String, String),
    /// `None` in live mode: nothing of the sample survives a server that offers real data.
    sample: Option<Sample>,
}

impl MainView {
    /// Resolve the words for the capability ids the server offered.
    #[must_use]
    pub fn new(ids: &[String], texts: &Mf2Texts) -> Self {
        Self {
            banner: (
                plain(texts, "m-banner-title"),
                plain(texts, "m-banner-text"),
            ),
            empty: (plain(texts, "m-empty-title"), plain(texts, "m-empty-text")),
            sample: (mode(ids) == Mode::Preview).then(|| Sample::new(texts)),
        }
    }

    /// Whether the banner "Preview" and the sample show.
    #[must_use]
    pub fn is_preview(&self) -> bool {
        self.sample.is_some()
    }

    /// The rows of the grid (participants, then the sums); none in live mode.
    #[must_use]
    pub fn grid_rows(&self) -> &[Row] {
        self.sample.as_ref().map_or(&[], |s| &s.grid_rows)
    }

    /// The suggestion sentence, if a window has commitments.
    #[must_use]
    pub fn suggestion(&self) -> Option<&str> {
        let sample = self.sample.as_ref()?;
        sample.suggestion.as_ref().map(|s| s.0.as_str())
    }

    /// The elements, for a window `width` pixels wide.
    #[must_use]
    pub fn build(&self, width: f32) -> Div {
        let inner = (width - 48.0).max(200.0);
        let page = div().flex_col().gap(12.0).p(24.0);
        let Some(sample) = &self.sample else {
            // The server offers decisions, but this window cannot read them yet: say it plainly, never a sample.
            return page.child(
                EmptyState::new(&self.empty.0)
                    .glyph(Glyph::Calendar)
                    .text(&self.empty.1)
                    .id("dc-empty")
                    .build(),
            );
        };
        let (title, text) = &self.banner;
        let banner = Alert::new(Severity::Info, text)
            .title(title)
            .id("dc-banner")
            .build();
        page.child(banner)
            .child(heading(sample.poll_title.clone()).build())
            .child(list(
                "dc-grid",
                &sample.grid_columns,
                &sample.grid_rows,
                inner,
            ))
            .child(sample.suggestion_card())
            .child(paragraph(sample.rule.clone()).build())
            .child(paragraph(sample.zone.clone()).build())
            .child(heading(sample.list_title.clone()).build())
            .child(list(
                "dc-list",
                &sample.list_columns,
                &sample.list_rows,
                inner,
            ))
    }
}

#[allow(clippy::cast_precision_loss)] // a handful of rows
fn list(name: &str, columns: &[Column], rows: &[Row], width: f32) -> Div {
    let height = HEAD + rows.len() as f32 * ROW;
    div().child(
        ListView::new(columns.to_vec(), Arc::new(VecRows::new(rows.to_vec())))
            .named(name)
            .row_height(ROW)
            .build(width, height),
    )
}

/// The style sheet of the view: only theme variables, like the sheets of the components.
pub const STYLES: &str = r"
.fs-dc-card { display: flex; flex-direction: column; padding: var(--space-3); border-radius: var(--message-radius); border-width: 1px; border-color: var(--border); background: var(--surface); }
";
