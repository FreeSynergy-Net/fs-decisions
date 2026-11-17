//! The sample of the preview names the suggested window in both languages (the counting rule itself is tested in
//! `fs-decisions-core`, `tests/count.rs`).
//!
//! Pattern: none (plain test functions, no shared state).
//! Tests: example-based on the sample of the preview.

use fs_components_words::Locale;
use fs_decisions_simple::{MainView, words};

#[test]
fn the_sample_suggests_its_second_window_by_the_tie_break_and_says_so_in_both_languages() {
    for (locale, word) in [
        (Locale::english(), "Suggestion"),
        (Locale::parse("de").unwrap(), "Vorschlag"),
    ] {
        let view = MainView::new(&[], &words(&locale));
        let said = view.suggestion().expect("the sample has a suggestion");
        assert!(said.contains(word), "{said}");
        // The windows 2 and 3 both have 4 commitments; window 2 has one "If needed", window 3 has two.
        assert!(said.contains("2026-12-04 09:00"), "{said}");
    }
}
