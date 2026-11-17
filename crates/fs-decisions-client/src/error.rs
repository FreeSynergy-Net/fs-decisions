//! Why a frontend could not use the server, in words for a person (ADR 0065: a visible message, never a black window).
//!
//! Pattern: none: a plain error enum.

use std::fmt;

/// Why no usable server was reached.
#[derive(Debug)]
pub enum Error {
    /// The server speaks another contract version than this client needs; `found` is what it said.
    Contract {
        /// What the server reported.
        found: String,
        /// What this client needs at least.
        expected: String,
    },
    /// No fitting server could be reached (not running, another user, another program on the socket, a restart failed).
    Unreachable(String),
    /// The server accepted the connection but did not answer in time (a hanging instance): nothing waits without a limit.
    Timeout {
        /// The limit that passed, in milliseconds.
        limit_ms: u64,
    },
    /// The server answered something this client cannot read.
    Protocol(String),
}

impl Error {
    /// The code a program turns into its own words (MF2, de and en): `contract`, `unreachable`, `timeout` or `protocol`.
    /// The sentences of `Display` are English and for logs; a window never shows them.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::Contract { .. } => "contract",
            Self::Unreachable(_) => "unreachable",
            Self::Timeout { .. } => "timeout",
            Self::Protocol(_) => "protocol",
        }
    }

    /// The technical detail of the error, a parameter for the words (empty for a contract error: it has its own parameters).
    #[must_use]
    pub fn detail(&self) -> String {
        match self {
            Self::Unreachable(d) | Self::Protocol(d) => d.clone(),
            Self::Timeout { limit_ms } => limit_ms.to_string(),
            Self::Contract { .. } => String::new(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract { found, expected } => write!(
                f,
                "the server speaks \"{found}\", this frontend needs \"{expected}\" (same main level, minor level not lower)"
            ),
            Self::Unreachable(why) => write!(f, "no fitting server: {why}"),
            Self::Timeout { limit_ms } => {
                write!(f, "the server did not answer within {limit_ms} ms")
            }
            Self::Protocol(why) => write!(f, "the server answered something unreadable: {why}"),
        }
    }
}

impl std::error::Error for Error {}
