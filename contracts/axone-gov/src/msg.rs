use crate::contract::AxoneGov;

use crate::domain::constitution::ConstitutionStatus;
use crate::domain::Constitution;
use crate::state::DecisionRecord;
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
///
///    - `decide/3` as `governance:decide(+Case, -Verdict, -Motivation)`
///
/// Where:
///
///    - `Case` is a Prolog dict term (typically `ctx{...}`) representing the decision context.
///      It can include any key-value facts required by the constitution (e.g. intent, actor, subject).
///
///    - `Verdict` is an arbitrary Prolog term (atom or compound) representing the decision outcome.
///
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
    /// Record a decision on-chain by deciding a case using the stored constitution.
    ///
    /// The `case` parameter is a Prolog dict term string (typically `ctx{...}`) representing
    /// the decision context provided by the caller.
    ///
    /// Before evaluation, the contract enriches the case with contract-derived facts:
    ///
    /// ```prolog
    /// ctx{
    ///   'gov:module': module{ id: <atom>, version: <atom> },
    ///   'gov:cosmwasm': cosmwasm{
    ///     message: message{
    ///       sender: <atom>,                    % Bech32 address of message sender
    ///       funds: [coin(Amount, Denom), ...]  % List of coins sent with message
    ///     },
    ///     block: block{
    ///       height: <integer>,        % Block height
    ///       time: <integer>,          % Block timestamp (seconds since epoch)
    ///       tx_index: <integer>       % Transaction index (optional)
    ///     }
    ///   },
    ///   <caller_provided_keys>: <caller_provided_values>
    /// }
    /// ```
    ///
    /// Injected keys are authoritative and overwrite any caller-provided value under the same keys.
    ///
    /// The contract evaluates `governance:decide/2` or `governance:decide/3` depending on
    /// `motivated`, and records the resulting verdict (and optional motivation) as a durable
    /// decision record.
    RecordDecision {
        /// The decision context.
        case: String,
        /// Whether to request a motivated decision (defaults to `false`).
        ///
        ///   - If `false`, the contract calls `governance:decide/2` and records only the verdict.
        ///   - If `true`, the contract calls `governance:decide/3` and records both verdict and motivation.
        motivated: Option<bool>,
    },
    /// Propose a constitutional revision (constitutional amendment).
    ///
    /// The contract asks the **current** constitution to decide whether the revision is allowed by
    /// evaluating a case that includes the intent `gov:revise_constitution`.
    ///
    /// The complete case structure is (keys containing `:` are quoted atoms):
    ///
    /// ```prolog
    /// ctx{
    ///   intent: 'gov:revise_constitution',
    ///   'gov:proposed_constitution_hash': <atom>,        % Hex string atom (authoritative SHA256 of payload)
    ///   'gov:module': module{
    ///     id: <atom>,       % Contract module ID (e.g., 'axone:axone-gov')
    ///     version: <atom>   % Contract version (e.g., '1.2.3')
    ///   },
    ///   'gov:cosmwasm': cosmwasm{
    ///     message: message{
    ///       sender: <atom>,                    % Bech32 address of message sender
    ///       funds: [coin(Amount, Denom), ...]  % List of coins sent with message
    ///     },
    ///     block: block{
    ///       height: <integer>,        % Block height
    ///       time: <integer>,          % Block timestamp (seconds since epoch)
    ///       tx_index: <integer>       % Transaction index
    ///     }
    ///   },
    ///   <caller_provided_keys>: <caller_provided_values>  % Any additional keys from caller's case
    /// }
    /// ```
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
    /// The returned `verdict` is an arbitrary Prolog term (atom or compound), for example:
    ///
    ///   - `gov:permitted`
    ///   - `gov:forbidden`
    ///   - `pay("did:...", 1000)`
    ///
    /// The optional `motivation` is an arbitrary Prolog term returned by the constitution and intended to
    /// justify the verdict (e.g. grounds/articles, findings, interpretation rules).
    ///
    /// Before evaluation, the contract enriches the case with module metadata (`'gov:module'`).
    ///
    /// Injected keys are authoritative and overwrite any caller-provided value under the same keys.
    ///
    #[returns(DecideResponse)]
    Decide {
        /// The decision context.
        case: String,
        /// Whether to request a motivated decision (defaults to `false`).
        ///
        ///   - If `false`, the contract calls `governance:decide/2` and returns only the verdict.
        ///   - If `true`, the contract calls `governance:decide/3` and returns both verdict and motivation.
        motivated: Option<bool>,
    },

    /// Return a recorded decision by its unique identifier.
    ///
    /// The returned record is created by `ExecuteMsg::RecordDecision` and includes the decision payload
    /// (case/verdict, optional motivation) along with constitution metadata (revision/hash) and block metadata.
    #[returns(DecisionResponse)]
    Decision {
        /// The unique decision identifier.
        decision_id: u64,
    },

    /// Return a paginated list of recorded decisions.
    ///
    /// Decisions are ordered by their unique identifier in ascending order.
    #[returns(DecisionsResponse)]
    Decisions {
        /// Optional decision ID to start after (exclusive).
        start_after: Option<u64>,
        /// Optional maximum number of decisions to return (default: 10).
        limit: Option<u32>,
    },
}

/// Response returned by `QueryMsg::Constitution`.
#[cosmwasm_schema::cw_serde]
pub struct ConstitutionResponse {
    /// The stored constitution (raw Prolog program bytes).
    pub constitution: Binary,
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
            constitution: constitution.bytes().clone(),
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

/// Response returned by `QueryMsg::Decision`.
#[cosmwasm_schema::cw_serde]
pub struct DecisionResponse {
    /// The unique decision identifier.
    pub decision_id: u64,
    /// The constitution revision number at the time of decision.
    pub constitution_revision: u64,
    /// The constitution hash at the time of decision (32 bytes, sha256).
    pub constitution_hash: Binary,
    /// The case term as a Prolog term string.
    pub case: String,
    /// The case hash (32 bytes, sha256).
    pub case_hash: Binary,
    /// The verdict term as a Prolog term string.
    pub verdict: String,
    /// The verdict hash (32 bytes, sha256).
    pub verdict_hash: Binary,
    /// Optional motivation term as a Prolog term string.
    pub motivation: Option<String>,
    /// The motivation hash (32 bytes, sha256).
    pub motivation_hash: Option<Binary>,
    /// The author Bech32 address.
    pub author: String,
    /// The block height at which the decision was recorded.
    pub block_height: u64,
    /// The block time (seconds since epoch) at which the decision was recorded.
    pub block_time_seconds: u64,
}

impl From<&DecisionRecord> for DecisionResponse {
    fn from(value: &DecisionRecord) -> Self {
        Self {
            decision_id: value.id(),
            constitution_revision: value.constitution_revision(),
            constitution_hash: Binary::from(value.constitution_hash()),
            case: value.case().clone(),
            case_hash: Binary::from(value.case_hash()),
            verdict: value.verdict().clone(),
            verdict_hash: Binary::from(value.verdict_hash()),
            motivation: value.motivation().clone(),
            motivation_hash: value.motivation_hash().map(Binary::from),
            author: value.author().to_string(),
            block_height: value.block_height(),
            block_time_seconds: value.block_time_seconds(),
        }
    }
}

impl From<DecisionRecord> for DecisionResponse {
    fn from(r: DecisionRecord) -> Self {
        Self::from(&r)
    }
}

/// Response returned by `QueryMsg::Decisions`.
#[cosmwasm_schema::cw_serde]
pub struct DecisionsResponse {
    pub decisions: Vec<DecisionResponse>,
}
