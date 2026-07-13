use crate::{contract::AxoneVc, domain::Uri};

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Binary, Timestamp};

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
    /// Issue a verifiable credential from this authority.
    ///
    /// Only the app authority is allowed to call this message.
    ///
    /// The submitted payload must match the declared `format` and must describe
    /// exactly one credential that satisfies the contract invariants.
    ///
    /// The credential is accepted only if it provides:
    /// - an identifier
    /// - either no issuer or an issuer equal to the authority DID exposed by this contract
    /// - an issuance date
    /// - a subject identifier
    /// - at least one type, including `VerifiableCredential`
    /// - optional `validFrom` and `validUntil` claims, when present, encoded as
    ///   `xsd:dateTimeStamp` instants with `validFrom < validUntil`
    ///
    /// The submitted payload may omit the issuer. In that case, the contract
    /// treats the credential as issued by its authority DID.
    ///
    /// Issuance fails if the payload format is not supported, if the credential
    /// representation cannot be interpreted according to that format, or if a
    /// credential with the same identifier has already been issued by this authority.
    IssueCredential {
        /// Serialized credential payload.
        ///
        /// The expected binary encoding and semantic representation are determined
        /// by the `format` field.
        credential: Binary,
        /// Encoding used by the submitted credential payload.
        ///
        /// Defaults to `n_quads` when omitted.
        #[serde(default)]
        format: Option<CredentialInputFormat>,
    },

    /// Revoke a verifiable credential from this authority.
    ///
    /// Only the app authority is allowed to call this message.
    ///
    /// The revocation is terminal, the same identifier cannot be issued again.
    ///
    /// Revocation fails if the identifier is unknown or already revoked.
    RevokeCredential {
        /// The credential identifier to revoke.
        identifier: String,
    },
}

/// Supported credential input encodings.
#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub enum CredentialInputFormat {
    /// UTF-8 RDF dataset serialized as N-Quads.
    ///
    /// N-Quads extends N-Triples to represent RDF datasets by allowing an
    /// optional fourth term that carries the graph name.
    /// See the [N-Quads specification](https://www.w3.org/TR/n-quads/).
    #[serde(rename = "n_quads")]
    #[default]
    NQuads,
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

    /// Check whether a credential is active and, optionally, valid at a given instant.
    ///
    /// Revoked credentials are not part of the active credential set and therefore
    /// return the same result as unknown identifiers.
    #[returns(VerifyCredentialResponse)]
    VerifyCredential {
        /// Identifier of the credential to check.
        identifier: Uri,
        /// Optional instant at which to evaluate the credential validity interval.
        ///
        /// When omitted, every active credential is considered valid.
        valid_at: Option<Timestamp>,
    },
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

/// Response returned by `AxoneVcQueryMsg::VerifyCredential`.
#[cosmwasm_schema::cw_serde]
pub struct VerifyCredentialResponse {
    /// Whether the identifier belongs to an active credential.
    pub exists: bool,
    /// Whether the active credential is valid for the requested instant.
    ///
    /// This is equal to `exists` when no instant was requested.
    pub valid: bool,
}
