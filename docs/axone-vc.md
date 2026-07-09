# AXONE Verifiable Credential Contract (`axone-vc`)

The AXONE Verifiable Credential contract attaches **verifiable credential capabilities**
to a resource represented by an **Abstract Account (AA)**.

It is the AXONE contract responsible for binding a credential authority to that resource
and structuring the credential lifecycle around submission, revocation, suspension,
reinstatement and verification.

It targets credentials represented as RDF datasets.

## Authority

The contract exposes the identifier of the credential authority through the `Authority`
query.

This identifier is the DID of the resource bound to the host Abstract Account for this
VC capability.

The current representation uses the `did:pkh` method and is grounded in the on-chain
address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos
Bech32 account address.

Form:

`did:pkh:cosmos:<chain_id>:cosmos1...`

## InstantiateMsg

Instantiate message.

Instantiating this app attaches a verifiable credential authority to the resource represented by the host Abstract Account.

This contract requires no caller-provided configuration.

| variant        | description                                                                                                                                                                                                                                 |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| InstantiateMsg | **object**. Instantiate message.<br /><br />Instantiating this app attaches a verifiable credential authority to the resource represented by the host Abstract Account.<br /><br />This contract requires no caller-provided configuration. |

## ExecuteMsg

Execute messages.

### ExecuteMsg::issue_credential

Issue a verifiable credential from this authority.

Only the app authority is allowed to call this message.

The submitted payload must match the declared `format` and must describe exactly one credential that satisfies the contract invariants.

The credential is accepted only if it provides: - an identifier - either no issuer or an issuer equal to the authority DID exposed by this contract - an issuance date - a subject identifier - at least one type, including `VerifiableCredential`

The submitted payload may omit the issuer. In that case, the contract treats the credential as issued by its authority DID.

Issuance fails if the payload format is not supported, if the credential representation cannot be interpreted according to that format, or if a credential with the same identifier has already been issued by this authority.

| parameter                     | description                                                                                                                                                                    |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `issue_credential`            | _(Required.) _ **object**.                                                                                                                                                     |
| `issue_credential.credential` | _(Required.) _ **[Binary](#binary)**. Serialized credential payload.<br /><br />The expected binary encoding and semantic representation are determined by the `format` field. |
| `issue_credential.format`     | **[CredentialInputFormat](#credentialinputformat)\|null**. Encoding used by the submitted credential payload.<br /><br />Defaults to `n_quads` when omitted.                   |

### ExecuteMsg::revoke_credential

Revoke a verifiable credential from this authority.

Only the app authority is allowed to call this message.

The revocation is terminal, the same identifier cannot be issued again.

Revocation fails if the identifier is unknown or already revoked.

| parameter                      | description                                                     |
| ------------------------------ | --------------------------------------------------------------- |
| `revoke_credential`            | _(Required.) _ **object**.                                      |
| `revoke_credential.identifier` | _(Required.) _ **string**. The credential identifier to revoke. |

## QueryMsg

Query messages.

### QueryMsg::authority

Return the DID of the credential authority attached to this contract.

This identifier is the authority identity recognized by the contract for issuing and managing credentials on behalf of the attached resource.

The returned DID uses the `did:pkh` method and is grounded in the on-chain address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos Bech32 account address.

Form:

`did:pkh:cosmos:&lt;chain_id&gt;:cosmos1...`

| parameter   | description                |
| ----------- | -------------------------- |
| `authority` | _(Required.) _ **object**. |

## MigrateMsg

Migrate message.

Reserved for future migrations.

### MigrateMsg::MigrateMsg

Migrate message.

Reserved for future migrations.

| parameter | description |
| --------- | ----------- |

## Responses

### authority

Response returned by `AxoneVcQueryMsg::Authority`.

| property | description                                                                                                                                                                                                                                                                                                                                |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `did`    | _(Required.) _ **string**. The authority DID recognized by this contract.<br /><br />This representation uses the `did:pkh` method over the on-chain address of the host Abstract Account, rendered as a CAIP-compatible canonical Cosmos Bech32 account address.<br /><br />Form:<br /><br />`did:pkh:cosmos:&lt;chain_id&gt;:cosmos1...` |

## Definitions

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### CredentialInputFormat

Supported credential input encodings.

| variant   | description                                                                                                                                                                                                                                                      |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| undefined | **string**: `n_quads`. UTF-8 RDF dataset serialized as N-Quads.<br /><br />N-Quads extends N-Triples to represent RDF datasets by allowing an optional fourth term that carries the graph name. See the [N-Quads specification](https://www.w3.org/TR/n-quads/). |

### undefined

UTF-8 RDF dataset serialized as N-Quads.

N-Quads extends N-Triples to represent RDF datasets by allowing an optional fourth term that carries the graph name. See the [N-Quads specification](https://www.w3.org/TR/n-quads/).

| literal     |
| ----------- |
| `"n_quads"` |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-vc.json` (`a466d08b9b644044`)_
