use crate::contract::AxoneVc;

use cosmwasm_schema::QueryResponses;

abstract_app::app_msg_types!(AxoneVc, AxoneVcExecuteMsg, AxoneVcQueryMsg);

/// Instantiate message.
///
/// Instantiating this app attaches a verifiable credential authority to the resource
/// represented by the host Abstract Account.
///
/// This contract requires no caller-provided configuration.
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AxoneVcInstantiateMsg {}

/// Execute messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum AxoneVcExecuteMsg {
    Foo { value: String },
}

/// Migrate message.
///
/// Reserved for future migrations.
#[cosmwasm_schema::cw_serde]
pub struct AxoneVcMigrateMsg {}

/// Query messages.
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum AxoneVcQueryMsg {
    /// Return the DID of the credential authority attached to this contract.
    ///
    /// This identifier is the authority identity recognized by the contract for
    /// issuing and managing credentials on behalf of the attached resource.
    ///
    /// The returned DID uses the `did:pkh` method and is grounded in the
    /// on-chain address of the host Abstract Account, rendered as a
    /// CAIP-compatible canonical Cosmos Bech32 account address.
    ///
    /// Form:
    ///
    /// `did:pkh:cosmos:<chain_id>:cosmos1...`
    #[returns(AuthorityResponse)]
    Authority {},
}

/// Response returned by `AxoneVcQueryMsg::Authority`.
#[cosmwasm_schema::cw_serde]
pub struct AuthorityResponse {
    /// The authority DID recognized by this contract.
    ///
    /// This representation uses the `did:pkh` method over the on-chain
    /// address of the host Abstract Account, rendered as a CAIP-compatible
    /// canonical Cosmos Bech32 account address.
    ///
    /// Form:
    ///
    /// `did:pkh:cosmos:<chain_id>:cosmos1...`
    pub did: String,
}
