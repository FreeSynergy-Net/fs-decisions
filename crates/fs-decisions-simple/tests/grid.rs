//! The rule of the date poll grid: the three answers are counted apart, "If needed" is not weighted, and the suggested
//! window follows the decision of the project owner (most commitments, then fewer "If needed", then the earliest).
//!
//! Pattern: none (plain test functions, no shared state).
//! Tests: example-based on the pure functions, plus the sample of the preview.

use fs_components_words::Locale;
use fs_decisions_simple::{Answer, MainView, Tally, best, tally, words};

fn t(yes: u32, if_needed: u32, no: u32) -> Tally {
    Tally { yes, if_needed, no }
}

#[test]
fn the_three_answers_are_counted_apart_and_commitments_are_yes_plus_if_needed() {
    let counted = tally([
        Answer::Yes,
        Answer::IfNeeded,
        Answer::No,
        Answer::Yes,
        Answer::No,
    ]);
    assert_eq!(counted, t(2, 1, 2));
    assert_eq!(counted.commitments(), 3);
}

#[test]
fn the_window_with_the_most_commitments_is_suggested() {
    assert_eq!(best(&[t(1, 1, 2), t(3, 0, 1), t(2, 0, 2)]), Some(1));
}

#[test]
fn on_a_tie_the_window_with_fewer_if_needed_wins_there_is_no_half_weight() {
    // 2 Yes + 2 If needed = 4 and 3 Yes + 1 If needed = 4: the second has fewer "If needed".
    assert_eq!(best(&[t(2, 2, 0), t(3, 1, 0)]), Some(1));
    assert_eq!(best(&[t(3, 1, 0), t(2, 2, 0)]), Some(0));
}

#[test]
fn on_a_further_tie_the_earliest_window_wins() {
    assert_eq!(best(&[t(2, 1, 1), t(2, 1, 1)]), Some(0));
}

#[test]
fn without_any_commitment_nothing_is_suggested() {
    assert_eq!(best(&[t(0, 0, 4), t(0, 0, 4)]), None);
    assert_eq!(best(&[]), None);
}

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
