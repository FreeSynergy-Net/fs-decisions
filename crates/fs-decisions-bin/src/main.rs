//! `fs-decisions`: one binary for the program (ADR 0063). Pattern: Facade over the crates that stay libraries.
//!
//! ```text
//! fs-decisions                          make sure the server runs, open the frontend `simple`
//! fs-decisions [--frontend ID] [--socket PATH]
//! fs-decisions serve [options]          the server (Cap'n Proto, surveys.capnp)
//! fs-decisions --version                the version, without opening anything
//! fs-decisions --help                   this help
//! ```

mod starter;

use std::process::ExitCode;

use fs_decisions_server::ServeError;

const HELP: &str = "fs-decisions [--frontend ID] [--socket PATH]\n\
fs-decisions serve [--socket PATH] [--data DIR] [--actor IRI] [--instance DOMAIN] [--facet NAME]\n\
fs-decisions --version | --help\n\
Without words: makes sure the server runs and opens the frontend (default: simple).\n\
  serve            run the server (see `fs-decisions serve --help`)\n\
  --frontend ID    the frontend to open; if its contract does not fit the server, the standard frontend opens\n\
  --socket PATH    the server's socket\n\
Exit codes: 0 done, 1 failed, 2 wrong words";

fn main() -> ExitCode {
    let raw: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    match raw.first().and_then(|w| w.to_str()) {
        Some("--version" | "-V") if raw.len() == 1 => {
            println!("fs-decisions {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("--help" | "-h") if raw.len() == 1 => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Some("serve") => serve(&raw[1..]),
        _ => starter::run(&raw),
    }
}

fn serve(words: &[std::ffi::OsString]) -> ExitCode {
    match fs_decisions_server::serve(words.iter().cloned()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(ServeError::Usage(m)) => {
            eprintln!("fs-decisions: {m}");
            ExitCode::from(2)
        }
        Err(ServeError::Failed(m)) => {
            eprintln!("fs-decisions: {m}");
            ExitCode::FAILURE
        }
    }
}
