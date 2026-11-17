//! The first conversation with the server: connect, ask what it offers, check the contract.
//!
//! Pattern: Template method: handshake (`Control.version`, replace a server that differs), call, check.

use std::path::Path;
use std::time::Duration;

use fs_interfaces::surveys_capnp::surveys;
use fs_rpc::{Expected, ServerStarter, connect_matching};

use crate::contract::{EXPECTED_CONTRACT, check_contract};
use crate::error::Error;
use crate::names::PROGRAM;

/// The longest the whole first conversation may take (connect, handshake, `capabilities`): a server that accepts and never
/// answers must end the start with an error, never a hang (`BEFUND-einzelinstanz`). fs-rpc limits each of its steps itself.
pub const FIRST_CALL_LIMIT: Duration = Duration::from_secs(15);

/// What the server said about itself in the first call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hello {
    /// The contract version the server reports (`surveys 0.1 (draft)`).
    pub contract: String,
    /// The capability ids the server offers; empty today (everything stays locked in the frontend).
    pub ids: Vec<String>,
}

impl Hello {
    /// Connect to the server on `socket` (the `starter` starts or replaces it when needed), ask `capabilities` and check
    /// the contract version against `expected` (usually [`EXPECTED_CONTRACT`]). Runs its own runtime, so no caller needs one.
    ///
    /// # Errors
    /// [`Error::Unreachable`] when no fitting server could be reached, [`Error::Contract`] when its contract does not fit,
    /// [`Error::Protocol`] when an answer cannot be read.
    pub fn fetch(
        socket: &Path,
        starter: &dyn ServerStarter,
        expected: &str,
    ) -> Result<Self, Error> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| Error::Unreachable(e.to_string()))?;
        let local = tokio::task::LocalSet::new();
        let limit = FIRST_CALL_LIMIT;
        local.block_on(&runtime, async {
            tokio::time::timeout(limit, Self::converse(socket, starter, expected))
                .await
                .unwrap_or(Err(Error::Timeout {
                    limit_ms: u64::try_from(limit.as_millis()).unwrap_or(u64::MAX),
                }))
        })
    }

    async fn converse(
        socket: &Path,
        starter: &dyn ServerStarter,
        expected: &str,
    ) -> Result<Self, Error> {
        {
            let wanted = Expected::new(env!("CARGO_PKG_VERSION")).with_program(PROGRAM);
            let link = connect_matching::<surveys::Client>(socket, &wanted, starter)
                .await
                .map_err(|e| Error::Unreachable(e.to_string()))?;
            let reply = link
                .client
                .capabilities_request()
                .send()
                .promise
                .await
                .map_err(|e| Error::Protocol(e.to_string()))?;
            let reader = reply.get().map_err(|e| Error::Protocol(e.to_string()))?;
            let text = |r: capnp::Result<capnp::text::Reader<'_>>| -> Result<String, Error> {
                r.map_err(|e| Error::Protocol(e.to_string()))?
                    .to_string()
                    .map_err(|e| Error::Protocol(e.to_string()))
            };
            // `Surveys.capabilities` answers a `CapabilityList` as one field, not as flat fields.
            let list = reader
                .get_capabilities()
                .map_err(|e| Error::Protocol(e.to_string()))?;
            let contract = text(list.get_contract_version())?;
            let mut ids = Vec::new();
            for id in list.get_ids().map_err(|e| Error::Protocol(e.to_string()))? {
                ids.push(text(id)?);
            }
            check_contract(&contract, expected)?;
            Ok(Self { contract, ids })
        }
    }

    /// [`Hello::fetch`] against the contract this client was built for.
    ///
    /// # Errors
    /// As for [`Hello::fetch`].
    pub fn fetch_expected(socket: &Path, starter: &dyn ServerStarter) -> Result<Self, Error> {
        Self::fetch(socket, starter, EXPECTED_CONTRACT)
    }
}
