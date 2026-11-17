//! The main view: the sample shows exactly while the server does not offer `decision.read`, in the right words, with
//! neutral names, and nothing of it survives a server that offers real data (rule AUSSEHEN-SOFORT).
//!
//! Pattern: none (plain test functions, no shared state).
//! Tests: example-based on the view model; the pictures of the window are made by the release check of the repository.

use fs_components_words::Locale;
use fs_decisions_simple::{MainView, Mode, READ_ID, mode, words};

fn ids(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn preview_mode_exactly_while_the_read_capability_is_missing() {
    assert_eq!(mode(&[]), Mode::Preview);
    assert_eq!(mode(&ids(&["survey.create"])), Mode::Preview);
    assert_eq!(mode(&ids(&["survey.create", READ_ID])), Mode::Live);
}

#[test]
fn the_sample_shows_without_the_capability_and_says_it_is_a_preview() {
    let view = MainView::new(&[], &words(&Locale::english()));
    assert!(view.is_preview());
    // 4 participants and 4 rows of sums (Yes, If needed, No, commitments).
    assert_eq!(view.grid_rows().len(), 8);
    let said = format!("{:?}", view.grid_rows());
    assert!(
        said.contains("Example A") && said.contains("If needed"),
        "{said}"
    );
}

#[test]
fn nothing_of_the_sample_survives_a_server_that_offers_real_data() {
    let view = MainView::new(&ids(&[READ_ID]), &words(&Locale::english()));
    assert!(!view.is_preview());
    assert!(view.grid_rows().is_empty());
    assert!(view.suggestion().is_none());
}

#[test]
fn the_names_are_neutral_examples_in_both_languages_and_the_keys_carry_the_preview_prefix() {
    for (locale, name) in [
        (Locale::english(), "Example D"),
        (Locale::parse("de").unwrap(), "Beispiel D"),
    ] {
        let view = MainView::new(&[], &words(&locale));
        let said = format!("{:?}", view.grid_rows());
        assert!(said.contains(name), "{said}");
    }
    let view = MainView::new(&[], &words(&Locale::english()));
    assert!(
        view.grid_rows()
            .iter()
            .all(|r| r.key.starts_with("preview:"))
    );
}

#[test]
fn the_three_answers_have_their_own_words_not_only_a_colour() {
    let view = MainView::new(&[], &words(&Locale::parse("de").unwrap()));
    let said = format!("{:?}", view.grid_rows());
    for word in ["Ja", "Wenn nötig", "Nein"] {
        assert!(said.contains(word), "{word}");
    }
}
