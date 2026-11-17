//! `fs-decisions-core`: the rules of FreeSynergy Decisions that are the program's own, as pure code (no storage, no window).
//!
//! Today: the counting rule of the date poll ([`count`]). Sorted in from the frontend `fs-decisions-simple` (rule
//! FRONTEND-NUR-KOMPONENTEN: logic belongs to the program, the frontend only plugs components together); sorted in without a build.
//!
//! Pattern: Facade: this file only declares modules and re-exports the public API.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod count;

pub use count::{Answer, Tally, best, tally};
