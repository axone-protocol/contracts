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

### ExecuteMsg::foo

| parameter   | description                |
| ----------- | -------------------------- |
| `foo`       | _(Required.) _ **object**. |
| `foo.value` | _(Required.) _ **string**. |

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

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-vc.json` (`0818b58701a06f58`)_
