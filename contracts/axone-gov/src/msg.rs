use crate::contract::AxoneGov;

use crate::domain::constitution::ConstitutionStatus;
use crate::domain::Constitution;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Binary;

abstract_app::app_msg_types!(AxoneGov, AxoneGovExecuteMsg, AxoneGovQueryMsg);

/// Instantiate message.
///
/// In Axone, a resource is represented by an Abstract Account (AA).
/// Instantiating this `gov` app on the resource AA attaches a **governance capability** to that resource.
///
/// A **constitution** (or governance constitution) is the Prolog program stored by this contract.
/// It expresses the resource governance as rules that decide cases and may query on-chain facts via the
/// Axone logic module.
///
/// The `constitution` payload MUST be a UTF-8 encoded Prolog program.
///
/// On instantiation, the contract validates that the program can be evaluated and that it defines
/// the required entrypoints:
///
///    - `decide/2` as `governance:decide(+Case, -Verdict)`
///    - `decide/3` as `governance:decide(+Case, -Verdict, -Motivation)`
///
/// Where:
///
///    - `Case` is a Prolog dict term (typically `ctx{...}`) representing the decision context.
///      It can include any key-value facts required by the constitution (e.g. intent, actor, subject).
///    - `Verdict` is an arbitrary Prolog term (atom or compound) representing the decision outcome.
///    - `Motivation` is an arbitrary Prolog term intended to justify the verdict (e.g. applicable articles,
///      findings, interpretation rules).
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneGovInstantiateMsg {
    /// The constitution (UTF-8 Prolog program bytes).
    pub constitution: Binary,
}

/// Execute messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneGovExecuteMsg {
    /// Propose a constitutional revision (constitutional amendment).
    ///
    /// The contract asks the **current** constitution to decide whether the revision is allowed by
    /// evaluating a case that includes the intent `gov:revise_constitution`.
    ///
    /// The revision is applied only if the decision verdict is exactly the atom `gov:permitted`.
    /// Any other verdict (atom or compound term) refuses the revision.
    ReviseConstitution {
        /// The proposed new constitution (UTF-8 Prolog program bytes).
        constitution: Binary,
        /// Optional additional decision context provided by the caller.
        ///
        /// This is a Prolog dict term string (typically `ctx{...}`) merged into the case used to
        /// evaluate the `gov:revise_constitution` intent.
        case: Option<String>,
    },
}

/// Migrate message.
///
/// Reserved for future migrations.
#[cosmwasm_schema::cw_serde]
pub struct AxoneGovMigrateMsg {}

/// Query messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneGovQueryMsg {
    /// Return the currently stored constitution (raw Prolog program bytes).
    #[returns(ConstitutionResponse)]
    Constitution {},

    /// Return the current constitution metadata (revision and hash).
    #[returns(ConstitutionStatusResponse)]
    ConstitutionStatus {},

    /// Decide a case using the stored constitution.
    ///
    /// The `case` parameter is a Prolog dict term string (typically `ctx{...}`) representing the decision context.
    /// This is passed as the `Case` argument to `governance:decide/2` or `governance:decide/3`.
    ///
    /// Example:
    ///
    /// `ctx{intent:read, user:"did:example:123", object:"obj:42"}`
    ///
    ///   - If `motivated` is `false`, the contract calls `decide/2` and returns only the verdict.
    ///   - If `motivated` is `true`, the contract calls `decide/3` and returns both verdict and motivation.
    ///
    /// The returned `verdict` is an arbitrary Prolog term (atom or compound), for example:
    ///
    ///   - `gov:permitted`
    ///   - `gov:forbidden`
    ///   - `pay("did:...", 1000)`
    ///
    /// The optional `motivation` is an arbitrary Prolog term returned by the constitution and intended to
    /// justify the verdict (e.g. grounds/articles, findings, interpretation rules).
    #[returns(DecideResponse)]
    Decide { case: String, motivated: bool },
}

/// Response returned by `QueryMsg::Constitution`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    /// The stored constitution (raw Prolog program bytes).
    pub governance: Binary,
}

/// Response returned by `QueryMsg::ConstitutionStatus`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionStatusResponse {
    /// The constitution revision number.
    ///
    /// The initially instantiated constitution has revision `0`.
    /// Each successful revision increments it by `1`.
    pub constitution_revision: u64,
    /// The stored constitution hash (32 bytes, sha256).
    pub constitution_hash: Binary,
}

impl From<&ConstitutionStatus> for ConstitutionStatusResponse {
    fn from(status: &ConstitutionStatus) -> Self {
        Self {
            constitution_revision: status.constitution_revision(),
            constitution_hash: Binary::from(status.constitution_hash()),
        }
    }
}

impl From<&Constitution> for ConstitutionResponse {
    fn from(constitution: &Constitution) -> Self {
        Self {
            governance: constitution.bytes().clone(),
        }
    }
}

/// Response returned by `QueryMsg::Decide`.
#[cosmwasm_schema::cw_serde]
pub struct DecideResponse {
    /// The verdict returned by the constitution as a Prolog term string.
    pub verdict: String,
    /// Optional motivation term returned as the third argument of `decide/3`.
    pub motivation: Option<String>,
}
