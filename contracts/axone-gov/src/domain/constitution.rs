use crate::contract::AxoneGovResult;
use crate::error::AxoneGovError;
use crate::gateway::logic::{query_service_ask, AxoneLogicQuery, QueryServiceAskRequest};
use crate::queries::validation::build_required_predicates_query;
use crate::state::StateAccess;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_hex, Binary, Checksum, QuerierWrapper};
use getset::{CopyGetters, Getters};

const REQUIRED_PREDICATES: [&str; 2] = ["decide/2", "decide/3"];

/// A governance constitution as a Prolog program.
///
/// Represents the governance rules that define how decisions are made
/// within the axone-gov contract.
#[derive(Clone, Debug, Getters, PartialEq)]
pub struct Constitution {
    #[getset(get = "pub")]
    bytes: Binary,
}

impl Constitution {
    /// Reconstruct a Constitution from bytes previously stored in contract state.
    pub(crate) fn from_state(bytes: Binary, _access: &StateAccess) -> Self {
        Self { bytes }
    }

    /// Create a new Constitution from a Prolog program.
    ///
    /// Returns an error if:
    /// - The bytes are not valid UTF-8
    /// - The Prolog program is syntactically invalid
    /// - The required predicates are missing
    /// - The Prolog engine encounters an error during validation
    pub fn try_new(
        bytes: Binary,
        querier: &QuerierWrapper<'_, AxoneLogicQuery>,
    ) -> AxoneGovResult<Self> {
        let source = std::str::from_utf8(bytes.as_slice())
            .map(ToString::to_string)
            .map_err(|err| AxoneGovError::ConstitutionUtf8(err.to_string()))?;

        let query = build_required_predicates_query(&REQUIRED_PREDICATES);
        let request = QueryServiceAskRequest::one(source, query);
        let response = query_service_ask(querier, request)
            .map_err(|err| AxoneGovError::PrologEngineError(err.to_string()))?;

        let answer = response
            .answer
            .as_ref()
            .ok_or(AxoneGovError::PrologEngineNoAnswer)?;

        if answer.results.is_empty() {
            let predicates = REQUIRED_PREDICATES.join(", ");
            return Err(AxoneGovError::ConstitutionInvalid(format!(
                "constitution is missing required predicates ({predicates})"
            )));
        }

        if let Some(error) = answer
            .results
            .iter()
            .find_map(|result| result.error.as_deref())
        {
            return Err(AxoneGovError::ConstitutionInvalid(format!(
                "predicate validation failed: {error}"
            )));
        }

        Ok(Self { bytes })
    }

    /// Get the constitution as a UTF-8 string.
    pub fn source(&self) -> &str {
        self.as_ref()
    }

    /// Compute the SHA256 hash of the constitution bytes.
    pub fn hash(&self) -> [u8; 32] {
        *Checksum::generate(self.bytes.as_slice()).as_ref()
    }

    /// Compute the SHA256 hash of the constitution bytes as a hex string.
    pub fn hash_hex(&self) -> String {
        to_hex(self.hash())
    }
}

impl AsRef<[u8]> for Constitution {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl AsRef<str> for Constitution {
    fn as_ref(&self) -> &str {
        #[allow(clippy::expect_used)] // Constitution bytes are guaranteed to be valid UTF-8
        std::str::from_utf8(<Self as AsRef<[u8]>>::as_ref(self))
            .expect("Constitution bytes are valid UTF-8")
    }
}

#[cw_serde]
#[derive(CopyGetters, Getters)]
pub struct ConstitutionStatus {
    #[getset(get_copy = "pub")]
    constitution_revision: u64,
    #[getset(get = "pub")]
    constitution_hash: [u8; 32],
}

impl ConstitutionStatus {
    pub fn new(constitution_revision: u64, constitution_hash: [u8; 32]) -> Self {
        Self {
            constitution_revision,
            constitution_hash,
        }
    }

    pub fn constitution_hash_hex(&self) -> String {
        to_hex(self.constitution_hash)
    }
}
