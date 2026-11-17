//! What the window knows about its server: the one state it shows at the top.
//!
//! Pattern: Value object; the state is decided once, by the handshake of the client crate. It holds codes and parameters,
//! never an English sentence of a library: the words come from the program's own MF2 files (de and en).

use fs_decisions_client::{EXPECTED_CONTRACT, Error, Hello, ServerStarter};
use std::path::Path;

/// The state of the link to the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Link {
    /// The server answered and speaks a contract this frontend understands.
    Connected {
        /// The contract version the server reported.
        contract: String,
        /// The capability ids the server offers (empty today); the main view shows its sample only while `decision.read` is missing.
        ids: Vec<String>,
    },
    /// The server speaks another contract version; the person is told, the window shows only what is planned.
    Mismatch {
        /// What the server reported.
        found: String,
        /// What this frontend needs at least.
        expected: String,
    },
    /// No fitting server could be reached.
    Down {
        /// `unreachable`, `timeout` or `protocol` (the code of the client error).
        code: &'static str,
        /// The technical detail, a parameter of the words.
        detail: String,
    },
}

impl Link {
    /// Ask the server on `socket` (never starting one: that is the starter's job, ADR 0094 draft item 4).
    #[must_use]
    pub fn probe(socket: &Path) -> Self {
        let nothing: &dyn ServerStarter = &|| {
            Err(std::io::Error::other(
                "this frontend does not start servers",
            ))
        };
        Self::of(Hello::fetch(socket, nothing, EXPECTED_CONTRACT))
    }

    /// The state for one answer of the client.
    #[must_use]
    pub fn of(answer: Result<Hello, Error>) -> Self {
        match answer {
            Ok(hello) => Self::Connected {
                contract: hello.contract,
                ids: hello.ids,
            },
            Err(Error::Contract { found, expected }) => Self::Mismatch { found, expected },
            Err(other) => Self::Down {
                code: other.code(),
                detail: other.detail(),
            },
        }
    }

    /// The capability ids the server offered; none when it is missing or does not fit.
    #[must_use]
    pub fn ids(&self) -> &[String] {
        match self {
            Self::Connected { ids, .. } => ids,
            _ => &[],
        }
    }

    /// The state for a missing socket path (no `$XDG_RUNTIME_DIR`).
    #[must_use]
    pub fn no_socket() -> Self {
        Self::Down {
            code: "unreachable",
            detail: "no socket".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Link;
    use fs_decisions_client::{Error, Hello};

    #[test]
    fn each_answer_gets_its_own_state() {
        let ok = Hello {
            contract: "0.1".into(),
            ids: vec![],
        };
        assert!(matches!(Link::of(Ok(ok)), Link::Connected { .. }));
        let bad = Error::Contract {
            found: "x".into(),
            expected: "y".into(),
        };
        assert!(matches!(Link::of(Err(bad)), Link::Mismatch { .. }));
        assert!(matches!(
            Link::of(Err(Error::Unreachable("no".into()))),
            Link::Down {
                code: "unreachable",
                ..
            }
        ));
        assert!(matches!(
            Link::of(Err(Error::Timeout { limit_ms: 1 })),
            Link::Down {
                code: "timeout",
                ..
            }
        ));
    }
}
