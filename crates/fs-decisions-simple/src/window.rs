//! The window: name, the real icon (embedded PNG, own design), the content.
//!
//! Pattern: Adapter: our blocks in, a screen of the fs-components shell out; no widget is painted by hand.

use std::sync::Arc;

use fs_components::blinc::layout::div::div;
use fs_components::blinc::layout::image::image;
use fs_components_app::{AppError, AppWidget, Element, Menu, MenuEntry, Screen, Status, Surface};
use fs_components_mf2::Mf2Texts;
use fs_components_widgets::{Block, Inline, TextView, styles};
use fs_components_words::{Args, Words};

use crate::content::{coming, start};
use crate::link::Link;
use crate::main_view::{self, MainView};

/// The icon of the program, embedded in the binary.
const ICON: &[u8] = include_bytes!("../assets/fs-decisions.png");

fn icon_uri() -> String {
    use base64::Engine;
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(ICON)
    )
}

/// What a screen shows.
enum Body {
    /// Text blocks, with the header (name, icon, goal) above them on the start page.
    Text { blocks: Vec<Block>, header: bool },
    /// The main view: the date poll grid and the list of decisions.
    Main(MainView),
}

/// The one kind of screen of the window.
struct Home {
    name: String,
    tagline: String,
    body: Body,
}

impl Home {
    fn header(&self) -> impl Element + use<> {
        // The icon of the program (own design, embedded; see packaging/ICON.md), as a data address Blinc loads; 64 px,
        // directly at the title (16 px apart), vertically centred; `.no_fade()` so it is not pale.
        let icon = image(icon_uri()).size(64.0, 64.0).no_fade();
        let title = TextView::new(vec![Block::Heading {
            level: 1,
            text: Inline::plain(self.name.as_str()),
        }])
        .build();
        let tagline =
            TextView::new(vec![Block::Paragraph(Inline::plain(self.tagline.as_str()))]).build();
        div()
            .flex_col()
            .gap(4.0)
            .child(
                div()
                    .flex_row()
                    .items_center()
                    .gap(16.0)
                    .child(icon)
                    .child(title),
            )
            .child(tagline)
    }
}

impl Surface for Home {
    fn build(&self, width: f32, _height: f32) -> Box<dyn Element> {
        match &self.body {
            Body::Text { blocks, header } => {
                let page = div().flex_col().gap(12.0).p(24.0);
                let body = TextView::new(blocks.clone()).build();
                Box::new(if *header {
                    page.child(self.header()).child(body)
                } else {
                    page.child(body)
                })
            }
            Body::Main(view) => Box::new(view.build(width)),
        }
    }

    fn styles(&self) -> String {
        format!("{}{}", styles(), main_view::STYLES)
    }
}

/// The window for `link`, with the menus whose entries are all locked ("coming soon").
///
/// # Errors
/// The shell could not be made (a name that is empty, for example).
pub fn app(texts: &Mf2Texts, link: &Link) -> Result<AppWidget, AppError> {
    let say = |id: &str| texts.say(&id.into(), &Args::new()).text().to_owned();
    let soon = say("soon");
    let page = |body: Body| {
        Arc::new(Home {
            name: say("app-name"),
            tagline: say("app-tagline"),
            body,
        })
    };
    // No data flows through the contract yet, so the main view shows the sample, marked as a preview, until the server
    // offers `decision.read` (`main_view::mode`).
    let main = MainView::new(link.ids(), texts);
    let title = if main.is_preview() {
        format!("{} ({})", say("app-name"), say("preview"))
    } else {
        say("app-name")
    };
    let widget = AppWidget::new(
        title,
        vec![
            Screen::new(
                "home",
                say("screen-start"),
                page(Body::Text {
                    blocks: start(texts, link),
                    header: true,
                }),
            ),
            Screen::new("main", say("screen-main"), page(Body::Main(main))),
            Screen::new(
                "coming",
                say("screen-coming"),
                page(Body::Text {
                    blocks: coming(texts),
                    header: false,
                }),
            ),
        ],
    )?;
    let locked = |id: &str| {
        MenuEntry::new(id, format!("{} ({soon})", say(&format!("menu-{id}")))).disabled()
    };
    widget.set_menus(
        vec![Menu::new(
            say("menu-program"),
            vec![locked("new"), locked("poll"), locked("record")],
        )],
        |_| {},
    )?;
    widget.set_status(Some(Status::Info(say("status-ready"))));
    Ok(widget)
}
