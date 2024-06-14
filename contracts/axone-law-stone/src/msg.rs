#[allow(unused_imports)]
use axone_logic_bindings::AskResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

/// Instantiate message
#[cw_serde]
pub struct InstantiateMsg {
    /// The Prolog program carrying law rules and facts.
    pub program: Binary,

    /// The `axone-objectarium` contract address on which to store the law program.
    pub storage_address: String,
}

/// Execute messages
#[cw_serde]
pub enum ExecuteMsg {
    /// # BreakStone
    /// Break the stone making this contract unusable, by clearing all the related resources:
    /// - Unpin all the pinned objects on `axone-objectarium` contracts, if any.
    /// - Forget the main program (i.e. or at least unpin it).
    ///
    /// Only the creator address (the address that instantiated the contract) is authorized to invoke
    /// this message.
    /// If already broken, this is a no-op.
    BreakStone {},
}

/// Query messages
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Ask
    /// Submits a Prolog query string to the `Logic` module, evaluating it against the
    /// law program associated with this contract.
    ///
    /// If the law stone is broken the query returns a response with the error `error(system_error(broken_law_stone),root)`
    /// set in the `answer` field.
    #[returns(AskResponse)]
    Ask { query: String },

    /// # Program
    /// Retrieves the location metadata of the law program bound to this contract.
    ///
    /// This includes the contract address of the `objectarium` and the program object ID,
    /// where the law program's code can be accessed.
    #[returns(ProgramResponse)]
    Program {},

    /// # ProgramCode
    /// Fetches the raw code of the law program tied to this contract.
    ///
    /// If the law stone is broken, the query may fail if the program is no longer available in the
    /// `Objectarium`.
    #[returns(Binary)]
    ProgramCode {},
}

/// # ProgramResponse
/// ProgramResponse carry elements to locate the program in a `axone-objectarium` contract.
#[cw_serde]
pub struct ProgramResponse {
    /// The program object id in the `axone-objectarium` contract.
    pub object_id: String,

    /// The `axone-objectarium` contract address on which the law program is stored.
    pub storage_address: String,
}
